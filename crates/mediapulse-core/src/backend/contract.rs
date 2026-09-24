use super::error::BackendError;
use super::model::{BackendKind, PlaybackCommand, PlaybackSnapshot, PropertyUpdate, PropertyValue};

/// Stable product-facing playback operations independent of mpv integration.
pub trait PlaybackBackend: Send + Sync {
    fn backend_kind(&self) -> BackendKind;

    /// Load or replace the active media source.
    ///
    /// # Errors
    ///
    /// Returns [`BackendError`] when the engine rejects the source.
    fn load(&self, source: &str) -> Result<(), BackendError>;

    /// Execute one pre-split mpv command.
    ///
    /// # Errors
    ///
    /// Returns [`BackendError`] when the engine rejects the command.
    fn command(&self, command: PlaybackCommand) -> Result<(), BackendError>;

    /// Update one typed mpv property.
    ///
    /// # Errors
    ///
    /// Returns [`BackendError`] when the property or value is invalid.
    fn set_property(&self, update: PropertyUpdate) -> Result<(), BackendError>;

    /// Read one typed mpv property.
    ///
    /// # Errors
    ///
    /// Returns [`BackendError`] when the property is missing or unreadable.
    fn get_property(&self, name: &str) -> Result<PropertyValue, BackendError>;

    /// Read the complete observable playback state.
    ///
    /// # Errors
    ///
    /// Returns [`BackendError`] when required state cannot be read.
    fn snapshot(&self) -> Result<PlaybackSnapshot, BackendError>;

    /// Stop and release the playback engine.
    ///
    /// # Errors
    ///
    /// Returns [`BackendError`] when shutdown fails.
    fn shutdown(&self) -> Result<(), BackendError>;
}
