// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::{collections::HashMap, sync::Mutex};

use connectors::{ClientConnection, QueryError, QueryResult};
use tauri::{CustomMenuItem, Menu, Submenu, Window, State};
use uuid::Uuid;

mod connectors;

#[tauri::command]
fn connect(window: Window, driver_opts: HashMap<String, String>, state_conn: State<Connections>) -> Result<bool, String> {
    let conn = connectors::mysql::Mysql::connect(driver_opts).or_else(|why| Err(why.to_string()))?;
    let mut connections = state_conn.0.try_lock().or_else(|why| Err(why.to_string()))?;
    connections.insert(window.label().to_string(), conn);
    Ok(true)
}

#[tauri::command]
fn query(window: Window, query: String, state_conn: State<Connections>) -> Result<QueryResult, QueryError> {
    let connections = state_conn.0.try_lock().or_else(|why| Err(QueryError { error: why.to_string() }))?;
    let uuid: String = window.label().into();
    let conn = connections.get(&uuid).ok_or(QueryError { error: "No connection found bound to the window!".to_string() })?;
    conn.query(query)
}

#[tauri::command]
fn test_connection(driver_opts: HashMap<String, String>) -> Result<String, String> {
    match connectors::mysql::Mysql::test(driver_opts) {
        Ok(_) => Ok("The connection test was a success!".to_string()),
        Err(why) => Err(why.to_string()),
    }
}

#[tauri::command]
fn change_schema(window: Window, state_conn: State<Connections>, schema: String) -> Result<bool, String> {
    let mut connections = state_conn.0.try_lock().or_else(|why| Err(why.to_string()))?;
    let uuid: String = window.label().into();
    let conn = connections.get(&uuid).ok_or("No connection found bound to the window!".to_string())?;
    let mut conn_cloned = conn.clone();
    conn_cloned.change_schema(schema).or_else(|why| Err(why.to_string()))?;
    connections.insert(window.label().to_string(), conn_cloned);
    Ok(true)
}

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

struct Connections(Mutex<HashMap<String, connectors::mysql::Mysql>>);

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
        .on_menu_event(|event| {
            match event.menu_item_id() {
                "new_window" => {
                    let window: Window = tauri::WindowBuilder::new(
                        event.window(),
                        Uuid::new_v4().to_string(),
                        tauri::WindowUrl::App("index.html".into()),
                    ).build().unwrap();
                    window.set_title("New Connection".into()).expect("Failed to set window title");
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
        .invoke_handler(tauri::generate_handler![connect, test_connection, query, change_schema])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
