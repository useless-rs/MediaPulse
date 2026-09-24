mod commands;
mod events;
mod state;
mod windowing;

use std::error::Error;
use std::path::PathBuf;
use std::sync::Arc;

use tauri::Manager;

pub use commands::IpcError;
use state::AppState;
use windowing::WindowError;

/// Starts the `MediaPulse` desktop application.
///
/// # Errors
/// Returns an error when Tauri cannot create the application, resolve the
/// bundled media engine, or initialize the managed playback backend.
pub fn run() -> Result<(), Box<dyn Error>> {
    let app = tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            commands::backend_kind,
            commands::playback_snapshot,
            commands::load_media,
            commands::playback_command,
            commands::set_playback_property,
            commands::get_playback_property,
            commands::set_window_fullscreen,
            commands::minimize_window,
            commands::toggle_maximize_window,
            commands::close_window,
        ])
        .setup(|app| {
            let window = windowing::create_main_window(app)?;
            let window_id = match windowing::native_window_id(&window) {
                Ok(window_id) => Some(window_id),
                Err(WindowError::Unsupported) => None,
                Err(error) => return Err(Box::new(error)),
            };
            let state = Arc::new(AppState::new(window_id, bundled_sidecar())?);
            if !app.manage(state.clone()) {
                return Err(std::io::Error::other("MediaPulse state was already managed").into());
            }
            events::start_state_emitter(app.handle().clone(), state);
            Ok(())
        })
        .build(tauri::generate_context!())?;

    app.run(|_app_handle, _event| {});
    Ok(())
}

fn bundled_sidecar() -> Option<PathBuf> {
    #[cfg(all(feature = "sidecar", not(feature = "libmpv")))]
    {
        let executable = std::env::current_exe().ok()?;
        let executable_dir = executable.parent()?.to_path_buf();
        mediapulse_core::cli::resolve_sidecar_path(None, &executable, &executable_dir).ok()
    }

    #[cfg(not(all(feature = "sidecar", not(feature = "libmpv"))))]
    None
}
