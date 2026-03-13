<template>
  <div class="settings-panel">
    <n-form :model="localSettings" label-placement="left" label-width="100">
      <!-- API设置 -->
      <n-divider>API 设置</n-divider>
      
      <n-form-item label="API Key">
        <n-input
          v-model:value="localSettings.apiKey"
          type="password"
          placeholder="输入 DeepSeek API Key"
          show-password-on="click"
        />
      </n-form-item>

      <n-form-item label="API 地址">
        <n-input
          v-model:value="localSettings.apiEndpoint"
          placeholder="https://api.deepseek.com"
        />
      </n-form-item>

      <!-- 快捷键设置 -->
      <n-divider>快捷键</n-divider>

      <n-form-item label="主窗口">
        <n-input
          v-model:value="localSettings.shortcuts.mainWindow"
          placeholder="Ctrl+Shift+T"
        />
      </n-form-item>

      <n-form-item label="划词翻译">
        <n-input
          v-model:value="localSettings.shortcuts.selectionTranslate"
          placeholder="Ctrl+Shift+D"
        />
      </n-form-item>

      <!-- 界面设置 -->
      <n-divider>界面</n-divider>

      <n-form-item label="主题">
        <n-radio-group v-model:value="localSettings.theme">
          <n-radio-button value="light">浅色</n-radio-button>
          <n-radio-button value="dark">深色</n-radio-button>
          <n-radio-button value="system">跟随系统</n-radio-button>
        </n-radio-group>
      </n-form-item>

      <n-form-item label="字体大小">
        <n-slider
          v-model:value="localSettings.fontSize"
          :min="12"
          :max="20"
          :step="1"
        />
      </n-form-item>

      <!-- 词库设置 -->
      <n-divider>词库</n-divider>

      <n-form-item label="古汉语词典">
        <n-switch v-model:value="localSettings.ancientEnabled" />
      </n-form-item>

      <n-form-item label="英汉词典">
        <n-switch v-model:value="localSettings.englishEnabled" />
      </n-form-item>

      <!-- 保存按钮 -->
      <n-button type="primary" block @click="handleSave" :loading="saving">
        保存设置
      </n-button>

      <!-- 关于 -->
      <n-divider>关于</n-divider>

      <div class="about">
        <p><strong>易译</strong> v0.1.0</p>
        <p>古文/英语划词翻译助手</p>
        <p class="hint">配置 DeepSeek API Key 后可使用 AI 翻译功能</p>
      </div>
    </n-form>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, reactive } from 'vue'
import { NForm, NFormItem, NInput, NDivider, NRadioGroup, NRadioButton, NSwitch, NSlider, NButton, useMessage } from 'naive-ui'
import { useSettingsStore } from '../stores/settings'
import api from '../stores/api'
import type { AppSettings } from '../types'

const settingsStore = useSettingsStore()
const message = useMessage()

const saving = ref(false)

const localSettings = reactive<AppSettings>({
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

onMounted(async () => {
  try {
    const settings = await api.getSettings()
    Object.assign(localSettings, settings)
  } catch (error) {
    console.error('加载设置失败:', error)
  }
})

async function handleSave() {
  saving.value = true
  try {
    await api.saveSettings(localSettings)
    settingsStore.updateSettings(localSettings)
    message.success('设置已保存')
  } catch (error: any) {
    message.error(error.message || '保存失败')
  } finally {
    saving.value = false
  }
}
</script>

<style scoped>
.settings-panel {
  padding: 0 8px;
}

.about {
  text-align: center;
  color: #666;
  font-size: 13px;
}

.about p {
  margin-bottom: 8px;
}

.about .hint {
  font-size: 12px;
  color: #999;
}
</style>