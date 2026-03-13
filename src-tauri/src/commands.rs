// Tauri Commands

use tauri::State;
use std::sync::Mutex;

use crate::database::Database;
use crate::api::DeepSeekClient;
use crate::models::*;

/// 应用状态
pub struct AppState {
    pub db: Mutex<Option<Database>>,
    pub api_client: Mutex<DeepSeekClient>,
    pub settings: Mutex<AppSettings>,
}

/// 初始化数据库
#[tauri::command]
pub async fn init_database(app: tauri::AppHandle, state: State<'_, AppState>) -> Result<(), String> {
    let db = Database::new(&app).map_err(|e| e.to_string())?;
    *state.db.lock().unwrap() = Some(db);
    Ok(())
}

/// 查询单词（支持多字典结果）
#[tauri::command]
pub async fn query_word_multi(word: String, dict_type: String, state: State<'_, AppState>) -> Result<Vec<DictionaryResult>, String> {
    let db_guard = state.db.lock().unwrap();
    let db = db_guard.as_ref().ok_or("数据库未初始化")?;
    
    let results = match dict_type.as_str() {
        "ancient" => {
            db.query_ancient_all(&word).map_err(|e| e.to_string())?
        },
        "english" => {
            let result = db.query_english(&word);
            if let Some(r) = result {
                vec![r]
            } else {
                vec![]
            }
        },
        "chinese" => {
            let result = db.query_chinese(&word);
            if let Some(r) = result {
                vec![r]
            } else {
                vec![]
            }
        },
        _ => {
            // 按优先级查询：古汉语 -> 中文 -> 英语
            let mut results = Vec::new();
            if let Some(r) = db.query_ancient(&word) {
                results.push(r);
            }
            if let Some(r) = db.query_chinese(&word) {
                results.push(r);
            }
            if let Some(r) = db.query_english(&word) {
                results.push(r);
            }
            results
        }
    };
    
    // 记录历史（只记录第一个结果 to avoid duplication）
    if !results.is_empty() {
        let first_result = &results[0];
        let result_json = serde_json::to_string(first_result).unwrap_or_default();
        let _ = db.add_history(&word, &dict_type, &result_json, "local");
    }
    
    Ok(results)
}

/// 查询单词
#[tauri::command]
pub async fn query_word(word: String, dict_type: String, state: State<'_, AppState>) -> Result<Option<DictionaryResult>, String> {
    let db_guard = state.db.lock().unwrap();
    let db = db_guard.as_ref().ok_or("数据库未初始化")?;
    
    let result = match dict_type.as_str() {
        "ancient" => db.query_ancient(&word),
        "english" => db.query_english(&word),
        "chinese" => db.query_chinese(&word),
        _ => {
            // 按优先级查询：古汉语 -> 中文 -> 英语
            db.query_ancient(&word).or_else(|| db.query_chinese(&word)).or_else(|| db.query_english(&word))
        }
    };
    
    // 记录历史
    if let Some(ref r) = result {
        let result_json = serde_json::to_string(r).unwrap_or_default();
        let _ = db.add_history(&word, &dict_type, &result_json, "local");
    }
    
    Ok(result)
}

/// AI翻译（优先查本地词典）
#[tauri::command]
pub async fn translate_text(text: String, source_lang: String, state: State<'_, AppState>) -> Result<TranslationResult, String> {
    let local_result: Option<TranslationResult> = {
        let db_guard = state.db.lock().unwrap();
        if let Some(db) = db_guard.as_ref() {
            match source_lang.as_str() {
                "ancient" => {
                    match db.query_ancient_all(&text) {
                        Ok(results) if !results.is_empty() => {
                            let mut translation = format_multiple_dicts_to_translation(&results);
                            let mut sources: Vec<String> = results.iter()
                                .filter_map(|r| r.source.clone())
                                .collect();
                            
                            if let Some(english) = db.query_chinese(&text) {
                                translation.push_str("\n\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n\n");
                                translation.push_str("【中英词典】\n");
                                translation.push_str(&format_dict_to_translation(&english));
                                sources.push("中英词典".to_string());
                            }
                            
                            let result = TranslationResult {
                                r#type: "translation".to_string(),
                                original: text.clone(),
                                translation,
                                notes: Some(vec![format!("来源：{}", sources.join("、"))]),
                            };
                            let result_json = serde_json::to_string(&result).unwrap_or_default();
                            let _ = db.add_history(&text, &source_lang, &result_json, "local");
                            Some(result)
                        }
                        _ => {
                            db.query_chinese(&text).map(|dict| {
                                let result = TranslationResult {
                                    r#type: "translation".to_string(),
                                    original: text.clone(),
                                    translation: format_dict_to_translation(&dict),
                                    notes: Some(vec!["来源：中英词典".to_string()]),
                                };
                                let result_json = serde_json::to_string(&result).unwrap_or_default();
                                let _ = db.add_history(&text, &source_lang, &result_json, "local");
                                result
                            })
                        }
                    }
                }
                "english" => {
                    db.query_english(&text).map(|dict| {
                        let translation = format_dict_to_translation(&dict);
                        let result = TranslationResult {
                            r#type: "translation".to_string(),
                            original: text.clone(),
                            translation,
                            notes: Some(vec![format!("来源：{}", dict.source.unwrap_or_else(|| "英汉词典".to_string()))]),
                        };
                        let result_json = serde_json::to_string(&result).unwrap_or_default();
                        let _ = db.add_history(&text, &source_lang, &result_json, "local");
                        result
                    })
                }
                "chinese" => {
                    db.query_chinese(&text).map(|dict| {
                        let translation = format_dict_to_translation(&dict);
                        let result = TranslationResult {
                            r#type: "translation".to_string(),
                            original: text.clone(),
                            translation,
                            notes: Some(vec![format!("来源：{}", dict.source.unwrap_or_else(|| "中英词典".to_string()))]),
                        };
                        let result_json = serde_json::to_string(&result).unwrap_or_default();
                        let _ = db.add_history(&text, &source_lang, &result_json, "local");
                        result
                    })
                }
                _ => {
                    let dict_result = db.query_ancient(&text)
                        .or_else(|| db.query_chinese(&text))
                        .or_else(|| db.query_english(&text));
                    
                    dict_result.map(|dict| {
                        let translation = format_dict_to_translation(&dict);
                        let result = TranslationResult {
                            r#type: "translation".to_string(),
                            original: text.clone(),
                            translation,
                            notes: Some(vec![format!("来源：{}", dict.source.unwrap_or_else(|| "本地词典".to_string()))]),
                        };
                        let result_json = serde_json::to_string(&result).unwrap_or_default();
                        let _ = db.add_history(&text, &source_lang, &result_json, "local");
                        result
                    })
                }
            }
        } else {
            None
        }
    };
    
    if let Some(result) = local_result {
        return Ok(result);
    }
    
    let client = state.api_client.lock().unwrap().clone();
    
    let result: Result<TranslationResult, anyhow::Error> = match source_lang.as_str() {
        "ancient" => client.translate_ancient(&text).await,
        "english" => client.translate_english(&text).await,
        "chinese" => client.translate_chinese(&text).await,
        _ => client.auto_translate(&text).await,
    };
    
    let result = result.map_err(|e| e.to_string())?;
    
    {
        let db_guard = state.db.lock().unwrap();
        if let Some(db) = db_guard.as_ref() {
            let result_json = serde_json::to_string(&result).unwrap_or_default();
            let _ = db.add_history(&text, &source_lang, &result_json, "api");
        }
    }
    
    Ok(result)
}

/// 合并多个词典结果为翻译文本
fn format_multiple_dicts_to_translation(dicts: &[DictionaryResult]) -> String {
    let mut sections = Vec::new();
    
    for dict in dicts {
        let source_name = dict.source.as_deref().unwrap_or("词典");
        let mut content = Vec::new();
        
        content.push(format!("【{}】", source_name));
        
        for def in &dict.definitions {
            if def.pos.is_empty() {
                content.push(def.definition.clone());
            } else {
                content.push(format!("{} {}", def.pos, def.definition));
            }
        }
        
        if let Some(examples) = &dict.examples {
            if !examples.is_empty() {
                content.push(String::new());
                content.push("例句：".to_string());
                for ex in examples {
                    let mut line = format!("• {}", ex.text);
                    if let Some(trans) = &ex.translation {
                        line.push_str(&format!(" —— {}", trans));
                    }
                    if let Some(src) = &ex.source {
                        line.push_str(&format!("（《{}》）", src));
                    }
                    content.push(line);
                }
            }
        }
        
        sections.push(content.join("\n"));
    }
    
    sections.join("\n\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n\n")
}

/// 将词典结果格式化为翻译文本
fn format_dict_to_translation(dict: &DictionaryResult) -> String {
    let mut lines = Vec::new();
    
    for def in &dict.definitions {
        lines.push(format!("【{}】{}", def.pos, def.definition));
    }
    
    if let Some(examples) = &dict.examples {
        if !examples.is_empty() {
            lines.push(String::new());
            lines.push("例句：".to_string());
            for ex in examples {
                let mut example_line = format!("• {}", ex.text);
                if let Some(trans) = &ex.translation {
                    example_line.push_str(&format!(" —— {}", trans));
                }
                if let Some(src) = &ex.source {
                    example_line.push_str(&format!(" ({})", src));
                }
                lines.push(example_line);
            }
        }
    }
    
    lines.join("\n")
}

/// 获取历史记录
#[tauri::command]
pub async fn get_history(limit: i32, state: State<'_, AppState>) -> Result<Vec<HistoryItem>, String> {
    let db_guard = state.db.lock().unwrap();
    let db = db_guard.as_ref().ok_or("数据库未初始化")?;
    
    db.get_history(limit).map_err(|e| e.to_string())
}

/// 清空历史记录
#[tauri::command]
pub async fn clear_history(state: State<'_, AppState>) -> Result<(), String> {
    let db_guard = state.db.lock().unwrap();
    let db = db_guard.as_ref().ok_or("数据库未初始化")?;
    
    db.clear_history().map_err(|e| e.to_string())
}

/// 添加到生词本
#[tauri::command]
pub async fn add_to_vocabulary(word: String, word_type: String, definition: String, note: Option<String>, state: State<'_, AppState>) -> Result<(), String> {
    let db_guard = state.db.lock().unwrap();
    let db = db_guard.as_ref().ok_or("数据库未初始化")?;
    
    // 检查是否已存在
    if db.vocabulary_exists(&word) {
        return Err("该词已在生词本中".to_string());
    }
    
    db.add_vocabulary(&word, &word_type, &definition, note.as_deref())
        .map_err(|e| e.to_string())?;
    
    Ok(())
}

/// 获取生词本
#[tauri::command]
pub async fn get_vocabulary(state: State<'_, AppState>) -> Result<Vec<VocabularyItem>, String> {
    let db_guard = state.db.lock().unwrap();
    let db = db_guard.as_ref().ok_or("数据库未初始化")?;
    
    db.get_vocabulary().map_err(|e| e.to_string())
}

/// 从生词本删除
#[tauri::command]
pub async fn remove_from_vocabulary(id: i64, state: State<'_, AppState>) -> Result<(), String> {
    let db_guard = state.db.lock().unwrap();
    let db = db_guard.as_ref().ok_or("数据库未初始化")?;
    
    db.remove_vocabulary(id).map_err(|e| e.to_string())
}

/// 获取设置
#[tauri::command]
pub async fn get_settings(state: State<'_, AppState>) -> Result<AppSettings, String> {
    let settings = state.settings.lock().unwrap().clone();
    Ok(settings)
}

/// 保存设置
#[tauri::command]
pub async fn save_settings(settings: AppSettings, state: State<'_, AppState>) -> Result<(), String> {
    let mut current = state.settings.lock().unwrap();
    *current = settings.clone();
    
    // 更新 API 客户端
    let mut client = state.api_client.lock().unwrap();
    client.update_settings(settings);
    
    Ok(())
}

/// 检测语言类型
#[tauri::command]
pub fn detect_language(text: String) -> String {
    let trimmed = text.trim();
    if trimmed.is_empty() {
        return "auto".to_string();
    }
    
    let has_chinese = trimmed.chars().any(|c| c >= '\u{4e00}' && c <= '\u{9fff}');
    let has_ascii = trimmed.chars().any(|c| c.is_ascii_alphabetic());
    let has_ascii_only = trimmed.chars().all(|c| c.is_ascii() || c.is_whitespace());
    
    if has_ascii_only && has_ascii {
        "english".to_string()
    } else if has_chinese {
        // 如果只包含中文字符（可能是单个或多个），优先检查是否为古文
        let non_chinese_chars: Vec<char> = trimmed.chars().filter(|c| !(*c >= '\u{4e00}' && *c <= '\u{9fff}') && !c.is_whitespace()).collect();
        
        if non_chinese_chars.is_empty() {
            // 纯中文文本
            if trimmed.chars().count() == 1 {
                // 单个中文字符，很可能是古文
                "ancient".to_string()
            } else {
                // 多个中文字符，检查是否包含古文特征字
                let ancient_chars = ["之", "乎", "者", "也", "矣", "焉", "哉", "曰", "于", "以", "而", "吾", "汝", "何", "乃", "若", "则", "虽", "故", "既", "且", "遂", "盖", "然", "或", "学", "哀"];
                let is_ancient = ancient_chars.iter().any(|&c| trimmed.contains(c));
                if is_ancient {
                    "ancient".to_string()
                } else {
                    "chinese".to_string()
                }
            }
        } else {
            // 包含非中文字符，可能是混合内容
            "chinese".to_string()
        }
    } else {
        "auto".to_string()
    }
}