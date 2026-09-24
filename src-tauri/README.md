# mediapulse

The publishable Rust package for the MediaPulse desktop application and its `mp` companion CLI.

## Binaries

- `mediapulse` — Tauri desktop player; this is Cargo's default binary.
- `mp` — mpv-compatible launcher that resolves only an explicit or bundled engine.

## Features

- `libmpv` (default) — direct native playback backend for the desktop shell.
- `sidecar` — JSON IPC sidecar backend and launcher support.

Desktop release bundles are the recommended self-contained installation path because libmpv and mpv sidecars are native, platform-specific artifacts. Source builds require the corresponding Tauri and mpv development packages.

## Validation

```bash
cargo test -p mediapulse --all-features
cargo clippy -p mediapulse --all-targets --all-features -- -D warnings
```

Licensed under GPL-3.0-or-later.
