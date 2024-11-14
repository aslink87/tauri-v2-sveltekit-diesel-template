/* eslint-disable no-underscore-dangle */
import { FlatCompat } from '@eslint/eslintrc';
import prettierConfig from 'eslint-config-prettier';
import eslintPluginSvelte from 'eslint-plugin-svelte';
import path from 'path';
import js from '@eslint/js';
import { fileURLToPath } from 'url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

const compat = new FlatCompat({
  baseDirectory: __dirname, // Optional, default to process.cwd()
});

export default [
  js.configs.recommended,
  ...eslintPluginSvelte.configs['flat/recommended'],
  ...eslintPluginSvelte.configs['flat/prettier'],
  prettierConfig,
  ...compat.extends('eslint-config-airbnb-base'),
  {
    rules: {
      'import/no-extraneous-dependencies': 0,
      'import/no-mutable-exports': 0,
      'import/extentions': 0,
      'import/first': 0,
      'linebreak-style': 'warn',
      'no-param-reassign': 0,
      'prefer-const': 0,
    },
    languageOptions: {
      ecmaVersion: 2020,
      sourceType: 'module',
      parserOptions: {
        parser: '@typescript-eslint/parser',
      },
    },
  },
  {
    ignores: ['server', 'node_modules', 'build', 'build-test', '.svelte-kit', '.env', '.env.*'],
  },
];
// .DS_Store
// node_modules
// /build
// /build-test
// /.svelte-kit
// /package
// .env
// .env.*
// /docker-data
// /server
//
// # Ignore files for PNPM, NPM and YARN
// pnpm-lock.yaml
// package-lock.json
// yarn.lock
// module.exports = {
//   root: true,
//   parser: '@typescript-eslint/parser',
//   extends: [
//     'eslint:recommended',
//     'plugin:@typescript-eslint/recommended',
//     'plugin:svelte/recommended',
//     'airbnb-base',
//     'airbnb-typescript/base',
//     'plugin:prettier/recommended',
//   ],
//   plugins: ['import-no-duplicates-prefix-resolved-path', '@typescript-eslint'],
//   ignorePatterns: ['*.cjs'],
//   // these rules are applied to .svelte files as they are causing unnecessary bugs
//   parserOptions: {
//     sourceType: 'module',
//     ecmaVersion: 2020,
//     tsconfigRootDir: __dirname,
//     project: './tsconfig.json',
//     extraFileExtensions: ['.svelte'],
//   },
//   env: {
//     browser: true,
//     es2017: true,
//     node: true,
//   },
//   globals: {
//     NodeJS: true,
//   },
//   settings: {
//     'import/parsers': {
//       '@typescript-eslint/parser': ['.cjs', '.js', '.ts'],
//     },
//     'import/resolver': {
//       typescript: {
//         alwaysTryTypes: true,
//       },
//     },
//   },
//   rules: {
//     'import/no-extraneous-dependencies': 0,
//     'import/no-mutable-exports': 0,
//     'import/prefer-default-export': 0,
//     'import/extentions': 0,
//     'import/first': 0,
//     'no-param-reassign': 0,
//     'linebreak-style': 'warn',
//     // 'no-restricted-imports': ['error', { paths: ['$env/static/private'] }],
//   },
//   overrides: [
//     {
//       files: ['**/*.svelte'],
//       parser: 'svelte-eslint-parser',
//       parserOptions: {
//         parser: '@typescript-eslint/parser',
//       },
//       rules: {
//         'import/no-named-as-default': 0,
//         'import/no-named-as-default-member': 0,
//         'import/no-extraneous-dependencies': 0,
//         'import/extensions': 0,
//         'import/newline-after-import': 0,
//         'import/first': 0,
//         'import/no-duplicates': 0,
//         'import/no-mutable-exports': 0,
//         'import/no-unresolved': 0,
//         'import/prefer-default-export': 0,
//         'import/order': 0,
//         '@typescript-eslint/no-use-before-define': 0,
//         'no-alert': 0,
//         'no-inner-declarations': 0,
//       },
//     },
//   ],
// };
