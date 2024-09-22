use tauri::{menu::{Menu, MenuItem, PredefinedMenuItem, Submenu}, App, Manager, WebviewUrl, WebviewWindowBuilder};

use crate::uuidv4;

pub fn customize(app_handle: &mut App) -> Result<bool, tauri::Error> {

    let menu = match app_handle.menu() {
        Some(menu) => menu,
        None => Menu::new(app_handle)?,
    };

    let menu_items = menu.items()?.clone();

    let submenus = menu_items.iter()
        .filter(|item| item.as_submenu().is_some())
        .map(|item| item.as_submenu().unwrap());

    let file_menu = match submenus.clone().find(|item| item.text().unwrap_or_default() == "File") {
        Some(menu) => menu,
        None => &Submenu::new(app_handle, "File", true)?
    };

    file_menu.remove_at(0)?;

    file_menu.insert_items(&[
        &MenuItem::with_id(app_handle, "new_window", "New Window", true, Some("CommandOrControl+N"))?,
        &MenuItem::with_id(app_handle, "new_tab", "New Tab", true, Some("CommandOrControl+T"))?,
        &PredefinedMenuItem::separator(app_handle)?,
        &MenuItem::with_id(app_handle, "close_tab", "Close Tab", true, Some("CommandOrControl+W"))?,
        &MenuItem::with_id(app_handle, "close_window", "Close Window", true, Some("CommandOrControl+Shift+W"))?,
    ], 0)?;

    app_handle.set_menu(menu)?;

    app_handle.on_menu_event(move |app, event| {
        println!("menu event: {:?}", event);
        match event.id().0.as_str() {
            "new_window" => {
                let url = WebviewUrl::App("/".into());
                let id = uuidv4!();
                let id_clone = id.clone();
                let window_builder = WebviewWindowBuilder::new(app, id, url)
                    .inner_size(1280.0, 800.0);

                match window_builder.build() {
                    Ok(_) => println!("Opened window {}", id_clone),
                    Err(why) => println!("Unable to open window {}! {:?}", id_clone, why),
                };
            },
            "close_window" => {
                app.webview_windows().iter_mut().for_each(|window| {
                    if window.1.is_focused().unwrap_or(false) {
                        window.1.close().unwrap();
                    }
                });
            }
            _ => ()
        }
    });

    
    // default_menu.

    
    // let file_menu_index = default_menu.items().iter()
    //     .
    //     .find(|item| item[0].is_submenu().unwrap().text() == "File")?;

    // println!("default_menu.items len: {}", default_menu.items()?.len());

    // for item in default_menu.items()? {
    //     let submenu = item.as_submenu().unwrap();
    //     println!("submenu name: {} id: {}", submenu.text().unwrap(), submenu.id().0);
    // }

    // default_menu.remove_at(1);

    // let menu = Menu::with_items(handle, &[
    //     &Submenu::with_items(
    //         handle,
    //         "File",
    //         true,
    //         &[
    //             &MenuItem::with_id(handle, "new_window", "New Window", true, None::<&str>)?,
    //             &PredefinedMenuItem::close_window(handle, None)?,
    //             &PredefinedMenuItem::quit(handle, None)?
    //         ],
    //     )?,
    //     &Submenu::with_items(
    //         handle,
    //         "Edit",
    //         true,
    //         &[
    //             &PredefinedMenuItem::undo(handle, None)?,
    //             &PredefinedMenuItem::redo(handle, None)?,
    //             &PredefinedMenuItem::copy(handle, None)?,
    //             &PredefinedMenuItem::paste(handle, None)?,
    //             &PredefinedMenuItem::cut(handle, None)?,
    //             &PredefinedMenuItem::select_all(handle, None)?,
    //         ],
    //     )?,
    //     &Submenu::with_items(
    //         handle,
    //         "Window",
    //         true,
    //         &[
    //             &PredefinedMenuItem::maximize(handle, None)?,
    //             &PredefinedMenuItem::minimize(handle, None)?,
    //             &PredefinedMenuItem::fullscreen(handle, None)?,
    //             &PredefinedMenuItem::hide(handle, None)?,
    //             &PredefinedMenuItem::hide_others(handle, None)?,
    //             &PredefinedMenuItem::show_all(handle, None)?,
    //         ],
    //     )?
    // ]);

    Ok(true)
}
