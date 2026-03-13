import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import type { AppSettings } from '../types'

export const useSettingsStore = defineStore('settings', () => {
  const settings = ref<AppSettings>({
    apiKey: '',
    apiEndpoint: 'https://api.deepseek.com',
    shortcuts: {
      mainWindow: 'Ctrl+Shift+T',
      selectionTranslate: 'Ctrl+Shift+D'
    },
    theme: 'light',
    fontSize: 14,
    ancientEnabled: true,
    englishEnabled: true
  })

  // 计算属性
  const theme = computed(() => settings.value.theme)
  const fontSize = computed(() => settings.value.fontSize)

  // 切换主题
  function toggleTheme() {
    settings.value.theme = settings.value.theme === 'dark' ? 'light' : 'dark'
  }

  // 更新设置
  function updateSettings(newSettings: Partial<AppSettings>) {
    settings.value = { ...settings.value, ...newSettings }
  }

  // 保存设置到本地
  async function saveSettings() {
    // TODO: 使用 tauri-plugin-store 保存
    console.log('保存设置:', settings.value)
  }

  // 加载设置
  async function loadSettings() {
    // TODO: 使用 tauri-plugin-store 加载
    console.log('加载设置')
  }

  return {
    settings,
    theme,
    fontSize,
    toggleTheme,
    updateSettings,
    saveSettings,
    loadSettings
  }
})