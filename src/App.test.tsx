import { render, screen, waitFor } from "@testing-library/react"
import userEvent from "@testing-library/user-event"
import { beforeEach, describe, expect, it, vi } from "vitest"
import { App } from "./App"
import { EMPTY_SNAPSHOT } from "./lib/playback"

const mocks = vi.hoisted(() => ({
  getBackendKind: vi.fn(),
  getSnapshot: vi.fn(),
  subscribe: vi.fn(),
  load: vi.fn(),
  setPaused: vi.fn(),
  seek: vi.fn(),
  setVolume: vi.fn(),
  setMuted: vi.fn(),
  setFullscreen: vi.fn(),
  openMedia: vi.fn(),
  subscribeFileDrop: vi.fn(),
  minimizeWindow: vi.fn(),
  toggleMaximizeWindow: vi.fn(),
  closeWindow: vi.fn(),
}))

vi.mock("./lib/desktop-api", () => ({
  desktopApi: {
    ...mocks,
  },
}))

describe("MediaPulse player", () => {
  beforeEach(() => {
    vi.clearAllMocks()
    mocks.getBackendKind.mockResolvedValue("libmpv")
    mocks.getSnapshot.mockResolvedValue(EMPTY_SNAPSHOT)
    mocks.subscribe.mockResolvedValue(() => undefined)
    mocks.subscribeFileDrop.mockResolvedValue(() => undefined)
    mocks.openMedia.mockResolvedValue([])
  })

  it("renders the approved MediaPulse brand mark", () => {
    const { container } = render(<App />)

    expect(container.querySelectorAll('svg[aria-label="MediaPulse"]')).toHaveLength(2)
  })

  it("keeps the empty state focused before media is loaded", () => {
    render(<App />)

    expect(screen.queryByRole("region", { name: "Playback controls" })).not.toBeInTheDocument()
    expect(screen.getByRole("button", { name: "Open playlist" })).toBeInTheDocument()
    expect(screen.queryByText("Native playback, focused by design")).not.toBeInTheDocument()
  })

  it("does not repeat the playlist action while the queue is open", async () => {
    const user = userEvent.setup()
    render(<App />)

    await user.click(screen.getByRole("button", { name: "Open playlist" }))

    expect(screen.queryByRole("button", { name: "Open playlist" })).not.toBeInTheDocument()
  })

  it("opens selected media and exposes it in the playlist", async () => {
    const user = userEvent.setup()
    mocks.openMedia.mockResolvedValue(["/media/creator-reference.mkv"])
    render(<App />)

    await user.click(screen.getByRole("button", { name: "Open media" }))

    await waitFor(() => {
      expect(mocks.load).toHaveBeenCalledWith("/media/creator-reference.mkv")
    })
    expect(
      await screen.findByRole("button", { name: "creator-reference.mkv, playing" }),
    ).toBeInTheDocument()
  })

  it("loads a file dropped onto the window", async () => {
    let deliverDrop: ((paths: string[]) => void) | undefined
    mocks.subscribeFileDrop.mockImplementation(async (onPaths: (paths: string[]) => void) => {
      deliverDrop = onPaths
      return () => undefined
    })
    render(<App />)

    await waitFor(() => {
      expect(deliverDrop).toBeDefined()
    })
    deliverDrop?.(["/media/dropped-clip.mp4"])

    await waitFor(() => {
      expect(mocks.load).toHaveBeenCalledWith("/media/dropped-clip.mp4")
    })
    expect(
      await screen.findByRole("button", { name: "dropped-clip.mp4, playing" }),
    ).toBeInTheDocument()
  })

  it("opens and closes the playlist and settings surfaces", async () => {
    const user = userEvent.setup()
    render(<App />)

    await user.click(screen.getByRole("button", { name: "Open playlist" }))
    expect(await screen.findByRole("complementary", { name: "Playlist" })).toBeInTheDocument()

    await user.click(screen.getByRole("button", { name: "Open settings" }))
    expect(await screen.findByRole("dialog", { name: "Settings" })).toBeInTheDocument()
    await user.click(screen.getByRole("button", { name: "Close settings" }))
    await waitFor(() => {
      expect(screen.queryByRole("dialog", { name: "Settings" })).not.toBeInTheDocument()
    })
  })

  it("supports playback and playlist keyboard shortcuts", async () => {
    const user = userEvent.setup()
    render(<App />)

    await user.keyboard(" ")
    expect(mocks.setPaused).toHaveBeenCalledWith(true)

    await user.keyboard("l")
    expect(await screen.findByRole("complementary", { name: "Playlist" })).toBeInTheDocument()
  })

  it("routes custom titlebar window actions", async () => {
    const user = userEvent.setup()
    render(<App />)

    await user.click(screen.getByRole("button", { name: "Minimize window" }))
    await user.click(screen.getByRole("button", { name: "Maximize window" }))
    await user.click(screen.getByRole("button", { name: "Close window" }))

    expect(mocks.minimizeWindow).toHaveBeenCalledOnce()
    expect(mocks.toggleMaximizeWindow).toHaveBeenCalledOnce()
    expect(mocks.closeWindow).toHaveBeenCalledOnce()
  })
})
