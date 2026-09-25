import { FolderOpen, TriangleAlert } from "lucide-react"
import { isLinux } from "../lib/platform"
import type { PlaybackSnapshot } from "../lib/playback"
import { BrandMark } from "./BrandMark"

interface EmptyStageProps {
  snapshot: PlaybackSnapshot
  errorMessage: string | null
  playlistOpen: boolean
  onOpenMedia: () => void
  onOpenPlaylist: () => void
}

export function EmptyStage({
  snapshot,
  errorMessage,
  playlistOpen,
  onOpenMedia,
  onOpenPlaylist,
}: EmptyStageProps) {
  const hasMedia = snapshot.filename.length > 0
  const detachedVideoNotice = hasMedia && isLinux() ? "Picture is playing in its own window." : null
  const error =
    errorMessage === null ? null : (
      <div className={hasMedia ? "stage-error" : "stage-error stage-error--inline"} role="alert">
        <TriangleAlert aria-hidden="true" />
        <span>{errorMessage}</span>
      </div>
    )

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
          {detachedVideoNotice ? (
            <span className="stage__detached-note">{detachedVideoNotice}</span>
          ) : null}
        </div>
      ) : (
        <div className="empty-stage">
          <BrandMark size="large" />
          <h1 className="empty-stage__title">Nothing playing yet.</h1>
          <p className="empty-stage__body">Choose a local file to start.</p>
          <div className="empty-stage__actions">
            <button type="button" className="primary-button" onClick={onOpenMedia}>
              <FolderOpen aria-hidden="true" />
              Open media
            </button>
            {playlistOpen ? null : (
              <button type="button" className="secondary-button" onClick={onOpenPlaylist}>
                Open playlist
              </button>
            )}
          </div>
          {error}
        </div>
      )}

      {hasMedia ? error : null}
    </section>
  )
}
