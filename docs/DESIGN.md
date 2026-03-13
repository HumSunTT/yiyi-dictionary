# 易译 - 设计文档

> 古文/英语划词翻译助手，支持离线词库 + 联网AI翻译

---

## 一、项目概述

### 1.1 产品定位

一款轻量级桌面翻译工具，主打：
- **划词即译**：选中文字即刻翻译
- **离线可用**：内置词库，无网也能查词
- **AI增强**：联网时调用大模型进行整句翻译

### 1.2 目标用户

- 古文爱好者、学生、研究者
- 英语学习者
- 需要频繁查阅古文/英文的用户

### 1.3 核心价值

| 场景 | 传统方案 | 易译方案 |
|------|----------|----------|
| 读古文遇到生词 | 翻书/搜网页 | 划词即查，释义+例句 |
| 无网络环境 | 无法使用 | 离线词库正常查询 |
| 整句古文理解 | 查多个词拼凑 | AI一键翻译+解析 |
| 英文阅读 | 切换翻译软件 | 统一工具，一套习惯 |

---

## 二、技术选型

### 2.1 技术栈

| 层级 | 技术 | 版本 | 说明 |
|------|------|------|------|
| 桌面框架 | Tauri | 2.x | 轻量（~3MB），Rust后端 |
| 前端框架 | Vue 3 | 最新 | 组合式API，TypeScript |
| UI组件 | Naive UI | 最新 | 轻量、美观、中文友好 |
| 状态管理 | Pinia | 最新 | Vue官方推荐 |
| 本地数据库 | SQLite | 3.x | 轻量、无需服务 |
| 词库数据 | ECDICT + 古汉语数据 | - | 开源词典 |
| AI翻译 | DeepSeek API | - | 国产大模型，性价比高 |

### 2.2 开发环境

- Node.js >= 18
- Rust >= 1.70
- pnpm（包管理器）

---

## 三、系统架构

### 3.1 整体架构

```
┌─────────────────────────────────────────────────────────┐
│                      Tauri 应用                         │
├─────────────────────────────────────────────────────────┤
│                                                         │
│  ┌─────────────────────────────────────────────────┐   │
│  │                   前端 (Vue 3)                    │   │
│  │  ┌───────────┐ ┌───────────┐ ┌───────────┐     │   │
│  │  │ 主窗口     │ │ 悬浮窗    │ │ 设置页    │     │   │
│  │  │ 输入翻译  │ │ 划词结果  │ │ 词库管理  │     │   │
│  │  └───────────┘ └───────────┘ └───────────┘     │   │
│  └─────────────────────────────────────────────────┘   │
│                          │                             │
│                     Tauri Commands                      │
│                          │                             │
│  ┌─────────────────────────────────────────────────┐   │
│  │                   后端 (Rust)                     │   │
│  │  ┌───────────┐ ┌───────────┐ ┌───────────┐     │   │
│  │  │ 划词捕获   │ │ 词库查询  │ │ API调用   │     │   │
│  │  │ 剪贴板    │ │ SQLite    │ │ DeepSeek  │     │   │
│  │  └───────────┘ └───────────┘ └───────────┘     │   │
│  └─────────────────────────────────────────────────┘   │
│                                                         │
├─────────────────────────────────────────────────────────┤
│                      本地数据                            │
│  ┌─────────────────┐  ┌─────────────────┐              │
│  │ SQLite 词库      │  │ 用户数据        │              │
│  │ - 古汉语词典     │  │ - 历史记录      │              │
│  │ - 英汉词典       │  │ - 生词本        │              │
│  │                 │  │ - 设置          │              │
│  └─────────────────┘  └─────────────────┘              │
└─────────────────────────────────────────────────────────┘
```

### 3.2 数据流

```
用户划词
    │
    ▼
┌─────────────┐
│ 文本捕获    │ ← 剪贴板监听 / 全局划词 Hook
└─────────────┘
    │
    ▼
┌─────────────┐
│ 本地词库查询 │ ← SQLite（古汉语词典 / 英汉词典）
└─────────────┘
    │
    ├── 命中 → 展示结果
    │
    └── 未命中 / 整句翻译
            │
            ▼
    ┌─────────────┐
    │ 网络检测    │
    └─────────────┘
            │
            ├── 有网 → DeepSeek API 翻译
            │
            └── 无网 → 提示"需要联网翻译"
```

---

## 四、功能规格

### 4.1 核心功能

| 功能 | 描述 | 优先级 |
|------|------|--------|
| 划词翻译 | 选中文字自动触发翻译 | P0 |
| 词库查询 | 本地SQLite词库快速查询 | P0 |
| AI翻译 | DeepSeek整句翻译 | P0 |
| 悬浮窗展示 | 结果以悬浮窗形式展示 | P0 |
| 快捷键触发 | 全局热键唤起翻译窗口 | P1 |
| 历史记录 | 查询历史保存 | P1 |
| 生词本 | 收藏生词 | P1 |
| 设置页 | API配置、快捷键设置 | P1 |
| 多语言支持 | 古文、英文、现代中文 | P2 |
| 导出生词本 | 导出为文本/CSV | P2 |

### 4.2 界面设计

#### 4.2.1 主窗口

```
┌─────────────────────────────────────┐
│  易译                          ─ □ ✕│
├─────────────────────────────────────┤
│                                     │
│  ┌───────────────────────────────┐  │
│  │ 输入或粘贴文字...             │  │
│  │                               │  │
│  │                               │  │
│  └───────────────────────────────┘  │
│                                     │
│  [ 自动检测 ▼ ]  [ 翻译 ]           │
│                                     │
│  ─────────────────────────────────  │
│                                     │
│  查询结果：                          │
│                                     │
│  ┌───────────────────────────────┐  │
│  │ 【词目】学                     │  │
│  │ 【拼音】xué                    │  │
│  │ 【释义】①学习；②学问；③学校   │  │
│  │ 【例句】学而时习之             │  │
│  │ 【出处】《论语》               │  │
│  └───────────────────────────────┘  │
│                                     │
│  [ 加入生词本 ]  [ 复制 ]           │
│                                     │
├─────────────────────────────────────┤
│  📖 历史  ⭐ 生词本  ⚙️ 设置       │
└─────────────────────────────────────┘
```

#### 4.2.2 悬浮窗（划词触发）

```
┌─────────────────────────────────┐
│ 学 (xué)                    📌 ✕│
├─────────────────────────────────┤
│ ①学习 ②学问 ③学校              │
│                                 │
│ 学而时习之，不亦说乎             │
│ ──《论语》                      │
│                                 │
│ [ AI翻译 ] [ 加生词本 ] [ 复制 ] │
└─────────────────────────────────┘
```

### 4.3 快捷键

| 快捷键 | 功能 | 可自定义 |
|--------|------|----------|
| `Ctrl/Cmd + Shift + T` | 唤起主窗口 | ✅ |
| `Ctrl/Cmd + Shift + D` | 划词翻译（选中文本后） | ✅ |
| `Esc` | 关闭悬浮窗 | - |

---

## 五、数据结构

### 5.1 SQLite 词库表结构

#### 古汉语词典表 `ancient_dict`

```sql
CREATE TABLE ancient_dict (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    word TEXT NOT NULL,           -- 词目
    pinyin TEXT,                  -- 拼音
    definition TEXT NOT NULL,     -- 释义（JSON数组）
    examples TEXT,                -- 例句（JSON数组）
    source TEXT,                  -- 出处
    frequency INTEGER DEFAULT 0,  -- 使用频率
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_ancient_word ON ancient_dict(word);
```

#### 英汉词典表 `english_dict`

```sql
CREATE TABLE english_dict (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    word TEXT NOT NULL,           -- 单词
    phonetic TEXT,                -- 音标
    pos TEXT,                     -- 词性
    definition TEXT NOT NULL,     -- 释义（JSON数组）
    examples TEXT,                -- 例句（JSON数组）
    frequency INTEGER DEFAULT 0,  -- 词频
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_english_word ON english_dict(word);
```

### 5.2 用户数据表

#### 历史记录 `history`

```sql
CREATE TABLE history (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    query TEXT NOT NULL,          -- 查询内容
    query_type TEXT,              -- 类型：ancient/english/sentence
    result TEXT,                  -- 结果（JSON）
    source TEXT,                  -- 来源：local/api
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_history_time ON history(created_at DESC);
```

#### 生词本 `vocabulary`

```sql
CREATE TABLE vocabulary (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    word TEXT NOT NULL,
    word_type TEXT,               -- ancient/english
    definition TEXT,
    note TEXT,                    -- 用户笔记
    review_count INTEGER DEFAULT 0,
    next_review DATE,             -- 下次复习日期
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_vocab_word ON vocabulary(word);
```

### 5.3 设置存储

使用 Tauri 的 `tauri-plugin-store` 存储用户设置：

```typescript
interface AppSettings {
  apiKey: string;              // DeepSeek API Key
  apiEndpoint: string;         // API 地址
  shortcuts: {
    mainWindow: string;
    selectionTranslate: string;
  };
  ui: {
    theme: 'light' | 'dark' | 'system';
    fontSize: number;
    showPhonetic: boolean;
  };
  dictionary: {
    ancientEnabled: boolean;
    englishEnabled: boolean;
  };
}
```

---

## 六、API 设计

### 6.1 Tauri Commands（Rust → Vue）

```rust
// 词库查询
#[tauri::command]
fn query_word(word: String, dict_type: String) -> Result<WordResult, String>;

// AI翻译
#[tauri::command]
async fn translate_text(text: String, source_lang: String, target_lang: String) -> Result<String, String>;

// 历史记录
#[tauri::command]
fn get_history(limit: i32) -> Result<Vec<HistoryItem>, String>;

#[tauri::command]
fn clear_history() -> Result<(), String>;

// 生词本
#[tauri::command]
fn add_to_vocabulary(word: String, word_type: String, definition: String) -> Result<(), String>;

#[tauri::command]
fn get_vocabulary() -> Result<Vec<VocabularyItem>, String>;

#[tauri::command]
fn remove_from_vocabulary(id: i32) -> Result<(), String>;

// 剪贴板
#[tauri::command]
fn get_clipboard_text() -> Result<String, String>;

// 设置
#[tauri::command]
fn get_settings() -> Result<AppSettings, String>;

#[tauri::command]
fn save_settings(settings: AppSettings) -> Result<(), String>;
```

### 6.2 DeepSeek API 集成

```rust
// 请求结构
struct TranslateRequest {
    model: String,        // "deepseek-chat"
    messages: Vec<Message>,
    temperature: f32,
}

struct Message {
    role: String,         // "system" | "user"
    content: String,
}

// Prompt 模板
const TRANSLATE_PROMPT: &str = r#"
你是一位专业的古文翻译专家。请将以下文本翻译成现代汉语。

要求：
1. 准确理解原文含义
2. 翻译通顺流畅
3. 如有典故或特殊用法，请简要说明
4. 保持原文的语气和风格

原文：{text}

请直接输出翻译结果，不需要额外解释。
"#;

const ENGLISH_PROMPT: &str = r#"
You are a professional translator. Please translate the following text into Chinese.

Requirements:
1. Accurate translation
2. Natural and fluent Chinese
3. Keep the original tone and style

Original text: {text}

Please output the translation directly.
"#;
```

---

## 七、词库数据

### 7.1 数据源

| 词库 | 数据源 | 许可证 | 数据量 |
|------|--------|--------|--------|
| 古汉语词典 | 开放古汉语数据 + 自建 | 学术使用 | ~10万词条 |
| 英汉词典 | ECDICT | MIT | ~100万词条 |

### 7.2 ECDICT 数据

ECDICT 是开源英汉词典项目：
- GitHub: https://github.com/skywind3000/ECDICT
- 包含：单词、音标、释义、例句、词频等
- 格式：CSV / SQLite

### 7.3 古汉语数据

可选数据源：
1. 《古汉语常用字字典》数据（需授权）
2. 国学网开放数据
3. 用户贡献 + 审核机制

---

## 八、项目结构

```
yi-yi/                          # 项目根目录
├── src-tauri/                  # Rust 后端
│   ├── src/
│   │   ├── main.rs            # 入口
│   │   ├── commands/           # Tauri Commands
│   │   │   ├── mod.rs
│   │   │   ├── dictionary.rs   # 词库查询
│   │   │   ├── translate.rs    # AI翻译
│   │   │   ├── history.rs      # 历史记录
│   │   │   └── vocabulary.rs   # 生词本
│   │   ├── database/           # 数据库操作
│   │   │   ├── mod.rs
│   │   │   ├── sqlite.rs
│   │   │   └── models.rs
│   │   └── utils/              # 工具函数
│   │       ├── mod.rs
│   │       ├── clipboard.rs
│   │       └── hotkey.rs
│   ├── Cargo.toml
│   └── tauri.conf.json        # Tauri 配置
│
├── src/                        # Vue 前端
│   ├── main.ts
│   ├── App.vue
│   ├── components/             # 组件
│   │   ├── MainWindow.vue     # 主窗口
│   │   ├── FloatWindow.vue    # 悬浮窗
│   │   ├── ResultCard.vue     # 结果卡片
│   │   ├── HistoryPanel.vue   # 历史面板
│   │   ├── VocabularyPanel.vue# 生词本面板
│   │   └── SettingsPanel.vue   # 设置面板
│   ├── stores/                 # Pinia 状态
│   │   ├── app.ts
│   │   ├── dictionary.ts
│   │   └── settings.ts
│   ├── types/                  # TypeScript 类型
│   │   └── index.ts
│   └── styles/                 # 样式
│       └── main.css
│
├── dictionaries/               # 词库数据（构建时打包）
│   ├── ancient.db
│   └── english.db
│
├── docs/                       # 文档
│   ├── DESIGN.md              # 本文档
│   └── API.md                 # API 文档
│
├── package.json
├── vite.config.ts
├── tsconfig.json
└── README.md
```

---

## 九、开发计划

### Phase 1: 项目骨架（Day 1）

- [ ] 初始化 Tauri + Vue 3 项目
- [ ] 配置 TypeScript + Pinia + Naive UI
- [ ] 搭建项目目录结构
- [ ] 创建基础窗口框架

### Phase 2: 词库系统（Day 2-3）

- [ ] SQLite 集成
- [ ] 下载并导入 ECDICT 英汉词典
- [ ] 导入古汉语词典数据
- [ ] 实现词库查询接口
- [ ] 结果展示 UI

### Phase 3: AI 翻译（Day 4）

- [ ] DeepSeek API 集成
- [ ] 翻译 Prompt 优化
- [ ] 错误处理与重试
- [ ] 加载状态与动画

### Phase 4: 划词功能（Day 5-6）

- [ ] 剪贴板监听
- [ ] 全局快捷键注册
- [ ] 悬浮窗实现
- [ ] 划词触发逻辑

### Phase 5: 用户体验（Day 7-8）

- [ ] 历史记录功能
- [ ] 生词本功能
- [ ] 设置页面
- [ ] 主题切换（亮/暗）

### Phase 6: 打包分发（Day 9）

- [ ] Windows 打包测试
- [ ] macOS 打包测试
- [ ] 安装程序制作
- [ ] 使用文档编写

---

## 十、后续扩展

### 10.1 短期扩展

- PDF 划词支持
- 浏览器插件版本
- OCR 截图翻译
- 生词本复习提醒

### 10.2 长期扩展

- 同步服务（多设备）
- 社区词库贡献
- 更多翻译引擎（GPT、Claude）
- 移动端适配

---

## 十一、风险与对策

| 风险 | 影响 | 对策 |
|------|------|------|
| 词库数据不完整 | 用户体验差 | 支持用户反馈补充，持续更新 |
| DeepSeek API 不稳定 | 翻译失败 | 多引擎备份，降级提示 |
| 划词兼容性问题 | 部分应用不生效 | 提供剪贴板备选方案 |
| 打包体积过大 | 下载慢 | 压缩词库，按需下载 |

---

*文档版本：v1.0*
*创建日期：2026-03-12*
*作者：小桃 🍑*