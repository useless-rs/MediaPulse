export type BackendKind = "libmpv" | "sidecar"

export type PlaybackPhase =
  | { kind: "idle" }
  | { kind: "playing" }
  | { kind: "paused" }
  | { kind: "ended" }
  | { kind: "error"; message: string }

export interface PlaybackSnapshot {
  phase: PlaybackPhase
  paused: boolean
  volume: number
  muted: boolean
  fullscreen: boolean
  speed: number
  timePosition: number
  duration: number | null
  filename: string
  mediaTitle: string
}

export interface DesktopError {
  code: string
  message: string
}

export type SnapshotListener = (snapshot: PlaybackSnapshot) => void
export type ErrorListener = (error: DesktopError) => void
export type Unsubscribe = () => void

export interface PlaylistItem {
  path: string
  title: string
}

export interface DesktopApi {
  getBackendKind(): Promise<BackendKind>
  getSnapshot(): Promise<PlaybackSnapshot>
  subscribe(onSnapshot: SnapshotListener, onError: ErrorListener): Promise<Unsubscribe>
  load(source: string): Promise<void>
  setPaused(paused: boolean): Promise<void>
  seek(seconds: number): Promise<void>
  setVolume(volume: number): Promise<void>
  setSpeed(speed: number): Promise<void>
  setMuted(muted: boolean): Promise<void>
  setFullscreen(enabled: boolean): Promise<void>
  openMedia(): Promise<string[]>
  subscribeFileDrop(onPaths: (paths: string[]) => void): Promise<() => void>
  minimizeWindow(): Promise<void>
  toggleMaximizeWindow(): Promise<void>
  closeWindow(): Promise<void>
}

export const EMPTY_SNAPSHOT: PlaybackSnapshot = {
  phase: { kind: "idle" },
  paused: false,
  volume: 100,
  muted: false,
  fullscreen: false,
  speed: 1,
  timePosition: 0,
  duration: null,
  filename: "",
  mediaTitle: "",
}
