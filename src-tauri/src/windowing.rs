use raw_window_handle::{HasWindowHandle, RawWindowHandle};
#[cfg(target_os = "macos")]
use tauri::TitleBarStyle;
use tauri::{App, WebviewUrl, WebviewWindow, WebviewWindowBuilder};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum WindowError {
    #[error("could not access the native window handle: {0}")]
    Handle(#[from] raw_window_handle::HandleError),
    #[error("native window embedding is unsupported for this display server")]
    Unsupported,
    #[error("native window id cannot be represented as a signed integer")]
    InvalidNativeWindowId,
}

pub fn create_main_window(app: &App) -> tauri::Result<WebviewWindow> {
    let builder = WebviewWindowBuilder::new(app, "main", WebviewUrl::default())
        .title("MediaPulse")
        .inner_size(1280.0, 800.0)
        .min_inner_size(800.0, 500.0)
        .resizable(true)
        .transparent(true)
        .shadow(true);

    #[cfg(target_os = "macos")]
    let window = builder.title_bar_style(TitleBarStyle::Overlay).build()?;

    #[cfg(not(target_os = "macos"))]
    let window = builder.decorations(false).build()?;

    Ok(window)
}

pub fn native_window_id(window: &WebviewWindow) -> Result<i64, WindowError> {
    let handle = window.window_handle()?;
    match handle.as_raw() {
        RawWindowHandle::Win32(handle) => {
            i64::try_from(handle.hwnd.get()).map_err(|_| WindowError::InvalidNativeWindowId)
        }
        RawWindowHandle::Xlib(handle) => {
            i64::try_from(handle.window).map_err(|_| WindowError::InvalidNativeWindowId)
        }
        RawWindowHandle::Xcb(handle) => Ok(i64::from(handle.window.get())),
        RawWindowHandle::AppKit(handle) => Ok(handle.ns_view.as_ptr() as i64),
        _ => Err(WindowError::Unsupported),
    }
}
