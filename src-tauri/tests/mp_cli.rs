#![cfg(unix)]

use std::fs::{self, File, set_permissions};
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::process::Command;

use tempfile::tempdir;

fn fake_sidecar(directory: &Path, exit_code: i32) -> PathBuf {
    let path = directory.join("mpv");
    let mut file = File::create(&path).expect("fake sidecar file");
    std::io::Write::write_all(
        &mut file,
        format!("#!/bin/sh\nprintf '<%s>\\n' \"$@\"\nexit {exit_code}\n").as_bytes(),
    )
    .expect("fake sidecar script");
    set_permissions(&path, std::fs::Permissions::from_mode(0o755)).expect("sidecar permissions");
    path
}

#[test]
fn mp_forwards_canonical_and_legacy_arguments_unchanged() {
    let temp = tempdir().expect("temp directory");
    let sidecar = fake_sidecar(temp.path(), 7);
    let canonical = ["--volume=80", "--no-fullscreen", "--speed=1.5", "video.mkv"];
    let legacy = [
        "-volume",
        "80",
        "-no-fullscreen",
        "-speed",
        "1.5",
        "video.mkv",
    ];
    let cases: &[&[&str]] = &[&canonical, &legacy];

    for &arguments in cases {
        let output = Command::new(env!("CARGO_BIN_EXE_mp"))
            .env("MEDIAPULSE_MPV_PATH", &sidecar)
            .args(arguments)
            .output()
            .expect("run mp");

        assert_eq!(output.status.code(), Some(7));
        let expected = arguments
            .iter()
            .fold(String::new(), |mut output, argument| {
                use std::fmt::Write as _;
                writeln!(output, "<{argument}>").expect("write to String");
                output
            });
        assert_eq!(
            String::from_utf8(output.stdout).expect("UTF-8 stdout"),
            expected
        );
        assert!(output.stderr.is_empty());
    }
}

#[test]
fn mp_forwards_list_options_to_the_bundled_engine() {
    let temp = tempdir().expect("temp directory");
    let sidecar = fake_sidecar(temp.path(), 0);

    let output = Command::new(env!("CARGO_BIN_EXE_mp"))
        .env("MEDIAPULSE_MPV_PATH", &sidecar)
        .args(["--no-config", "--list-options"])
        .output()
        .expect("run mp");

    assert!(output.status.success());
    assert_eq!(
        String::from_utf8(output.stdout).expect("UTF-8 stdout"),
        "<--no-config>\n<--list-options>\n"
    );
}

#[test]
fn mp_never_falls_back_to_system_path() {
    let temp = tempdir().expect("temp directory");
    let isolated_executable = temp.path().join("mp");
    fs::copy(env!("CARGO_BIN_EXE_mp"), &isolated_executable).expect("copy mp executable");
    set_permissions(&isolated_executable, std::fs::Permissions::from_mode(0o755))
        .expect("mp permissions");

    let output = Command::new(&isolated_executable)
        .env_remove("MEDIAPULSE_MPV_PATH")
        .env_remove("MEDIAPULSE_RESOURCE_DIR")
        .env("PATH", "")
        .arg("--version")
        .output()
        .expect("run mp");

    assert_eq!(output.status.code(), Some(1));
    assert!(
        String::from_utf8(output.stderr)
            .expect("UTF-8 stderr")
            .contains("bundled mpv sidecar was not found")
    );
}
