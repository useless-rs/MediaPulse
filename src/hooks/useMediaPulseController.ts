import { useReducedMotion } from "motion/react"
import { useCallback, useEffect, useRef, useState } from "react"

import { desktopApi, toDesktopError } from "../lib/desktop-api"
import { fileTitle } from "../lib/format"
import {
  type BackendKind,
  EMPTY_SNAPSHOT,
  type PlaybackSnapshot,
  type PlaylistItem,
} from "../lib/playback"
import { readBooleanPreference, writeBooleanPreference } from "../lib/preferences"

const PIN_PREFERENCE = "mediapulse.playlist.pinned"
const AUTO_HIDE_PREFERENCE = "mediapulse.controls.auto-hide"
const AUTO_HIDE_DELAY_MS = 2500

function toPlaylistItems(paths: readonly string[]): PlaylistItem[] {
  return paths.map((path) => ({ path, title: fileTitle(path) }))
}

export function useMediaPulseController() {
  const reduceMotion = useReducedMotion()
  const lastInteraction = useRef(Date.now())
  const [snapshot, setSnapshot] = useState<PlaybackSnapshot>(EMPTY_SNAPSHOT)
  const [backendKind, setBackendKind] = useState<BackendKind>("libmpv")
  const [errorMessage, setErrorMessage] = useState<string | null>(null)
  const [playlistItems, setPlaylistItems] = useState<PlaylistItem[]>([])
  const [selectedPath, setSelectedPath] = useState("")
  const [playlistOpen, setPlaylistOpen] = useState(false)
  const [playlistPinned, setPlaylistPinnedState] = useState(() =>
    readBooleanPreference(PIN_PREFERENCE, false),
  )
  const [settingsOpen, setSettingsOpen] = useState(false)
  const [autoHideControls, setAutoHideControlsState] = useState(() =>
    readBooleanPreference(AUTO_HIDE_PREFERENCE, true),
  )
  const [controlsVisible, setControlsVisible] = useState(true)

  useEffect(() => {
    let cancelled = false
    let unlisten: (() => void) | undefined

    const initialize = async () => {
      try {
        const [kind, initialSnapshot] = await Promise.all([
          desktopApi.getBackendKind(),
          desktopApi.getSnapshot(),
        ])
        if (cancelled) return
        setBackendKind(kind)
        setSnapshot(initialSnapshot)
        if (initialSnapshot.filename.length > 0) {
          const initialItem = {
            path: initialSnapshot.filename,
            title: fileTitle(initialSnapshot.filename),
          }
          setPlaylistItems([initialItem])
          setSelectedPath(initialItem.path)
        }
        const stopListening = await desktopApi.subscribe(
          (nextSnapshot) => {
            setSnapshot(nextSnapshot)
            if (nextSnapshot.filename.length > 0) {
              setPlaylistItems((items) => {
                if (items.some((item) => item.path === nextSnapshot.filename)) return items
                return [
                  ...items,
                  { path: nextSnapshot.filename, title: fileTitle(nextSnapshot.filename) },
                ]
              })
              setSelectedPath(nextSnapshot.filename)
            }
          },
          (error) => setErrorMessage(error.message),
        )
        if (cancelled) stopListening()
        else unlisten = stopListening
      } catch (error: unknown) {
        if (!cancelled) setErrorMessage(toDesktopError(error).message)
      }
    }

    void initialize()
    return () => {
      cancelled = true
      unlisten?.()
    }
  }, [])

  useEffect(() => {
    writeBooleanPreference(PIN_PREFERENCE, playlistPinned)
    if (playlistPinned) setPlaylistOpen(true)
  }, [playlistPinned])

  useEffect(() => {
    writeBooleanPreference(AUTO_HIDE_PREFERENCE, autoHideControls)
  }, [autoHideControls])

  useEffect(() => {
    const keepVisible =
      !autoHideControls || snapshot.paused || playlistOpen || settingsOpen || reduceMotion
    if (keepVisible) {
      setControlsVisible(true)
      return
    }
    const timer = window.setInterval(() => {
      if (Date.now() - lastInteraction.current >= AUTO_HIDE_DELAY_MS) setControlsVisible(false)
    }, 250)
    return () => window.clearInterval(timer)
  }, [autoHideControls, playlistOpen, reduceMotion, settingsOpen, snapshot.paused])

  const revealControls = useCallback(() => {
    lastInteraction.current = Date.now()
    setControlsVisible(true)
  }, [])

  const runAction = useCallback(async (action: () => Promise<void>) => {
    try {
      setErrorMessage(null)
      await action()
    } catch (error: unknown) {
      setErrorMessage(toDesktopError(error).message)
    }
  }, [])

  const setPlaylistPinned = useCallback((pinned: boolean) => {
    setPlaylistPinnedState(pinned)
  }, [])

  const setAutoHideControls = useCallback((enabled: boolean) => {
    setAutoHideControlsState(enabled)
  }, [])

  const openMedia = useCallback(async () => {
    await runAction(async () => {
      const paths = await desktopApi.openMedia()
      const firstPath = paths[0]
      if (firstPath === undefined) return
      setPlaylistItems(toPlaylistItems(paths))
      setSelectedPath(firstPath)
      setPlaylistOpen(true)
      await desktopApi.load(firstPath)
    })
  }, [runAction])

  const selectMedia = useCallback(
    (item: PlaylistItem) => {
      void runAction(async () => {
        setSelectedPath(item.path)
        await desktopApi.load(item.path)
      })
    },
    [runAction],
  )

  const removeMedia = useCallback(
    (item: PlaylistItem) => {
      setPlaylistItems((items) => items.filter((candidate) => candidate.path !== item.path))
      if (item.path === selectedPath) setSelectedPath("")
    },
    [selectedPath],
  )

  const togglePlaylist = useCallback(() => {
    if (playlistOpen && playlistPinned) setPlaylistPinnedState(false)
    setPlaylistOpen((open) => !open)
    revealControls()
  }, [playlistOpen, playlistPinned, revealControls])

  const closePlaylist = useCallback(() => {
    if (playlistPinned) setPlaylistPinnedState(false)
    setPlaylistOpen(false)
    revealControls()
  }, [playlistPinned, revealControls])

  const togglePlayback = useCallback(() => {
    void runAction(() => desktopApi.setPaused(!snapshot.paused))
  }, [runAction, snapshot.paused])

  useEffect(() => {
    const handleKeyDown = (event: KeyboardEvent) => {
      if (event.metaKey || event.ctrlKey || event.altKey) return
      const target = event.target
      if (
        target instanceof HTMLInputElement ||
        target instanceof HTMLTextAreaElement ||
        target instanceof HTMLSelectElement
      )
        return
      const key = event.key.toLowerCase()
      if (event.code === "Space") {
        event.preventDefault()
        togglePlayback()
      } else if (key === "l") togglePlaylist()
      else if (key === "s") setSettingsOpen(true)
      else if (key === "f") void runAction(() => desktopApi.setFullscreen(!snapshot.fullscreen))
      else if (event.key === "ArrowLeft")
        void runAction(() => desktopApi.seek(snapshot.timePosition - 10))
      else if (event.key === "ArrowRight")
        void runAction(() => desktopApi.seek(snapshot.timePosition + 10))
    }
    window.addEventListener("keydown", handleKeyDown)
    return () => window.removeEventListener("keydown", handleKeyDown)
  }, [runAction, snapshot.fullscreen, snapshot.timePosition, togglePlayback, togglePlaylist])

  return {
    state: {
      snapshot,
      backendKind,
      errorMessage,
      playlistItems,
      selectedPath,
      playlistOpen,
      playlistPinned,
      settingsOpen,
      autoHideControls,
      controlsVisible,
      reduceMotion,
    },
    actions: {
      revealControls,
      runAction,
      openMedia,
      selectMedia,
      removeMedia,
      togglePlaylist,
      closePlaylist,
      togglePlayback,
      setPlaylistPinned,
      setAutoHideControls,
      setSettingsOpen,
      setVolume: (volume: number) => runAction(() => desktopApi.setVolume(volume)),
      setMuted: (muted: boolean) => runAction(() => desktopApi.setMuted(muted)),
      setSpeed: (speed: number) => runAction(() => desktopApi.setSpeed(speed)),
      setFullscreen: (enabled: boolean) => runAction(() => desktopApi.setFullscreen(enabled)),
    },
  }
}

export type MediaPulseController = ReturnType<typeof useMediaPulseController>
