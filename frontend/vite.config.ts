import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'
import wasm from 'vite-plugin-wasm'

export default defineConfig({
  plugins: [vue(), wasm()],
  server: {
    port: 5173,
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
    exclude: ['satisflow-wasm']
  }
})