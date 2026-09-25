use std::fs;
use std::io;
use std::path::PathBuf;

use serde_json::Value;

#[test]
fn main_window_grants_dialog_default_permission() -> Result<(), Box<dyn std::error::Error>> {
    let capability_path =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("capabilities/default.json");
    let capability: Value = serde_json::from_str(&fs::read_to_string(capability_path)?)?;
    let permissions = capability
        .get("permissions")
        .and_then(Value::as_array)
        .ok_or_else(|| io::Error::other("main capability has no permissions array"))?;

    assert!(
        permissions
            .iter()
            .any(|permission| permission.as_str() == Some("dialog:default")),
        "main capability must grant dialog:default for the Open media command"
    );

    Ok(())
}

#[test]
fn tauri_commands_use_the_managed_arc_state() -> Result<(), Box<dyn std::error::Error>> {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let lib_source = fs::read_to_string(manifest_dir.join("src/lib.rs"))?;
    let commands_source = fs::read_to_string(manifest_dir.join("src/commands.rs"))?;

    assert!(lib_source.contains("app.manage(state.clone())"));
    assert_eq!(
        commands_source.matches("State<'_, Arc<AppState>>").count(),
        6,
        "all state-backed Tauri commands must request the managed Arc<AppState>"
    );

    Ok(())
}
