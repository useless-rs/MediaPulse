import { Minus, Square, X } from "lucide-react"

import { cn } from "../lib/cn"
import { BrandMark } from "./BrandMark"
import { IconButton } from "./IconButton"

interface TitleBarProps {
  appleStyle: boolean
  title: string
  backendLabel: string
  onMinimize: () => void
  onMaximize: () => void
  onClose: () => void
}

export function TitleBar({
  appleStyle,
  title,
  backendLabel,
  onMinimize,
  onMaximize,
  onClose,
}: TitleBarProps) {
  return (
    <header
      className={cn("titlebar", appleStyle && "titlebar--apple")}
      data-tauri-drag-region
      data-tauri-drag-region-everywhere
    >
      <div className="titlebar__identity" data-tauri-drag-region>
        <BrandMark />
        <span className="titlebar__title" data-tauri-drag-region>
          {title}
        </span>
        <span className="titlebar__backend" data-tauri-drag-region>
          {backendLabel}
        </span>
      </div>

      {!appleStyle ? (
        <fieldset className="titlebar__window-controls">
          <legend className="visually-hidden">Window controls</legend>
          <IconButton label="Minimize window" onClick={onMinimize} className="window-control">
            <Minus />
          </IconButton>
          <IconButton label="Maximize window" onClick={onMaximize} className="window-control">
            <Square />
          </IconButton>
          <IconButton
            label="Close window"
            onClick={onClose}
            className="window-control window-control--close"
          >
            <X />
          </IconButton>
        </fieldset>
      ) : null}
    </header>
  )
}
