// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod cmd;

fn main() {
    tracing_subscriber::fmt()
        .with_line_number(true)
        .with_file(true)
        .init();
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![cmd::scan, cmd::get_interfaces])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
