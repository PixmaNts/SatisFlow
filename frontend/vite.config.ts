import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'

export default defineConfig({
  plugins: [vue()],
  server: {
    port: 5173,
    fs: {
      // Allow serving files from WASM directory
      allow: ['..']
    },
    proxy: {
      '/api': {
        target: 'http://localhost:3000',
        changeOrigin: true
      }
    }
  },
  build: {
    target: 'esnext'
  },
  optimizeDeps: {
    // Exclude WASM files from pre-bundling
    exclude: ['./src/wasm/satisflow_wasm.js']
  }
})
