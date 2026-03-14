import { defineStore } from 'pinia'
import { ref } from 'vue'
import type { DictionaryResult, TranslationResult } from '../types'
import api from './api'

export const useDictionaryStore = defineStore('dictionary', () => {
  const loading = ref(false)
  const lastResult = ref<DictionaryResult | TranslationResult | null>(null)

  async function queryAncientWord(word: string): Promise<DictionaryResult[]> {
    loading.value = true
    try {
      const results = await api.queryWordMulti(word, 'ancient')
      if (results.length > 0) {
        lastResult.value = results[0]
      }
      return results
    } finally {
      loading.value = false
    }
  }

  async function queryWordMulti(word: string, type: 'ancient' | 'english' | 'chinese' | 'auto' = 'auto'): Promise<DictionaryResult[]> {
    loading.value = true
    try {
      const results = await api.queryWordMulti(word, type)
      if (results.length > 0) {
        lastResult.value = results[0]
      }
      return results
    } finally {
      loading.value = false
    }
  }

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

  async function detectLanguage(text: string): Promise<string> {
    return api.detectLanguage(text)
  }

  async function smartQuery(text: string): Promise<TranslationResult> {
    const trimmedText = text.trim()
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