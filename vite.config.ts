import process from "node:process"
import tailwindcss from "@tailwindcss/vite"
import react from "@vitejs/plugin-react"
import { defineConfig } from "vite"

const { TAURI_DEV_HOST: host } = process.env

export default defineConfig({
  plugins: [react(), tailwindcss()],
  build: {
    outDir: "src-tauri/dist",
    emptyOutDir: true,
  },
  clearScreen: false,
  server: {
    port: 1420,
    strictPort: true,
    host: host || false,
    ...(host
      ? {
          hmr: {
            protocol: "ws" as const,
            host,
            port: 1421,
          },
        }
      : {}),
    watch: {
      ignored: ["**/src-tauri/**", "**/crates/**"],
    },
  },
})
