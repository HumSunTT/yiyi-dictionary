# 易译 🍑

> 古文/英语划词翻译助手 - 轻量、离线、AI增强



## ✨ 功能特性

- 🎯 **划词即译** - 选中文字即刻翻译
- 📚 **离线词库** - 内置古汉语词典、英汉词典，无网也能查词
- 🤖 **AI增强** - 联网时调用 DeepSeek 进行整句翻译
- ⭐ **生词本** - 收藏生词，支持导出
- 📜 **历史记录** - 查询历史随时查看
- 🎨 **多主题** - 支持浅色/深色/跟随系统

## 📦 安装

### 方式一：一键安装（推荐）

```bash
git clone https://github.com/your-repo/yi-yi.git
cd yi-yi
chmod +x install.sh
./install.sh
```

### 方式二：手动安装

#### 1. 安装系统依赖

**Ubuntu/Debian:**
```bash
sudo apt-get update
sudo apt-get install -y libgtk-3-dev libwebkit2gtk-4.1-dev libappindicator3-dev librsvg2-dev patchelf
```

**Fedora:**
```bash
sudo dnf install -y gtk3-devel webkit2gtk4.1-devel libappindicator-gtk3-devel librsvg2-devel openssl-devel
```

**Arch Linux:**
```bash
sudo pacman -S gtk3 webkit2gtk-4.1 libappindicator-gtk3 librsvg openssl
```

#### 2. 安装 Rust

```bash
# 使用国内镜像加速
export RUSTUP_DIST_SERVER="https://rsproxy.cn"
export RUSTUP_UPDATE_ROOT="https://rsproxy.cn/rustup"
curl --proto '=https' --tlsv1.2 -sSf https://rsproxy.cn/rustup-init.sh | sh -s -- -y
source "$HOME/.cargo/env"
```

#### 3. 安装项目依赖

```bash
cd yi-yi
npm install
```

## 🚀 使用

### 开发模式

```bash
# 启动前端开发服务器 + Tauri
npm run tauri dev
```

### 构建

```bash
# 构建生产版本
npm run tauri build
```

构建产物位于 `src-tauri/target/release/bundle/` 目录。

## 📖 使用说明

### 配置 AI 翻译

1. 打开设置面板
2. 输入 [DeepSeek API Key](https://platform.deepseek.com/)
3. 点击保存

配置完成后，输入整句古文或英文即可使用 AI 翻译。

### 快捷键

| 快捷键 | 功能 |
|--------|------|
| `Ctrl/Cmd + Shift + T` | 唤起主窗口 |
| `Ctrl/Cmd + Shift + D` | 划词翻译 |
| `Ctrl + Enter` | 在输入框中快速翻译 |

## 🏗️ 项目结构

```
yi-yi/
├── src/                  # Vue 3 前端
│   ├── components/       # UI 组件
│   ├── stores/           # Pinia 状态管理
│   ├── types/            # TypeScript 类型
│   └── styles/           # 样式
├── src-tauri/            # Rust 后端
│   ├── src/
│   │   ├── commands.rs   # Tauri 命令
│   │   ├── database.rs   # SQLite 操作
│   │   └── api.rs        # DeepSeek API
│   └── tauri.conf.json   # Tauri 配置
├── dictionaries/         # 词库数据
└── docs/                 # 文档
```

## 🔧 技术栈

| 层级 | 技术 |
|------|------|
| 桌面框架 | Tauri 2.0 |
| 前端 | Vue 3 + TypeScript |
| UI | Naive UI |
| 状态管理 | Pinia |
| 本地数据库 | SQLite |
| AI 翻译 | DeepSeek API |

## 📝 词库数据

目前内置示例词库包含：
- 古汉语词典：10个常用字
- 英汉词典：10个常用单词

完整词库可通过以下方式导入：
1. 下载 ECDICT 英汉词典数据
2. 导入古汉语词典数据
3. 自定义词库

详见 [词库导入指南](docs/dictionary-import.md)

## 🤝 贡献

欢迎贡献代码、词库数据或建议！

1. Fork 本仓库
2. 创建功能分支 (`git checkout -b feature/amazing-feature`)
3. 提交更改 (`git commit -m 'Add amazing feature'`)
4. 推送到分支 (`git push origin feature/amazing-feature`)
5. 创建 Pull Request

## 📄 许可证

[MIT License](LICENSE)

## 🙏 致谢

- [Tauri](https://tauri.app/) - 轻量级桌面应用框架
- [Naive UI](https://www.naiveui.com/) - Vue 3 UI 组件库
- [DeepSeek](https://deepseek.com/) - AI 翻译服务
- [ECDICT](https://github.com/skywind3000/ECDICT) - 开源英汉词典

---

🍑 **易译** - 让古文阅读更轻松
