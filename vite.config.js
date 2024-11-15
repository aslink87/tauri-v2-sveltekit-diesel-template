import { defineConfig } from 'vite';
import { sveltekit } from '@sveltejs/kit/vite';

// https://vitejs.dev/config/
export default defineConfig(async () => ({
  plugins: [sveltekit()],
  clearScreen: false,
  server: {
    port: 1420,
    strictPort: true,
    watch: {
      ignored: ['**/src-tauri/**'],
    },
  },
  define: {
    APP_VERSION: JSON.stringify(process.env.npm_package_version),
  },
}));
