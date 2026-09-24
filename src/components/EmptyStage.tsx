import { FolderOpen, TriangleAlert } from "lucide-react"
import type { PlaybackSnapshot } from "../lib/playback"
import { BrandMark } from "./BrandMark"

interface EmptyStageProps {
  snapshot: PlaybackSnapshot
  errorMessage: string | null
  onOpenMedia: () => void
  onOpenPlaylist: () => void
}

export function EmptyStage({
  snapshot,
  errorMessage,
  onOpenMedia,
  onOpenPlaylist,
}: EmptyStageProps) {
  const hasMedia = snapshot.filename.length > 0

  return (
    <section
      className="stage"
      data-has-media={hasMedia || undefined}
      aria-label="Video output"
      onDoubleClick={onOpenPlaylist}
    >
      <div className="stage__atmosphere" aria-hidden="true" />
      <div className="native-video-surface" aria-hidden="true" />

      {hasMedia ? (
        <div className="stage__now-playing">
          <span className="stage__live-dot" aria-hidden="true" />
          <span>{snapshot.mediaTitle || snapshot.filename}</span>
        </div>
      ) : (
        <div className="empty-stage">
          <BrandMark size="large" />
          <p className="empty-stage__eyebrow">Native playback, focused by design</p>
          <h1 className="empty-stage__title">Your media, uninterrupted.</h1>
          <p className="empty-stage__body">
            Open a local file and MediaPulse will play it through its private embedded mpv engine.
          </p>
          <div className="empty-stage__actions">
            <button type="button" className="primary-button" onClick={onOpenMedia}>
              <FolderOpen aria-hidden="true" />
              Open media
            </button>
            <button type="button" className="secondary-button" onClick={onOpenPlaylist}>
              View playlist
            </button>
          </div>
        </div>
      )}

      {errorMessage !== null ? (
        <div className="stage-error" role="alert">
          <TriangleAlert aria-hidden="true" />
          <span>{errorMessage}</span>
        </div>
      ) : null}
    </section>
  )
}
