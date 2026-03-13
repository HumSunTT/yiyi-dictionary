<template>
  <n-layout class="main-window">
    <n-layout-header class="header" bordered>
      <div class="logo">
        <span class="logo-icon">易</span>
        <span class="logo-text">易译</span>
      </div>
      <div class="header-actions">
        <n-button quaternary size="small" @click="showHistory = true">
          <template #icon><n-icon :component="TimeOutline" /></template>
          历史
        </n-button>
        <n-button quaternary size="small" @click="showVocabulary = true">
          <template #icon><n-icon :component="StarOutline" /></template>
          生词本
        </n-button>
        <n-button quaternary size="small" @click="showSettings = true">
          <template #icon><n-icon :component="SettingsOutline" /></template>
          设置
        </n-button>
        <n-button quaternary size="small" @click="showHelp = true">
          <template #icon><n-icon :component="HelpOutline" /></template>
          帮助
        </n-button>
      </div>
    </n-layout-header>

    <n-layout-content class="content">
      <!-- 输入区域 -->
      <n-card class="input-card" :bordered="false">
        <n-input
          v-model:value="inputText"
          type="textarea"
          placeholder="输入中文或英文，按 Enter 翻译..."
          :autosize="{ minRows: 3, maxRows: 6 }"
          @keydown.enter="handleEnterKey"
        />
        <div class="input-actions">
          <span class="hint">自动识别语言</span>
          <n-button type="primary" @click="handleTranslate" :loading="dictionaryStore.loading">
            翻译
          </n-button>
        </div>
      </n-card>

      <!-- 结果区域 -->
      <n-card v-if="result" class="result-card" :bordered="false">
        <ResultCard :result="result" @add-to-vocabulary="handleAddToVocabulary" />
      </n-card>
      
      <!-- 空状态 -->
      <n-empty v-else-if="!dictionaryStore.loading" description="输入文字开始翻译" class="empty-state" />
    </n-layout-content>
  </n-layout>

  <!-- 抽屉 -->
  <n-drawer v-model:show="showHistory" width="300px">
    <n-drawer-content title="历史记录">
      <HistoryPanel @select="handleHistorySelect" />
    </n-drawer-content>
  </n-drawer>

  <n-drawer v-model:show="showVocabulary" width="300px">
    <n-drawer-content title="生词本">
      <VocabularyPanel />
    </n-drawer-content>
  </n-drawer>

  <n-drawer v-model:show="showSettings" width="300px">
    <n-drawer-content title="设置">
      <SettingsPanel />
    </n-drawer-content>
  </n-drawer>

  <n-drawer v-model:show="showHelp" width="300px">
    <n-drawer-content title="帮助">
      <HelpPanel />
    </n-drawer-content>
  </n-drawer>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import {
  NLayout, NLayoutHeader, NLayoutContent,
  NCard, NInput, NButton, NIcon, NDrawer, NDrawerContent, NEmpty, useMessage
} from 'naive-ui'
import { TimeOutline, StarOutline, SettingsOutline, HelpOutline } from '@vicons/ionicons5'
import ResultCard from './ResultCard.vue'
import HistoryPanel from './HistoryPanel.vue'
import VocabularyPanel from './VocabularyPanel.vue'
import SettingsPanel from './SettingsPanel.vue'
import HelpPanel from './HelpPanel.vue'
import { useDictionaryStore } from '../stores/dictionary'
import api from '../stores/api'
import type { DictionaryResult, TranslationResult } from '../types'

const dictionaryStore = useDictionaryStore()
const message = useMessage()

const emit = defineEmits<{
  (e: 'show-float', text: string): void
}>()

const inputText = ref('')
const result = ref<DictionaryResult | TranslationResult | null>(null)

const showHistory = ref(false)
const showVocabulary = ref(false)
const showSettings = ref(false)
const showHelp = ref(false)

function detectLanguage(text: string): 'ancient' | 'english' {
  const trimmed = text.trim()
  const hasChinese = trimmed.split('').some(c => c >= '\u4e00' && c <= '\u9fff')
  return hasChinese ? 'ancient' : 'english'
}

function handleEnterKey(event: KeyboardEvent) {
  if (event.shiftKey) return
  event.preventDefault()
  handleTranslate()
}

async function handleTranslate() {
  if (!inputText.value.trim()) return
  
  try {
    const lang = detectLanguage(inputText.value)
    result.value = await dictionaryStore.translateText(inputText.value, lang)
  } catch (error: any) {
    message.error(error.message || '翻译失败')
  }
}

function handleHistorySelect(query: string) {
  inputText.value = query
  showHistory.value = false
  handleTranslate()
}

async function handleAddToVocabulary() {
  if (!result.value) return
  
  const word = result.value.type === 'dictionary' 
    ? (result.value as DictionaryResult).word 
    : (result.value as TranslationResult).original.slice(0, 20)
  
  const definitionText = result.value.type === 'dictionary'
    ? (result.value as DictionaryResult).definitions.map(d => d.definition).join('；')
    : (result.value as TranslationResult).translation.slice(0, 100)
  
  await api.addToVocabulary(word, result.value.type === 'dictionary' ? 'ancient' : 'english', definitionText)
  message.success(`已添加"${word}"到生词本`)
}
</script>

<style scoped>
.main-window {
  height: 100vh;
  display: flex;
  flex-direction: column;
  background-color: var(--n-color);
}

.header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 8px 16px;
  background-color: #fff;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.08);
}

.logo {
  display: flex;
  align-items: center;
  gap: 8px;
}

.logo-icon {
  width: 28px;
  height: 28px;
  background: linear-gradient(135deg, #6366f1, #8b5cf6);
  border-radius: 6px;
  display: flex;
  align-items: center;
  justify-content: center;
  color: white;
  font-weight: bold;
  font-size: 16px;
}

.logo-text {
  font-size: 18px;
  font-weight: 600;
  color: #1f2937;
}

.header-actions {
  display: flex;
  gap: 4px;
}

.content {
  flex: 1;
  padding: 16px;
  overflow-y: auto;
  background-color: #f8fafc;
  display: flex;
  flex-direction: column;
  align-items: center;
}

.input-card {
  margin-bottom: 16px;
  background-color: #fff;
  border-radius: 12px;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.06);
  width: 100%;
  max-width: 900px;
}

.input-actions {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-top: 12px;
}

.hint {
  font-size: 13px;
  color: #9ca3af;
}

.input-actions .n-button {
  background: linear-gradient(135deg, #6366f1, #8b5cf6);
  border: none;
}

.input-actions .n-button:hover {
  background: linear-gradient(135deg, #4f46e5, #7c3aed);
}

.result-card {
  margin-bottom: 16px;
  background-color: #fff;
  border-radius: 12px;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.06);
  width: 100%;
  max-width: 900px;
}

.empty-state {
  margin-top: 60px;
  width: 100%;
  max-width: 900px;
}
</style>