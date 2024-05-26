// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use rand::{thread_rng, Rng};
use ssh_jumper::{
    model::{HostAddress, HostSocketParams, JumpHostAuthParams, SshTunnelParams},
    SshJumper,
};
use std::{borrow::Cow, collections::HashMap, path::Path, sync::Mutex};
use serde::Serialize;
use users::get_current_username;

// use mysql::*;
// use mysql::prelude::*;
// use sqlx::AnyPool;

use adapters::{Adapter, AdapterOpts, QueryResult, SshOpts};
use tauri::{CustomMenuItem, Menu, MenuItem, State, Submenu, Window};
use uuid::Uuid;
use crate::adapters::AdapterEnum;

mod adapters;
mod ssh;

#[derive(Serialize, Debug)]
pub struct AppError {
    // query: String,
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

#[tauri::command]
async fn adapter_connect(
    window: Window,
    driver_opts: AdapterOpts,
    use_ssh: bool,
    ssh_opts: Option<SshOpts>,
    state: State<'_, Connections>,
) -> Result<bool, AppError> {
    let conn = if use_ssh {
        Adapter::connect(driver_opts, ssh_opts).await?
    } else {
        Adapter::connect(driver_opts, None).await?
    };
    {
        let mut connections = state.0.try_lock().map_err(AppError::from)?;
        connections.insert(window.label().to_string(), conn);
    }
    Ok(true)
}

#[tauri::command]
async fn adapter_query<'conn>(
    window: Window,
    query: String,
    database: Option<String>,
    state: State<'_, Connections>,
) -> Result<QueryResult, AppError> {
    let uuid: String = window.label().into();
    let conn: Adapter = {
        let connections = state.0.try_lock().map_err(AppError::from)?;
        connections
            .get(&uuid)
            .ok_or(AppError::from("No connection found bound to the window!"))?
            .clone()
    };

    // let conn: &'_ Adapter = get_window_connection(state, window)?;
    conn.query(query, database).await
}

#[tauri::command]
async fn adapter_test(
    driver_opts: AdapterOpts,
    use_ssh: bool,
    ssh_opts: Option<SshOpts>,
) -> Result<String, AppError> {
    let mut conn = if use_ssh {
        Adapter::connect(driver_opts, ssh_opts).await?
    } else {
        Adapter::connect(driver_opts, None).await?
    };
    conn.disconnect().await;

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
fn build_menu() -> Menu {
    // here `"quit".to_string()` defines the menu item id, and the second parameter is the menu item label.
    let new_window =
        CustomMenuItem::new("new_window".to_string(), "New Window").accelerator("cmdOrCtrl+N");
    let close_window =
        CustomMenuItem::new("close_window".to_string(), "Close Window").accelerator("cmdOrCtrl+W");
    let quit = CustomMenuItem::new("quit".to_string(), "Quit").accelerator("cmdOrCtrl+Q");

    let menu = Menu::new();

    let file_menu = Submenu::new(
        "File",
        Menu::new()
            .add_item(new_window)
            .add_item(close_window)
            .add_item(quit),
    );

    let edit_menu = Submenu::new(
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

    menu.add_submenu(file_menu).add_submenu(edit_menu)

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

struct Connections(Mutex<HashMap<String, AdapterEnum>>);

fn main() {
    tauri::Builder::default()
        .manage(Connections(Default::default()))
        .setup(|app| {
            let local_data_dir = app.path_resolver().app_local_data_dir().unwrap();
            if !local_data_dir.exists() {
                std::fs::create_dir_all(&local_data_dir)?;
            }

            Ok(())
        })
        .menu(build_menu())
        .on_menu_event(|event| match event.menu_item_id() {
            "new_window" => {
                let window: Window = tauri::WindowBuilder::new(
                    event.window(),
                    Uuid::new_v4().to_string(),
                    tauri::WindowUrl::App("index.html".into()),
                )
                .build()
                .unwrap();
                window
                    .set_title("New Connection".into())
                    .expect("Failed to set window title");
            }
            "close_window" => {
                event.window().close().unwrap();
            }
            "quit" => {
                std::process::exit(0);
            }
            _ => {}
        })
        .invoke_handler(tauri::generate_handler![
            adapter_connect,
            adapter_test,
            adapter_query,
            fetch_key
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
