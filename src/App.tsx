import { AnimatePresence, motion } from "motion/react"

import { EmptyStage } from "./components/EmptyStage"
import { PlayerControls } from "./components/PlayerControls"
import { PlaylistPanel } from "./components/PlaylistPanel"
import { SettingsSheet } from "./components/SettingsSheet"
import { TitleBar } from "./components/TitleBar"
import { useMediaPulseController } from "./hooks/useMediaPulseController"
import { desktopApi } from "./lib/desktop-api"
import { isApplePlatform } from "./lib/platform"

export function App() {
  const { state, actions } = useMediaPulseController()
  const title = state.snapshot.mediaTitle || "MediaPulse"
  const backendLabel = state.backendKind === "libmpv" ? "embedded" : "sidecar"

  return (
    <main
      className="app-shell"
      aria-label="MediaPulse"
      onPointerMove={actions.revealControls}
      onFocusCapture={actions.revealControls}
    >
      <TitleBar
        appleStyle={isApplePlatform()}
        title={title}
        backendLabel={backendLabel}
        onMinimize={() => void actions.runAction(desktopApi.minimizeWindow)}
        onMaximize={() => void actions.runAction(desktopApi.toggleMaximizeWindow)}
        onClose={() => void actions.runAction(desktopApi.closeWindow)}
      />

      <div className="app-shell__body">
        <section className="player-shell" aria-label="Player">
          <EmptyStage
            snapshot={state.snapshot}
            errorMessage={state.errorMessage}
            onOpenMedia={() => void actions.openMedia()}
            onOpenPlaylist={actions.togglePlaylist}
          />

          <AnimatePresence initial={false}>
            {state.controlsVisible ? (
              <motion.div
                className="player-controls-layer"
                initial={state.reduceMotion ? { opacity: 0 } : { opacity: 0, y: 8 }}
                animate={{ opacity: 1, y: 0 }}
                exit={state.reduceMotion ? { opacity: 0 } : { opacity: 0, y: 8 }}
                transition={{ duration: state.reduceMotion ? 0 : 0.12 }}
              >
                <PlayerControls
                  snapshot={state.snapshot}
                  onTogglePlay={actions.togglePlayback}
                  onSeek={(seconds) => void actions.runAction(() => desktopApi.seek(seconds))}
                  onVolume={actions.setVolume}
                  onMute={() =>
                    void actions.runAction(() => desktopApi.setMuted(!state.snapshot.muted))
                  }
                  onOpenSettings={() => actions.setSettingsOpen(true)}
                  onOpenPlaylist={actions.togglePlaylist}
                />
              </motion.div>
            ) : null}
          </AnimatePresence>
        </section>

        <PlaylistPanel
          open={state.playlistOpen}
          pinned={state.playlistPinned}
          items={state.playlistItems}
          selectedPath={state.selectedPath}
          onClose={actions.closePlaylist}
          onPinnedChange={actions.setPlaylistPinned}
          onOpenMedia={() => void actions.openMedia()}
          onSelect={actions.selectMedia}
          onRemove={actions.removeMedia}
        />

        <SettingsSheet
          open={state.settingsOpen}
          backendKind={state.backendKind}
          snapshot={state.snapshot}
          playlistPinned={state.playlistPinned}
          autoHideControls={state.autoHideControls}
          onPlaylistPinnedChange={actions.setPlaylistPinned}
          onAutoHideControlsChange={actions.setAutoHideControls}
          onVolumeChange={actions.setVolume}
          onMuteChange={actions.setMuted}
          onSpeedChange={actions.setSpeed}
          onFullscreenChange={actions.setFullscreen}
          onClose={() => actions.setSettingsOpen(false)}
        />
      </div>
    </main>
  )
}
