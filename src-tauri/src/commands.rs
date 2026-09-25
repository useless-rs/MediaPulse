// Tauri's generated command wrapper only implements `CommandArg` for owned
// managed state and window handles, so these public boundary signatures are
// intentionally values even when the command body only reads from them.
#![allow(clippy::needless_pass_by_value)]

use std::sync::Arc;

use mediapulse_core::backend::{
    BackendError, BackendKind, PlaybackCommand, PlaybackSnapshot, PropertyUpdate, PropertyValue,
};
use serde::Serialize;
use tauri::{State, WebviewWindow};

use crate::state::AppState;

#[derive(Debug, Clone, Serialize)]
pub struct IpcError {
    pub code: String,
    pub message: String,
}

impl From<BackendError> for IpcError {
    fn from(error: BackendError) -> Self {
        let code = match error {
            BackendError::NotReady => "notReady",
            BackendError::Unsupported { .. } => "unsupported",
            BackendError::InvalidProperty { .. } => "invalidProperty",
            BackendError::Io { .. } => "io",
            BackendError::Timeout { .. } => "timeout",
            BackendError::Protocol { .. } => "protocol",
            BackendError::Engine { .. } => "engine",
        }
        .to_string();
        Self {
            code,
            message: error.to_string(),
        }
    }
}

#[tauri::command]
pub fn backend_kind(state: State<'_, Arc<AppState>>) -> BackendKind {
    state.backend_kind()
}

#[tauri::command]
pub fn playback_snapshot(state: State<'_, Arc<AppState>>) -> Result<PlaybackSnapshot, IpcError> {
    state.snapshot().map_err(IpcError::from)
}

#[tauri::command]
pub fn load_media(source: &str, state: State<'_, Arc<AppState>>) -> Result<(), IpcError> {
    state.load(source).map_err(IpcError::from)
}

#[tauri::command]
pub fn playback_command(
    command: PlaybackCommand,
    state: State<'_, Arc<AppState>>,
) -> Result<(), IpcError> {
    state.command(command).map_err(IpcError::from)
}

#[tauri::command]
pub fn set_playback_property(
    update: PropertyUpdate,
    state: State<'_, Arc<AppState>>,
) -> Result<(), IpcError> {
    state.set_property(update).map_err(IpcError::from)
}

#[tauri::command]
pub fn get_playback_property(
    name: &str,
    state: State<'_, Arc<AppState>>,
) -> Result<PropertyValue, IpcError> {
    state.get_property(name).map_err(IpcError::from)
}

#[tauri::command]
pub fn set_window_fullscreen(window: WebviewWindow, enabled: bool) -> Result<(), IpcError> {
    window
        .set_fullscreen(enabled)
        .map_err(|error| window_error(&error))
}

#[tauri::command]
pub fn minimize_window(window: WebviewWindow) -> Result<(), IpcError> {
    window.minimize().map_err(|error| window_error(&error))
}

#[tauri::command]
pub fn toggle_maximize_window(window: WebviewWindow) -> Result<(), IpcError> {
    if window
        .is_maximized()
        .map_err(|error| window_error(&error))?
    {
        window.unmaximize().map_err(|error| window_error(&error))
    } else {
        window.maximize().map_err(|error| window_error(&error))
    }
}

#[tauri::command]
pub fn close_window(window: WebviewWindow) -> Result<(), IpcError> {
    window.close().map_err(|error| window_error(&error))
}

fn window_error(error: &tauri::Error) -> IpcError {
    IpcError {
        code: "window".to_string(),
        message: error.to_string(),
    }
}
