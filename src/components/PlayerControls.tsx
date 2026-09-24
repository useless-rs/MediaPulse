import {
  ListVideo,
  Pause,
  Play,
  Settings,
  SkipBack,
  SkipForward,
  Volume2,
  VolumeX,
} from "lucide-react"

import { formatTime } from "../lib/format"
import type { PlaybackSnapshot } from "../lib/playback"
import { IconButton } from "./IconButton"

interface PlayerControlsProps {
  snapshot: PlaybackSnapshot
  onTogglePlay: () => void
  onSeek: (seconds: number) => void
  onVolume: (volume: number) => void
  onMute: () => void
  onOpenSettings: () => void
  onOpenPlaylist: () => void
}

const SKIP_SECONDS = 10

export function PlayerControls({
  snapshot,
  onTogglePlay,
  onSeek,
  onVolume,
  onMute,
  onOpenSettings,
  onOpenPlaylist,
}: PlayerControlsProps) {
  const duration = snapshot.duration ?? 0
  const hasMedia = snapshot.filename.length > 0
  const progress = duration > 0 ? (snapshot.timePosition / duration) * 100 : 0

  return (
    <section className="osc" aria-label="Playback controls">
      <div className="osc__top-row">
        <div className="osc__volume-group">
          <IconButton label={snapshot.muted ? "Unmute" : "Mute"} onClick={onMute}>
            {snapshot.muted ? <VolumeX /> : <Volume2 />}
          </IconButton>
          <input
            className="range range--volume"
            type="range"
            min="0"
            max="130"
            value={snapshot.muted ? 0 : snapshot.volume}
            aria-label="Volume"
            onChange={(event) => onVolume(event.currentTarget.valueAsNumber)}
          />
        </div>

        <div className="osc__transport">
          <IconButton
            label="Skip back 10 seconds"
            disabled={!hasMedia}
            onClick={() => onSeek(snapshot.timePosition - SKIP_SECONDS)}
          >
            <SkipBack fill="currentColor" />
          </IconButton>
          <IconButton
            label={snapshot.paused ? "Play" : "Pause"}
            className="osc__play"
            disabled={!hasMedia}
            onClick={onTogglePlay}
          >
            {snapshot.paused ? <Play fill="currentColor" /> : <Pause fill="currentColor" />}
          </IconButton>
          <IconButton
            label="Skip forward 10 seconds"
            disabled={!hasMedia}
            onClick={() => onSeek(snapshot.timePosition + SKIP_SECONDS)}
          >
            <SkipForward fill="currentColor" />
          </IconButton>
        </div>

        <div className="osc__utility-group">
          <IconButton label="Open settings" onClick={onOpenSettings}>
            <Settings />
          </IconButton>
          <IconButton label="Open playlist" onClick={onOpenPlaylist}>
            <ListVideo />
          </IconButton>
        </div>
      </div>

      <div className="osc__timeline-row">
        <span className="osc__time">{formatTime(snapshot.timePosition)}</span>
        <input
          className="range range--progress"
          type="range"
          min="0"
          max={Math.max(duration, 0)}
          step="0.1"
          value={Math.min(snapshot.timePosition, Math.max(duration, 0))}
          aria-label="Playback position"
          disabled={!hasMedia || duration <= 0}
          style={{ backgroundSize: `${progress}% 100%` }}
          onChange={(event) => onSeek(event.currentTarget.valueAsNumber)}
        />
        <span className="osc__time">{formatTime(duration)}</span>
      </div>
    </section>
  )
}
