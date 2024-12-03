import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'
import { nodePolyfills } from 'vite-plugin-node-polyfills'

// https://vite.dev/config/
export default defineConfig({
  plugins: [
    react(),
    nodePolyfills()
  ],
   // Optional: add fallback values
   define: {
    'import.meta.env.VITE_REMOTE': JSON.stringify(process.env.VITE_REMOTE || '127.0.0.1'),
    'import.meta.env.VITE_REMOTE_PORT': JSON.stringify(process.env.VITE_REMOTE_PORT || '3600')
  }
})
