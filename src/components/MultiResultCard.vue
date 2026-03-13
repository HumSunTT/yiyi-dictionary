<template>
  <div class="multi-result-card">
    <div v-if="results.length > 0" class="results-container">
      <!-- 如果只有一个结果，使用现有的 ResultCard -->
      <ResultCard 
        v-if="results.length === 1" 
        :result="results[0]" 
        @add-to-vocabulary="handleAddToVocabulary"
      />
      <!-- 如果有多个结果，显示每个结果 -->
      <div v-else class="multiple-results">
        <div v-for="(result, index) in results" :key="index" class="single-result">
          <div class="source-header">
            <n-tag size="small" type="info">{{ result.source || '古汉语词典' }}</n-tag>
          </div>
          <ResultCard 
            :result="result" 
            @add-to-vocabulary="() => handleAddToVocabulary(index)"
          />
        </div>
      </div>
    </div>
    <div v-else class="no-results">
      <n-empty description="未找到相关词典条目" />
    </div>
  </div>
</template>

<script setup lang="ts">
import { NTag, NEmpty } from 'naive-ui'
import ResultCard from './ResultCard.vue'
import type { DictionaryResult } from '../types'

const props = defineProps<{
  results: DictionaryResult[]
}>()

const emit = defineEmits<{
  (e: 'add-to-vocabulary', index: number): void
}>()

function handleAddToVocabulary(index: number = 0) {
  emit('add-to-vocabulary', index)
}
</script>

<style scoped>
.multi-result-card {
  width: 100%;
}

.results-container {
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.multiple-results {
  display: flex;
  flex-direction: column;
  gap: 24px;
}

.single-result {
  border: 1px solid var(--n-border-color);
  border-radius: 8px;
  padding: 12px;
  background-color: var(--n-color);
}

.source-header {
  margin-bottom: 12px;
  text-align: center;
}

.no-results {
  padding: 20px;
}
</style>