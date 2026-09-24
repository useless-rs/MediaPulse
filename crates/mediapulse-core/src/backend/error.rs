use thiserror::Error;

use super::model::BackendKind;

/// Typed failure raised by a playback adapter.
#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum BackendError {
    #[error("playback engine is not ready")]
    NotReady,
    #[error("{backend:?} backend does not support this operation")]
    Unsupported { backend: BackendKind },
    #[error("invalid value for mpv property {property}: {message}")]
    InvalidProperty { property: String, message: String },
    #[error("mpv I/O operation {operation} failed: {message}")]
    Io { operation: String, message: String },
    #[error("mpv operation timed out: {operation}")]
    Timeout { operation: String },
    #[error("invalid mpv IPC response: {message}")]
    Protocol { message: String },
    #[error("mpv operation failed: {message}")]
    Engine { message: String },
}
