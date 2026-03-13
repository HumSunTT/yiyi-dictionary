import { defineStore } from 'pinia'
import { ref } from 'vue'
import type { DictionaryResult, TranslationResult } from '../types'
import api from './api'

export const useDictionaryStore = defineStore('dictionary', () => {
  const loading = ref(false)
  const lastResult = ref<DictionaryResult | TranslationResult | null>(null)

  // 查询古汉语单词（多字典支持）
  async function queryAncientWord(word: string): Promise<DictionaryResult[]> {
    loading.value = true
    try {
      const results = await api.queryWordMulti(word, 'ancient')
      if (results.length > 0) {
        lastResult.value = results[0] // Keep first result for backward compatibility
      }
      return results
    } finally {
      loading.value = false
    }
  }

  // 查询单词（本地词库，支持多字典结果）
  async function queryWordMulti(word: string, type: 'ancient' | 'english' | 'chinese' | 'auto' = 'auto'): Promise<DictionaryResult[]> {
    loading.value = true
    try {
      const results = await api.queryWordMulti(word, type)
      if (results.length > 0) {
        // For now, just use the first result as lastResult
        lastResult.value = results[0]
      }
      return results
    } finally {
      loading.value = false
    }
  }

  // 查询单词（本地词库）
  async function queryWord(word: string, type: 'ancient' | 'english' | 'chinese' | 'auto' = 'auto'): Promise<DictionaryResult | null> {
    loading.value = true
    try {
      const result = await api.queryWord(word, type)
      if (result) {
        lastResult.value = result
      }
      return result
    } finally {
      loading.value = false
    }
  }

  // AI翻译
  async function translateText(text: string, sourceLang: string = 'auto'): Promise<TranslationResult> {
    loading.value = true
    try {
      const result = await api.translateText(text, sourceLang)
      lastResult.value = result
      return result
    } finally {
      loading.value = false
    }
  }

  // 检测语言
  async function detectLanguage(text: string): Promise<string> {
    return api.detectLanguage(text)
  }

  // 智能查询：先查本地词库，未命中则调用AI
  async function smartQuery(text: string): Promise<DictionaryResult | TranslationResult> {
    // 先尝试作为单词查询
    const trimmedText = text.trim()
    console.log('[Dictionary] smartQuery:', trimmedText)
    
    // 如果是短文本（可能是单词），先查词库
    if (trimmedText.length <= 10) {
      // 自动检测语言类型
      const lang = await detectLanguage(trimmedText)
      console.log('[Dictionary] detected language:', lang)
      
      // 尝试本地词库 - 使用检测到的语言作为字典类型
      const dictType = lang === 'english' ? 'english' : lang === 'ancient' ? 'ancient' : lang === 'chinese' ? 'chinese' : 'auto'
      console.log('[Dictionary] trying local dict with type:', dictType)
      
      try {
        // 对于古汉语，使用多字典查询并返回第一个结果 for now
        if (dictType === 'ancient') {
          const dictResults = await queryAncientWord(trimmedText)
          console.log('[Dictionary] local dict results:', dictResults)
          
          if (dictResults.length > 0) {
            // Return the first result for now, but in the future we can return all
            return dictResults[0]
          }
        } else {
          const dictResult = await queryWord(trimmedText, dictType)
          console.log('[Dictionary] local dict result:', dictResult)
          
          if (dictResult) {
            return dictResult
          }
        }
      } catch (error) {
        console.error('[Dictionary] local dict error:', error)
      }
    }
    
    // 调用AI翻译
    console.log('[Dictionary] falling back to AI translation')
    const lang = await detectLanguage(trimmedText)
    return translateText(trimmedText, lang)
  }

  return {
    loading,
    lastResult,
    queryWord,
    translateText,
    detectLanguage,
    smartQuery
  }
})