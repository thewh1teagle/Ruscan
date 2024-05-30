// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use env_logger;
mod cmd;

fn main() {
    env_logger::init();
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![cmd::scan, cmd::get_interfaces])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
