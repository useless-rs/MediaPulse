import { StrictMode } from "react"
import { createRoot } from "react-dom/client"
import { App } from "./App"
import "./index.css"

const rootElement = document.querySelector<HTMLElement>("#root")

if (rootElement === null) {
  throw new TypeError("MediaPulse root element is missing")
}

const devToolsEnabled = import.meta.env.DEV && import.meta.env.VITE_DISABLE_REACT_DEVTOOLS !== "1"

if (devToolsEnabled) {
  void Promise.all([import("react-grab"), import("react-scan")]).catch((error: unknown) => {
    if (error instanceof Error) {
      console.error("React development tools failed to load", error)
      return
    }
    throw error
  })
}

createRoot(rootElement).render(
  <StrictMode>
    <App />
  </StrictMode>,
)
