// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use tauri::Manager;
use window_shadows::set_shadow;
use env_logger;
mod cmd;

fn main() {
    env_logger::init();
    let builder = tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![cmd::scan, cmd::get_interfaces]);

    #[cfg(any(windows, target_os = "macos"))]
    let builder = builder
        .setup(|app| {
            let window = app.get_window("main").unwrap();
            #[cfg(any(windows, target_os = "macos"))]
            set_shadow(&window, true).expect("Unsupported platform!");
            Ok(())
        });

    builder
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
