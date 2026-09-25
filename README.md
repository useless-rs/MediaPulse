# MediaPulse

MediaPulse is a private, offline-first desktop media player built with Tauri, React, Rust, and mpv. It combines a focused desktop player with an mpv-compatible command-line launcher in one GPLv3 project.

[Download the latest release](https://github.com/useless-rs/MediaPulse/releases)

## Highlights

- Direct `libmpv` playback for the desktop application.
- A bundled `mp` launcher that forwards arguments, streams, and exit codes unchanged to its bundled mpv sidecar.
- Full-canvas player with an mpv-style OSC, keyboard shortcuts, local playlist, and a centered settings sheet.
- No accounts, telemetry, cloud processing, or automatic system-mpv discovery.
- Typed Rust playback contracts and event-driven React state.
- Native Tauri dialogs for selecting local media.

## Desktop app

```bash
bun install
bun run tauri dev
```

Build installable bundles with:

```bash
bun run tauri build
```

The release bundle contains the frontend, desktop executable, bundled mpv sidecar, and license metadata. Native libmpv runtimes remain platform-specific: the Linux package declares `libmpv2`, while release automation must supply the matching runtime for each target platform.

## Install from Cargo

Install the published desktop binary and its companion `mp` launcher with:

```bash
cargo install mediapulse
```

The default `libmpv` feature requires the platform mpv development package and Tauri system libraries at compile time. Linux distributions commonly provide `libmpv-dev`, `libwebkit2gtk-4.1-dev`, and `libgtk-3-dev`; see the Tauri prerequisites for your platform. The Cargo-installed desktop binary uses the system libmpv runtime, while downloadable desktop bundles carry their platform-native sidecar assets.

Cargo packages cannot include a platform-native mpv sidecar. The Cargo-installed `mp` launcher therefore remains available but needs an explicit engine path, for example `MEDIAPULSE_MPV_PATH=/usr/bin/mpv mp video.mkv`; it never searches `PATH` automatically.

## `mp` CLI

The `mp` binary is installed beside the desktop executable in release bundles. It preserves the original argument vector and never searches `PATH` for an engine.

```bash
mp video.mkv
mp --volume=80 --no-fullscreen video.mkv
mp -volume 80 -no-fullscreen video.mkv
mp --list-options
```

For local development only, an explicit engine path may be supplied:

```bash
MEDIAPULSE_MPV_PATH=/absolute/path/to/mpv mp video.mkv
```

`MEDIAPULSE_MPV_PATH` is intentionally unsupported as a system-install mechanism: a missing explicit path is an error, and the launcher never falls back to `PATH`.

## Keyboard shortcuts

| Shortcut | Action |
| --- | --- |
| `Space` | Play or pause |
| `←` / `→` | Seek backward or forward |
| `L` | Toggle playlist |
| `S` | Open settings |
| `F` | Toggle fullscreen |

## Workspace

- `src-tauri/` — Tauri desktop shell, window controls, typed IPC, and `mp` binary.
- `crates/mediapulse-core/` — shared playback contract, direct libmpv adapter, sidecar adapter, and CLI argument handling.
- `src/` — React player shell and interaction tests.
- `design-system.json` — visual and interaction contract.

## Quality gates

```bash
bun run check
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
```

## Platform notes

- macOS uses an overlay title bar with native traffic lights.
- Windows and Linux use MediaPulse window controls backed by Tauri commands.
- Linux video embedding currently uses native window IDs on X11. Wayland support requires a native surface/render-API path and is tracked as platform-specific engineering work.
- Cargo installs provide the Rust binaries; self-contained desktop bundles are the recommended end-user distribution because native mpv libraries differ by platform.

## License

MediaPulse is licensed under the GNU General Public License v3.0 or later. See [`LICENSE`](LICENSE).
