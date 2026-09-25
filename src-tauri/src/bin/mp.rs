use std::env;
use std::path::PathBuf;
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

fn run() -> Result<i32, CliError> {
    let invocation = parse_invocation(env::args_os().skip(1));
    let executable = env::current_exe().map_err(CliError::CurrentExecutable)?;
    let executable_dir = executable
        .parent()
        .map_or_else(|| PathBuf::from("."), PathBuf::from);
    let resource_dir = env::var_os(RESOURCE_DIR_ENV).map_or(executable_dir.clone(), PathBuf::from);
    let explicit = env::var_os(SIDECAR_PATH_ENV).map(PathBuf::from);
    let sidecar =
        resolve_sidecar_path(explicit.as_deref(), &executable, &resource_dir).map_err(|error| {
            match &error {
                SidecarError::MissingExplicit { .. } => CliError::Sidecar(error),
                SidecarError::NotFound { .. } => CliError::MissingSidecar(error),
            }
        })?;

    let status = Command::new(&sidecar)
        .args(invocation.engine_args())
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
