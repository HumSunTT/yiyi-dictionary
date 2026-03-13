<template>
  <div class="result-card">
    <!-- 翻译结果 -->
    <div v-if="result.type === 'translation'" class="translation-container">
      <!-- 左栏：词典内容 -->
      <div class="left-column">
        <template v-for="(section, index) in parsedSections" :key="index">
          <div v-if="section.type === 'chinese-english' || section.type === 'english-chinese'" class="dict-section">
            <div class="dict-title">{{ section.title }}</div>
            <div class="dict-content" v-html="formatContent(section.content || '')"></div>
          </div>
          <div v-if="section.type === 'ancient'" class="dict-section ancient-section">
            <div class="dict-title">{{ section.title }}</div>
            <div class="ancient-grid">
              <div class="ancient-defs" v-html="formatAncientDefs(section.definitions || '')"></div>
              <div class="ancient-examples" v-html="formatAncientExamples(section.examples || '')"></div>
            </div>
          </div>
        </template>
      </div>
      
      <!-- 右栏：相关短语 -->
      <div class="right-column">
        <template v-for="(section, index) in parsedSections" :key="'r'+index">
          <div v-if="section.type === 'phrases'" class="phrase-section">
            <div class="dict-title">相关短语</div>
            <div class="phrase-list">
              <div v-for="(phrase, pIndex) in section.phrases" :key="pIndex" class="phrase-item">
                <span class="phrase-word">{{ phrase.word }}</span>
                <span class="phrase-meaning">{{ phrase.meaning }}</span>
              </div>
            </div>
          </div>
        </template>
      </div>
    </div>
    
    <!-- 词库结果 -->
    <div v-else-if="result.type === 'dictionary'" class="dict-result">
      <div class="word-header">
        <span class="word">{{ result.word }}</span>
        <span v-if="result.phonetic" class="phonetic">{{ result.phonetic }}</span>
      </div>
      <div class="definitions">
        <div v-for="(item, index) in result.definitions" :key="index" class="def-item">
          <span v-if="item.pos" class="pos">{{ item.pos }}</span>
          <span class="def-text">{{ item.definition }}</span>
        </div>
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
import { computed } from 'vue'
import { NButton, NIcon, useMessage } from 'naive-ui'
import { StarOutline, CopyOutline } from '@vicons/ionicons5'

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

interface ParsedSection {
  type: 'chinese-english' | 'english-chinese' | 'phrases' | 'ancient'
  title: string
  content?: string
  phrases?: { word: string; meaning: string }[]
  definitions?: string
  examples?: string
}

const parsedSections = computed(() => {
  if (!props.result.translation) return []
  
  const text = props.result.translation
    .replace(/\\n/g, '\n')
    .replace(/\r\n/g, '\n')
  
  const sections: ParsedSection[] = []
  const parts = text.split(/【([^】]+)】/).filter(p => p.trim())
  
  for (let i = 0; i < parts.length; i += 2) {
    const title = parts[i]?.trim() || ''
    const content = parts[i + 1]?.trim() || ''
    
    if (title === '中英词典') {
      sections.push({ type: 'chinese-english', title: '中英词典', content })
    } else if (title === '英汉词典') {
      sections.push({ type: 'english-chinese', title: '英汉词典', content })
    } else if (title === '相关短语') {
      const lines = content.split('\n').filter(l => l.trim())
      const phrases = lines.map(line => {
        const parts = line.split(/\s+/, 2)
        return { word: parts[0] || '', meaning: parts[1] || '' }
      })
      sections.push({ type: 'phrases', title: '相关短语', phrases })
    } else if (title === '古汉语词典' || title === '古汉语常用字字典' || title === '康熙字典') {
      const { defs, examples } = parseAncientContent(content)
      sections.push({ 
        type: 'ancient', 
        title, 
        definitions: defs,
        examples: examples
      })
    }
  }
  
  return sections
})

function parseAncientContent(content: string) {
  const lines = content.split('\n').filter(l => l.trim())
  const defLines: string[] = []
  const exampleLines: string[] = []
  let inExample = false
  
  for (const line of lines) {
    if (line.includes('例句') || line.startsWith('•')) {
      inExample = true
    }
    if (inExample) {
      exampleLines.push(line)
    } else {
      defLines.push(line)
    }
  }
  
  return { defs: defLines.join('\n'), examples: exampleLines.join('\n') }
}

function formatContent(content: string): string {
  return content
    .split('\n')
    .filter(l => l.trim())
    .map(line => `<div class="content-line">${line}</div>`)
    .join('')
}

function formatAncientDefs(defs: string): string {
  if (!defs) return ''
  return defs
    .split('\n')
    .filter(l => l.trim())
    .map(line => {
      const formatted = line
        .replace(/([①②③④⑤⑥⑦⑧⑨⑩])/g, '<span class="num">$1</span>')
        .replace(/<([形动名代副介连助数量]+)>/g, '<span class="pos-tag">$1</span>')
        .replace(/《([^》]+)》/g, '<span class="book">《$1》</span>')
      return `<div class="def-line">${formatted}</div>`
    })
    .join('')
}

function formatAncientExamples(examples: string): string {
  if (!examples) return ''
  return examples
    .split('\n')
    .filter(l => l.trim())
    .map(line => {
      const formatted = line
        .replace(/•/g, '<span class="bullet">•</span>')
        .replace(/《([^》]+)》/g, '<span class="book">《$1》</span>')
      return `<div class="example-line">${formatted}</div>`
    })
    .join('')
}

function handleAddToVocabulary() {
  emit('add-to-vocabulary')
  message.success('已添加到生词本')
}

function handleCopy() {
  const textToCopy = props.result.type === 'translation' 
    ? props.result.translation 
    : props.result.word + ' ' + (props.result.definitions?.map(d => d.definition).join('; ') || '')
  
  navigator.clipboard.writeText(textToCopy || '')
  message.success('已复制到剪贴板')
}
</script>

<style scoped>
.result-card {
  padding: 16px;
}

.translation-container {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 24px;
}

@media (max-width: 799px) {
  .translation-container {
    grid-template-columns: 1fr;
  }
}

.left-column, .right-column {
  min-width: 0;
}

.dict-section {
  margin-bottom: 20px;
}

.dict-title {
  font-size: 14px;
  font-weight: 700;
  color: #fff;
  background: linear-gradient(135deg, #6366f1, #8b5cf6);
  padding: 6px 12px;
  border-radius: 6px;
  margin-bottom: 12px;
}

.dict-content {
  font-size: 14px;
  line-height: 1.8;
  color: #374151;
}

.content-line {
  padding: 4px 0;
}

.content-line :deep(.pos-tag) {
  display: inline-block;
  background: #ede9fe;
  color: #7c3aed;
  padding: 1px 6px;
  border-radius: 4px;
  font-size: 12px;
  font-weight: 600;
  margin-right: 4px;
}

.ancient-section {
  background: #fafafa;
  border-radius: 8px;
  padding: 12px;
}

.ancient-grid {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 16px;
}

.ancient-defs {
  font-size: 14px;
  line-height: 1.8;
}

.ancient-defs :deep(.num) {
  display: inline-block;
  background: linear-gradient(135deg, #6366f1, #8b5cf6);
  color: #fff;
  width: 20px;
  height: 20px;
  border-radius: 50%;
  text-align: center;
  line-height: 20px;
  font-size: 12px;
  font-weight: 600;
  margin-right: 6px;
}

.ancient-defs :deep(.pos-tag) {
  display: inline-block;
  background: #fef3c7;
  color: #92400e;
  padding: 1px 6px;
  border-radius: 4px;
  font-size: 12px;
  font-weight: 600;
  margin: 0 2px;
}

.ancient-defs :deep(.book) {
  color: #6366f1;
  font-weight: 600;
}

.ancient-examples {
  font-size: 13px;
  line-height: 1.7;
  color: #6b7280;
  background: #f3f4f6;
  padding: 10px;
  border-radius: 6px;
}

.ancient-examples :deep(.bullet) {
  color: #8b5cf6;
  font-weight: bold;
  margin-right: 4px;
}

.ancient-examples :deep(.book) {
  color: #6366f1;
}

.phrase-section {
  background: #f8fafc;
  border-radius: 8px;
  padding: 12px;
}

.phrase-list {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.phrase-item {
  display: flex;
  gap: 8px;
  padding: 6px 8px;
  background: #fff;
  border-radius: 6px;
  border-left: 3px solid #8b5cf6;
}

.phrase-word {
  font-weight: 700;
  color: #6366f1;
  min-width: 120px;
}

.phrase-meaning {
  color: #6b7280;
  font-size: 13px;
}

.dict-result {
  padding: 8px;
}

.word-header {
  display: flex;
  align-items: baseline;
  gap: 12px;
  margin-bottom: 16px;
}

.word {
  font-size: 28px;
  font-weight: 700;
  color: #1f2937;
}

.phonetic {
  font-size: 14px;
  color: #9ca3af;
  font-style: italic;
}

.definitions {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.def-item {
  display: flex;
  gap: 8px;
  font-size: 15px;
  line-height: 1.6;
}

.pos {
  display: inline-block;
  background: linear-gradient(135deg, #6366f1, #8b5cf6);
  color: #fff;
  padding: 2px 8px;
  border-radius: 4px;
  font-size: 12px;
  font-weight: 600;
  white-space: nowrap;
  height: fit-content;
}

.def-text {
  color: #374151;
}

.actions {
  display: flex;
  gap: 8px;
  margin-top: 16px;
  padding-top: 16px;
  border-top: 1px solid #e5e7eb;
}
</style>