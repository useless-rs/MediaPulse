#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    if let Err(error) = mediapulse_lib::run(media_sources()) {
        eprintln!("MediaPulse failed to start: {error}");
        std::process::exit(1);
    }
}

fn media_sources() -> Vec<String> {
    std::env::args_os()
        .skip(1)
        .filter(|argument| !argument.to_string_lossy().starts_with('-'))
        .map(|argument| argument.to_string_lossy().into_owned())
        .collect()
}
