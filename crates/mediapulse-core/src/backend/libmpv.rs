use std::sync::Arc;

use rsmpv::Mpv;

use super::contract::PlaybackBackend;
use super::error::BackendError;
use super::model::{
    BackendKind, PlaybackCommand, PlaybackPhase, PlaybackSnapshot, PropertyUpdate, PropertyValue,
};

/// Direct libmpv playback adapter.
#[derive(Clone)]
pub struct LibmpvBackend {
    player: Arc<Mpv>,
}

impl LibmpvBackend {
    /// Create and initialize the direct libmpv backend.
    ///
    /// # Errors
    ///
    /// Returns [`BackendError`] when libmpv is unavailable or rejects options.
    pub fn new() -> Result<Self, BackendError> {
        Self::build(None)
    }

    /// Create libmpv with a native window for direct embedded output.
    ///
    /// # Errors
    ///
    /// Returns [`BackendError`] when libmpv rejects the window or options.
    pub fn new_with_window_id(window_id: i64) -> Result<Self, BackendError> {
        Self::build(Some(window_id))
    }

    fn build(window_id: Option<i64>) -> Result<Self, BackendError> {
        let mut builder = Mpv::builder().map_err(engine_error)?;
        builder = builder.set_property("vo", "libmpv").map_err(engine_error)?;
        builder = builder
            .set_property("hwdec", "auto-safe")
            .map_err(engine_error)?;
        builder = builder.set_property("config", "no").map_err(engine_error)?;
        builder = builder
            .set_property("keep-open", "yes")
            .map_err(engine_error)?;
        builder = builder.set_property("idle", "yes").map_err(engine_error)?;
        let builder = match window_id {
            Some(window_id) => builder
                .set_property("wid", window_id)
                .map_err(engine_error)?,
            None => builder,
        };
        let player = builder.build().map_err(engine_error)?;
        Ok(Self {
            player: Arc::new(player),
        })
    }

    /// Shared handle used by the platform renderer.
    #[must_use]
    pub fn player(&self) -> Arc<Mpv> {
        Arc::clone(&self.player)
    }
}

impl PlaybackBackend for LibmpvBackend {
    fn backend_kind(&self) -> BackendKind {
        BackendKind::Libmpv
    }

    fn load(&self, source: &str) -> Result<(), BackendError> {
        self.player
            .command(&["loadfile", source, "replace"])
            .map_err(engine_error)
    }

    fn command(&self, command: PlaybackCommand) -> Result<(), BackendError> {
        let mut arguments = Vec::with_capacity(command.arguments.len() + 1);
        arguments.push(command.name.as_str());
        arguments.extend(command.arguments.iter().map(String::as_str));
        self.player.command(&arguments).map_err(engine_error)
    }

    fn set_property(&self, update: PropertyUpdate) -> Result<(), BackendError> {
        let result = match update.value {
            PropertyValue::Boolean(value) => self.player.set_property(&update.name, value),
            PropertyValue::Number(value) => self.player.set_property(&update.name, value),
            PropertyValue::Text(value) => self.player.set_property(&update.name, value),
            PropertyValue::Unit => self.player.del_property(&update.name),
        };
        result.map_err(|error| BackendError::InvalidProperty {
            property: update.name,
            message: error.to_string(),
        })
    }

    fn get_property(&self, name: &str) -> Result<PropertyValue, BackendError> {
        match name {
            "pause" | "fullscreen" | "mute" | "core-idle" => self
                .player
                .get_property::<bool>(name)
                .map(PropertyValue::Boolean)
                .map_err(|error| property_error(name, error)),
            "volume" | "speed" | "time-pos" | "duration" => self
                .player
                .get_property::<f64>(name)
                .map(PropertyValue::Number)
                .map_err(|error| property_error(name, error)),
            _ => self
                .player
                .get_property::<String>(name)
                .map(PropertyValue::Text)
                .map_err(|error| property_error(name, error)),
        }
    }

    fn snapshot(&self) -> Result<PlaybackSnapshot, BackendError> {
        let paused = read_bool(&self.player, "pause")?;
        let volume = read_number(&self.player, "volume")?;
        let muted = read_bool(&self.player, "mute")?;
        let fullscreen = read_bool(&self.player, "fullscreen")?;
        let speed = read_number(&self.player, "speed")?;
        let time_position = optional_number(&self.player, "time-pos").unwrap_or(0.0);
        let duration = optional_number(&self.player, "duration");
        let filename = optional_text(&self.player, "filename").unwrap_or_default();
        let media_title = optional_text(&self.player, "media-title").unwrap_or_default();
        let idle = read_bool(&self.player, "core-idle")?;
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
        self.player.command(&["quit"]).map_err(engine_error)
    }
}

fn read_bool(player: &Mpv, name: &str) -> Result<bool, BackendError> {
    player
        .get_property::<bool>(name)
        .map_err(|error| property_error(name, error))
}

fn read_number(player: &Mpv, name: &str) -> Result<f64, BackendError> {
    player
        .get_property::<f64>(name)
        .map_err(|error| property_error(name, error))
}

fn optional_number(player: &Mpv, name: &str) -> Option<f64> {
    player.get_property::<f64>(name).ok()
}

fn optional_text(player: &Mpv, name: &str) -> Option<String> {
    player.get_property::<String>(name).ok()
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

fn property_error(name: &str, error: rsmpv::Error) -> BackendError {
    BackendError::InvalidProperty {
        property: name.to_string(),
        message: error.to_string(),
    }
}

fn engine_error(error: rsmpv::Error) -> BackendError {
    BackendError::Engine {
        message: error.to_string(),
    }
}
