<template>
  <div class="history-panel">
    <n-spin :show="loading">
      <n-empty v-if="history.length === 0" description="暂无历史记录" />
      <div v-else class="history-list">
        <div
          v-for="item in history"
          :key="item.id"
          class="history-item"
          @click="handleSelect(item)"
        >
          <div class="query">{{ item.query }}</div>
          <div class="meta">
            <n-tag size="small">{{ item.source === 'local' ? '本地词库' : 'AI翻译' }}</n-tag>
            <span class="time">{{ formatTime(item.createdAt) }}</span>
          </div>
        </div>
      </div>
    </n-spin>
    <n-button v-if="history.length > 0" block @click="handleClear" type="error" ghost style="margin-top: 12px">
      清空历史
    </n-button>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { NEmpty, NTag, NButton, NSpin, useDialog, useMessage } from 'naive-ui'
import api from '../stores/api'
import type { HistoryItem } from '../types'

const emit = defineEmits<{
  (e: 'select', query: string): void
}>()

const dialog = useDialog()
const message = useMessage()

const history = ref<HistoryItem[]>([])
const loading = ref(true)

onMounted(async () => {
  await loadHistory()
})

async function loadHistory() {
  loading.value = true
  try {
    history.value = await api.getHistory()
  } catch (error) {
    console.error('加载历史记录失败:', error)
  } finally {
    loading.value = false
  }
}

function formatTime(dateStr: string): string {
  const date = new Date(dateStr)
  const now = new Date()
  const diff = now.getTime() - date.getTime()
  const minutes = Math.floor(diff / 60000)
  
  if (minutes < 60) return `${minutes}分钟前`
  const hours = Math.floor(minutes / 60)
  if (hours < 24) return `${hours}小时前`
  return date.toLocaleDateString()
}

function handleSelect(item: HistoryItem) {
  emit('select', item.query)
}

function handleClear() {
  dialog.warning({
    title: '确认清空',
    content: '确定要清空所有历史记录吗？',
    positiveText: '确定',
    negativeText: '取消',
    onPositiveClick: async () => {
      await api.clearHistory()
      history.value = []
      message.success('历史记录已清空')
    }
  })
}
</script>

<style scoped>
.history-panel {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.history-list {
  max-height: 400px;
  overflow-y: auto;
}

.history-item {
  padding: 12px;
  border-radius: 8px;
  cursor: pointer;
  transition: background 0.2s;
}

.history-item:hover {
  background: #f5f5f5;
}

.query {
  font-size: 15px;
  margin-bottom: 4px;
  word-break: break-all;
}

.meta {
  display: flex;
  align-items: center;
  gap: 8px;
  font-size: 12px;
  color: #999;
}

.time {
  margin-left: auto;
}
</style>