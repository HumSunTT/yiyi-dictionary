<template>
  <div class="result-card">
    <!-- 翻译结果 -->
    <div v-if="result.type === 'translation'" class="translation-container">
      <!-- 第一行：中英词典 + 相关短语 -->
      <div class="top-section" v-if="hasDictOrPhrases">
        <div class="left-column">
          <template v-for="(section, index) in parsedSections" :key="index">
            <div v-if="section.type === 'chinese-english' || section.type === 'english-chinese'" class="dict-section">
              <div class="dict-title">{{ section.title }}</div>
              <div class="dict-content" v-html="formatDictContent(section.content || '')"></div>
            </div>
          </template>
        </div>
        
        <div class="right-column">
          <template v-for="(section, index) in parsedSections" :key="'phrase-'+index">
            <div v-if="section.type === 'phrases'" class="dict-section">
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
      
      <!-- 古汉语常用字字典（全宽） -->
      <div v-if="commonDictSection" class="dict-section ancient-common">
        <div class="dict-title">{{ commonDictSection.title }}</div>
        <div class="ancient-content">
          <div class="ancient-defs" v-html="formatAncientDefs(commonDictSection.definitions || '')"></div>
          <div class="ancient-examples" v-html="formatAncientExamples(commonDictSection.examples || '')"></div>
        </div>
      </div>
      
      <!-- 古汉语词典 + 康熙字典（左右并排） -->
      <div class="bottom-dicts" v-if="hasOtherAncient">
        <div class="left-column">
          <div v-if="ancientDictSection" class="dict-section ancient-side">
            <div class="dict-title">{{ ancientDictSection.title }}</div>
            <div class="ancient-simple" v-html="formatSimpleAncient(ancientDictSection.content || '')"></div>
          </div>
        </div>
        <div class="right-column">
          <div v-if="kangxiSection" class="dict-section kangxi-section">
            <div class="dict-title">{{ kangxiSection.title }}</div>
            <div class="kangxi-content" v-html="formatKangxi(kangxiSection.content || '')"></div>
          </div>
        </div>
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
  type: 'chinese-english' | 'english-chinese' | 'phrases' | 'ancient-common' | 'ancient' | 'kangxi'
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
        const colonIndex = line.indexOf('：')
        if (colonIndex > 0) {
          return { word: line.substring(0, colonIndex), meaning: line.substring(colonIndex + 1) }
        }
        const parts = line.split(/\s+/, 2)
        return { word: parts[0] || '', meaning: parts[1] || '' }
      })
      sections.push({ type: 'phrases', title: '相关短语', phrases })
    } else if (title === '古汉语常用字字典') {
      const { defs, examples } = parseAncientContent(content)
      sections.push({ 
        type: 'ancient-common', 
        title, 
        definitions: defs,
        examples: examples
      })
    } else if (title === '古汉语词典') {
      sections.push({ type: 'ancient', title, content })
    } else if (title === '康熙字典') {
      sections.push({ type: 'kangxi', title, content })
    }
  }
  
  return sections
})

const hasDictOrPhrases = computed(() => {
  return parsedSections.value.some(s => 
    s.type === 'chinese-english' || 
    s.type === 'english-chinese' || 
    s.type === 'phrases'
  )
})

const commonDictSection = computed(() => {
  return parsedSections.value.find(s => s.type === 'ancient-common')
})

const ancientDictSection = computed(() => {
  return parsedSections.value.find(s => s.type === 'ancient')
})

const kangxiSection = computed(() => {
  return parsedSections.value.find(s => s.type === 'kangxi')
})

const hasOtherAncient = computed(() => {
  return ancientDictSection.value || kangxiSection.value
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
    .map(line => '<div class="content-line">' + line + '</div>')
    .join('')
}

function formatDictContent(content: string): string {
  const lines = content.split('\n').filter(l => l.trim())
  const defLines: string[] = []
  const exampleLines: string[] = []
  let inExample = false
  
  for (const line of lines) {
    if (line.includes('例句')) {
      inExample = true
      continue
    }
    if (inExample) {
      exampleLines.push(line)
    } else {
      defLines.push(line)
    }
  }
  
  let html = ''
  if (defLines.length > 0) {
    html += '<div class="dict-defs">' + defLines.map(l => '<div class="def-line">' + l + '</div>').join('') + '</div>'
  }
  if (exampleLines.length > 0) {
    html += '<div class="dict-examples">' + exampleLines.map(l => {
      let formatted = l
        .replace(/•/g, '<span class="bullet">•</span>')
        .replace(/——/g, '<span class="sep">——</span>')
      return '<div class="example-line">' + formatted + '</div>'
    }).join('') + '</div>'
  }
  return html
}

function formatAncientDefs(defs: string): string {
  if (!defs) return ''
  return defs
    .split('\n')
    .filter(l => l.trim())
    .map(line => {
      let formatted = line
        .replace(/([①②③④⑤⑥⑦⑧⑨⑩])/g, '<span class="num">$1</span>')
        .replace(/<([形动名代副介连助数量]+)>/g, '<span class="pos-tag">$1</span>')
        .replace(/《([^》]+)》/g, '<span class="book">《$1》</span>')
      return '<div class="def-line">' + formatted + '</div>'
    })
    .join('')
}

function formatAncientExamples(examples: string): string {
  if (!examples) return ''
  return examples
    .split('\n')
    .filter(l => l.trim())
    .map(line => {
      let formatted = line
        .replace(/•/g, '<span class="bullet">•</span>')
        .replace(/《([^》]+)》/g, '<span class="book">《$1》</span>')
      return '<div class="example-line">' + formatted + '</div>'
    })
    .join('')
}

function formatSimpleAncient(content: string): string {
  if (!content) return ''
  return content
    .split('\n')
    .filter(l => l.trim())
    .map(line => '<div class="simple-line">' + line + '</div>')
    .join('')
}

function formatKangxi(content: string): string {
  if (!content) return ''
  let formatted = content
    .replace(/\\n/g, '\n')
    .split('\n')
    .filter(l => l.trim())
    .map(line => {
      let f = line
        .replace(/〔古文〕/g, '<span class="kangxi-tag">〔古文〕</span>')
        .replace(/又/g, '</div><div class="kangxi-item"><span class="kangxi-sep">又</span> ')
        .replace(/《([^》]+)》/g, '<span class="book">《$1》</span>')
        .replace(/([①②③④⑤⑥⑦⑧⑨⑩])/g, '<span class="num">$1</span>')
      return f
    })
    .join('</div><div class="kangxi-item">')
  
  return '<div class="kangxi-item">' + formatted + '</div>'
}

function handleAddToVocabulary() {
  emit('add-to-vocabulary')
  message.success('已添加到生词本')
}

function handleCopy() {
  let textToCopy = ''
  if (props.result.type === 'translation') {
    textToCopy = props.result.translation || ''
  } else {
    textToCopy = props.result.word + ' ' + (props.result.definitions?.map(d => d.definition).join('; ') || '')
  }
  navigator.clipboard.writeText(textToCopy)
  message.success('已复制到剪贴板')
}
</script>

<style scoped>
.result-card {
  padding: 16px;
}

.top-section {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 24px;
  margin-bottom: 20px;
}

.left-column, .right-column {
  min-width: 0;
}

.bottom-dicts {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 24px;
  margin-top: 20px;
}

.dict-section {
  margin-bottom: 16px;
}

.dict-title {
  font-size: 14px;
  font-weight: 700;
  color: #fff;
  background: #6366f1;
  padding: 6px 12px;
  border-radius: 6px;
  margin-bottom: 12px;
}

.dict-content {
  font-size: 14px;
  line-height: 1.8;
  color: #374151;
}

.dict-defs {
  margin-bottom: 12px;
}

.def-line {
  padding: 4px 0;
}

.dict-examples {
  background: #f3f4f6;
  padding: 10px;
  border-radius: 6px;
  margin-top: 8px;
}

.dict-examples .example-line {
  font-size: 13px;
  color: #6b7280;
  line-height: 1.7;
  padding: 2px 0;
}

.dict-examples .bullet {
  color: #8b5cf6;
  font-weight: bold;
  margin-right: 4px;
}

.dict-examples .sep {
  color: #9ca3af;
  margin: 0 4px;
}

.content-line {
  padding: 4px 0;
}

/* 相关短语 */
.phrase-list {
  display: flex;
  flex-direction: column;
  gap: 6px;
}

.phrase-item {
  display: flex;
  gap: 8px;
  font-size: 13px;
  line-height: 1.6;
}

.phrase-word {
  color: #6366f1;
}

.phrase-meaning {
  color: #6b7280;
}

/* 古汉语常用字字典 */
.ancient-common {
  background: #fafafa;
  border-radius: 8px;
  padding: 12px;
  margin-top: 20px;
}

.ancient-content {
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
  background: #6366f1;
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

/* 古汉语词典（简化显示） */
.ancient-side {
  background: #fafafa;
  border-radius: 8px;
  padding: 12px;
}

.ancient-simple {
  font-size: 14px;
  line-height: 1.8;
  color: #374151;
}

.simple-line {
  padding: 2px 0;
}

/* 康熙字典 */
.kangxi-section {
  background: #fafafa;
  border-radius: 8px;
  padding: 12px;
}

.kangxi-content {
  font-size: 13px;
  line-height: 1.8;
  color: #374151;
}

.kangxi-content :deep(.kangxi-item) {
  margin-bottom: 8px;
  padding-bottom: 8px;
  border-bottom: 1px dashed #e5e7eb;
}

.kangxi-content :deep(.kangxi-item:last-child) {
  border-bottom: none;
  margin-bottom: 0;
  padding-bottom: 0;
}

.kangxi-content :deep(.kangxi-tag) {
  background: #fef3c7;
  color: #92400e;
  padding: 1px 6px;
  border-radius: 4px;
  font-weight: 600;
  font-size: 12px;
}

.kangxi-content :deep(.kangxi-sep) {
  display: inline-block;
  background: #dbeafe;
  color: #1e40af;
  padding: 1px 6px;
  border-radius: 4px;
  font-weight: 600;
  font-size: 12px;
  margin-right: 4px;
}

.kangxi-content :deep(.book) {
  color: #6366f1;
  font-weight: 600;
}

.kangxi-content :deep(.num) {
  display: inline-block;
  background: #6366f1;
  color: #fff;
  width: 18px;
  height: 18px;
  border-radius: 50%;
  text-align: center;
  line-height: 18px;
  font-size: 11px;
  font-weight: 600;
  margin-right: 4px;
}

/* 词库结果 */
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
  background: #6366f1;
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
