// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(async_fn_in_trait)]

use adapters::{connect_adapter, Adapter, AdapterEnum, AdapterOpts, QueryResult};
use rand::{thread_rng, Rng};
use serde::Serialize;
use std::{collections::HashMap, sync::Mutex};
use users::get_current_username;
// use tauri::{menu::{Menu, MenuItem, PredefinedMenuItem, Submenu}, Runtime, State, Window};
use tauri::{State, Window};

pub mod adapters;
pub mod menu;
pub mod ssh;
pub mod tests;

#[macro_export]
macro_rules! uuidv4 {
    () => {
        uuid::Uuid::new_v4().to_string()
    };
}

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
impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.error)
    }
}
impl std::error::Error for AppError {}

#[derive(Default)]
pub struct AppState {
    adapters: Mutex<HashMap<String, AdapterEnum>>,
}
impl AppState {
    /**
     * Returns a cloned version of the value in the map. If issues occur, try implementing a Deref.
     * @see https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html
     * @see https://doc.rust-lang.org/book/ch15-02-deref.html
     */
    pub fn get_adapter(&self, window: &Window) -> Result<AdapterEnum, AppError> {
        let uuid: String = window.label().into();
        let map_mutex = self.adapters.try_lock().map_err(AppError::from)?;
        match map_mutex.get(&uuid) {
            Some(adapter) => Ok(adapter.clone()),
            None => Err(AppError::from(
                "Unable to get_adapter. No connection bound to the window!",
            )),
        }
    }
    pub fn set_adapter(&self, window: &Window, adapter: AdapterEnum) -> Result<bool, AppError> {
        self.adapters
            .try_lock()
            .map_err(AppError::from)?
            .insert(window.label().to_string(), adapter.clone());
        Ok(true)
    }
    pub fn remove_adapter(&self, window: &Window) -> Result<AdapterEnum, AppError> {
        let uuid: String = window.label().into();
        self.adapters
            .try_lock()
            .map_err(AppError::from)?
            .remove(&uuid)
            .ok_or(AppError::from(
                "Unable to remove_adapter. No connection bound to the window!",
            ))
    }
}

#[tauri::command]
async fn adapter_connect(
    window: Window,
    title: String,
    driver_opts: AdapterOpts,
    ssh_opts: Option<ssh::SshOpts>,
    state: State<'_, AppState>,
) -> Result<bool, AppError> {
    let adapter = connect_adapter(driver_opts, ssh_opts).await?;
    window.set_title(title.as_str()).unwrap_or_default();
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
async fn adapter_disconnect(window: Window, state: State<'_, AppState>) -> Result<bool, AppError> {
    let mut adapter: AdapterEnum = state.remove_adapter(&window)?;
    adapter.disconnect().await?;
    window.set_title("New Connection").unwrap_or_default();
    Ok(true)
}

#[tauri::command]
async fn adapter_test(
    driver_opts: AdapterOpts,
    ssh_opts: Option<ssh::SshOpts>,
) -> Result<String, AppError> {
    let mut adapter = connect_adapter(driver_opts, ssh_opts).await?;
    adapter.disconnect().await?;

    Ok("The connection test was a success!".to_string())
}

#[tauri::command]
fn fetch_key() -> Result<String, String> {
    let service = "torqle";
    let account = match get_current_username() {
        Some(username) => format!("{:?}", username),
        None => "torqle-default".to_string(),
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

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_fs::init())
        .manage(AppState::default())
        .invoke_handler(tauri::generate_handler![
            adapter_connect,
            adapter_disconnect,
            adapter_test,
            adapter_query,
            fetch_key
        ])
        .setup(|app_handle| -> Result<(), Box<dyn std::error::Error>> {
            menu::customize(app_handle)?;
            // other app setup code can go here...
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// .menu(|handle| {
//     let new_window_menu_item = MenuItem::with_id(handle, "new_window", "New Window", true, Some("CommandOrControl+N"))?;
//     // let close_window = PredefinedMenuItem::close_window(handle, None)?;

//     let menu = Menu::with_items(handle, &[
//         &Submenu::with_items(
//             handle,
//             "File",
//             true,
//             &[
//                 &new_window_menu_item,
//                 // &close_window,
//                 &PredefinedMenuItem::quit(handle, None)?
//             ],
//         )?,
//         &Submenu::with_items(
//             handle,
//             "Edit",
//             true,
//             &[
//                 &PredefinedMenuItem::undo(handle, None)?,
//                 &PredefinedMenuItem::redo(handle, None)?,
//                 &PredefinedMenuItem::copy(handle, None)?,
//                 &PredefinedMenuItem::paste(handle, None)?,
//                 &PredefinedMenuItem::cut(handle, None)?,
//                 &PredefinedMenuItem::select_all(handle, None)?,
//             ],
//         )?,
//         &Submenu::with_items(
//             handle,
//             "Window",
//             true,
//             &[
//                 &PredefinedMenuItem::maximize(handle, None)?,
//                 &PredefinedMenuItem::minimize(handle, None)?,
//                 &PredefinedMenuItem::fullscreen(handle, None)?,
//                 &PredefinedMenuItem::hide(handle, None)?,
//                 &PredefinedMenuItem::hide_others(handle, None)?,
//                 &PredefinedMenuItem::show_all(handle, None)?,
//             ],
//         )?
//     ]);

//     handle.on_menu_event(move |app, event| {
//         if event.id() == new_window_menu_item.id() {
//             // emit a window event to the frontend
//             let window_id = format!("{}", Uuid::new_v4());
//             tauri::WebviewWindowBuilder::new(app, window_id, tauri::WebviewUrl::App("index.html".into()))
//                 .title("New Connection")
//                 .inner_size(1280.0, 800.0)
//                 .build()
//                 .unwrap();
//         }
//     });

//     menu
// })
