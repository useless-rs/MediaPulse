import { Info, Keyboard, SlidersHorizontal, Video, Volume2, X } from "lucide-react"
import { AnimatePresence, motion, useReducedMotion } from "motion/react"
import { useEffect, useRef, useState } from "react"

import type { BackendKind, PlaybackSnapshot } from "../lib/playback"
import { IconButton } from "./IconButton"
import { SettingsContent, type SettingsView } from "./SettingsContent"

interface SettingsSheetProps {
  open: boolean
  backendKind: BackendKind
  snapshot: PlaybackSnapshot
  playlistPinned: boolean
  autoHideControls: boolean
  onPlaylistPinnedChange: (pinned: boolean) => void
  onAutoHideControlsChange: (enabled: boolean) => void
  onVolumeChange: (volume: number) => void
  onMuteChange: (muted: boolean) => void
  onSpeedChange: (speed: number) => void
  onFullscreenChange: (enabled: boolean) => void
  onClose: () => void
}

const VIEWS: ReadonlyArray<{ id: SettingsView; label: string; icon: typeof SlidersHorizontal }> = [
  { id: "playback", label: "Playback", icon: SlidersHorizontal },
  { id: "audio", label: "Audio", icon: Volume2 },
  { id: "video", label: "Video", icon: Video },
  { id: "shortcuts", label: "Shortcuts", icon: Keyboard },
  { id: "about", label: "About", icon: Info },
]

export function SettingsSheet({
  open,
  backendKind,
  snapshot,
  playlistPinned,
  autoHideControls,
  onPlaylistPinnedChange,
  onAutoHideControlsChange,
  onVolumeChange,
  onMuteChange,
  onSpeedChange,
  onFullscreenChange,
  onClose,
}: SettingsSheetProps) {
  const reduceMotion = useReducedMotion()
  const closeButtonRef = useRef<HTMLButtonElement>(null)
  const [activeView, setActiveView] = useState<SettingsView>("playback")

  useEffect(() => {
    if (!open) return
    const previousFocus = document.activeElement
    const handleKeyDown = (event: KeyboardEvent) => {
      if (event.key === "Escape") onClose()
    }
    document.addEventListener("keydown", handleKeyDown)
    queueMicrotask(() => closeButtonRef.current?.focus())
    return () => {
      document.removeEventListener("keydown", handleKeyDown)
      if (previousFocus instanceof HTMLElement) previousFocus.focus()
    }
  }, [onClose, open])

  return (
    <AnimatePresence initial={false}>
      {open ? (
        <>
          <motion.button
            type="button"
            className="settings-backdrop"
            aria-label="Close settings backdrop"
            initial={{ opacity: 0 }}
            animate={{ opacity: 1 }}
            exit={{ opacity: 0 }}
            transition={{ duration: reduceMotion ? 0 : 0.12 }}
            onClick={onClose}
          />
          <div className="settings-layer">
            <motion.section
              className="settings-sheet"
              role="dialog"
              aria-modal="true"
              aria-label="Settings"
              initial={reduceMotion ? { opacity: 0 } : { opacity: 0, y: 20, scale: 0.97 }}
              animate={reduceMotion ? { opacity: 1 } : { opacity: 1, y: 0, scale: 1 }}
              exit={reduceMotion ? { opacity: 0 } : { opacity: 0, y: 12, scale: 0.98 }}
              transition={
                reduceMotion
                  ? { duration: 0.12 }
                  : { type: "spring", stiffness: 420, damping: 32, mass: 0.8 }
              }
            >
              <header className="settings-sheet__header">
                <div>
                  <p className="panel-eyebrow">MediaPulse</p>
                  <h2 className="panel-title">Settings</h2>
                </div>
                <IconButton label="Close settings" onClick={onClose} ref={closeButtonRef}>
                  <X />
                </IconButton>
              </header>

              <div className="settings-sheet__body">
                <nav className="settings-nav" aria-label="Settings sections">
                  {VIEWS.map((view) => {
                    const ViewIcon = view.icon
                    return (
                      <button
                        type="button"
                        key={view.id}
                        aria-current={activeView === view.id ? "page" : undefined}
                        onClick={() => setActiveView(view.id)}
                      >
                        <ViewIcon aria-hidden="true" />
                        {view.label}
                      </button>
                    )
                  })}
                </nav>

                <div className="settings-content" key={activeView}>
                  <SettingsContent
                    activeView={activeView}
                    backendKind={backendKind}
                    snapshot={snapshot}
                    playlistPinned={playlistPinned}
                    autoHideControls={autoHideControls}
                    onPlaylistPinnedChange={onPlaylistPinnedChange}
                    onAutoHideControlsChange={onAutoHideControlsChange}
                    onVolumeChange={onVolumeChange}
                    onMuteChange={onMuteChange}
                    onSpeedChange={onSpeedChange}
                    onFullscreenChange={onFullscreenChange}
                  />
                </div>
              </div>
            </motion.section>
          </div>
        </>
      ) : null}
    </AnimatePresence>
  )
}
