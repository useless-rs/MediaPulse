# MediaPulse

A private, offline media player for Linux, macOS and Windows, plus `mp`, a
drop-in command line player for the same engine.

No accounts, no telemetry, no cloud processing. Playback runs locally through
[mpv](https://mpv.io/).

## Install

Grab a build for your platform from the
[releases page](https://github.com/useless-rs/MediaPulse/releases). The bundle
contains the app, the `mp` launcher and its own copy of mpv, so there is
nothing else to install.

### Install with Cargo

```bash
cargo install mediapulse
```

This builds from source, so it needs the platform toolchain: a C compiler,
the mpv development headers (`libmpv-dev` on Debian and Ubuntu,
`mpv-devel` on Fedora) and the Tauri system libraries
(`libwebkit2gtk-4.1-dev`, `libgtk-3-dev` on Debian and Ubuntu).

A Cargo install ships **no** mpv, because Cargo packages cannot carry a
platform-native binary. The `mediapulse` app still uses your system's libmpv,
but the `mp` launcher needs to be told where mpv is:

```bash
MEDIAPULSE_MPV_PATH=/usr/bin/mpv mp video.mkv
```

## Use the app

Open a file with **Open media**, or drop one onto the window. The playlist
sits on the right, and settings opens as a centred sheet.

| Shortcut | Action |
| --- | --- |
| <kbd>Space</kbd> | Play or pause |
| <kbd>←</kbd> <kbd>→</kbd> | Seek backward or forward |
| <kbd>L</kbd> | Toggle playlist |
| <kbd>S</kbd> | Open settings |
| <kbd>F</kbd> | Toggle fullscreen |

## Use `mp` from the terminal

`mp` forwards every argument to mpv untouched, so anything mpv accepts works
here too. It exits with mpv's exit code and writes mpv's output straight to
your terminal.

```bash
mp video.mkv
mp --volume=80 --no-fullscreen video.mkv
mp --list-options
```

`mp` looks for mpv next to itself, inside the app bundle, and in
`MEDIAPULSE_MPV_PATH`. It never searches your `PATH`, so the engine it picks is
always the one that shipped with the app rather than whatever happens to be
installed on the machine.

## Development

```bash
bun install
bun run tauri dev
```

The workspace holds three pieces:

| Path | What it is |
| --- | --- |
| `src-tauri/` | Tauri shell, typed IPC commands and the `mp` binary |
| `crates/mediapulse-core/` | Shared playback contract and the mpv adapters |
| `src/` | React player shell |

Run the checks before opening a pull request:

```bash
bun run check                      # lint, types, frontend tests
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
```

## Platform notes

- Video is embedded into the app window through native window handles, which
  means X11. Under Wayland there is no handle to hand to mpv, so the video
  opens in mpv's own window beside the app instead. Playback, the playlist and
  the controls behave the same either way.
- Bundled builds carry the mpv build for their own platform. If you are on
  Linux, the `.deb`/`.AppImage` expect `libmpv2` at runtime.

## License

GPL-3.0-or-later. See [`LICENSE`](LICENSE).
