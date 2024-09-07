# Possible Names
- Taurbee
- SeaQuill
- IvyEye
- SeaCat
- Voxide (vue + iron oxide)
- Vuxide


# Tauri + Vue 3

https://tauri.app/

This template should help get you started developing with Tauri + Vue 3 in Vite. The template uses Vue 3 `<script setup>` SFCs, check out the [script setup docs](https://v3.vuejs.org/api/sfc-script-setup.html#sfc-script-setup) to learn more.

## Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/) + [Vue Plugin](https://marketplace.visualstudio.com/items?itemName=Vue.volar) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

## Getting Started

- Simply run `npm ci && npm run tauri dev` to get started!

## Features

### Favorites
Favorites are stored within the application data directory under $APPLOCALDATA. They are encrypted with a secret generated on first-load, and stored in the operating system using Keytar.