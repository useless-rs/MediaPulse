#![doc = "Shared playback contracts and command translation for `MediaPulse`."]
#![forbid(unsafe_code)]

pub mod backend;
pub mod cli;

/// Product version shared by the desktop shell and command-line player.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
