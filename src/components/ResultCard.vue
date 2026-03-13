<template>
  <div class="result-card">
    <!-- 词库结果 -->
    <div v-if="result.type === 'dictionary'" class="dict-result">
      <div class="word-header">
        <span class="word">{{ result.word }}</span>
        <span v-if="result.phonetic" class="phonetic">{{ result.phonetic }}</span>
        <n-tag v-if="result.source" size="small" type="info">{{ result.source }}</n-tag>
      </div>
      
      <n-divider style="margin: 12px 0" />
      
      <div class="definitions">
        <div v-for="(item, index) in result.definitions" :key="index" class="def-item">
          <div v-if="item.pos" class="def-line">
            <span class="pos">{{ item.pos }}</span>
            <span class="def-text">{{ item.definition }}</span>
          </div>
          <div v-else class="def-raw" v-html="formatRawDefinition(item.definition)"></div>
        </div>
      </div>

      <div v-if="result.examples?.length" class="examples">
        <n-divider style="margin: 12px 0" />
        <div class="section-title">例句</div>
        <div v-for="(ex, index) in result.examples" :key="index" class="example-item">
          <p class="example-text" v-html="highlightWord(ex.text, result.word || '')"></p>
          <p class="trans" v-if="ex.translation">{{ ex.translation }}</p>
          <p class="source" v-if="ex.source">—— {{ ex.source }}</p>
        </div>
      </div>
    </div>

    <!-- AI翻译结果 -->
    <div v-else-if="result.type === 'translation'" class="ai-result">
      <div class="section-title">
        <n-icon :component="SparklesOutline" />
        AI翻译
      </div>
      <div class="translation" v-html="formatTranslation(result.translation || '')"></div>
      
      <div v-if="result.notes?.length" class="notes">
        <n-divider style="margin: 12px 0" />
        <div class="section-title">注释</div>
        <ul>
          <li v-for="(note, index) in result.notes" :key="index">{{ note }}</li>
        </ul>
      </div>
    </div>

    <!-- 操作按钮 -->
    <div class="actions">
      <n-button size="small" @click="handleAddToVocabulary">
        <template #icon><n-icon :component="StarOutline" /></template>
        加入生词本
      </n-button>
      <n-button size="small" @click="handleCopy">
        <template #icon><n-icon :component="CopyOutline" /></template>
        复制结果
      </n-button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { NDivider, NTag, NButton, NIcon, useMessage } from 'naive-ui'
import { StarOutline, CopyOutline, SparklesOutline } from '@vicons/ionicons5'

const props = defineProps<{
  result: {
    type: 'dictionary' | 'translation'
    word?: string
    phonetic?: string
    source?: string
    definitions?: { pos: string; definition: string }[]
    examples?: { text: string; translation?: string; source?: string }[]
    translation?: string
    notes?: string[]
  }
}>()

const emit = defineEmits<{
  (e: 'add-to-vocabulary'): void
}>()

const message = useMessage()

function formatRawDefinition(text: string): string {
  // 先将各种换行符统一处理
  let result = text
    // 处理实际的换行符
    .replace(/\r\n/g, '\n')
    .replace(/\r/g, '\n')
    // 处理字面的 \n 字符串
    .replace(/\\n/g, '\n')
  
  // 词性标签高亮 (n. vt. vi. adj. adv. prep. conj. int. 等)
  result = result.replace(/\b([a-z]+\.)/gi, '<span class="pos-tag">$1</span>')
  
  // 序号高亮
  result = result.replace(/([①②③④⑤⑥⑦⑧⑨⑩])/g, '<span class="num">$1</span>')
  
  // 书名号高亮
  result = result.replace(/《([^》]+)》/g, '<span class="book">《$1》</span>')
  
  // "又" 开头的分段高亮
  result = result.replace(/又/g, '<span class="separator">又</span>')
  
  // 按换行分割成段落
  const lines = result.split('\n')
  const formattedLines = lines
    .filter(line => line.trim())
    .map(line => `<div class="def-paragraph">${line}</div>`)
    .join('')
  
  return formattedLines || `<div class="def-paragraph">${result}</div>`
}

function highlightWord(text: string, word: string): string {
  if (!word) return text
  const regex = new RegExp(`(${word})`, 'g')
  return text.replace(regex, '<span class="highlight">$1</span>')
}

function formatTranslation(text: string): string {
  let result = text
    .replace(/\\n/g, '\n')
    .replace(/\r\n/g, '\n')
    .replace(/\r/g, '\n')
  
  // 字典来源标签高亮
  result = result.replace(/【(古汉语常用字字典|古汉语词典|康熙字典|中英词典|相关短语)】/g, '<div class="dict-source">$1</div>')
  
  // 例句标签
  result = result.replace(/【例句】/g, '<div class="example-label">例句</div>')
  
  // 词性标注高亮 <形><动><名><代><副><介><连><助><数><量>
  result = result.replace(/<([形动名代副介连助数量]+)>/g, '<span class="pos-ancient">$1</span>')
  
  // 例句项目符号
  result = result.replace(/• /g, '<span class="bullet">•</span> ')
  
  // 书名号高亮
  result = result.replace(/《([^》]+)》/g, '<span class="book-title">《$1》</span>')
  
  // 换行处理
  result = result.replace(/\n/g, '<br/>')
  
  return result
}

function handleAddToVocabulary() {
  emit('add-to-vocabulary')
  message.success('已添加到生词本')
}

function handleCopy() {
  let textToCopy = ''
  
  if (props.result.type === 'dictionary') {
    // 复制完整的词典结果
    const parts: string[] = []
    
    if (props.result.word) {
      parts.push(`【${props.result.word}】`)
    }
    if (props.result.phonetic) {
      parts.push(`拼音: ${props.result.phonetic}`)
    }
    if (props.result.definitions) {
      parts.push('\n释义:')
      props.result.definitions.forEach(def => {
        if (def.pos) {
          parts.push(`  ${def.pos} ${def.definition}`)
        } else {
          parts.push(`  ${def.definition}`)
        }
      })
    }
    if (props.result.examples?.length) {
      parts.push('\n例句:')
      props.result.examples.forEach(ex => {
        parts.push(`  ${ex.text}`)
        if (ex.translation) parts.push(`  译: ${ex.translation}`)
        if (ex.source) parts.push(`  —— ${ex.source}`)
      })
    }
    
    textToCopy = parts.join('\n')
  } else if (props.result.type === 'translation') {
    // 复制翻译结果
    textToCopy = props.result.translation || ''
  }
  
  navigator.clipboard.writeText(textToCopy)
  message.success('已复制到剪贴板')
}
</script>

<style scoped>
.result-card {
  padding: 4px;
}

.word-header {
  display: flex;
  align-items: baseline;
  gap: 12px;
  flex-wrap: wrap;
}

.word {
  font-size: 28px;
  font-weight: 700;
  color: var(--n-text-color);
}

.phonetic {
  font-size: 14px;
  color: #999;
  font-style: italic;
}

.definitions {
  margin-top: 8px;
}

.def-item {
  margin-bottom: 12px;
  line-height: 1.8;
}

.def-line {
  display: flex;
  align-items: flex-start;
  gap: 8px;
}

.def-text {
  font-size: 15px;
  line-height: 1.6;
}

.pos {
  display: inline-block;
  background: linear-gradient(135deg, #6366f1, #8b5cf6);
  color: white;
  padding: 2px 8px;
  border-radius: 4px;
  font-size: 12px;
  font-weight: 500;
  white-space: nowrap;
  flex-shrink: 0;
}

.def-raw {
  font-size: 15px;
  line-height: 1.8;
  color: var(--n-text-color);
}

.def-paragraph {
  margin-bottom: 10px;
  padding: 8px 12px;
  background: var(--n-color-target);
  border-radius: 6px;
  border-left: 3px solid #6366f1;
}

.def-raw :deep(.pos-tag) {
  display: inline-block;
  background: linear-gradient(135deg, #6366f1, #8b5cf6);
  color: white;
  padding: 2px 6px;
  border-radius: 4px;
  font-size: 12px;
  font-weight: 500;
  margin-right: 6px;
}

.def-raw :deep(.num) {
  display: inline-block;
  background: #ede9fe;
  color: #7c3aed;
  padding: 1px 6px;
  border-radius: 4px;
  font-weight: 600;
  margin-right: 4px;
}

.def-raw :deep(.book) {
  color: #6366f1;
  font-weight: 500;
}

.def-raw :deep(.separator) {
  display: inline-block;
  background: #fef3c7;
  color: #d97706;
  padding: 1px 6px;
  border-radius: 4px;
  font-weight: 600;
  margin-right: 6px;
}

.examples {
  margin-top: 8px;
}

.section-title {
  font-size: 13px;
  color: #666;
  margin-bottom: 8px;
  display: flex;
  align-items: center;
  gap: 4px;
  font-weight: 500;
}

.example-item {
  background: var(--n-color-target);
  padding: 10px 12px;
  border-radius: 8px;
  margin-bottom: 8px;
  border-left: 3px solid #8b5cf6;
}

.example-text {
  font-size: 14px;
  line-height: 1.6;
  margin: 0;
}

.example-text :deep(.highlight) {
  background: linear-gradient(135deg, #fef3c7, #fde68a);
  color: #92400e;
  padding: 1px 4px;
  border-radius: 3px;
  font-weight: 600;
}

.example-item .trans {
  color: #666;
  font-size: 13px;
  margin-top: 6px;
  margin-bottom: 0;
}

.example-item .source {
  color: #999;
  font-size: 12px;
  margin-top: 4px;
  margin-bottom: 0;
  text-align: right;
}

.ai-result .translation {
  font-size: 16px;
  line-height: 1.8;
  padding: 14px;
  background: linear-gradient(135deg, #f5f3ff, #faf5ff);
  border-radius: 10px;
  border-left: 4px solid #8b5cf6;
}

.translation :deep(.tag) {
  display: inline-block;
  background: #ede9fe;
  color: #7c3aed;
  padding: 2px 6px;
  border-radius: 4px;
  font-size: 13px;
  font-weight: 500;
}

.translation :deep(.dict-separator) {
  height: 1px;
  background: linear-gradient(90deg, transparent, #c5cae9, transparent);
  margin: 20px 0;
}

.translation :deep(.dict-source) {
  display: block;
  font-size: 16px;
  font-weight: 700;
  color: #4c1d95;
  padding: 10px 16px;
  margin: 12px 0 8px 0;
  background: linear-gradient(135deg, #f5f3ff, #ede9fe);
  border-radius: 8px;
  border-left: 4px solid #8b5cf6;
}

.translation :deep(.dict-source:first-child) {
  margin-top: 0;
}

.translation :deep(.tag-label) {
  display: inline-block;
  background: linear-gradient(135deg, #6366f1, #8b5cf6);
  color: white;
  padding: 3px 10px;
  border-radius: 4px;
  font-size: 13px;
  font-weight: 600;
  margin: 8px 0;
}

.translation :deep(.pos-ancient) {
  display: inline-block;
  background: linear-gradient(135deg, #6366f1, #8b5cf6);
  color: white;
  padding: 1px 6px;
  border-radius: 3px;
  font-size: 12px;
  font-weight: 600;
  margin: 0 2px;
}

.translation :deep(.bullet) {
  color: #8b5cf6;
  font-weight: bold;
  margin-right: 4px;
}

.translation :deep(.book-title) {
  color: #6366f1;
  font-weight: 600;
}

.notes ul {
  padding-left: 20px;
  margin: 8px 0;
}

.notes li {
  margin-bottom: 4px;
  font-size: 14px;
  color: #666;
  line-height: 1.5;
}

.actions {
  display: flex;
  gap: 8px;
  margin-top: 16px;
}
</style>