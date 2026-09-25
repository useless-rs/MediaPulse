interface MediaPulseIconProps {
  className?: string
}

export function MediaPulseIcon({ className }: MediaPulseIconProps) {
  return (
    <svg
      className={className}
      viewBox="0 0 1024 1024"
      role="img"
      aria-label="MediaPulse"
      focusable="false"
    >
      <rect x="56" y="56" width="912" height="912" rx="224" fill="#202832" />
      <rect
        x="72"
        y="72"
        width="880"
        height="880"
        rx="208"
        fill="none"
        stroke="#33404e"
        strokeWidth="16"
      />
      <path
        d="M224 512h116l36-96 58 192 66-252 58 156h86l38-74 42 148 38-74h128"
        fill="none"
        stroke="#59d7ff"
        strokeWidth="42"
        strokeLinecap="round"
        strokeLinejoin="round"
        opacity="0.88"
      />
      <path d="M444 376 444 648 618 512Z" fill="#59d7ff" />
      <circle
        cx="512"
        cy="512"
        r="346"
        fill="none"
        stroke="#59d7ff"
        strokeWidth="8"
        opacity="0.12"
      />
    </svg>
  )
}
