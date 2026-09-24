use std::io::{BufRead, BufReader, Read, Write};
use std::path::Path;
use std::sync::mpsc::{self, Receiver};
use std::thread;

use serde_json::Value;

pub(super) trait IpcTransport: Read + Write + Send {}
impl<T> IpcTransport for T where T: Read + Write + Send {}

#[cfg(unix)]
pub(super) fn connect_endpoint(path: &Path) -> std::io::Result<Box<dyn IpcTransport>> {
    Ok(Box::new(std::os::unix::net::UnixStream::connect(path)?))
}

#[cfg(windows)]
pub(super) fn connect_endpoint(path: &Path) -> std::io::Result<Box<dyn IpcTransport>> {
    Ok(Box::new(
        std::fs::OpenOptions::new()
            .read(true)
            .write(true)
            .open(path)?,
    ))
}

pub(super) fn spawn_reader(transport: Box<dyn IpcTransport>) -> Receiver<Value> {
    let (sender, receiver) = mpsc::channel();
    thread::spawn(move || {
        let mut reader = BufReader::new(transport);
        let mut line = String::new();
        loop {
            line.clear();
            match reader.read_line(&mut line) {
                Ok(0) => break,
                Err(error) => {
                    eprintln!("mpv IPC reader failed: {error}");
                    break;
                }
                Ok(_) => match serde_json::from_str::<Value>(line.trim()) {
                    Ok(value) => {
                        if sender.send(value).is_err() {
                            break;
                        }
                    }
                    Err(error) => eprintln!("ignored invalid mpv IPC message: {error}"),
                },
            }
        }
    });
    receiver
}
