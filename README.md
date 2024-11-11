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

### Local Development Containers

The project includes a docker-compose file specifically for local development. Run `docker-compose up -d` to initialize them.

#### SSHD
To utilize the ssh tunnel, use `root@127.0.0.1:10022` with no password, or a password of `littlebuddy`. RSA key coming soon

#### MySQL
To connect to MySQL, use `root:mypassword@127.0.0.1:3306`. If connecting through the tunnel, set the host to `mysql`.

#### Sqlite
The Sqlite implementation does not currently support an SSH tunnel. However, there is a test database in the `./data` directory

## Features

### Favorites
Favorites are stored within the application data directory under $APPLOCALDATA. They are encrypted with a secret generated on first-load, and stored in the operating system using Keytar.

## Troubleshooting

### Various errors about architecture or platform related to @tauri-app/cli, darwin / linux, etc

1. First just delete `node_modules` and attempt an `npm ci` again.
1. If that fails, delete `package-lock.json` and `node_modules` and run `npm i`.
1. If that fails, check your cpu and os:
    - Run ``node -e 'console.log(`os: ${process.platform}\ncpu: ${process.arch}\`)'`` and make note of the "os" and "cpu" values.
    - Check the "os" and "cpu" options in `package.json`.
    - If cpu or os is not represented, you can add it to the appropriate option array, and then run step 2 again.
1. If you _still_ get this error, you can manually install the package it says is missing without saving it to the package.json, or force install it for a given platform using the `--os` and `--cpu` options of `npm install`. This is not a permanent solution, and may need to be repeated on subsequent installs.
    - `npm install "@tauri-apps/cli-darwin-x64"` (be careful to match version in package)
    - Or maybe `npm install @tauri-apps/cli --os darwin --cpu x64`

### On Mac: Error about missing cargo metadata

Rust / Cargo isn't installed or isn't initialized correctly.

If using Homebrew:
 - `brew install rustup`
 - Make sure correct rustup / cargo is in path, add `export PATH="/usr/local/opt/rustup/bin:$PATH"` to your profile
 - `rustup default stable`
 - Verify with `cargo --version`

### On Linux: Error 71 (Protocol error) dispatching to Wayland display.

See https://github.com/tauri-apps/tauri/issues/10702

`WEBKIT_DISABLE_DMABUF_RENDERER=1 npm run tauri dev`

### Ensure Synchronized Tauri Version
Check the tauri version in `src-tauri/Cargo.toml` and `src-tauri/Cargo.lock`. The tauri version of each plugin needs to line up with
the corresponding plugins installed in `package.json` down to the exact patch version.