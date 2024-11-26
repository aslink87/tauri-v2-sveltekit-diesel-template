import adapter from '@sveltejs/adapter-static';
import { vitePreprocess } from '@sveltejs/vite-plugin-svelte';
import path from 'path';

/** @type {import('@sveltejs/kit').Config} */
const config = {
  preprocess: vitePreprocess(),
  kit: {
    adapter: adapter(),
    alias: {
      // this will match a file
      // 'my-file': 'path/to/my-file.js',

      // this will match a directory and its contents
      // (`my-directory/x` resolves to `path/to/my-directory/x`)
      $lib: path.resolve('./src/lib'),

      // an alias ending /* will only match
      // the contents of a directory, not the directory itself
      // 'my-directory/*': 'path/to/my-directory/*'
    },
  },
};

export default config;
