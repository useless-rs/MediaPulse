mod contract;
mod error;
mod model;

#[cfg(feature = "libmpv")]
mod libmpv;

#[cfg(feature = "sidecar")]
mod ipc;
#[cfg(feature = "sidecar")]
mod sidecar;

pub use contract::PlaybackBackend;
pub use error::BackendError;
pub use model::{
    BackendKind, PlaybackCommand, PlaybackPhase, PlaybackSnapshot, PropertyUpdate, PropertyValue,
};

#[cfg(feature = "libmpv")]
pub use libmpv::LibmpvBackend;
#[cfg(feature = "sidecar")]
pub use sidecar::SidecarBackend;

#[cfg(feature = "libmpv")]
use std::sync::Arc;

/// Construct the default built-in engine backend.
///
/// # Errors
///
/// Returns [`BackendError`] when the default backend cannot initialize.
#[cfg(feature = "libmpv")]
pub fn default_backend() -> Result<Arc<dyn PlaybackBackend>, BackendError> {
    Ok(Arc::new(LibmpvBackend::new()?))
}
