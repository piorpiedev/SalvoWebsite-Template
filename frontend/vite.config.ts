import { defineConfig } from 'vite'
import { svelte } from '@sveltejs/vite-plugin-svelte'
import tailwindcss from '@tailwindcss/vite'
import { router } from 'sv-router/vite-plugin'

// https://vite.dev/config/
export default defineConfig({
  plugins: [
    tailwindcss(),
    svelte(),
    router(),
  ],
  server: {
    proxy: {
      "/api": {
        target: "http://127.0.0.1:8008",
        changeOrigin: true,
      }
    }
  }
})
