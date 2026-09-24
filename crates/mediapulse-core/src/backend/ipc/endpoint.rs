use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};

use super::transport::{IpcTransport, connect_endpoint};

static ENDPOINT_SEQUENCE: AtomicU64 = AtomicU64::new(1);

pub(super) struct Endpoint {
    path: PathBuf,
}

impl Endpoint {
    pub(super) fn new() -> Self {
        let sequence = ENDPOINT_SEQUENCE.fetch_add(1, Ordering::Relaxed);
        let name = format!("mediapulse-mpv-{}-{sequence}", std::process::id());
        #[cfg(unix)]
        let path = std::env::temp_dir().join(name);
        #[cfg(windows)]
        let path = PathBuf::from(format!(r"\\.\pipe\{name}"));
        Self { path }
    }

    pub(super) fn display(&self) -> String {
        self.path.to_string_lossy().into_owned()
    }

    pub(super) fn is_available(&self) -> bool {
        #[cfg(unix)]
        {
            self.path.exists()
        }
        #[cfg(windows)]
        {
            true
        }
    }

    pub(super) fn connect(&self) -> std::io::Result<Box<dyn IpcTransport>> {
        connect_endpoint(&self.path)
    }

    pub(super) fn cleanup(&self) {
        #[cfg(unix)]
        if self.path.exists() {
            match std::fs::remove_file(&self.path) {
                Ok(()) => {}
                Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
                Err(error) => eprintln!("failed to remove mpv IPC endpoint: {error}"),
            }
        }
    }
}
