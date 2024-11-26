import { sveltekit } from '@sveltejs/kit/vite';
import { readFileSync } from 'fs';
import path from 'node:path';
import { defineConfig } from 'vite';

const json = readFileSync('./package.json', 'utf8');
const pkg = JSON.parse(json);

// https://vitejs.dev/config/
export default defineConfig(async () => ({
  plugins: [sveltekit()],
  resolve: {
    alias: {
      '@src': path.resolve(__dirname, './src'),
      '@lib': path.resolve(__dirname, './src/lib'),
    },
  },

  // Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
  //
  // 1. prevent vite from obscuring rust errors
  clearScreen: false,
  server: {
    port: 1420,
    strictPort: true,
    watch: {
      ignored: ['**/src-tauri/**'],
    },
  },
  define: {
    __APP_VERSION__: JSON.stringify(pkg.version),
  },
}));
