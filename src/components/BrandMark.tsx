import { cn } from "../lib/cn"
import { MediaPulseIcon } from "./MediaPulseIcon"

interface BrandMarkProps {
  size?: "small" | "large"
}

export function BrandMark({ size = "small" }: BrandMarkProps) {
  return (
    <span className={cn("brand-mark", size === "large" && "brand-mark--large")} aria-hidden="true">
      <MediaPulseIcon className="brand-mark__icon" />
    </span>
  )
}
