import { invoke, isTauri } from "@tauri-apps/api/core"
import { listen } from "@tauri-apps/api/event"
import { getCurrentWebview } from "@tauri-apps/api/webview"
import { open } from "@tauri-apps/plugin-dialog"

import { clamp, fileTitle } from "./format"
import {
  type BackendKind,
  type DesktopApi,
  type DesktopError,
  EMPTY_SNAPSHOT,
  type PlaybackSnapshot,
} from "./playback"

const MEDIA_EXTENSIONS = [
  "3gp",
  "aac",
  "avi",
  "flac",
  "flv",
  "m3u8",
  "m4a",
  "m4v",
  "mkv",
  "mov",
  "mp3",
  "mp4",
  "ogg",
  "opus",
  "wav",
  "webm",
  "wmv",
]

function normalizeError(error: unknown): DesktopError {
  if (typeof error === "object" && error !== null && "code" in error && "message" in error) {
    const code = Reflect.get(error, "code")
    const message = Reflect.get(error, "message")
    if (typeof code === "string" && typeof message === "string") {
      return { code, message }
    }
  }
  return {
    code: "unknown",
    message: error instanceof Error ? error.message : String(error),
  }
}

function setBooleanProperty(name: string, value: boolean): Promise<void> {
  return invoke("set_playback_property", {
    update: { name, value: { type: "boolean", value } },
  })
}

function setNumberProperty(name: string, value: number): Promise<void> {
  return invoke("set_playback_property", {
    update: { name, value: { type: "number", value } },
  })
}

const tauriDesktopApi: DesktopApi = {
  getBackendKind: () => invoke<BackendKind>("backend_kind"),
  getSnapshot: () => invoke<PlaybackSnapshot>("playback_snapshot"),
  subscribe: async (onSnapshot, onError) => {
    const [unlistenState, unlistenError] = await Promise.all([
      listen<PlaybackSnapshot>("playback-state", (event) => onSnapshot(event.payload)),
      listen<DesktopError>("playback-error", (event) => onError(event.payload)),
    ])
    return () => {
      unlistenState()
      unlistenError()
    }
  },
  load: (source) => invoke("load_media", { source }),
  setPaused: (paused) => setBooleanProperty("pause", paused),
  seek: (seconds) => setNumberProperty("time-pos", Math.max(0, seconds)),
  setVolume: (volume) => setNumberProperty("volume", clamp(volume, 0, 130)),
  setSpeed: (speed) => setNumberProperty("speed", clamp(speed, 0.25, 4)),
  setMuted: (muted) => setBooleanProperty("mute", muted),
  setFullscreen: (enabled) => invoke("set_window_fullscreen", { enabled }),
  openMedia: async () => {
    const selected = await open({
      multiple: true,
      fileAccessMode: "copy",
      filters: [{ name: "Media", extensions: MEDIA_EXTENSIONS }],
    })
    if (selected === null) return []
    return Array.isArray(selected) ? selected : [selected]
  },
  subscribeFileDrop: async (onPaths) => {
    const unlisten = await getCurrentWebview().onDragDropEvent((event) => {
      if (event.payload.type === "drop" && event.payload.paths.length > 0) {
        onPaths(event.payload.paths)
      }
    })
    return () => {
      void unlisten()
    }
  },
  minimizeWindow: () => invoke("minimize_window"),
  toggleMaximizeWindow: () => invoke("toggle_maximize_window"),
  closeWindow: () => invoke("close_window"),
}

const browserListeners = new Set<(snapshot: PlaybackSnapshot) => void>()
let browserSnapshot = EMPTY_SNAPSHOT

function publishBrowserSnapshot(): void {
  for (const listener of browserListeners) listener(browserSnapshot)
}

const browserDesktopApi: DesktopApi = {
  getBackendKind: async () => "libmpv",
  getSnapshot: async () => browserSnapshot,
  subscribe: async (onSnapshot) => {
    browserListeners.add(onSnapshot)
    onSnapshot(browserSnapshot)
    return () => browserListeners.delete(onSnapshot)
  },
  load: async (source) => {
    browserSnapshot = {
      ...browserSnapshot,
      filename: source,
      mediaTitle: fileTitle(source),
      phase: { kind: "playing" },
      paused: false,
      timePosition: 0,
      duration: 180,
    }
    publishBrowserSnapshot()
  },
  setPaused: async (paused) => {
    const hasMedia = browserSnapshot.filename.length > 0
    browserSnapshot = {
      ...browserSnapshot,
      paused,
      phase: !hasMedia ? { kind: "idle" } : paused ? { kind: "paused" } : { kind: "playing" },
    }
    publishBrowserSnapshot()
  },
  seek: async (seconds) => {
    browserSnapshot = { ...browserSnapshot, timePosition: Math.max(0, seconds) }
    publishBrowserSnapshot()
  },
  setVolume: async (volume) => {
    browserSnapshot = { ...browserSnapshot, volume: clamp(volume, 0, 130) }
    publishBrowserSnapshot()
  },
  setSpeed: async (speed) => {
    browserSnapshot = { ...browserSnapshot, speed: clamp(speed, 0.25, 4) }
    publishBrowserSnapshot()
  },
  setMuted: async (muted) => {
    browserSnapshot = { ...browserSnapshot, muted }
    publishBrowserSnapshot()
  },
  setFullscreen: async (enabled) => {
    if (enabled) {
      await document.documentElement.requestFullscreen()
    } else if (document.fullscreenElement !== null) {
      await document.exitFullscreen()
    }
  },
  openMedia: async () => [],
  subscribeFileDrop: async () => () => undefined,
  minimizeWindow: async () => undefined,
  toggleMaximizeWindow: async () => undefined,
  closeWindow: async () => window.close(),
}

export const desktopApi: DesktopApi = isTauri() ? tauriDesktopApi : browserDesktopApi

export function toDesktopError(error: unknown): DesktopError {
  return normalizeError(error)
}
