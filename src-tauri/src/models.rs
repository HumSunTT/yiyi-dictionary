// 数据模型定义

use serde::{Deserialize, Serialize};

/// 词库查询结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DictionaryResult {
    pub r#type: String,
    pub word: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phonetic: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    pub definitions: Vec<DefinitionItem>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub examples: Option<Vec<ExampleItem>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefinitionItem {
    pub pos: String,
    pub definition: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExampleItem {
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub translation: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
}

/// AI翻译结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranslationResult {
    pub r#type: String,
    pub original: String,
    pub translation: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<Vec<String>>,
}

/// 历史记录
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryItem {
    pub id: i64,
    pub query: String,
    pub query_type: String,
    pub result: String,
    pub source: String,
    pub created_at: String,
}

/// 生词本
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VocabularyItem {
    pub id: i64,
    pub word: String,
    pub word_type: String,
    pub definition: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    pub added_at: String,
}

/// 应用设置
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppSettings {
    #[serde(default)]
    pub api_key: String,
    #[serde(default = "default_api_endpoint")]
    pub api_endpoint: String,
    #[serde(default)]
    pub shortcuts: Shortcuts,
    #[serde(default)]
    pub theme: String,
    #[serde(default = "default_font_size")]
    pub font_size: u8,
    #[serde(default = "default_true")]
    pub ancient_enabled: bool,
    #[serde(default = "default_true")]
    pub english_enabled: bool,
}

fn default_api_endpoint() -> String {
    "https://api.deepseek.com".to_string()
}

fn default_font_size() -> u8 {
    14
}

fn default_true() -> bool {
    true
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Shortcuts {
    #[serde(default = "default_main_window_shortcut")]
    pub main_window: String,
    #[serde(default = "default_selection_shortcut")]
    pub selection_translate: String,
}

fn default_main_window_shortcut() -> String {
    "Ctrl+Shift+T".to_string()
}

fn default_selection_shortcut() -> String {
    "Ctrl+Shift+D".to_string()
}