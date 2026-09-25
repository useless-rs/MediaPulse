use std::sync::{Arc, Mutex};

use mediapulse_core::backend::{
    BackendError, BackendKind, PlaybackBackend, PlaybackCommand, PlaybackSnapshot, PropertyUpdate,
    PropertyValue,
};
use tauri::test::{INVOKE_KEY, get_ipc_response, mock_builder, mock_context, noop_assets};
use tauri::webview::InvokeRequest;
use tauri::{WebviewUrl, http::HeaderMap};

use crate::commands;
use crate::state::AppState;

#[derive(Default)]
struct StubBackend {
    loaded: Mutex<Vec<String>>,
}

impl PlaybackBackend for StubBackend {
    fn backend_kind(&self) -> BackendKind {
        BackendKind::Libmpv
    }

    fn load(&self, source: &str) -> Result<(), BackendError> {
        self.loaded
            .lock()
            .expect("loaded lock")
            .push(source.to_owned());
        Ok(())
    }

    fn command(&self, _command: PlaybackCommand) -> Result<(), BackendError> {
        Ok(())
    }

    fn set_property(&self, _update: PropertyUpdate) -> Result<(), BackendError> {
        Ok(())
    }

    fn get_property(&self, _name: &str) -> Result<PropertyValue, BackendError> {
        Ok(PropertyValue::Unit)
    }

    fn snapshot(&self) -> Result<PlaybackSnapshot, BackendError> {
        Ok(PlaybackSnapshot::default())
    }

    fn shutdown(&self) -> Result<(), BackendError> {
        Ok(())
    }
}

fn invoke<W: AsRef<tauri::Webview<tauri::test::MockRuntime>>>(
    webview: &W,
    cmd: &str,
    body: serde_json::Value,
) -> Result<tauri::ipc::InvokeResponseBody, serde_json::Value> {
    get_ipc_response(
        webview,
        InvokeRequest {
            cmd: cmd.into(),
            callback: tauri::ipc::CallbackFn(0),
            error: tauri::ipc::CallbackFn(1),
            url: "tauri://localhost".parse().expect("invoke url"),
            body: tauri::ipc::InvokeBody::Json(body),
            headers: HeaderMap::new(),
            invoke_key: INVOKE_KEY.to_string(),
        },
    )
}

#[test]
fn load_media_resolves_managed_state_and_dispatches_to_the_backend() {
    let backend = Arc::new(StubBackend::default());
    let app = mock_builder()
        .manage(Arc::new(AppState::with_backend(backend.clone())))
        .invoke_handler(tauri::generate_handler![commands::load_media])
        .build(mock_context(noop_assets()))
        .expect("mock app");

    let window = tauri::WebviewWindowBuilder::new(&app, "main", WebviewUrl::default())
        .build()
        .expect("mock window");

    let source = "/tmp/opencode/mediapulse-ipc-fixture.mp4";
    let response = invoke(
        &window,
        "load_media",
        serde_json::json!({ "source": source }),
    );

    assert!(
        response.is_ok(),
        "load_media must resolve the managed Arc<AppState>, got: {response:?}"
    );
    assert_eq!(
        backend.loaded.lock().expect("loaded lock").as_slice(),
        [source]
    );
}
