<template>
  <div class="float-window">
    <n-card size="small" :bordered="false">
      <template #header>
        <div class="float-header">
          <span class="word">{{ displayWord }}</span>
          <span v-if="phonetic" class="phonetic">{{ phonetic }}</span>
          <n-tag v-if="source" size="small" type="info">{{ source }}</n-tag>
          <n-button quaternary circle size="small" @click="$emit('close')">
            <template #icon><n-icon :component="CloseOutline" /></template>
          </n-button>
        </div>
      </template>
      
      <!-- 词库结果 -->
      <template v-if="result?.type === 'dictionary'">
        <div class="definitions">
          <div v-for="(item, index) in result.definitions" :key="index" class="def-item">
            <span v-if="item.pos" class="pos">{{ item.pos }}</span>
            <span class="text" v-html="formatText(item.definition)"></span>
          </div>
        </div>

        <div v-if="result.examples?.length" class="example">
          <n-divider style="margin: 8px 0" />
          <div v-for="(ex, index) in result.examples" :key="index" class="example-item">
            <p v-html="highlightWord(ex.text, result.word)"></p>
            <p v-if="ex.translation" class="trans">{{ ex.translation }}</p>
            <p v-if="ex.source" class="source">—— {{ ex.source }}</p>
          </div>
        </div>
      </template>

      <!-- AI翻译结果 -->
      <template v-else-if="result?.type === 'translation'">
        <div class="translation" v-html="formatText(result.translation)"></div>
        <div v-if="result.notes?.length" class="notes">
          <n-divider style="margin: 8px 0" />
          <ul>
            <li v-for="(note, i) in result.notes" :key="i">{{ note }}</li>
          </ul>
        </div>
      </template>

      <template #action>
        <n-space>
          <n-button size="small" @click="handleAddToVocabulary">
            加生词本
          </n-button>
          <n-button size="small" @click="handleCopy">
            复制
          </n-button>
        </n-space>
      </template>
    </n-card>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { NCard, NButton, NSpace, NIcon, NTag, NDivider, useMessage } from 'naive-ui'
import { CloseOutline } from '@vicons/ionicons5'
import api from '../stores/api'
import type { DictionaryResult, TranslationResult } from '../types'

const props = defineProps<{
  result: DictionaryResult | TranslationResult | null
}>()

const emit = defineEmits<{
  (e: 'close'): void
}>()

const message = useMessage()

const displayWord = computed(() => {
  if (!props.result) return ''
  return props.result.type === 'dictionary' ? props.result.word : props.result.original.slice(0, 20) + '...'
})

const phonetic = computed(() => {
  if (props.result?.type === 'dictionary') {
    return props.result.phonetic
  }
  return null
})

const source = computed(() => {
  if (props.result?.type === 'dictionary') {
    return props.result.source
  }
  return null
})

function formatText(text: string): string {
  if (!text) return ''
  return text
    .replace(/\\n/g, '<br/>')
    .replace(/\n/g, '<br/>')
    .replace(/【([^】]+)】/g, '<span class="tag">【$1】</span>')
}

function highlightWord(text: string, word: string): string {
  if (!word) return text
  const regex = new RegExp(`(${word})`, 'g')
  return text.replace(regex, '<span class="highlight">$1</span>')
}

async function handleAddToVocabulary() {
  if (!props.result) return
  
  const word = props.result.type === 'dictionary' ? props.result.word : props.result.original
  const definition = props.result.type === 'dictionary' 
    ? props.result.definitions.map(d => d.definition).join('；')
    : props.result.translation
  
  try {
    await api.addToVocabulary(word, props.result.type === 'dictionary' ? 'ancient' : 'english', definition)
    message.success(`已添加"${word}"到生词本`)
  } catch (error: any) {
    message.error(error.message || '添加失败')
  }
}

function handleCopy() {
  let text = ''
  if (props.result?.type === 'dictionary') {
    text = `${props.result.word}\n${props.result.definitions.map(d => `${d.pos} ${d.definition}`).join('\n')}`
  } else if (props.result?.type === 'translation') {
    text = props.result.translation || ''
  }
  navigator.clipboard.writeText(text)
  message.success('已复制')
}
</script>

<style scoped>
.float-window {
  padding: 8px;
  min-width: 320px;
  max-width: 450px;
}

.float-header {
  display: flex;
  align-items: center;
  gap: 10px;
}

.word {
  font-size: 22px;
  font-weight: 700;
}

.phonetic {
  font-size: 14px;
  color: #999;
  font-style: italic;
}

.definitions {
  margin-bottom: 8px;
}

.def-item {
  margin-bottom: 8px;
  line-height: 1.6;
}

.pos {
  display: inline-block;
  background: linear-gradient(135deg, #667eea, #764ba2);
  color: white;
  padding: 2px 6px;
  border-radius: 4px;
  font-size: 12px;
  font-weight: 500;
  margin-right: 8px;
}

.text {
  font-size: 14px;
}

.text :deep(.tag) {
  display: inline-block;
  background: #e3f2fd;
  color: #1565c0;
  padding: 1px 4px;
  border-radius: 4px;
  font-size: 12px;
}

.example {
  margin-top: 8px;
}

.example-item {
  background: var(--n-color-target);
  padding: 8px 10px;
  border-radius: 6px;
  margin-bottom: 8px;
  border-left: 2px solid #18a058;
}

.example-item p {
  margin: 0;
  font-size: 13px;
}

.example-item :deep(.highlight) {
  background: linear-gradient(135deg, #fff3cd, #ffeaa7);
  color: #856404;
  padding: 1px 3px;
  border-radius: 3px;
  font-weight: 600;
}

.example-item .trans {
  color: #666;
  margin-top: 4px;
}

.example-item .source {
  color: #999;
  font-size: 12px;
  text-align: right;
  margin-top: 4px;
}

.translation {
  font-size: 15px;
  line-height: 1.7;
  padding: 10px;
  background: linear-gradient(135deg, #e8f5e9, #f3e5f5);
  border-radius: 8px;
}

.translation :deep(.tag) {
  display: inline-block;
  background: #e3f2fd;
  color: #1565c0;
  padding: 1px 4px;
  border-radius: 4px;
  font-size: 12px;
}

.notes ul {
  padding-left: 16px;
  margin: 8px 0 0 0;
}

.notes li {
  font-size: 13px;
  color: #666;
  margin-bottom: 2px;
}
</style>