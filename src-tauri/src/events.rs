use std::sync::Arc;
use std::thread;
use std::time::Duration;

use tauri::{AppHandle, Emitter};

use crate::commands::IpcError;
use crate::state::AppState;

const STATE_EVENT: &str = "playback-state";
const ERROR_EVENT: &str = "playback-error";
const EMIT_INTERVAL: Duration = Duration::from_millis(100);

pub fn start_state_emitter(app: AppHandle, state: Arc<AppState>) {
    thread::spawn(move || {
        loop {
            match state.snapshot() {
                Ok(snapshot) => {
                    if app.emit(STATE_EVENT, snapshot).is_err() {
                        break;
                    }
                }
                Err(error) => {
                    let payload = IpcError::from(error);
                    if app.emit(ERROR_EVENT, &payload).is_err() {
                        break;
                    }
                }
            }
            thread::sleep(EMIT_INTERVAL);
        }
    });
}
