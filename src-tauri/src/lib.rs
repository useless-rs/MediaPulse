mod commands;
mod events;
mod locale;
mod state;
mod windowing;

#[cfg(test)]
mod ipc_tests;

use std::error::Error;
use std::path::PathBuf;
use std::sync::Arc;

use tauri::Manager;

pub use commands::IpcError;
use state::AppState;
#[cfg(not(target_os = "linux"))]
use windowing::WindowError;

/// Starts the `MediaPulse` desktop application.
///
/// When `initial_sources` is non-empty the first entry is loaded as soon as the
/// window exists, so `mediapulse video.mkv` opens straight into playback.
///
/// # Errors
/// Returns an error when Tauri cannot create the application, resolve the
/// bundled media engine, or initialize the managed playback backend.
pub fn run(initial_sources: Vec<String>) -> Result<(), Box<dyn Error>> {
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
        .setup(move |app| {
            let window = windowing::create_main_window(app)?;
            let window_id = embed_target(&window)?;
            locale::set_numeric_c()?;
            let state = Arc::new(AppState::new(window_id, bundled_sidecar())?);
            if !app.manage(state.clone()) {
                return Err(std::io::Error::other("MediaPulse state was already managed").into());
            }
            events::start_state_emitter(app.handle().clone(), state.clone());
            if let Some(source) = initial_sources.first() {
                state.load(source)?;
            }
            Ok(())
        })
        .build(tauri::generate_context!())?;

    app.run(|_app_handle, _event| {});
    Ok(())
}

/// The window mpv should draw into, when the platform can provide one.
///
/// On Linux the webview owns the window's rendering surface and paints over
/// it, so handing mpv that window id produces audio with an invisible picture:
/// exactly the failure this is meant to avoid. There is no API to draw native
/// content behind a `WebKitGTK` webview, so Linux always plays into a separate
/// mpv window instead. That keeps the picture visible and the behaviour the
/// same under X11, `XWayland` and native Wayland.
#[cfg(target_os = "linux")]
#[allow(clippy::unnecessary_wraps)]
fn embed_target(_window: &tauri::WebviewWindow) -> Result<Option<i64>, Box<dyn Error>> {
    Ok(None)
}

#[cfg(not(target_os = "linux"))]
fn embed_target(window: &tauri::WebviewWindow) -> Result<Option<i64>, Box<dyn Error>> {
    match windowing::native_window_id(window) {
        Ok(window_id) => Ok(Some(window_id)),
        Err(WindowError::Unsupported) => Ok(None),
        Err(error) => Err(Box::new(error)),
    }
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
