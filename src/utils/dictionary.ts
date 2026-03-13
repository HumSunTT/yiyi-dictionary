// 词库数据加载和导入工具

import type { DefinitionItem, ExampleItem } from '../types'

/// 词库词条接口
export interface DictEntry {
  word: string
  phonetic?: string
  pos?: string
  definitions: DefinitionItem[]
  examples?: ExampleItem[]
  source?: string
  frequency?: number
}

/// 导入古汉语词典
export async function importAncientDictionary(entries: DictEntry[]): Promise<void> {
  // 在 Tauri 环境中，通过 API 导入
  // 在浏览器环境中，使用 IndexedDB 或 localStorage
  console.log('导入古汉语词典:', entries.length, '条')
}

/// 导入英汉词典
export async function importEnglishDictionary(entries: DictEntry[]): Promise<void> {
  console.log('导入英汉词典:', entries.length, '条')
}

/// 从 JSON 文件加载词库
export async function loadDictionaryFromJson(url: string): Promise<DictEntry[]> {
  const response = await fetch(url)
  const data = await response.json()
  return data.entries || data
}

/// 词库统计信息
export interface DictStats {
  ancientCount: number
  englishCount: number
  lastUpdate: string
}

/// 获取词库统计
export async function getDictStats(): Promise<DictStats> {
  // 模拟数据
  return {
    ancientCount: 25,
    englishCount: 20,
    lastUpdate: new Date().toISOString()
  }
}

/// ECDICT 词典格式
export interface ECDICTEntry {
  word: string
  phonetic: string
  definition: string
  translation: string
  pos: string
  collins: number
  oxford: number
  tag: string
  bnc: number | null
  frq: number | null
  exchange: string
  detail: string
  audio: string
}

/// 转换 ECDICT 格式到内部格式
export function convertFromECDICT(entry: ECDICTEntry): DictEntry {
  const definitions: DefinitionItem[] = entry.translation
    .split('\\n')
    .map(line => {
      const match = line.match(/^(n\.|v\.|adj\.|adv\.|prep\.|conj\.|int\.|pron\.)\s*(.*)/)
      if (match) {
        return { pos: match[1], definition: match[2] }
      }
      return { pos: '', definition: line }
    })
    .filter(d => d.definition)
  
  return {
    word: entry.word,
    phonetic: entry.phonetic,
    pos: entry.pos,
    definitions,
    source: 'ECDICT',
    frequency: entry.frq || 0
  }
}

/// 批量导入 ECDICT 数据
export async function batchImportECDICT(entries: ECDICTEntry[], batchSize = 100): Promise<number> {
  let imported = 0
  
  for (let i = 0; i < entries.length; i += batchSize) {
    const batch = entries.slice(i, i + batchSize)
    const converted = batch.map(convertFromECDICT)
    await importEnglishDictionary(converted)
    imported += batch.length
    
    // 进度回调
    console.log(`导入进度: ${imported}/${entries.length}`)
  }
  
  return imported
}