// 模拟数据 - 用于浏览器开发模式

import type { DictionaryResult, TranslationResult, HistoryItem, VocabularyItem, AppSettings } from '../types'

/// 模拟古汉语词典数据
export const mockAncientWords: Record<string, DictionaryResult> = {
  '哀': {
    type: 'dictionary',
    word: '哀',
    phonetic: 'āi',
    source: '古汉语词典',
    definitions: [
      { pos: '形', definition: '悲痛；伤心' },
      { pos: '动', definition: '怜悯；同情' },
      { pos: '名', definition: '丧事' }
    ],
    examples: [
      { text: '伏屍而哭，極哀', source: '《荊軻刺秦王》' },
      { text: '君將哀而生之乎', source: '《捕蛇者說》' }
    ]
  },
  '学': {
    type: 'dictionary',
    word: '学',
    phonetic: 'xué',
    source: '古汉语词典',
    definitions: [
      { pos: '动', definition: '学习' },
      { pos: '名', definition: '学问' },
      { pos: '名', definition: '学校' }
    ],
    examples: [
      { text: '学而时习之，不亦说乎', source: '《论语》' },
      { text: '学而不思则罔，思而不学则殆', source: '《论语》' }
    ]
  },
  '之': {
    type: 'dictionary',
    word: '之',
    phonetic: 'zhī',
    source: '古汉语词典',
    definitions: [
      { pos: '代', definition: '他/她/它' },
      { pos: '助', definition: '的' },
      { pos: '动', definition: '到...去' }
    ],
    examples: [
      { text: '学而时习之', source: '《论语》' },
      { text: '之子于归', source: '《诗经》' }
    ]
  },
  '者': {
    type: 'dictionary',
    word: '者',
    phonetic: 'zhě',
    source: '古汉语词典',
    definitions: [
      { pos: '助', definition: '...的人' },
      { pos: '助', definition: '...的事物' },
      { pos: '助', definition: '表示停顿' }
    ],
    examples: [
      { text: '知者乐水，仁者乐山', source: '《论语》' }
    ]
  },
  '也': {
    type: 'dictionary',
    word: '也',
    phonetic: 'yě',
    source: '古汉语词典',
    definitions: [
      { pos: '助', definition: '句末语气词，表示判断或解释' },
      { pos: '助', definition: '句末语气词，表示疑问或感叹' }
    ],
    examples: [
      { text: '是吾剑之所从坠', source: '《吕氏春秋》' }
    ]
  },
  '乎': {
    type: 'dictionary',
    word: '乎',
    phonetic: 'hū',
    source: '古汉语词典',
    definitions: [
      { pos: '助', definition: '句末语气词，表示疑问' },
      { pos: '助', definition: '句末语气词，表示感叹' },
      { pos: '介', definition: '相当于"于"' }
    ],
    examples: [
      { text: '学而时习之，不亦说乎', source: '《论语》' }
    ]
  },
  '曰': {
    type: 'dictionary',
    word: '曰',
    phonetic: 'yuē',
    source: '古汉语词典',
    definitions: [
      { pos: '动', definition: '说' }
    ],
    examples: [
      { text: '子曰：学而时习之', source: '《论语》' }
    ]
  },
  '于': {
    type: 'dictionary',
    word: '于',
    phonetic: 'yú',
    source: '古汉语词典',
    definitions: [
      { pos: '介', definition: '在' },
      { pos: '介', definition: '向' },
      { pos: '介', definition: '对于' }
    ],
    examples: [
      { text: '己所不欲，勿施于人', source: '《论语》' }
    ]
  },
  '而': {
    type: 'dictionary',
    word: '而',
    phonetic: 'ér',
    source: '古汉语词典',
    definitions: [
      { pos: '连', definition: '并且' },
      { pos: '连', definition: '却' },
      { pos: '连', definition: '如果' }
    ],
    examples: [
      { text: '学而时习之', source: '《论语》' }
    ]
  },
  '吾': {
    type: 'dictionary',
    word: '吾',
    phonetic: 'wú',
    source: '古汉语词典',
    definitions: [
      { pos: '代', definition: '我，我的' }
    ],
    examples: [
      { text: '吾日三省吾身', source: '《论语》' }
    ]
  },
  '何': {
    type: 'dictionary',
    word: '何',
    phonetic: 'hé',
    source: '古汉语词典',
    definitions: [
      { pos: '代', definition: '什么' },
      { pos: '副', definition: '多么' }
    ],
    examples: [
      { text: '何陋之有', source: '《陋室铭》' }
    ]
  },
  // 更多常用古汉语
  '子': {
    type: 'dictionary',
    word: '子',
    phonetic: 'zǐ',
    source: '古汉语词典',
    definitions: [
      { pos: '名', definition: '子女' },
      { pos: '代', definition: '你' },
      { pos: '名', definition: '先生（尊称）' }
    ],
    examples: [
      { text: '子曰：学而时习之', source: '《论语》' }
    ]
  },
  '人': {
    type: 'dictionary',
    word: '人',
    phonetic: 'rén',
    source: '古汉语词典',
    definitions: [
      { pos: '名', definition: '人；别人' }
    ],
    examples: [
      { text: '己所不欲，勿施于人', source: '《论语》' }
    ]
  },
  '有': {
    type: 'dictionary',
    word: '有',
    phonetic: 'yǒu',
    source: '古汉语词典',
    definitions: [
      { pos: '动', definition: '拥有；存在' }
    ],
    examples: [
      { text: '有朋自远方来', source: '《论语》' }
    ]
  },
  '无': {
    type: 'dictionary',
    word: '无',
    phonetic: 'wú',
    source: '古汉语词典',
    definitions: [
      { pos: '动', definition: '没有' },
      { pos: '副', definition: '不要' }
    ],
    examples: [
      { text: '无欲则刚', source: '《论语》' }
    ]
  },
  '不': {
    type: 'dictionary',
    word: '不',
    phonetic: 'bù',
    source: '古汉语词典',
    definitions: [
      { pos: '副', definition: '不；没有' }
    ],
    examples: [
      { text: '不亦说乎', source: '《论语》' }
    ]
  },
  '可': {
    type: 'dictionary',
    word: '可',
    phonetic: 'kě',
    source: '古汉语词典',
    definitions: [
      { pos: '动', definition: '可以' },
      { pos: '副', definition: '大约' }
    ],
    examples: [
      { text: '可以为师矣', source: '《论语》' }
    ]
  },
  '自': {
    type: 'dictionary',
    word: '自',
    phonetic: 'zì',
    source: '古汉语词典',
    definitions: [
      { pos: '代', definition: '自己' },
      { pos: '介', definition: '从' }
    ],
    examples: [
      { text: '有朋自远方来', source: '《论语》' }
    ]
  }
}

/// 模拟英汉词典数据
export const mockEnglishWords: Record<string, DictionaryResult> = {
  'test': {
    type: 'dictionary',
    word: 'test',
    phonetic: '/test/',
    source: '英汉词典',
    definitions: [
      { pos: 'n.', definition: '测试；试验；检验' },
      { pos: 'v.', definition: '测试；试验' }
    ],
    examples: [
      { text: 'This is just a test.', translation: '这只是个测试。' }
    ]
  },
  'go': {
    type: 'dictionary',
    word: 'go',
    phonetic: '/ɡəʊ/',
    source: '英汉词典',
    definitions: [
      { pos: 'v.', definition: '去；走；离开' },
      { pos: 'n.', definition: '尝试' }
    ],
    examples: [
      { text: 'Let it go.', translation: '随它去吧。' }
    ]
  },
  'ai': {
    type: 'dictionary',
    word: 'AI',
    phonetic: '/ˌeɪˈaɪ/',
    source: '英汉词典',
    definitions: [
      { pos: 'n.', definition: '人工智能 (Artificial Intelligence)' }
    ],
    examples: [
      { text: 'AI is changing the world.', translation: '人工智能正在改变世界。' }
    ]
  },
  'hello': {
    type: 'dictionary',
    word: 'hello',
    phonetic: '/həˈləʊ/',
    source: '英汉词典',
    definitions: [
      { pos: 'int.', definition: '你好；喂' },
      { pos: 'n.', definition: '问候' }
    ],
    examples: [
      { text: 'Hello, how are you?', translation: '你好，你怎么样？' }
    ]
  },
  'world': {
    type: 'dictionary',
    word: 'world',
    phonetic: '/wɜːld/',
    source: '英汉词典',
    definitions: [
      { pos: 'n.', definition: '世界；地球' },
      { pos: 'n.', definition: '界；领域' }
    ],
    examples: [
      { text: 'The world is beautiful.', translation: '世界是美丽的。' }
    ]
  },
  'book': {
    type: 'dictionary',
    word: 'book',
    phonetic: '/bʊk/',
    source: '英汉词典',
    definitions: [
      { pos: 'n.', definition: '书' },
      { pos: 'v.', definition: '预订' }
    ],
    examples: [
      { text: 'I am reading a book.', translation: '我正在读一本书。' }
    ]
  },
  'time': {
    type: 'dictionary',
    word: 'time',
    phonetic: '/taɪm/',
    source: '英汉词典',
    definitions: [
      { pos: 'n.', definition: '时间；次；倍' }
    ],
    examples: [
      { text: 'Time flies.', translation: '光阴似箭。' }
    ]
  },
  'love': {
    type: 'dictionary',
    word: 'love',
    phonetic: '/lʌv/',
    source: '英汉词典',
    definitions: [
      { pos: 'v.', definition: '爱；喜爱' },
      { pos: 'n.', definition: '爱；爱情' }
    ],
    examples: [
      { text: 'I love you.', translation: '我爱你。' }
    ]
  },
  'know': {
    type: 'dictionary',
    word: 'know',
    phonetic: '/nəʊ/',
    source: '英汉词典',
    definitions: [
      { pos: 'v.', definition: '知道；认识' }
    ],
    examples: [
      { text: 'I know him well.', translation: '我很了解他。' }
    ]
  },
  'think': {
    type: 'dictionary',
    word: 'think',
    phonetic: '/θɪŋk/',
    source: '英汉词典',
    definitions: [
      { pos: 'v.', definition: '想；认为' }
    ],
    examples: [
      { text: 'I think so.', translation: '我也这么认为。' }
    ]
  },
  'good': {
    type: 'dictionary',
    word: 'good',
    phonetic: '/ɡʊd/',
    source: '英汉词典',
    definitions: [
      { pos: 'adj.', definition: '好的；优秀的' }
    ],
    examples: [
      { text: 'Good morning!', translation: '早上好！' }
    ]
  },
  'way': {
    type: 'dictionary',
    word: 'way',
    phonetic: '/weɪ/',
    source: '英汉词典',
    definitions: [
      { pos: 'n.', definition: '方式；道路' }
    ],
    examples: [
      { text: 'This is the way.', translation: '就是这样。' }
    ]
  },
  'day': {
    type: 'dictionary',
    word: 'day',
    phonetic: '/deɪ/',
    source: '英汉词典',
    definitions: [
      { pos: 'n.', definition: '天；白天' }
    ],
    examples: [
      { text: 'Have a nice day!', translation: '祝你有美好的一天！' }
    ]
  }
}

/// 模拟历史记录
export const mockHistory: HistoryItem[] = [
  { id: 1, query: '学而时习之', queryType: 'ancient', result: '{}', source: 'local', createdAt: new Date().toISOString() },
  { id: 2, query: 'hello', queryType: 'english', result: '{}', source: 'local', createdAt: new Date(Date.now() - 3600000).toISOString() },
  { id: 3, query: 'The world is beautiful', queryType: 'english', result: '{}', source: 'api', createdAt: new Date(Date.now() - 7200000).toISOString() },
  { id: 4, query: '之', queryType: 'ancient', result: '{}', source: 'local', createdAt: new Date(Date.now() - 86400000).toISOString() }
]

/// 模拟生词本
export const mockVocabulary: VocabularyItem[] = [
  { id: 1, word: '学', wordType: 'ancient', definition: '①学习②学问③学校', reviewCount: 0, addedAt: new Date().toISOString() },
  { id: 2, word: 'eloquent', wordType: 'english', definition: '雄辩的；有说服力的', reviewCount: 2, addedAt: new Date(Date.now() - 86400000).toISOString() },
  { id: 3, word: '之', wordType: 'ancient', definition: '①他/她/它②的③到...去', reviewCount: 1, addedAt: new Date(Date.now() - 172800000).toISOString() }
]

/// 模拟设置
export const mockSettings: AppSettings = {
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
}

/// 模拟 AI 翻译结果
export function mockTranslate(text: string, sourceLang: string): TranslationResult {
  const translations: Record<string, string> = {
    'ancient': `【翻译】这是对古文"${text}"的现代汉语翻译。

【注释】
- 这是模拟的翻译结果
- 实际使用时需要配置 DeepSeek API Key`,
    'english': `【翻译】这是对英文"${text}"的中文翻译。

【注释】
- This is a simulated translation
- Configure DeepSeek API Key for real translations`,
    'chinese': `【Translation】This is the English translation of "${text}".

【Notes】
- This is a simulated result
- Configure DeepSeek API Key for real translations`
  }
  
  return {
    type: 'translation',
    original: text,
    translation: translations[sourceLang] || translations['ancient'],
    notes: ['这是模拟数据，配置 API Key 后可使用真实翻译']
  }
}