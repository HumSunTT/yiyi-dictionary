// 应用类型定义

// ========== 词库相关 ==========

/// 词库查询结果
export interface DictionaryResult {
  type: 'dictionary'
  word: string
  phonetic?: string
  source?: string
  definitions: DefinitionItem[]
  examples?: ExampleItem[]
}

/// 释义项
export interface DefinitionItem {
  pos: string
  definition: string
}

/// 例句项
export interface ExampleItem {
  text: string
  translation?: string
  source?: string
}

// ========== AI翻译相关 ==========

/// AI翻译结果
export interface TranslationResult {
  type: 'translation'
  original: string
  translation: string
  notes?: string[]
}

// ========== 历史记录相关 ==========

/// 历史记录项
export interface HistoryItem {
  id: number
  query: string
  queryType: QueryType
  result: string
  source: ResultSource
  createdAt: string
}

/// 查询类型
export type QueryType = 'ancient' | 'english' | 'chinese' | 'sentence'

/// 结果来源
export type ResultSource = 'local' | 'api'

// ========== 生词本相关 ==========

/// 生词本项
export interface VocabularyItem {
  id: number
  word: string
  wordType: WordType
  definition: string
  note?: string
  reviewCount: number
  nextReview?: string
  addedAt: string
}

/// 单词类型
export type WordType = 'ancient' | 'english'

// ========== 设置相关 ==========

/// 应用设置
export interface AppSettings {
  apiKey: string
  apiEndpoint: string
  shortcuts: Shortcuts
  theme: ThemeType
  fontSize: number
  ancientEnabled: boolean
  englishEnabled: boolean
}

/// 快捷键设置
export interface Shortcuts {
  mainWindow: string
  selectionTranslate: string
}

/// 主题类型
export type ThemeType = 'light' | 'dark' | 'system'

// ========== API 响应类型 ==========

/// 统一查询结果（词库或翻译）
export type QueryResult = DictionaryResult | TranslationResult

/// API 错误响应
export interface ApiError {
  code: string
  message: string
}

// ========== 工具类型 ==========

/// 语言检测结果
export type LanguageType = 'ancient' | 'english' | 'chinese' | 'auto'

/// 翻译请求参数
export interface TranslateParams {
  text: string
  sourceLang: LanguageType
}

/// 查词请求参数
export interface QueryWordParams {
  word: string
  dictType: 'ancient' | 'english' | 'chinese' | 'auto'
}

// ========== DeepSeek API 类型 ==========

/// DeepSeek Chat 请求
export interface DeepSeekChatRequest {
  model: string
  messages: DeepSeekMessage[]
  temperature?: number
  max_tokens?: number
}

/// DeepSeek 消息
export interface DeepSeekMessage {
  role: 'system' | 'user' | 'assistant'
  content: string
}

/// DeepSeek Chat 响应
export interface DeepSeekChatResponse {
  id: string
  object: string
  created: number
  model: string
  choices: DeepSeekChoice[]
  usage: {
    prompt_tokens: number
    completion_tokens: number
    total_tokens: number
  }
}

/// DeepSeek 选择项
export interface DeepSeekChoice {
  index: number
  message: DeepSeekMessage
  finish_reason: string
}