#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    if let Err(error) = mediapulse_lib::run() {
        eprintln!("MediaPulse failed to start: {error}");
        std::process::exit(1);
    }
}
