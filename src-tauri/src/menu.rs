use tauri::menu::{CustomMenuItem, Menu, Submenu, MenuItem};

pub fn build(handle: &AppHandle) {

    let file_submenu = Submenu::new(
        "File",
        Menu::new()
            .add_native_item(MenuItem::New)
            .add_native_item(MenuItem::Separator)
            .add_native_item(MenuItem::Close)
            .add_native_item(MenuItem::Quit)
    );

    let edit_submenu = Submenu::new(
        "Edit",
        Menu::new()
            .add_native_item(MenuItem::Undo)
            .add_native_item(MenuItem::Redo)
            .add_native_item(MenuItem::Separator)
            .add_native_item(MenuItem::Cut)
            .add_native_item(MenuItem::Copy)
            .add_native_item(MenuItem::Paste)
            .add_native_item(MenuItem::SelectAll),
    );

    Menu::new()
        .add_submenu(file_submenu)
        .add_submenu(edit_submenu)
}

pub fn handle_event() {

}