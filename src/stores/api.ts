// Tauri API 调用封装

import type { DictionaryResult, TranslationResult, HistoryItem, VocabularyItem, AppSettings, WordType } from '../types'
import { mockAncientWords, mockEnglishWords, mockHistory, mockVocabulary, mockSettings, mockTranslate } from '../data/mockData'

// 检测是否在 Tauri 环境中
function isTauriEnv(): boolean {
  return typeof window !== 'undefined' && '__TAURI_INTERNALS__' in window
}

// 动态导入 Tauri API
async function invoke<T>(cmd: string, args?: Record<string, unknown>): Promise<T> {
  if (!isTauriEnv()) {
    console.log('[API] Not in Tauri environment, using mock data')
    return mockInvoke(cmd, args) as Promise<T>
  }
  
  try {
    const { invoke: tauriInvoke } = await import('@tauri-apps/api/core')
    return tauriInvoke<T>(cmd, args)
  } catch (error) {
    console.warn('[API] Tauri invoke failed, using mock data:', error)
    return mockInvoke(cmd, args) as Promise<T>
  }
}

// 模拟 API 调用
async function mockInvoke(cmd: string, args?: Record<string, unknown>): Promise<unknown> {
  // 模拟网络延迟
  await new Promise(r => setTimeout(r, 300 + Math.random() * 200))
  
  switch (cmd) {
    case 'query_word':
      return mockQueryWord(args?.word as string, args?.dictType as string)
    
    case 'translate_text':
      return mockTranslate(args?.text as string, args?.sourceLang as string || 'auto')
    
    case 'detect_language':
      return detectLanguageLocal(args?.text as string)
    
    case 'get_history':
      return Promise.resolve(mockHistory)
    
    case 'clear_history':
      mockHistory.length = 0
      return Promise.resolve()
    
    case 'add_to_vocabulary': {
      const newId = Math.max(0, ...mockVocabulary.map(v => v.id)) + 1
      mockVocabulary.unshift({
        id: newId,
        word: args?.word as string,
        wordType: args?.wordType as WordType,
        definition: args?.definition as string,
        note: args?.note as string | undefined,
        reviewCount: 0,
        addedAt: new Date().toISOString()
      })
      return Promise.resolve()
    }
    
    case 'get_vocabulary':
      return Promise.resolve(mockVocabulary)
    
    case 'remove_from_vocabulary': {
      const id = args?.id as number
      const index = mockVocabulary.findIndex(v => v.id === id)
      if (index >= 0) mockVocabulary.splice(index, 1)
      return Promise.resolve()
    }
    
    case 'get_settings':
      return Promise.resolve(mockSettings)
    
    case 'save_settings':
      Object.assign(mockSettings, args?.settings)
      return Promise.resolve()
    
    default:
      return Promise.resolve(null)
  }
}

// 模拟查词
function mockQueryWord(word: string, dictType: string): DictionaryResult | null {
  // 先查古汉语
  if (dictType === 'ancient' || dictType === 'auto') {
    if (mockAncientWords[word]) {
      return mockAncientWords[word]
    }
  }
  
  // 再查英语
  if (dictType === 'english' || dictType === 'auto') {
    const lowerWord = word.toLowerCase()
    if (mockEnglishWords[lowerWord]) {
      return mockEnglishWords[lowerWord]
    }
  }
  
  return null
}

// 本地语言检测
function detectLanguageLocal(text: string): string {
  const hasChinese = text.split('').some(c => c.charCodeAt(0) >= 0x4e00 && c.charCodeAt(0) <= 0x9fff)
  const hasAsciiLetter = text.split('').some(c => (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z'))
  const isAsciiOnly = text.split('').every(c => c.charCodeAt(0) < 128)
  
  if (isAsciiOnly && hasAsciiLetter) return 'english'
  
  if (hasChinese) {
    // 检测古文特征词
    const ancientMarkers = ['之', '乎', '者', '也', '矣', '焉', '哉', '曰', '于', '以', '而', '其', '乃', '所', '与']
    const hasAncientMarker = ancientMarkers.some(m => text.includes(m))
    return hasAncientMarker ? 'ancient' : 'chinese'
  }
  
  return 'auto'
}

// 导出的 API 函数
export const api = {
  // 查询单词（支持多字典结果）
  async queryWordMulti(word: string, dictType: string = 'auto'): Promise<DictionaryResult[]> {
    console.log('[API] queryWordMulti called:', word, dictType)
    try {
      const result = await invoke<DictionaryResult[]>('query_word_multi', { 
        word,
        dictType
      })
      
      console.log('[API] queryWordMulti result:', result)
      return result
    } catch (error) {
      console.error('[API] queryWordMulti error:', error)
      return []
    }
  },
  
  // 查询单词
  async queryWord(word: string, dictType: string = 'auto'): Promise<DictionaryResult | null> {
    console.log('[API] queryWord called:', word, dictType)
    console.log('[API] isTauriEnv:', isTauriEnv())
    try {
      const result = await invoke<DictionaryResult | null>('query_word', { 
        word,
        dictType
      })
      
      console.log('[API] queryWord result:', result)
      return result
    } catch (error) {
      console.error('[API] queryWord error:', error)
      return null
    }
  },
  
// AI翻译
  async translateText(text: string, sourceLang: string = 'auto'): Promise<TranslationResult> {
    console.log('[API] translateText called:', text, sourceLang)
    try {
      const result = await invoke<TranslationResult>('translate_text', { 
        text, 
        sourceLang
      })
      console.log('[API] translateText result:', result)
      return result
    } catch (error) {
      console.error('[API] translateText error:', error)
      // 翻译失败时返回错误结果
      const errorMessage = error instanceof Error ? error.message : String(error)
      return {
        type: 'translation',
        original: text,
        translation: `翻译失败: ${errorMessage || '未知错误'}`,
        notes: ['请检查网络连接和 API 配置', '在设置中配置 DeepSeek API Key']
      }
    }
  },
  
  // 检测语言
  async detectLanguage(text: string): Promise<string> {
    try {
      return await invoke<string>('detect_language', { text })
    } catch (error) {
      console.warn('[API] detectLanguage error, using local:', error)
      return detectLanguageLocal(text)
    }
  },
  
  // 历史记录
  async getHistory(limit: number = 50): Promise<HistoryItem[]> {
    return invoke<HistoryItem[]>('get_history', { limit })
  },
  
  async clearHistory(): Promise<void> {
    return invoke('clear_history')
  },
  
  // 生词本
  async addToVocabulary(word: string, wordType: string, definition: string, note?: string): Promise<void> {
    return invoke('add_to_vocabulary', { word, wordType, definition, note })
  },
  
  async getVocabulary(): Promise<VocabularyItem[]> {
    return invoke<VocabularyItem[]>('get_vocabulary')
  },
  
  async removeFromVocabulary(id: number): Promise<void> {
    return invoke('remove_from_vocabulary', { id })
  },
  
  // 设置
  async getSettings(): Promise<AppSettings> {
    return invoke<AppSettings>('get_settings')
  },
  
  async saveSettings(settings: AppSettings): Promise<void> {
    return invoke('save_settings', { settings })
  },
}

// 默认导出
export default api