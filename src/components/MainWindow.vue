<template>
  <n-layout class="main-window">
    <n-layout-header class="header" bordered>
      <div class="logo">
        <span class="logo-icon">易</span>
        <span class="logo-text">易译</span>
      </div>
      <n-button quaternary circle @click="toggleTheme">
        <template #icon>
          <n-icon :component="themeIcon" />
        </template>
      </n-button>
    </n-layout-header>

    <n-layout-content class="content">
      <!-- 输入区域 -->
      <n-card class="input-card" :bordered="false">
        <n-input
          v-model:value="inputText"
          type="textarea"
          placeholder="输入或粘贴文字，按 Enter 翻译..."
          :autosize="{ minRows: 3, maxRows: 6 }"
          @keydown.enter="handleEnterKey"
        />
        <div class="input-actions">
          <n-select
            v-model:value="sourceLang"
            :options="langOptions"
            style="width: 120px"
            size="small"
          />
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

    <n-layout-footer class="footer" bordered>
      <n-button quaternary @click="showHistory = true">
        <template #icon><n-icon :component="TimeOutline" /></template>
        历史
      </n-button>
      <n-button quaternary @click="showVocabulary = true">
        <template #icon><n-icon :component="StarOutline" /></template>
        生词本
      </n-button>
      <n-button quaternary @click="showHelp = true">
        <template #icon><n-icon :component="HelpOutline" /></template>
        帮助
      </n-button>
      <n-button quaternary @click="showSettings = true">
        <template #icon><n-icon :component="SettingsOutline" /></template>
        设置
      </n-button>
    </n-layout-footer>
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
import { ref, computed } from 'vue'
import {
  NLayout, NLayoutHeader, NLayoutContent, NLayoutFooter,
  NCard, NInput, NButton, NSelect, NIcon, NDrawer, NDrawerContent, NEmpty, useMessage
} from 'naive-ui'
import { TimeOutline, StarOutline, SettingsOutline, HelpOutline, SunnyOutline, MoonOutline } from '@vicons/ionicons5'
import ResultCard from './ResultCard.vue'
import HistoryPanel from './HistoryPanel.vue'
import VocabularyPanel from './VocabularyPanel.vue'
import SettingsPanel from './SettingsPanel.vue'
import HelpPanel from './HelpPanel.vue'
import { useDictionaryStore } from '../stores/dictionary'
import { useSettingsStore } from '../stores/settings'
import api from '../stores/api'
import type { DictionaryResult, TranslationResult } from '../types'

const dictionaryStore = useDictionaryStore()
const settingsStore = useSettingsStore()
const message = useMessage()

const emit = defineEmits<{
  (e: 'show-float', text: string): void
}>()

const inputText = ref('')
const sourceLang = ref('auto')
const result = ref<DictionaryResult | TranslationResult | null>(null)

const showHistory = ref(false)
const showVocabulary = ref(false)
const showSettings = ref(false)
const showHelp = ref(false)

const langOptions = [
  { label: '自动检测', value: 'auto' },
  { label: '古文', value: 'ancient' },
  { label: '英文', value: 'english' },
  { label: '中文', value: 'chinese' }
]

const themeIcon = computed(() => {
  return settingsStore.theme === 'dark' ? SunnyOutline : MoonOutline
})

function toggleTheme() {
  settingsStore.toggleTheme()
}

function handleEnterKey(event: KeyboardEvent) {
  if (event.shiftKey) return
  event.preventDefault()
  handleTranslate()
}

async function handleTranslate() {
  if (!inputText.value.trim()) return
  
  try {
    if (sourceLang.value === 'auto') {
      // 智能模式：先查词库，再翻译
      result.value = await dictionaryStore.smartQuery(inputText.value)
    } else {
      // 指定语言模式
      const dictResult = await dictionaryStore.queryWord(inputText.value, sourceLang.value as 'ancient' | 'english' | 'auto')
      if (dictResult) {
        result.value = dictResult
      } else {
        result.value = await dictionaryStore.translateText(inputText.value, sourceLang.value)
      }
    }
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
  background-color: var(--n-color);
}

.logo {
  display: flex;
  align-items: center;
  gap: 8px;
}

.logo-icon {
  width: 28px;
  height: 28px;
  background: linear-gradient(135deg, #18a058, #36d399);
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
  color: #333;
}

.content {
  flex: 1;
  padding: 16px;
  overflow-y: auto;
  background-color: #f5f7fa;
}

.input-card {
  margin-bottom: 16px;
  background-color: #fff;
}

.input-actions {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-top: 12px;
}

.result-card {
  margin-bottom: 16px;
  background-color: #fff;
}

.empty-state {
  margin-top: 60px;
}

.footer {
  display: flex;
  justify-content: space-around;
  padding: 8px;
  background-color: #fff;
}
</style>