<template>
  <div class="vocabulary-panel">
    <n-spin :show="loading">
      <n-empty v-if="vocabulary.length === 0" description="生词本是空的" />
      <div v-else class="vocab-list">
        <div
          v-for="item in vocabulary"
          :key="item.id"
          class="vocab-item"
        >
          <div class="word-info">
            <span class="word">{{ item.word }}</span>
            <n-tag size="small">{{ item.wordType === 'ancient' ? '古文' : '英文' }}</n-tag>
          </div>
          <div class="definition">{{ item.definition }}</div>
          <div class="actions">
            <span class="added-time">{{ formatTime(item.addedAt) }}</span>
            <n-button size="tiny" @click="handleRemove(item.id)">
              删除
            </n-button>
          </div>
        </div>
      </div>
    </n-spin>
    <n-button v-if="vocabulary.length > 0" block @click="handleExport" style="margin-top: 12px">
      导出生词本
    </n-button>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { NEmpty, NTag, NButton, NSpin, useMessage } from 'naive-ui'
import api from '../stores/api'
import type { VocabularyItem } from '../types'

const message = useMessage()

const vocabulary = ref<VocabularyItem[]>([])
const loading = ref(true)

onMounted(async () => {
  await loadVocabulary()
})

async function loadVocabulary() {
  loading.value = true
  try {
    vocabulary.value = await api.getVocabulary()
  } catch (error) {
    console.error('加载生词本失败:', error)
  } finally {
    loading.value = false
  }
}

function formatTime(dateStr: string): string {
  return new Date(dateStr).toLocaleDateString()
}

async function handleRemove(id: number) {
  try {
    await api.removeFromVocabulary(id)
    vocabulary.value = vocabulary.value.filter(v => v.id !== id)
    message.success('已删除')
  } catch (error: any) {
    message.error(error.message || '删除失败')
  }
}

function handleExport() {
  const text = vocabulary.value.map(v => `${v.word}\t${v.definition}`).join('\n')
  const blob = new Blob([text], { type: 'text/plain;charset=utf-8' })
  const url = URL.createObjectURL(blob)
  const a = document.createElement('a')
  a.href = url
  a.download = `生词本_${new Date().toISOString().slice(0, 10)}.txt`
  a.click()
  URL.revokeObjectURL(url)
  message.success('导出成功')
}
</script>

<style scoped>
.vocabulary-panel {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.vocab-list {
  max-height: 400px;
  overflow-y: auto;
}

.vocab-item {
  padding: 12px;
  border-radius: 8px;
  background: #f9f9f9;
  margin-bottom: 8px;
}

.word-info {
  display: flex;
  align-items: center;
  gap: 8px;
  margin-bottom: 4px;
}

.word {
  font-size: 16px;
  font-weight: 600;
}

.definition {
  font-size: 13px;
  color: #666;
  margin-bottom: 8px;
}

.actions {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.added-time {
  font-size: 12px;
  color: #999;
}
</style>