// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{Emitter, Manager};
use yi_yi_lib::{commands, commands::*, models::AppSettings, api::DeepSeekClient};
use std::sync::Mutex;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

static MONITOR_RUNNING: AtomicBool = AtomicBool::new(false);

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
            get_selected_text,
        ])
        .setup(|app| {
            let handle = app.handle().clone();
            
            tauri::async_runtime::spawn(async move {
                let state = handle.state::<commands::AppState>();
                let db = yi_yi_lib::database::Database::new(&handle).expect("无法初始化数据库");
                *state.db.lock().unwrap() = Some(db);
                log::info!("数据库初始化成功");
            });
            
            let app_handle = app.handle().clone();
            
            if !MONITOR_RUNNING.swap(true, Ordering::SeqCst) {
                std::thread::spawn(move || {
                    log::info!("开始监听划词选择...");
                    monitor_selection(app_handle);
                });
            }
            
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn monitor_selection(app: tauri::AppHandle) {
    use tauri_plugin_clipboard_manager::ClipboardExt;
    
    let mut last_text = String::new();
    
    loop {
        if app.webview_windows().iter().count() == 0 {
            std::thread::sleep(Duration::from_millis(500));
            continue;
        }
        
        match app.clipboard().read_text() {
            Ok(text) => {
                let text = text.trim().to_string();
                if !text.is_empty() && text.len() <= 100 && text != last_text {
                    last_text = text.clone();
                    log::info!("检测到新文字: {}", text);
                    
                    let handle = app.clone();
                    let text_clone = text.clone();
                    tauri::async_runtime::spawn(async move {
                        if let Err(e) = handle.emit("selection-translate", &text_clone) {
                            log::error!("发送事件失败: {}", e);
                        }
                    });
                }
            }
            Err(_) => {}
        }
        
        std::thread::sleep(Duration::from_millis(300));
    }
}