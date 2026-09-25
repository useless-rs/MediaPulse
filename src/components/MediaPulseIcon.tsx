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
      <rect x="48" y="48" width="928" height="928" rx="236" fill="#1b2027" />
      <rect
        x="66"
        y="66"
        width="892"
        height="892"
        rx="218"
        fill="none"
        stroke="#3b4653"
        strokeWidth="12"
      />
      <path
        d="M170 290c72-62 166-94 274-94 134 0 252 52 338 140"
        fill="none"
        stroke="#7d8a99"
        strokeWidth="14"
        strokeLinecap="round"
        opacity="0.22"
      />
      <path
        d="M164 512h144c42 0 56-90 94-90 40 0 52 180 94 180 42 0 56-90 98-90h166"
        fill="none"
        stroke="#61d9f7"
        strokeWidth="34"
        strokeLinecap="round"
        strokeLinejoin="round"
      />
      <path d="M420 350 420 674 650 512Z" fill="#5cd8f5" />
      <path
        d="M420 350 650 512 420 674"
        fill="none"
        stroke="#c0f5ff"
        strokeWidth="8"
        strokeLinejoin="round"
        opacity="0.72"
      />
    </svg>
  )
}
