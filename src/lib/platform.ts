export function isApplePlatform(): boolean {
  const platform = typeof navigator === "undefined" ? "" : navigator.platform || navigator.userAgent
  return /Mac|iPhone|iPad|iPod/i.test(platform)
}

export function isLinux(): boolean {
  const platform = typeof navigator === "undefined" ? "" : navigator.platform || navigator.userAgent
  return /Linux|X11/i.test(platform)
}
