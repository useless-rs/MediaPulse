use std::env;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use mediapulse_core::cli::{SidecarError, parse_invocation, resolve_sidecar_path};
use thiserror::Error;

const SIDECAR_PATH_ENV: &str = "MEDIAPULSE_MPV_PATH";
const RESOURCE_DIR_ENV: &str = "MEDIAPULSE_RESOURCE_DIR";

#[derive(Debug, Error)]
enum CliError {
    #[error(transparent)]
    Sidecar(#[from] SidecarError),
    #[error(
        "no usable mpv engine was found. Cargo installs do not include native mpv; \
         set MEDIAPULSE_MPV_PATH=/absolute/path/to/mpv or use a MediaPulse desktop bundle. {0}"
    )]
    MissingSidecar(SidecarError),
    #[error("could not locate the MediaPulse executable: {0}")]
    CurrentExecutable(std::io::Error),
    #[error("could not start bundled mpv at {path}: {source}")]
    Spawn {
        path: PathBuf,
        source: std::io::Error,
    },
}

fn main() {
    let exit_code = match run() {
        Ok(exit_code) => exit_code,
        Err(error) => {
            eprintln!("mp: {error}");
            1
        }
    };
    std::process::exit(exit_code);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Local {
    Help,
    Version,
}

fn local_request(args: &[std::ffi::OsString]) -> Option<Local> {
    match args.first()?.to_str()? {
        "--help" | "-h" => Some(Local::Help),
        "--version" | "-V" => Some(Local::Version),
        _ => None,
    }
}

fn help_text() -> String {
    format!(
        "\
mp {version} - MediaPulse's mpv-compatible command line player

Usage:
  mp [mpv options] <file>...
  mp --help
  mp --version

Arguments are passed to mpv unchanged, so every mpv option works. Options
that MediaPulse mirrors into its own UI (volume, pause, fullscreen, speed)
are tracked by the desktop app as well.

Engine:
  mp never searches your PATH for mpv. A release bundle uses the mpv it
  ships with. A `cargo install mediapulse` build has no bundled mpv, so
  point it at one explicitly:

    MEDIAPULSE_MPV_PATH=/absolute/path/to/mpv mp video.mkv

Examples:
  mp video.mkv
  mp --volume=80 --no-fullscreen video.mkv
  mp --list-options
",
        version = env!("CARGO_PKG_VERSION"),
    )
}

fn print_version(engine: Option<&Path>) {
    println!("mp {} (MediaPulse)", env!("CARGO_PKG_VERSION"));
    match engine {
        Some(path) => println!("mpv engine: {}", path.display()),
        None => println!(
            "mpv engine: not configured (set {SIDECAR_PATH_ENV} or use a MediaPulse desktop bundle)"
        ),
    }
}

fn resolve_engine() -> Result<PathBuf, CliError> {
    let executable = env::current_exe().map_err(CliError::CurrentExecutable)?;
    let executable_dir = executable
        .parent()
        .map_or_else(|| PathBuf::from("."), PathBuf::from);
    let resource_dir = env::var_os(RESOURCE_DIR_ENV).map_or(executable_dir, PathBuf::from);
    let explicit = env::var_os(SIDECAR_PATH_ENV).map(PathBuf::from);
    resolve_sidecar_path(explicit.as_deref(), &executable, &resource_dir).map_err(|error| {
        match &error {
            SidecarError::MissingExplicit { .. } => CliError::Sidecar(error),
            SidecarError::NotFound { .. } => CliError::MissingSidecar(error),
        }
    })
}

fn run() -> Result<i32, CliError> {
    let invocation = parse_invocation(env::args_os().skip(1));
    let engine_args = invocation.engine_args();

    match local_request(engine_args) {
        Some(Local::Help) => {
            print!("{}", help_text());
            return Ok(0);
        }
        Some(Local::Version) => {
            let engine = resolve_engine().ok();
            print_version(engine.as_deref());
            return Ok(0);
        }
        None if engine_args.is_empty() => {
            eprint!("{}", help_text());
            return Ok(1);
        }
        None => {}
    }

    let sidecar = resolve_engine()?;

    let status = Command::new(&sidecar)
        .args(engine_args)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .map_err(|source| CliError::Spawn {
            path: sidecar,
            source,
        })?;

    Ok(exit_code(status))
}

#[cfg(unix)]
fn exit_code(status: std::process::ExitStatus) -> i32 {
    use std::os::unix::process::ExitStatusExt;

    status
        .code()
        .unwrap_or_else(|| 128 + status.signal().unwrap_or_default())
}

#[cfg(not(unix))]
fn exit_code(status: std::process::ExitStatus) -> i32 {
    status.code().unwrap_or(1)
}
