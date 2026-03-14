<template>
  <n-config-provider :theme="theme" :locale="zhCN" :date-locale="dateZhCN">
    <n-global-style />
    <n-message-provider>
      <n-dialog-provider>
        <MainWindow 
          ref="mainWindowRef"
          :external-result="externalResult"
          @clear-external="externalResult = null"
        />
      </n-dialog-provider>
    </n-message-provider>
  </n-config-provider>
</template>

<script setup lang="ts">
import { ref, computed, watch, onMounted, onUnmounted } from 'vue'
import { NConfigProvider, NMessageProvider, NDialogProvider, NGlobalStyle, darkTheme, zhCN, dateZhCN } from 'naive-ui'
import { listen } from '@tauri-apps/api/event'
import MainWindow from './components/MainWindow.vue'
import { useSettingsStore } from './stores/settings'
import { useDictionaryStore } from './stores/dictionary'
import type { DictionaryResult, TranslationResult } from './types'

const settingsStore = useSettingsStore()
const dictionaryStore = useDictionaryStore()

const mainWindowRef = ref<InstanceType<typeof MainWindow> | null>(null)
const externalResult = ref<TranslationResult | null>(null)

const theme = computed(() => {
  return settingsStore.theme === 'dark' ? darkTheme : null
})

let clipboardCheckInterval: ReturnType<typeof setInterval> | null = null
let lastClipboardText = ''
let isProcessingClipboard = false
let unlistenSelection: (() => void) | null = null

watch(() => settingsStore.theme, (newTheme) => {
  if (newTheme === 'dark') {
    document.documentElement.classList.add('dark')
  } else {
    document.documentElement.classList.remove('dark')
  }
}, { immediate: true })

async function handleSelectionTranslate(text: string) {
  if (!text.trim()) return
  
  try {
    const result = await dictionaryStore.smartQuery(text)
    externalResult.value = result
  } catch (error) {
    console.error('查询失败:', error)
  }
}

async function checkClipboard() {
  if (isProcessingClipboard) return
  
  try {
    const text = await navigator.clipboard.readText()
    if (text && text !== lastClipboardText && text.length <= 50 && text.length > 0) {
      isProcessingClipboard = true
      lastClipboardText = text
      
      await handleSelectionTranslate(text)
      
      setTimeout(() => {
        isProcessingClipboard = false
      }, 1000)
    }
  } catch (error) {
    isProcessingClipboard = false
  }
}

function startClipboardWatch() {
  if (clipboardCheckInterval) return
  
  checkClipboard()
  clipboardCheckInterval = setInterval(checkClipboard, 300)
}

function stopClipboardWatch() {
  if (clipboardCheckInterval) {
    clearInterval(clipboardCheckInterval)
    clipboardCheckInterval = null
  }
}

onMounted(async () => {
  if (settingsStore.settings.ancientEnabled) {
    startClipboardWatch()
  }
  
  unlistenSelection = await listen<string>('selection-translate', async (event) => {
    const text = event.payload
    if (text && text.trim()) {
      await handleSelectionTranslate(text)
    }
  })
})

onUnmounted(() => {
  stopClipboardWatch()
  if (unlistenSelection) {
    unlistenSelection()
  }
})

watch(() => settingsStore.settings.ancientEnabled, (enabled) => {
  if (enabled) {
    startClipboardWatch()
  } else {
    stopClipboardWatch()
  }
})
</script>

<style>
html, body, #app {
  width: 100%;
  height: 100%;
  margin: 0;
  padding: 0;
}
</style>