mod endpoint;
mod transport;

use std::io::{BufWriter, Write};
use std::path::Path;
use std::process::{Child, Command, Stdio};
use std::sync::mpsc::{Receiver, RecvTimeoutError};
use std::thread;
use std::time::{Duration, Instant};

use serde_json::{Value, json};

use super::error::BackendError;
use endpoint::Endpoint;
use transport::{IpcTransport, spawn_reader};

const STARTUP_TIMEOUT: Duration = Duration::from_secs(5);
const RESPONSE_TIMEOUT: Duration = Duration::from_secs(5);

pub(super) struct IpcSession {
    child: Child,
    writer: BufWriter<Box<dyn IpcTransport>>,
    responses: Receiver<Value>,
    next_request_id: u64,
    endpoint: Endpoint,
    closed: bool,
}

impl IpcSession {
    pub(super) fn spawn(executable: &Path) -> Result<Self, BackendError> {
        let endpoint = Endpoint::new();
        let endpoint_arg = format!("--input-ipc-server={}", endpoint.display());
        let mut child = Command::new(executable)
            .args([
                "--idle=yes",
                "--no-terminal",
                "--force-window=no",
                "--hwdec=auto-safe",
                endpoint_arg.as_str(),
            ])
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::inherit())
            .spawn()
            .map_err(|error| io_error("spawn mpv sidecar", &error))?;

        if let Err(error) = wait_until_listening(&mut child, &endpoint) {
            terminate(&mut child);
            endpoint.cleanup();
            return Err(error);
        }
        let writer = endpoint.connect().map_err(|error| {
            terminate(&mut child);
            io_error("connect mpv IPC writer", &error)
        })?;
        let reader = endpoint.connect().map_err(|error| {
            terminate(&mut child);
            io_error("connect mpv IPC reader", &error)
        })?;

        Ok(Self {
            child,
            writer: BufWriter::new(writer),
            responses: spawn_reader(reader),
            next_request_id: 1,
            endpoint,
            closed: false,
        })
    }

    pub(super) fn request(&mut self, command: &[Value]) -> Result<Option<Value>, BackendError> {
        if self.closed {
            return Err(BackendError::NotReady);
        }
        let request_id = self.next_request_id;
        self.next_request_id = self.next_request_id.wrapping_add(1).max(1);
        self.write_request(request_id, command)?;

        loop {
            let response = self
                .responses
                .recv_timeout(RESPONSE_TIMEOUT)
                .map_err(|error| recv_error("receive mpv IPC response", request_id, error))?;
            if response.get("request_id").and_then(Value::as_u64) == Some(request_id) {
                return decode_response(request_id, &response);
            }
        }
    }

    pub(super) fn shutdown(&mut self) -> Result<(), BackendError> {
        if self.closed {
            return Ok(());
        }
        let request_id = self.next_request_id;
        let write_result = self.write_request(request_id, &[json!("quit")]);
        let status = self
            .child
            .wait()
            .map_err(|error| io_error("wait for mpv sidecar", &error))?;
        self.closed = true;
        self.endpoint.cleanup();
        write_result?;
        if status.success() {
            Ok(())
        } else {
            Err(BackendError::Engine {
                message: format!("mpv sidecar exited with {status}"),
            })
        }
    }

    fn write_request(&mut self, request_id: u64, command: &[Value]) -> Result<(), BackendError> {
        let payload = json!({ "command": command, "request_id": request_id });
        let encoded = serde_json::to_string(&payload).map_err(|error| BackendError::Protocol {
            message: error.to_string(),
        })?;
        self.writer
            .write_all(encoded.as_bytes())
            .and_then(|()| self.writer.write_all(b"\n"))
            .and_then(|()| self.writer.flush())
            .map_err(|error| io_error("write mpv IPC request", &error))
    }
}

impl Drop for IpcSession {
    fn drop(&mut self) {
        if !self.closed {
            terminate(&mut self.child);
        }
        self.endpoint.cleanup();
    }
}

fn wait_until_listening(child: &mut Child, endpoint: &Endpoint) -> Result<(), BackendError> {
    let deadline = Instant::now() + STARTUP_TIMEOUT;
    while Instant::now() < deadline {
        if let Some(status) = child
            .try_wait()
            .map_err(|error| io_error("poll mpv sidecar", &error))?
        {
            return Err(BackendError::Engine {
                message: format!("mpv sidecar exited before IPC was ready: {status}"),
            });
        }
        if endpoint.is_available() {
            return Ok(());
        }
        thread::sleep(Duration::from_millis(10));
    }
    Err(BackendError::Timeout {
        operation: "start mpv IPC endpoint".to_string(),
    })
}

fn terminate(child: &mut Child) {
    if let Err(error) = child.kill() {
        eprintln!("failed to terminate mpv sidecar: {error}");
    }
    if let Err(error) = child.wait() {
        eprintln!("failed to reap mpv sidecar: {error}");
    }
}

fn decode_response(request_id: u64, response: &Value) -> Result<Option<Value>, BackendError> {
    let error = response
        .get("error")
        .and_then(Value::as_str)
        .unwrap_or("success");
    if error != "success" {
        return Err(BackendError::Engine {
            message: format!("mpv request {request_id} failed: {error}"),
        });
    }
    Ok(response.get("data").cloned())
}

fn recv_error(operation: &str, request_id: u64, error: RecvTimeoutError) -> BackendError {
    match error {
        RecvTimeoutError::Timeout => BackendError::Timeout {
            operation: format!("{operation} {request_id}"),
        },
        RecvTimeoutError::Disconnected => BackendError::Engine {
            message: format!("mpv IPC closed before response {request_id}"),
        },
    }
}

fn io_error(operation: &str, error: &std::io::Error) -> BackendError {
    BackendError::Io {
        operation: operation.to_string(),
        message: error.to_string(),
    }
}
