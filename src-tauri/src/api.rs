// DeepSeek API 调用模块

use reqwest::Client;
use serde::{Deserialize, Serialize};
use anyhow::{Result, anyhow};

use crate::models::{TranslationResult, AppSettings};

#[derive(Debug, Serialize)]
struct ChatRequest {
    model: String,
    messages: Vec<Message>,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Debug, Deserialize)]
struct ChatResponse {
    choices: Vec<Choice>,
}

#[derive(Debug, Deserialize)]
struct Choice {
    message: Message,
}

#[derive(Debug, Clone)]
pub struct DeepSeekClient {
    settings: AppSettings,
}

impl DeepSeekClient {
    pub fn new(settings: AppSettings) -> Self {
        Self {
            settings,
        }
    }
    
    pub fn update_settings(&mut self, settings: AppSettings) {
        self.settings = settings;
    }
    
    fn get_client() -> Client {
        Client::new()
    }
    
    /// 翻译古文到现代汉语
    pub async fn translate_ancient(&self, text: &str) -> Result<TranslationResult> {
        let prompt = format!(
            r#"你是一位专业的古文翻译专家。请将以下古文翻译成现代汉语。

要求：
1. 准确理解原文含义
2. 翻译通顺流畅
3. 如有典故或特殊用法，请简要说明
4. 保持原文的语气和风格

原文：{}

请按以下格式输出：
【翻译】你的翻译内容
【注释】如有典故或特殊用法，在此说明（可选）"#,
            text
        );
        
        self.translate(text, &prompt).await
    }
    
    /// 翻译英文到中文
    pub async fn translate_english(&self, text: &str) -> Result<TranslationResult> {
        let prompt = format!(
            r#"You are a professional translator. Please translate the following English text into Chinese.

Requirements:
1. Accurate translation
2. Natural and fluent Chinese
3. Keep the original tone and style
4. If there are idioms or cultural references, explain them briefly

Original text: {}

Please output in this format:
【翻译】Your translation
【注释】Explanations if any (optional)"#,
            text
        );
        
        self.translate(text, &prompt).await
    }
    
    /// 翻译中文到英文
    pub async fn translate_chinese(&self, text: &str) -> Result<TranslationResult> {
        let prompt = format!(
            r#"You are a professional translator. Please translate the following Chinese text into English.

Requirements:
1. Accurate translation
2. Natural and fluent English
3. Keep the original tone and style

Original text: {}

Please output in this format:
【翻译】Your translation
【注释】Explanations if any (optional)"#,
            text
        );
        
        self.translate(text, &prompt).await
    }
    
    /// 自动检测语言并翻译
    pub async fn auto_translate(&self, text: &str) -> Result<TranslationResult> {
        // 简单的语言检测逻辑
        let has_chinese = text.chars().any(|c| c >= '\u{4e00}' && c <= '\u{9fff}');
        let has_ascii = text.chars().any(|c| c.is_ascii_alphabetic());
        
        // 检测古文特征
        let ancient_markers = ["之", "乎", "者", "也", "矣", "焉", "哉", "曰", "于", "以", "而", "其", "乃", "所", "与"];
        let is_ancient = ancient_markers.iter().any(|&m| text.contains(m));
        
        if has_chinese && is_ancient {
            // 古文 → 现代汉语
            self.translate_ancient(text).await
        } else if has_chinese && !has_ascii {
            // 纯中文 → 英文
            self.translate_chinese(text).await
        } else if has_ascii && !has_chinese {
            // 英文 → 中文
            self.translate_english(text).await
        } else {
            // 混合内容，尝试智能翻译
            self.translate_chinese(text).await
        }
    }
    
    async fn translate(&self, original: &str, prompt: &str) -> Result<TranslationResult> {
        if self.settings.api_key.is_empty() {
            // Provide more specific error message based on the content
            let has_chinese = original.chars().any(|c| c >= '\u{4e00}' && c <= '\u{9fff}');
            if has_chinese {
                return Err(anyhow!("中文翻译需要配置 DeepSeek API Key。请在设置中配置 API Key，或使用已内置词典的常见词汇。"));
            } else {
                return Err(anyhow!("API Key 未设置，请在设置中配置 DeepSeek API Key"));
            }
        }
        
        let url = format!("{}/chat/completions", self.settings.api_endpoint);
        
        let request = ChatRequest {
            model: "deepseek-chat".to_string(),
            messages: vec![
                Message {
                    role: "user".to_string(),
                    content: prompt.to_string(),
                }
            ],
            temperature: Some(0.3),
        };
        
        let response = Self::get_client()
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.settings.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await?;
        
        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(anyhow!("API 请求失败: {} - {}", status, body));
        }
        
        let chat_response: ChatResponse = response.json().await?;
        
        let content = chat_response
            .choices
            .first()
            .map(|c| c.message.content.clone())
            .ok_or_else(|| anyhow!("API 返回结果为空"))?;
        
        // 如果内容为空，返回明确的错误信息
        if content.trim().is_empty() {
            return Err(anyhow!("API 返回了空内容"));
        }
        
        // 解析结果
        let (translation, notes) = parse_translation_result(&content);
        
        // 如果解析后的翻译仍然为空，返回错误
        if translation.trim().is_empty() {
            return Err(anyhow!("翻译结果为空，请检查API响应格式"));
        }
        
        Ok(TranslationResult {
            r#type: "translation".to_string(),
            original: original.to_string(),
            translation,
            notes,
        })
    }
}

/// 解析翻译结果
fn parse_translation_result(content: &str) -> (String, Option<Vec<String>>) {
    let mut translation = String::new();
    let mut notes: Vec<String> = Vec::new();
    
    let mut in_translation = false;
    let mut in_notes = false;
    
    for line in content.lines() {
        let line = line.trim();
        
        if line.starts_with("【翻译】") {
            in_translation = true;
            in_notes = false;
            translation = line.strip_prefix("【翻译】").unwrap_or(line).trim().to_string();
        } else if line.starts_with("【注释】") {
            in_translation = false;
            in_notes = true;
            let note = line.strip_prefix("【注释】").unwrap_or(line).trim();
            if !note.is_empty() {
                notes.push(note.to_string());
            }
        } else if in_notes && !line.is_empty() {
            notes.push(line.to_string());
        } else if in_translation && !line.is_empty() {
            translation.push_str(line);
        }
    }
    
    // 如果没有找到格式化的输出，直接返回原文
    if translation.is_empty() {
        translation = content.to_string();
    }
    
    // 如果翻译结果仍然是空的，提供一个默认错误消息
    if translation.trim().is_empty() {
        translation = "翻译失败：API 返回了空内容".to_string();
    }
    
    let notes = if notes.is_empty() { None } else { Some(notes) };
    
    (translation, notes)
}