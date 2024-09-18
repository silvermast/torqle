use tauri::{menu::{Menu, MenuItem, PredefinedMenuItem, Submenu}, AppHandle, State, Window};
// use tauri::menu::{CustomMenuItem, Menu, Submenu, MenuItem};

pub fn build<R>(handle: &AppHandle) -> Menu<R> {
    let menu = Menu::with_items(handle, &[
        &Submenu::with_items(
            handle,
            "File",
            true,
            &[
                &MenuItem::with_id(handle, "new_window", "New Window", true, None::<&str>)?,
                &PredefinedMenuItem::close_window(handle, None)?,
                &PredefinedMenuItem::quit(handle, None)?
            ],
        )?,
        &Submenu::with_items(
            handle,
            "Edit",
            true,
            &[
                &PredefinedMenuItem::undo(handle, None)?,
                &PredefinedMenuItem::redo(handle, None)?,
                &PredefinedMenuItem::copy(handle, None)?,
                &PredefinedMenuItem::paste(handle, None)?,
                &PredefinedMenuItem::cut(handle, None)?,
                &PredefinedMenuItem::select_all(handle, None)?,
            ],
        )?,
        &Submenu::with_items(
            handle,
            "Window",
            true,
            &[
                &PredefinedMenuItem::maximize(handle, None)?,
                &PredefinedMenuItem::minimize(handle, None)?,
                &PredefinedMenuItem::fullscreen(handle, None)?,
                &PredefinedMenuItem::hide(handle, None)?,
                &PredefinedMenuItem::hide_others(handle, None)?,
                &PredefinedMenuItem::show_all(handle, None)?,
            ],
        )?
    ]);

    menu
}

pub fn handle_event() {

}