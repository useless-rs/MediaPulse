import { FolderPlus, Pin, PinOff, Play, Trash2, X } from "lucide-react"
import { AnimatePresence, motion, useReducedMotion } from "motion/react"

import { cn } from "../lib/cn"
import type { PlaylistItem } from "../lib/playback"
import { IconButton } from "./IconButton"

interface PlaylistPanelProps {
  open: boolean
  pinned: boolean
  items: readonly PlaylistItem[]
  selectedPath: string
  onClose: () => void
  onPinnedChange: (pinned: boolean) => void
  onOpenMedia: () => void
  onSelect: (item: PlaylistItem) => void
  onRemove: (item: PlaylistItem) => void
}

export function PlaylistPanel({
  open,
  pinned,
  items,
  selectedPath,
  onClose,
  onPinnedChange,
  onOpenMedia,
  onSelect,
  onRemove,
}: PlaylistPanelProps) {
  const reduceMotion = useReducedMotion()

  return (
    <AnimatePresence initial={false}>
      {open ? (
        <>
          <motion.button
            type="button"
            className="playlist-dismiss"
            aria-label="Dismiss playlist"
            initial={{ opacity: 0 }}
            animate={{ opacity: 1 }}
            exit={{ opacity: 0 }}
            transition={{ duration: reduceMotion ? 0 : 0.12 }}
            onClick={onClose}
          />
          <motion.aside
            className={cn("playlist-panel", pinned && "playlist-panel--pinned")}
            role="complementary"
            aria-label="Playlist"
            initial={reduceMotion ? { opacity: 0 } : { x: "100%" }}
            animate={reduceMotion ? { opacity: 1 } : { x: 0 }}
            exit={reduceMotion ? { opacity: 0 } : { x: "100%" }}
            transition={
              reduceMotion
                ? { duration: 0.12 }
                : { type: "spring", stiffness: 420, damping: 32, mass: 0.8 }
            }
          >
            <header className="playlist-panel__header">
              <div>
                <p className="panel-eyebrow">Queue</p>
                <h2 className="panel-title">Playlist</h2>
              </div>
              <div className="cluster">
                <IconButton
                  label={pinned ? "Unpin playlist" : "Pin playlist"}
                  active={pinned}
                  onClick={() => onPinnedChange(!pinned)}
                >
                  {pinned ? <PinOff /> : <Pin />}
                </IconButton>
                <IconButton label="Close playlist" onClick={onClose}>
                  <X />
                </IconButton>
              </div>
            </header>

            <div className="playlist-panel__toolbar">
              <span className="playlist-panel__count">
                {items.length} {items.length === 1 ? "item" : "items"}
              </span>
              <button type="button" className="text-button" onClick={onOpenMedia}>
                <FolderPlus aria-hidden="true" />
                Add media
              </button>
            </div>

            <ul className="playlist-panel__items" aria-label="Playlist items">
              {items.length === 0 ? (
                <li className="playlist-empty">
                  <FolderPlus aria-hidden="true" />
                  <p>Your playlist is empty.</p>
                  <span>Add local media to keep it close.</span>
                </li>
              ) : (
                items.map((item) => {
                  const selected = item.path === selectedPath
                  return (
                    <li
                      className="playlist-row"
                      data-selected={selected || undefined}
                      key={item.path}
                    >
                      <button
                        type="button"
                        className="playlist-row__select"
                        aria-label={`${item.title}${selected ? ", playing" : ""}`}
                        onClick={() => onSelect(item)}
                      >
                        <span className="playlist-row__index" aria-hidden="true">
                          {selected ? (
                            <Play fill="currentColor" />
                          ) : (
                            String(items.indexOf(item) + 1)
                          )}
                        </span>
                        <span className="playlist-row__copy">
                          <strong>{item.title}</strong>
                          <span>{item.path}</span>
                        </span>
                      </button>
                      <IconButton
                        label={`Remove ${item.title}`}
                        className="playlist-row__remove"
                        onClick={() => onRemove(item)}
                      >
                        <Trash2 />
                      </IconButton>
                    </li>
                  )
                })
              )}
            </ul>
          </motion.aside>
        </>
      ) : null}
    </AnimatePresence>
  )
}
