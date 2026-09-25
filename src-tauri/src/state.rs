use std::sync::Arc;

use mediapulse_core::backend::{
    BackendError, PlaybackBackend, PlaybackCommand, PlaybackSnapshot, PropertyUpdate, PropertyValue,
};

#[cfg(feature = "libmpv")]
use mediapulse_core::backend::LibmpvBackend;

pub struct AppState {
    backend: Arc<dyn PlaybackBackend>,
}

impl AppState {
    pub fn new(
        window_id: Option<i64>,
        _sidecar: Option<std::path::PathBuf>,
    ) -> Result<Self, BackendError> {
        #[cfg(feature = "libmpv")]
        let backend = match window_id {
            Some(window_id) => LibmpvBackend::new_with_window_id(window_id)?,
            None => LibmpvBackend::new()?,
        };

        #[cfg(all(not(feature = "libmpv"), feature = "sidecar"))]
        let backend = {
            let sidecar = _sidecar.ok_or(BackendError::NotReady)?;
            mediapulse_core::backend::SidecarBackend::new(sidecar)?
        };

        Ok(Self {
            backend: Arc::new(backend),
        })
    }

    #[cfg(test)]
    pub(crate) fn with_backend(backend: Arc<dyn PlaybackBackend>) -> Self {
        Self { backend }
    }

    pub fn backend_kind(&self) -> mediapulse_core::backend::BackendKind {
        self.backend.backend_kind()
    }

    pub fn load(&self, source: &str) -> Result<(), BackendError> {
        self.backend.load(source)
    }

    pub fn command(&self, command: PlaybackCommand) -> Result<(), BackendError> {
        self.backend.command(command)
    }

    pub fn set_property(&self, update: PropertyUpdate) -> Result<(), BackendError> {
        self.backend.set_property(update)
    }

    pub fn get_property(&self, name: &str) -> Result<PropertyValue, BackendError> {
        self.backend.get_property(name)
    }

    pub fn snapshot(&self) -> Result<PlaybackSnapshot, BackendError> {
        self.backend.snapshot()
    }
}
