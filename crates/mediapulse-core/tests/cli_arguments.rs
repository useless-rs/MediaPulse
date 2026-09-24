use std::ffi::OsString;
use std::fs::{File, create_dir_all};
use std::path::{Path, PathBuf};

use mediapulse_core::cli::{SidecarError, UiOverrides, parse_invocation, resolve_sidecar_path};

fn os_args(values: &[&str]) -> Vec<OsString> {
    values.iter().map(OsString::from).collect()
}

#[test]
fn canonical_options_map_to_ui_state_without_rewriting_engine_arguments() {
    let args = os_args(&["--volume=80", "--no-fullscreen", "--speed=1.5", "video.mkv"]);

    let invocation = parse_invocation(args.clone());

    assert_eq!(invocation.engine_args(), args);
    assert_eq!(invocation.ui_overrides().volume, Some(80.0));
    assert_eq!(invocation.ui_overrides().paused, None);
    assert_eq!(invocation.ui_overrides().fullscreen, Some(false));
    assert_eq!(invocation.ui_overrides().speed, Some(1.5));
    assert!(!invocation.lists_options());
}

#[test]
fn legacy_options_map_to_ui_state_and_preserve_original_argv() {
    let args = os_args(&[
        "-volume",
        "80",
        "-no-fullscreen",
        "-pause",
        "-speed",
        "1.5",
        "video.mkv",
    ]);

    let invocation = parse_invocation(args.clone());

    assert_eq!(invocation.engine_args(), args);
    assert_eq!(invocation.ui_overrides().volume, Some(80.0));
    assert_eq!(invocation.ui_overrides().paused, Some(true));
    assert_eq!(invocation.ui_overrides().fullscreen, Some(false));
    assert_eq!(invocation.ui_overrides().speed, Some(1.5));
}

#[test]
fn unknown_options_and_non_utf8_arguments_remain_untouched() {
    let mut args = os_args(&["--sub-scale=1.25", "--custom=visible", "-x"]);
    #[cfg(unix)]
    {
        use std::os::unix::ffi::OsStringExt;
        args.push(OsString::from_vec(vec![0x66, 0x80, 0x6f]));
    }

    let invocation = parse_invocation(args.clone());

    assert_eq!(invocation.engine_args(), args);
    assert_eq!(invocation.ui_overrides(), &UiOverrides::default());
}

#[test]
fn double_dash_stops_ui_registry_scanning() {
    let args = os_args(&["--", "--volume=99", "video.mkv"]);

    let invocation = parse_invocation(args.clone());

    assert_eq!(invocation.engine_args(), args);
    assert_eq!(invocation.ui_overrides(), &UiOverrides::default());
}

#[test]
fn list_options_is_detected_and_forwarded_verbatim() {
    let args = os_args(&["--no-config", "--list-options"]);

    let invocation = parse_invocation(args.clone());

    assert!(invocation.lists_options());
    assert_eq!(invocation.engine_args(), args);
}

#[test]
fn last_known_ui_option_wins() {
    let args = os_args(&[
        "--volume=20",
        "--volume=90",
        "--fullscreen",
        "--no-fullscreen",
    ]);

    let invocation = parse_invocation(args);

    assert_eq!(invocation.ui_overrides().volume, Some(90.0));
    assert_eq!(invocation.ui_overrides().fullscreen, Some(false));
}

#[test]
fn sidecar_resolution_searches_only_bundle_locations() {
    let temp = tempfile::tempdir().expect("temp directory");
    let executable = temp.path().join("bin").join("mediapulse");
    let resource_dir = temp.path().join("resources");
    create_dir_all(executable.parent().expect("executable parent")).expect("bin directory");
    create_dir_all(&resource_dir).expect("resource directory");
    let sidecar = resource_dir.join(if cfg!(windows) { "mpv.exe" } else { "mpv" });
    File::create(&sidecar).expect("sidecar placeholder");

    let resolved = resolve_sidecar_path(None, &executable, &resource_dir)
        .expect("bundled sidecar should resolve");

    assert_eq!(resolved, sidecar);
}

#[test]
fn sidecar_resolution_does_not_fall_back_to_path() {
    let temp = tempfile::tempdir().expect("temp directory");
    let executable = temp.path().join("mediapulse");
    let resource_dir = temp.path().join("resources");
    create_dir_all(&resource_dir).expect("resource directory");

    let result = resolve_sidecar_path(None, &executable, &resource_dir);

    match result {
        Err(SidecarError::NotFound { searched }) => assert_eq!(searched.len(), 4),
        other => panic!("expected a bundled-sidecar lookup failure, got {other:?}"),
    }
}

#[test]
fn explicit_sidecar_path_is_used_when_present() {
    let temp = tempfile::tempdir().expect("temp directory");
    let explicit = temp
        .path()
        .join(if cfg!(windows) { "mpv.exe" } else { "mpv" });
    File::create(&explicit).expect("explicit sidecar placeholder");
    let executable: PathBuf = temp.path().join("mediapulse");

    let resolved = resolve_sidecar_path(
        Some(Path::new(&explicit)),
        &executable,
        Path::new("resources"),
    )
    .expect("explicit sidecar should resolve");

    assert_eq!(resolved, explicit);
}
