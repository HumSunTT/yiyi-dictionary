// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Manager;
use yi_yi_lib::{commands, commands::*, models::AppSettings, api::DeepSeekClient};
use std::sync::Mutex;

fn main() {
    let settings = AppSettings::default();
    let api_client = DeepSeekClient::new(settings.clone());
    
    tauri::Builder::default()
        .plugin(tauri_plugin_log::Builder::new().build())
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .manage(commands::AppState {
            db: Mutex::new(None),
            api_client: Mutex::new(api_client),
            settings: Mutex::new(settings),
        })
        .invoke_handler(tauri::generate_handler![
            init_database,
            query_word,
            translate_text,
            get_history,
            clear_history,
            add_to_vocabulary,
            get_vocabulary,
            remove_from_vocabulary,
            get_settings,
            save_settings,
            detect_language,
        ])
        .setup(|app| {
            // 初始化数据库
            let handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                let state = handle.state::<commands::AppState>();
                let db = yi_yi_lib::database::Database::new(&handle).expect("无法初始化数据库");
                *state.db.lock().unwrap() = Some(db);
                log::info!("数据库初始化成功");
            });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}