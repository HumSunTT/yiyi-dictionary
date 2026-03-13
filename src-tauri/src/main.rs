// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{Emitter, Manager};
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut};
use yi_yi_lib::{commands, commands::*, models::AppSettings, api::DeepSeekClient};
use std::sync::Mutex;

fn main() {
    let settings = AppSettings::default();
    let api_client = DeepSeekClient::new(settings.clone());
    
    tauri::Builder::default()
        .plugin(tauri_plugin_log::Builder::new().build())
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
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
            
            let shortcut = Shortcut::new(Some(Modifiers::CONTROL | Modifiers::SHIFT), Code::KeyT);
            
            let app_handle = app.handle().clone();
            app.global_shortcut().on_shortcut(shortcut, move |_app, _shortcut, _event| {
                log::info!("快捷键触发: Ctrl+Shift+T");
                
                let handle = app_handle.clone();
                tauri::async_runtime::spawn(async move {
                    match get_clipboard_text(&handle).await {
                        Ok(text) if !text.is_empty() => {
                            log::info!("获取到剪贴板文字: {}", text);
                            if let Err(e) = handle.emit("selection-translate", &text) {
                                log::error!("发送事件失败: {}", e);
                            }
                        }
                        Ok(_) => {
                            log::info!("剪贴板为空，请先复制文字");
                        }
                        Err(e) => {
                            log::error!("获取剪贴板失败: {}", e);
                        }
                    }
                });
            })?;
            
            log::info!("全局快捷键注册成功: Ctrl+Shift+T");
            log::info!("使用方法: 先复制文字(Ctrl+C)，再按 Ctrl+Shift+T 翻译");
            
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

async fn get_clipboard_text(app: &tauri::AppHandle) -> Result<String, String> {
    use tauri_plugin_clipboard_manager::ClipboardExt;
    
    let text: String = app.clipboard().read_text()
        .map_err(|e| e.to_string())?;
    
    Ok(text.trim().to_string())
}