# mediapulse-core

The playback contract shared by the MediaPulse app and the `mp` launcher.
Nothing in here knows about Tauri or React.

## What lives here

- `backend` — the `PlaybackBackend` trait plus the mpv implementations: an
  in-process `libmpv` backend and a `sidecar` backend that speaks mpv's JSON
  IPC. The app depends on the trait, so either can be swapped in.
- `cli` — argument parsing for `mp`. It records the UI-owned subset of options
  (volume, pause, fullscreen, speed) for the app to mirror, while keeping the
  original argument vector intact for mpv.
- Engine resolution. An explicit path wins, then locations inside the app
  bundle. There is deliberately no `PATH` search: the app must never end up
  playing through an mpv the user did not install alongside it.

## Features

| Feature | Effect |
| --- | --- |
| `libmpv` | Compiles the in-process libmpv backend. |
| `sidecar` | Compiles the JSON IPC backend and the `mp` launcher's sidecar support. |

## Verify

```bash
cargo test -p mediapulse-core --all-features
cargo clippy -p mediapulse-core --all-targets --all-features -- -D warnings
```

Licensed under GPL-3.0-or-later.
