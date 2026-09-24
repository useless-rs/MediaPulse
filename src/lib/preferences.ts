export function readBooleanPreference(key: string, fallback: boolean): boolean {
  try {
    const value = window.localStorage.getItem(key)
    return value === null ? fallback : value === "true"
  } catch (error: unknown) {
    console.warn(`Could not read preference ${key}`, error)
    return fallback
  }
}

export function writeBooleanPreference(key: string, value: boolean): void {
  try {
    window.localStorage.setItem(key, String(value))
  } catch (error: unknown) {
    console.warn(`Could not persist preference ${key}`, error)
  }
}
