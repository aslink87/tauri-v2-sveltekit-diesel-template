// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use endure::routes::users;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            users::get_user,
            users::create_user,
            users::verify_user
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
