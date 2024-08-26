TO DO List
===================

## App
- Test stale connections & ability to reconnect easily. Ensure loading state displayed when connecting
- Add Menus back. Tauri 1 managed these in Rust, but Tauri 2 manages them in JS. Waiting on documentation.
- Settings!
- - query history length
- - editor configs (tabs, indent, theme, font size)
- - keyboard shortcuts

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