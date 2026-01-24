import { defineConfig } from 'vite';
import solid from 'vite-plugin-solid';
import path from 'path';

export default defineConfig({
  plugins: [solid()],
  base: process.env.VITE_BASE_PATH || '/',
  build: {
    outDir: 'dist',
    assetsDir: 'assets',
  },
  resolve: {
    alias: {
      // Resolve @shared to ../shared (works in both local dev and Docker)
      // In Docker: /build/../shared = /shared
      // In local: ui/website/../shared = ui/shared
      '@shared': path.resolve(__dirname, '../shared'),
      '@shared-portals': path.resolve(__dirname, '../shared-portals'),
    },
    // Ensure dependencies are resolved from website's node_modules
    dedupe: ['solid-js'],
  },
  optimizeDeps: {
    // Include shared components in dependency optimization
    include: ['solid-js'],
  },
  server: {
    port: 3000,
    open: true,
  },
});
