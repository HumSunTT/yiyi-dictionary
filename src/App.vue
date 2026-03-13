<template>
  <n-config-provider :theme="theme" :locale="zhCN" :date-locale="dateZhCN">
    <n-global-style />
    <n-message-provider>
      <n-dialog-provider>
        <!-- 主窗口 -->
        <MainWindow v-if="!showFloatWindow" @show-float="showFloat" />
        
        <!-- 悬浮窗 -->
        <FloatWindow 
          v-if="showFloatWindow" 
          :result="floatResult"
          @close="closeFloatWindow" 
        />
      </n-dialog-provider>
    </n-message-provider>
  </n-config-provider>
</template>

<script setup lang="ts">
import { ref, computed, watch, onMounted, onUnmounted } from 'vue'
import { NConfigProvider, NMessageProvider, NDialogProvider, NGlobalStyle, darkTheme, zhCN, dateZhCN } from 'naive-ui'
import MainWindow from './components/MainWindow.vue'
import FloatWindow from './components/FloatWindow.vue'
import { useSettingsStore } from './stores/settings'
import { useDictionaryStore } from './stores/dictionary'
import type { DictionaryResult, TranslationResult } from './types'

const settingsStore = useSettingsStore()
const dictionaryStore = useDictionaryStore()

const theme = computed(() => {
  return settingsStore.theme === 'dark' ? darkTheme : null
})

// 悬浮窗状态
const showFloatWindow = ref(false)
const floatResult = ref<DictionaryResult | TranslationResult | null>(null)

// 剪贴板监听 - 改进版本
let clipboardCheckInterval: ReturnType<typeof setInterval> | null = null
let lastClipboardText = ''
let isProcessingClipboard = false

// 同步 dark class 到 html 元素
watch(() => settingsStore.theme, (newTheme) => {
  if (newTheme === 'dark') {
    document.documentElement.classList.add('dark')
  } else {
    document.documentElement.classList.remove('dark')
  }
}, { immediate: true })

// 显示悬浮窗
async function showFloat(text: string) {
  if (!text.trim()) return
  
  try {
    const result = await dictionaryStore.smartQuery(text)
    floatResult.value = result
    showFloatWindow.value = true
  } catch (error) {
    console.error('查询失败:', error)
  }
}

// 关闭悬浮窗
function closeFloatWindow() {
  showFloatWindow.value = false
  floatResult.value = null
}

// 监听剪贴板 - 改进版本
async function checkClipboard() {
  if (isProcessingClipboard) return
  
  try {
    const text = await navigator.clipboard.readText()
    if (text && text !== lastClipboardText && text.length <= 50 && text.length > 0) {
      // 避免重复处理
      isProcessingClipboard = true
      lastClipboardText = text
      
      // 自动查询
      await showFloat(text)
      
      // 重置处理状态 after a short delay
      setTimeout(() => {
        isProcessingClipboard = false
      }, 1000)
    }
  } catch (error) {
    // 剪贴板访问失败，忽略
    isProcessingClipboard = false
  }
}

function startClipboardWatch() {
  if (clipboardCheckInterval) return
  
  // Initial check
  checkClipboard()
  
  // Check every 300ms for better responsiveness
  clipboardCheckInterval = setInterval(checkClipboard, 300)
}

function stopClipboardWatch() {
  if (clipboardCheckInterval) {
    clearInterval(clipboardCheckInterval)
    clipboardCheckInterval = null
  }
}

onMounted(() => {
  // 如果启用了划词翻译，启动监听
  // Note: ancientEnabled setting is used to control clipboard monitoring (word selection translation)
  if (settingsStore.settings.ancientEnabled) {
    startClipboardWatch()
  }
})

onUnmounted(() => {
  stopClipboardWatch()
})

// 监听设置变化
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