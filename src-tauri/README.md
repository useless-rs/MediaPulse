# mediapulse

The desktop app and the `mp` launcher, published together as one Cargo
package.

| Binary | What it does |
| --- | --- |
| `mediapulse` | The Tauri desktop player. This is the package's default binary. |
| `mp` | Terminal player. Forwards its arguments to mpv and exits with mpv's code. |

## Install

```bash
cargo install mediapulse
```

The default `libmpv` feature needs the platform mpv development package and
the Tauri system libraries. See the [project README](../../README.md) for the
package names per platform.

Cargo packages cannot carry a platform-native mpv binary, so a Cargo install
has no bundled engine. Point `mp` at one:

```bash
MEDIAPULSE_MPV_PATH=/absolute/path/to/mpv mp video.mkv
```

`mp` resolves the engine from that variable, then from locations inside the app
bundle, and never from `PATH`.

## Features

- `libmpv` (default) — plays through libmpv in-process.
- `sidecar` — talks to mpv over its JSON IPC socket instead. Used by the `mp`
  launcher.

Self-contained release bundles are the recommended way to install, because the
mpv runtime is platform-specific and cannot be shipped inside a Cargo package.

## Verify

```bash
cargo test -p mediapulse --all-features
cargo clippy -p mediapulse --all-targets --all-features -- -D warnings
```

Licensed under GPL-3.0-or-later.
