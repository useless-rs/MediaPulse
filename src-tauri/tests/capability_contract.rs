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

/// Every command name the frontend invokes must be registered in the Tauri
/// handler, or the button silently does nothing at runtime.
///
/// The handler block is parsed with comment lines removed first: a test that
/// greps raw source can be satisfied by a commented-out registration, which
/// is exactly the bug it is meant to catch.
#[test]
fn every_invoked_command_is_registered() -> Result<(), Box<dyn std::error::Error>> {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .ok_or_else(|| io::Error::other("workspace root"))?
        .to_path_buf();

    let lib_source = fs::read_to_string(root.join("src-tauri/src/lib.rs"))?;
    let api_source = fs::read_to_string(root.join("src/lib/desktop-api.ts"))?;

    let handler_start = lib_source
        .find("generate_handler![")
        .ok_or_else(|| io::Error::other("no generate_handler! block"))?
        + "generate_handler![".len();
    let handler_end = lib_source[handler_start..]
        .find(']')
        .ok_or_else(|| io::Error::other("unterminated generate_handler! block"))?
        + handler_start;

    let mut registered: Vec<&str> = Vec::new();
    for line in lib_source[handler_start..handler_end].lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with("//") {
            continue;
        }
        let Some(name) = line.strip_prefix("commands::") else {
            return Err(io::Error::other(format!("unexpected handler entry: {line}")).into());
        };
        registered.push(name.trim_end_matches(','));
    }

    let mut invoked: Vec<&str> = Vec::new();
    let mut cursor = 0;
    while let Some(offset) = api_source[cursor..].find("invoke") {
        let start = cursor + offset + "invoke".len();
        let rest = &api_source[start..];
        let trimmed = rest.trim_start();
        // Require real call syntax: `invoke(` or `invoke<Kind>(`, so the
        // import line is not mistaken for a command name.
        let body = if let Some(generic) = trimmed.strip_prefix('<') {
            let Some(close) = generic.find('>') else {
                cursor = start;
                continue;
            };
            &generic[close + 1..]
        } else {
            trimmed
        };
        if !body.starts_with('(') {
            cursor = start;
            continue;
        }
        let Some(quote) = api_source[start..].find('"') else {
            break;
        };
        let name_start = start + quote + 1;
        let Some(end) = api_source[name_start..].find('"') else {
            break;
        };
        invoked.push(&api_source[name_start..name_start + end]);
        cursor = name_start + end;
    }

    assert!(
        !registered.is_empty(),
        "no commands parsed from the handler block"
    );
    assert!(
        !invoked.is_empty(),
        "no commands parsed from the desktop API"
    );

    for name in &invoked {
        assert!(
            registered.contains(name),
            "the frontend invokes `{name}` but it is not registered in generate_handler!; \
             registered: {registered:?}"
        );
    }

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
