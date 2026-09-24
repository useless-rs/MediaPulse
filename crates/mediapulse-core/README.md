# mediapulse-core

Shared Rust contracts for MediaPulse playback and command-line behavior.

## Responsibilities

- Typed playback snapshots, phases, commands, and property updates.
- Direct `libmpv` backend behind the `libmpv` feature.
- JSON IPC sidecar backend behind the `sidecar` feature.
- Original-argument-preserving mpv invocation and bundle-only sidecar resolution.

The core crate performs no system-`PATH` fallback for mpv. Callers must provide an explicit path or bundle-owned engine location.

Licensed under GPL-3.0-or-later.
