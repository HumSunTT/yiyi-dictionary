# 易译 API 文档

## Tauri Commands

### 查询相关

#### `query_word`
查询单词（本地词库）

**参数：**
| 参数 | 类型 | 必填 | 说明 |
|------|------|------|------|
| word | string | 是 | 要查询的单词 |
| dict_type | string | 是 | 词典类型：`ancient`/`english`/`auto` |

**返回：**
```typescript
{
  type: "dictionary",
  word: string,
  phonetic?: string,
  source?: string,
  definitions: Array<{ pos: string, definition: string }>,
  examples?: Array<{ text: string, translation?: string, source?: string }>
}
```

---

#### `translate_text`
AI 翻译

**参数：**
| 参数 | 类型 | 必填 | 说明 |
|------|------|------|------|
| text | string | 是 | 要翻译的文本 |
| source_lang | string | 是 | 源语言：`ancient`/`english`/`chinese`/`auto` |

**返回：**
```typescript
{
  type: "translation",
  original: string,
  translation: string,
  notes?: string[]
}
```

---

#### `detect_language`
检测语言类型

**参数：**
| 参数 | 类型 | 必填 | 说明 |
|------|------|------|------|
| text | string | 是 | 要检测的文本 |

**返回：** `string` - 语言类型：`ancient`/`english`/`chinese`/`auto`

---

### 历史记录

#### `get_history`
获取历史记录

**参数：**
| 参数 | 类型 | 必填 | 说明 |
|------|------|------|------|
| limit | number | 否 | 返回条数，默认 50 |

**返回：** `HistoryItem[]`

---

#### `clear_history`
清空历史记录

**参数：** 无

**返回：** `void`

---

### 生词本

#### `add_to_vocabulary`
添加到生词本

**参数：**
| 参数 | 类型 | 必填 | 说明 |
|------|------|------|------|
| word | string | 是 | 单词 |
| word_type | string | 是 | 类型：`ancient`/`english` |
| definition | string | 是 | 释义 |
| note | string | 否 | 备注 |

**返回：** `void`

---

#### `get_vocabulary`
获取生词本

**参数：** 无

**返回：** `VocabularyItem[]`

---

#### `remove_from_vocabulary`
从生词本删除

**参数：**
| 参数 | 类型 | 必填 | 说明 |
|------|------|------|------|
| id | number | 是 | 记录 ID |

**返回：** `void`

---

### 设置

#### `get_settings`
获取设置

**参数：** 无

**返回：** `AppSettings`

---

#### `save_settings`
保存设置

**参数：**
| 参数 | 类型 | 必填 | 说明 |
|------|------|------|------|
| settings | AppSettings | 是 | 设置对象 |

**返回：** `void`

---

## TypeScript 类型定义

```typescript
// 历史记录项
interface HistoryItem {
  id: number
  query: string
  queryType: string
  result: string
  source: string
  createdAt: string
}

// 生词本项
interface VocabularyItem {
  id: number
  word: string
  wordType: string
  definition: string
  note?: string
  addedAt: string
}

// 应用设置
interface AppSettings {
  apiKey: string
  apiEndpoint: string
  shortcuts: { mainWindow: string, selectionTranslate: string }
  theme: 'light' | 'dark' | 'system'
  fontSize: number
  ancientEnabled: boolean
  englishEnabled: boolean
}
```

---

## 使用示例

```typescript
import { invoke } from '@tauri-apps/api/core'

// 查询单词
const result = await invoke('query_word', {
  word: '学',
  dictType: 'ancient'
})

// AI 翻译
const translation = await invoke('translate_text', {
  text: '学而时习之',
  sourceLang: 'ancient'
})

// 获取历史
const history = await invoke('get_history', { limit: 20 })
```