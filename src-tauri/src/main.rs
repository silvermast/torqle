// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use tauri::{Context, CustomMenuItem, Menu, MenuItem, Submenu, Manager, Window};
use uuid::Uuid;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn query(query: &str) -> String {
    format!("{}", query)
}

// #[tauri::command]
// fn setup() -> String {
//     let appLocalDataDir = resolve_path(BaseDirectory::AppLocalData);
//     let context = tauri::generate_context!("test/fixture/src-tauri/tauri.conf.json");
//     let path = resolve_path(
//         context.config(),
//         context.package_info(),
//         &Env::default(),
//         "data",
//         Some(BaseDirectory::AppLocalData))
//       .expect("failed to resolve path");
//     if !fs::metadata(path).is_dir() {
//         fs::create_dir_all(path);
//     }
//     format!("Success")
// }


fn build_menu() -> Menu {
    
    // here `"quit".to_string()` defines the menu item id, and the second parameter is the menu item label.
    let new_window = CustomMenuItem::new("new_window".to_string(), "New Window").accelerator("cmdOrCtrl+N");
    let close_window = CustomMenuItem::new("close_window".to_string(), "Close Window").accelerator("cmdOrCtrl+W");
    let quit = CustomMenuItem::new("quit".to_string(), "Quit").accelerator("cmdOrCtrl+Q");
    

    let file_submenu = Submenu::new("File", Menu::new()
    .add_item(new_window)
    .add_item(close_window)
    .add_item(quit));

    return Menu::new()
        .add_submenu(file_submenu);

    // let menu = Menu::new();
    // menu.add_submenu(Submenu::new("File", file_menu));

    // use default OS menu as a base
    // let mut menu: Menu = tauri::Menu::os_default(app_name);
    // menu.
    // for sub_menu in menu.items.iter() {
    //     if sub_menu.title == "File" {
    //         std::mem::replace(sub_menu, file_menu);
    //     }
    // }

}

fn main() {
    let app_name: &str = "Taurbee";
    tauri::Builder::default()
        .setup(|app| {
            let local_data_dir = app.path_resolver().app_local_data_dir().unwrap();
            if !local_data_dir.exists() {
                std::fs::create_dir_all(&local_data_dir)?;
            }
            Ok(())
        })
        .menu(build_menu())
        .on_menu_event(|event| {
            match event.menu_item_id() {
                "new_window" => {
                    let window: Window = tauri::WindowBuilder::new(
                        event.window(),
                        Uuid::new_v4().to_string(),
                        tauri::WindowUrl::App("index.html".into()),
                    ).build().unwrap();
                }
                "close_window" => {
                    event.window().close().unwrap();
                }
                "quit" => {
                    std::process::exit(0);
                }
                _ => {}
            }
          })
        .invoke_handler(tauri::generate_handler![query])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
