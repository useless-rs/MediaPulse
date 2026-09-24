import type { ReactNode } from "react"

import type { BackendKind, PlaybackSnapshot } from "../lib/playback"

export type SettingsView = "playback" | "audio" | "video" | "shortcuts" | "about"

interface SettingsContentProps {
  activeView: SettingsView
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
}

export function SettingsContent({
  activeView,
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
}: SettingsContentProps) {
  switch (activeView) {
    case "playback":
      return (
        <SettingsGroup
          title="Playback controls"
          description="Keep the player surface quiet without losing access."
        >
          <PreferenceToggle
            label="Auto-hide playback controls"
            description="Hide the OSC after inactivity during playback."
            checked={autoHideControls}
            onChange={onAutoHideControlsChange}
          />
          <PreferenceToggle
            label="Keep playlist open"
            description="Remember the playlist as a persistent right sidebar."
            checked={playlistPinned}
            onChange={onPlaylistPinnedChange}
          />
        </SettingsGroup>
      )
    case "audio":
      return (
        <SettingsGroup title="Audio" description="Output volume and playback speed.">
          <label className="settings-field">
            <span>Volume</span>
            <input
              className="range"
              type="range"
              min="0"
              max="130"
              value={snapshot.muted ? 0 : snapshot.volume}
              onChange={(event) => onVolumeChange(event.currentTarget.valueAsNumber)}
            />
          </label>
          <PreferenceToggle
            label="Mute audio"
            description="Silence output without changing the volume level."
            checked={snapshot.muted}
            onChange={onMuteChange}
          />
          <fieldset className="speed-control">
            <legend className="visually-hidden">Playback speed</legend>
            {[0.5, 1, 1.5, 2].map((speed) => (
              <button
                type="button"
                key={speed}
                aria-pressed={snapshot.speed === speed}
                onClick={() => onSpeedChange(speed)}
              >
                {speed}×
              </button>
            ))}
          </fieldset>
        </SettingsGroup>
      )
    case "video":
      return (
        <SettingsGroup
          title="Video output"
          description="MediaPulse renders through its embedded native engine."
        >
          <div className="settings-readout">
            <span>Renderer</span>
            <strong>{backendKind === "libmpv" ? "Embedded libmpv" : "mpv sidecar"}</strong>
          </div>
          <button
            type="button"
            className="secondary-button settings-fullscreen"
            aria-pressed={snapshot.fullscreen}
            onClick={() => onFullscreenChange(!snapshot.fullscreen)}
          >
            {snapshot.fullscreen ? "Leave fullscreen" : "Enter fullscreen"}
          </button>
        </SettingsGroup>
      )
    case "shortcuts":
      return <ShortcutSettings />
    case "about":
      return (
        <SettingsGroup title="About MediaPulse" description="Private, offline-first playback.">
          <div className="settings-readout">
            <span>Version</span>
            <strong>0.1.0</strong>
          </div>
          <div className="settings-readout">
            <span>License</span>
            <strong>GNU GPL v3.0 or later</strong>
          </div>
          <p className="settings-note">
            MediaPulse never requires an account or uploads local media.
          </p>
        </SettingsGroup>
      )
    default:
      return null
  }
}

interface SettingsGroupProps {
  title: string
  description: string
  children: ReactNode
}

function SettingsGroup({ title, description, children }: SettingsGroupProps) {
  return (
    <section className="settings-group">
      <h3>{title}</h3>
      <p>{description}</p>
      <div className="settings-group__content">{children}</div>
    </section>
  )
}

interface PreferenceToggleProps {
  label: string
  description: string
  checked: boolean
  onChange: (checked: boolean) => void
}

function PreferenceToggle({ label, description, checked, onChange }: PreferenceToggleProps) {
  return (
    <label className="preference-toggle">
      <span>
        <strong>{label}</strong>
        <small>{description}</small>
      </span>
      <input
        type="checkbox"
        checked={checked}
        onChange={(event) => onChange(event.currentTarget.checked)}
      />
      <span className="preference-toggle__track" aria-hidden="true" />
    </label>
  )
}

function ShortcutSettings() {
  return (
    <SettingsGroup
      title="Keyboard shortcuts"
      description="Inputs and text fields are never intercepted."
    >
      <dl className="shortcut-list">
        <div>
          <dt>Play / pause</dt>
          <dd>Space</dd>
        </div>
        <div>
          <dt>Seek backward / forward</dt>
          <dd>← / →</dd>
        </div>
        <div>
          <dt>Playlist</dt>
          <dd>L</dd>
        </div>
        <div>
          <dt>Settings</dt>
          <dd>S</dd>
        </div>
        <div>
          <dt>Fullscreen</dt>
          <dd>F</dd>
        </div>
      </dl>
    </SettingsGroup>
  )
}
