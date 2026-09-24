use serde::{Deserialize, Serialize};

/// Identifies the active playback implementation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum BackendKind {
    Libmpv,
    Sidecar,
}

/// Typed value exchanged with mpv properties.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", content = "value", rename_all = "camelCase")]
pub enum PropertyValue {
    Boolean(bool),
    Number(f64),
    Text(String),
    Unit,
}

/// Property name and value update.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PropertyUpdate {
    pub name: String,
    pub value: PropertyValue,
}

impl PropertyUpdate {
    #[must_use]
    pub fn boolean(name: impl Into<String>, value: bool) -> Self {
        Self {
            name: name.into(),
            value: PropertyValue::Boolean(value),
        }
    }

    #[must_use]
    pub fn number(name: impl Into<String>, value: f64) -> Self {
        Self {
            name: name.into(),
            value: PropertyValue::Number(value),
        }
    }

    #[must_use]
    pub fn text(name: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            value: PropertyValue::Text(value.into()),
        }
    }
}

/// Pre-split mpv command arguments.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaybackCommand {
    pub name: String,
    pub arguments: Vec<String>,
}

impl PlaybackCommand {
    #[must_use]
    pub fn new(name: impl Into<String>, arguments: Vec<String>) -> Self {
        Self {
            name: name.into(),
            arguments,
        }
    }
}

/// High-level state suitable for the desktop shell.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", tag = "kind", content = "message")]
pub enum PlaybackPhase {
    Idle,
    Playing,
    Paused,
    Ended,
    Error(String),
}

/// Complete observable playback state.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaybackSnapshot {
    pub phase: PlaybackPhase,
    pub paused: bool,
    pub volume: f64,
    pub muted: bool,
    pub fullscreen: bool,
    pub speed: f64,
    pub time_position: f64,
    pub duration: Option<f64>,
    pub filename: String,
    pub media_title: String,
}

impl Default for PlaybackSnapshot {
    fn default() -> Self {
        Self {
            phase: PlaybackPhase::Idle,
            paused: false,
            volume: 100.0,
            muted: false,
            fullscreen: false,
            speed: 1.0,
            time_position: 0.0,
            duration: None,
            filename: String::new(),
            media_title: String::new(),
        }
    }
}
