import { type ButtonHTMLAttributes, forwardRef, type ReactNode } from "react"

import { cn } from "../lib/cn"

interface IconButtonProps extends ButtonHTMLAttributes<HTMLButtonElement> {
  label: string
  active?: boolean
  children: ReactNode
}

export const IconButton = forwardRef<HTMLButtonElement, IconButtonProps>(function IconButton(
  { label, active = false, children, className, type = "button", ...props },
  ref,
) {
  return (
    <button
      ref={ref}
      type={type}
      aria-label={label}
      title={label}
      aria-pressed={active || undefined}
      className={cn("icon-button", active && "icon-button--active", className)}
      {...props}
    >
      {children}
    </button>
  )
})
