use std::path::PathBuf;
use std::sync::{Arc, Mutex, MutexGuard};

use serde_json::{Value, json};

use super::contract::PlaybackBackend;
use super::error::BackendError;
use super::ipc::IpcSession;
use super::model::{
    BackendKind, PlaybackCommand, PlaybackPhase, PlaybackSnapshot, PropertyUpdate, PropertyValue,
};

/// JSON IPC adapter for a privately bundled mpv process.
#[derive(Clone)]
pub struct SidecarBackend {
    session: Arc<Mutex<IpcSession>>,
}

impl SidecarBackend {
    /// Start a private mpv process and attach its JSON IPC endpoint.
    ///
    /// # Errors
    ///
    /// Returns [`BackendError`] when the process or IPC endpoint cannot start.
    pub fn new(executable: impl Into<PathBuf>) -> Result<Self, BackendError> {
        let executable = executable.into();
        let session = IpcSession::spawn(&executable)?;
        Ok(Self {
            session: Arc::new(Mutex::new(session)),
        })
    }

    fn session(&self) -> MutexGuard<'_, IpcSession> {
        self.session
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
    }
}

impl PlaybackBackend for SidecarBackend {
    fn backend_kind(&self) -> BackendKind {
        BackendKind::Sidecar
    }

    fn load(&self, source: &str) -> Result<(), BackendError> {
        self.command(PlaybackCommand::new(
            "loadfile",
            vec![source.to_string(), "replace".to_string()],
        ))
    }

    fn command(&self, command: PlaybackCommand) -> Result<(), BackendError> {
        let mut values = Vec::with_capacity(command.arguments.len() + 1);
        values.push(Value::String(command.name));
        values.extend(command.arguments.into_iter().map(Value::String));
        self.session().request(&values)?;
        Ok(())
    }

    fn set_property(&self, update: PropertyUpdate) -> Result<(), BackendError> {
        self.session().request(&[
            json!("set_property"),
            json!(update.name),
            property_to_json(update.value),
        ])?;
        Ok(())
    }

    fn get_property(&self, name: &str) -> Result<PropertyValue, BackendError> {
        let value = self
            .session()
            .request(&[json!("get_property"), json!(name)])?;
        Ok(value.map_or(PropertyValue::Unit, json_to_property))
    }

    fn snapshot(&self) -> Result<PlaybackSnapshot, BackendError> {
        let mut session = self.session();
        let paused = read_bool(&mut session, "pause")?;
        let volume = read_number(&mut session, "volume")?;
        let muted = read_bool(&mut session, "mute")?;
        let fullscreen = read_bool(&mut session, "fullscreen")?;
        let speed = read_number(&mut session, "speed")?;
        let time_position = read_optional_number(&mut session, "time-pos")?.unwrap_or(0.0);
        let duration = read_optional_number(&mut session, "duration")?;
        let filename = read_optional_text(&mut session, "filename")?.unwrap_or_default();
        let media_title = read_optional_text(&mut session, "media-title")?.unwrap_or_default();
        let idle = read_bool(&mut session, "core-idle")?;
        let phase = playback_phase(idle, paused);

        Ok(PlaybackSnapshot {
            phase,
            paused,
            volume,
            muted,
            fullscreen,
            speed,
            time_position,
            duration,
            filename,
            media_title,
        })
    }

    fn shutdown(&self) -> Result<(), BackendError> {
        self.session().shutdown()
    }
}

fn property_to_json(value: PropertyValue) -> Value {
    match value {
        PropertyValue::Boolean(value) => json!(value),
        PropertyValue::Number(value) => json!(value),
        PropertyValue::Text(value) => json!(value),
        PropertyValue::Unit => Value::Null,
    }
}

fn json_to_property(value: Value) -> PropertyValue {
    match value {
        Value::Bool(value) => PropertyValue::Boolean(value),
        Value::Number(value) => value
            .as_f64()
            .map_or(PropertyValue::Unit, PropertyValue::Number),
        Value::String(value) => PropertyValue::Text(value),
        _ => PropertyValue::Unit,
    }
}

fn read_bool(session: &mut IpcSession, name: &str) -> Result<bool, BackendError> {
    let value = session.request(&[json!("get_property"), json!(name)])?;
    match value.map(json_to_property) {
        Some(PropertyValue::Boolean(value)) => Ok(value),
        _ => Err(property_type_error(name, "boolean")),
    }
}

fn read_number(session: &mut IpcSession, name: &str) -> Result<f64, BackendError> {
    let value = session.request(&[json!("get_property"), json!(name)])?;
    match value.map(json_to_property) {
        Some(PropertyValue::Number(value)) => Ok(value),
        Some(PropertyValue::Text(value)) => value
            .parse::<f64>()
            .map_err(|_| property_type_error(name, "number")),
        _ => Err(property_type_error(name, "number")),
    }
}

fn read_optional_number(session: &mut IpcSession, name: &str) -> Result<Option<f64>, BackendError> {
    match read_number(session, name) {
        Ok(value) => Ok(Some(value)),
        Err(error) if is_unavailable(&error) => Ok(None),
        Err(error) => Err(error),
    }
}

fn read_optional_text(
    session: &mut IpcSession,
    name: &str,
) -> Result<Option<String>, BackendError> {
    let value = match session.request(&[json!("get_property"), json!(name)]) {
        Ok(value) => value,
        Err(error) if is_unavailable(&error) => return Ok(None),
        Err(error) => return Err(error),
    };
    match value.map(json_to_property) {
        Some(PropertyValue::Text(value)) => Ok(Some(value)),
        _ => Ok(None),
    }
}

fn is_unavailable(error: &BackendError) -> bool {
    matches!(error, BackendError::Engine { message } if message.contains("unavailable"))
}

fn property_type_error(name: &str, expected: &str) -> BackendError {
    BackendError::InvalidProperty {
        property: name.to_string(),
        message: format!("expected {expected}"),
    }
}

const fn playback_phase(idle: bool, paused: bool) -> PlaybackPhase {
    if paused {
        PlaybackPhase::Paused
    } else if idle {
        PlaybackPhase::Idle
    } else {
        PlaybackPhase::Playing
    }
}
