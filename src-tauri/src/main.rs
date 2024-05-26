// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use rand::{thread_rng, Rng};
use serde::Serialize;
use std::{collections::HashMap, sync::{Arc, Mutex}};
use users::get_current_username;

use adapters::{connect_adapter, Adapter, AdapterEnum, AdapterOpts, QueryResult, SshOpts};
use tauri::{menu::{PredefinedMenuItem, Submenu}, State, Window};
use tauri::menu::{Menu, MenuItem};

mod adapters;
mod ssh;

#[derive(Serialize, Debug)]
pub struct AppError {
    pub error: String,
}
impl AppError {
    pub fn from<E: std::fmt::Display>(err: E) -> AppError {
        let error_string = format!("{}", err);
        println!("Error: {}", error_string); // debug!
        AppError {
            error: error_string,
        }
    }
}

#[derive(Default)]
pub struct AppState {
    adapters: Arc<Mutex<HashMap<String, AdapterEnum>>>
}
impl AppState {
    /**
     * Returns a cloned version of the value in the map. If issues occur, try implementing a Deref.
     * @see https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html
     * @see https://doc.rust-lang.org/book/ch15-02-deref.html
     */
    pub fn get_adapter<'a>(&self, window: &Window) -> Result<AdapterEnum, AppError> {
        let uuid: String = window.label().into();
        let map_mutex = self.adapters.try_lock().map_err(AppError::from)?;
        let adapter = map_mutex.get(&uuid).ok_or(AppError::from("No connection bound to the window!"))?.clone();
        Ok(adapter.clone())
    }
    pub fn set_adapter<'a>(&self, window: &Window, adapter: AdapterEnum) -> Result<bool, AppError> {
        self.adapters
            .try_lock()
            .map_err(AppError::from)?
            .insert(window.label().to_string(), adapter);
        Ok(true)
    }
    pub fn remove_adapter(&self, window: &Window) -> Result<AdapterEnum, AppError> {
        let uuid: String = window.label().into();
        self.adapters
            .try_lock()
            .map_err(AppError::from)?
            .remove(&uuid)
            .ok_or(AppError::from("No connection bound to the window!"))
    }
}

#[tauri::command]
async fn adapter_connect(
    window: Window,
    driver_opts: AdapterOpts,
    ssh_opts: Option<SshOpts>,
    state: State<'_, AppState>,
) -> Result<bool, AppError> {
    let adapter = connect_adapter(driver_opts, ssh_opts).await?;
    state.set_adapter(&window, adapter)
}

#[tauri::command]
async fn adapter_query(
    window: Window,
    query: String,
    database: Option<String>,
    state: State<'_, AppState>,
) -> Result<QueryResult, AppError> {
    let adapter = state.get_adapter(&window)?;
    adapter.query(query, database).await
}

#[tauri::command]
async fn adapter_disconnect(
    window: Window,
    state: State<'_, AppState>,
) -> Result<bool, AppError> {
    let mut adapter: AdapterEnum = state.remove_adapter(&window)?;
    adapter.disconnect().await;
    Ok(true)
}

#[tauri::command]
async fn adapter_test(
    driver_opts: AdapterOpts,
    ssh_opts: Option<SshOpts>,
) -> Result<String, AppError> {
    let mut adapter = connect_adapter(driver_opts, ssh_opts).await?;
    adapter.disconnect().await;

    Ok("The connection test was a success!".to_string())
}

#[tauri::command]
fn fetch_key() -> Result<String, String> {
    let service = "taurbee";
    let account = match get_current_username() {
        Some(username) => format!("{:?}", username),
        None => "taurbee-default".to_string(),
    };

    match keytar::get_password(service, &account) {
        Ok(pass) => {
            if pass.success {
                Ok(pass.password)
            } else {
                create_new_app_key(account, service.to_string())
            }
        }
        Err(why) => Err(why.to_string()),
    }
}

fn create_new_app_key(account: String, service: String) -> Result<String, String> {
    let new_key = generate_aes_256_key();
    match keytar::set_password(&service, &account, &new_key) {
        Ok(_result) => Ok(new_key),
        Err(why) => Err(why.to_string()),
    }
}

fn generate_aes_256_key() -> String {
    let mut key = [0u8; 32];
    thread_rng().fill(&mut key);
    hex::encode(key)
}

/** @todo build out menus more appropriately, and track windows */
// fn build_menu(handle: &AppHandle) -> Menu {
    // // here `"quit".to_string()` defines the menu item id, and the second parameter is the menu item label.
    // let new_window =
    //     MenuItemBuilder::new("new_window".to_string(), "New Window").accelerator("cmdOrCtrl+N");
    // let close_window =
    //     MenuItemBuilder::new("close_window".to_string(), "Close Window").accelerator("cmdOrCtrl+W");
    // let quit = MenuItemBuilder::new("quit".to_string(), "Quit").accelerator("cmdOrCtrl+Q");

    // let menu = Menu::new();

    // let file_menu = Submenu::new(
    //     "File",
    //     Menu::new()
    //         .add_item(new_window)
    //         .add_item(close_window)
    //         .add_item(quit),
    // );

    // let edit_menu = Submenu::new(
    //     "Edit",
    //     Menu::new()
    //         .add_native_item(MenuItem::Undo)
    //         .add_native_item(MenuItem::Redo)
    //         .add_native_item(MenuItem::Separator)
    //         .add_native_item(MenuItem::Cut)
    //         .add_native_item(MenuItem::Copy)
    //         .add_native_item(MenuItem::Paste)
    //         .add_native_item(MenuItem::SelectAll),
    // );

    // menu.add_submenu(file_menu).add_submenu(edit_menu)

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
// }


fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_fs::init())
        .manage(AppState::default())
        .menu(|handle| Menu::with_items(handle, &[
            &Submenu::with_items(
                handle,
                "File",
                true,
                &[
                  &PredefinedMenuItem::close_window(handle, None)?,
                  #[cfg(target_os = "macos")]
                  &MenuItem::new(handle, "Hello", true, None::<&str>)?,
                ],
              )?
        ]))
        .setup(|app| {
            // let local_data_dir = app.path_resolver().app_local_data_dir().unwrap();
            // if !local_data_dir.exists() {
            //     std::fs::create_dir_all(&local_data_dir)?;
            // }

            // app.on_menu_event(|event| match event.menu_item_id() {
            //     "new_window" => {
            //         let window: Window = tauri::WindowBuilder::new(
            //             event.window(),
            //             Uuid::new_v4().to_string(),
            //             tauri::WindowUrl::App("index.html".into()),
            //         )
            //         .build()
            //         .unwrap();
            //         window
            //             .set_title("New Connection".into())
            //             .expect("Failed to set window title");
    
            //         window.manage(AppState::default());
            //     }
            //     "close_window" => {
            //         event.window().close().unwrap();
            //     }
            //     "quit" => {
            //         std::process::exit(0);
            //     }
            //     _ => {}
            // });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            adapter_connect,
            adapter_disconnect,
            adapter_test,
            adapter_query,
            fetch_key
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
