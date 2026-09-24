import { Activity, Play } from "lucide-react"

import { cn } from "../lib/cn"

interface BrandMarkProps {
  size?: "small" | "large"
}

export function BrandMark({ size = "small" }: BrandMarkProps) {
  return (
    <span className={cn("brand-mark", size === "large" && "brand-mark--large")} aria-hidden="true">
      <span className="brand-mark__glyph">
        <Play fill="currentColor" />
      </span>
      <Activity className="brand-mark__pulse" />
    </span>
  )
}
