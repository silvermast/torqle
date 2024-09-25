Torqle
==================================
**T**auri **O**bject **R**elational **Q**uery **L**anguage **E**ditor

In other words, it's a generic database query editor written in Tauri.

![Torqle Logo](src-tauri/icons/128x128.png)

# Tauri + Vue 3 + Vuetify

- https://tauri.app/
- https://vuejs.org/
- https://vuetifyjs.com/

This template should help get you started developing with Tauri + Vue 3 in Vite. The template uses Vue 3 `<script setup>` SFCs, check out the [script setup docs](https://v3.vuejs.org/api/sfc-script-setup.html#sfc-script-setup) to learn more.

## Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/) + [Vue Plugin](https://marketplace.visualstudio.com/items?itemName=Vue.volar) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

## Getting Started

- Follow the instructions for installing [Tauri Prerequisites](https://v2.tauri.app/start/prerequisites/) for your system.
- Then, simply run `npm ci && npm run tauri dev` to get started!

## Features

### Favorites
Favorites are stored within the application data directory under $APPLOCALDATA. They are encrypted with a secret generated on first-load, and stored in the operating system using Keytar.

## Troubleshooting

### On Linux: Error 71 (Protocol error) dispatching to Wayland display.

See https://github.com/tauri-apps/tauri/issues/10702

`WEBKIT_DISABLE_DMABUF_RENDERER=1 npm run tauri dev`
