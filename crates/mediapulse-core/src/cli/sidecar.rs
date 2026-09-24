use std::path::{Path, PathBuf};

use thiserror::Error;

/// Failure to locate the bundled mpv executable.
#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum SidecarError {
    #[error("configured mpv sidecar does not exist: {path}")]
    MissingExplicit { path: PathBuf },
    #[error("bundled mpv sidecar was not found; searched: {searched:?}")]
    NotFound { searched: Vec<PathBuf> },
}

/// Resolve only explicit or bundle-owned mpv locations, never the system `PATH`.
///
/// # Errors
///
/// Returns [`SidecarError`] when an explicit path is missing or no bundled
/// executable exists in any supported bundle location.
pub fn resolve_sidecar_path(
    explicit: Option<&Path>,
    executable: &Path,
    resource_dir: &Path,
) -> Result<PathBuf, SidecarError> {
    if let Some(path) = explicit {
        return path.is_file().then(|| path.to_path_buf()).ok_or_else(|| {
            SidecarError::MissingExplicit {
                path: path.to_path_buf(),
            }
        });
    }

    let executable_dir = executable.parent().unwrap_or_else(|| Path::new("."));
    let sidecar_name = format!("mpv{}", std::env::consts::EXE_SUFFIX);
    let directories = [
        executable_dir.to_path_buf(),
        executable_dir.join("bin"),
        resource_dir.to_path_buf(),
        resource_dir.join("bin"),
    ];
    let mut searched = Vec::with_capacity(directories.len());

    for directory in directories {
        let candidate = directory.join(&sidecar_name);
        searched.push(candidate.clone());
        if candidate.is_file() {
            return Ok(candidate);
        }
    }

    Err(SidecarError::NotFound { searched })
}
