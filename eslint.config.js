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
  baseDirectory: __dirname,
});

export default [
  js.configs.recommended,
  ...eslintPluginSvelte.configs['flat/recommended'],
  ...eslintPluginSvelte.configs['flat/prettier'],
  prettierConfig,
  ...compat.extends('airbnb-base'),
  {
    rules: {
      ...compat.extends('airbnb-base').rules,
      ...compat.extends('airbnb-typescript/base').rules,
      'import/no-extraneous-dependencies': 0,
      'import/no-mutable-exports': 0,
      'import/extentions': 0,
      'import/no-unresolved': 0,
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
        project: true,
        tsconfigRootDir: import.meta.dirname,
        extraFileExtensions: ['.svelte'],
      },
    },
  },
  {
    ignores: ['server', 'node_modules', 'build', 'build-test', '.svelte-kit', '.env', '.env.*'],
  },
];
