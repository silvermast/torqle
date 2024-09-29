TO DO List
===================

## App
- Test stale connections & ability to reconnect easily. Ensure loading state displayed when connecting
- Add Menus back. Tauri 1 managed these in Rust, but Tauri 2 manages them in JS. Waiting on documentation.
- Settings!
  - query history length
  - editor configs (tabs, indent, theme, font size)
  - keyboard shortcuts

## Rust

- implement multiple windows
- Finish Sqlite adapter
- add MongoDB
- add Postgres
- Auto Updater

## Vue

- Rename QueryView to: `Conduct`, `Command`, `Operate`, `Wield`, `Action`, `Execute`
- Keyboard shortcut help menu
- Editor Tabs
- database selector hotkey (Fix for global multi-window)
- Disable ACE hotkeys not in use
- ACE Editor Sessions (save & restore)
- Saved queries
- Query history faced by connection
- Replace Vuetify with something less gross

## Issues

- "database" doesn't stick when selecting one
- When writing a query, there's logic to highlight the active query based on cursor location.
  If you type out a query, then ;, then hit enter, it's not finding the query just before the cursor
- The docker-compose ssh tunnel is not fully set up
- When hitting the "Filter Database" keyboard shortcut, the user experience is awful
- `npm i` creates a lockfile with hard dependencies on platform binaries (eg. linux-x64) that causes unintuituve errors
  when moving to another platform. Currently, the package-lock.json is being ignored in git, but this isn't ideal.
  Figure out a way to fix this problem; also why is this even a problem, NPM?