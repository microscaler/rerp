import { defineConfig } from 'vite';
import solid from 'vite-plugin-solid';

export default defineConfig({
  plugins: [solid()],
  base: process.env.VITE_BASE_PATH || '/',
  build: {
    outDir: 'dist',
    assetsDir: 'assets',
  },
  resolve: {
    dedupe: ['solid-js'],
  },
  server: {
    port: 3000,
    open: true,
  },
});
