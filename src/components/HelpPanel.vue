<template>
  <div class="help-panel">
    <n-collapse>
      <n-collapse-item title="📖 如何使用" name="usage">
        <div class="help-content">
          <p><strong>1. 查询单词</strong></p>
          <p>在输入框中输入古文或英文单词，按 <kbd>Enter</kbd> 即可翻译。按 <kbd>Shift+Enter</kbd> 换行。</p>
          
          <p><strong>2. AI 翻译</strong></p>
          <p>输入整句或段落，如果本地词库没有匹配，会自动调用 AI 进行翻译。</p>
          
          <p><strong>3. 生词本</strong></p>
          <p>点击"加入生词本"保存单词，方便复习。</p>
        </div>
      </n-collapse-item>
      
      <n-collapse-item title="⌨️ 快捷键" name="shortcuts">
        <div class="help-content">
          <table class="shortcut-table">
            <tbody>
              <tr><td><kbd>Enter</kbd></td><td>翻译当前输入</td></tr>
              <tr><td><kbd>Shift + Enter</kbd></td><td>换行</td></tr>
              <tr><td><kbd>Ctrl + Shift + T</kbd></td><td>显示/隐藏主窗口</td></tr>
            </tbody>
          </table>
        </div>
      </n-collapse-item>
      
      <n-collapse-item title="🔑 配置 API" name="api">
        <div class="help-content">
          <p>要使用 AI 翻译功能，需要配置 DeepSeek API Key：</p>
          <ol>
            <li>访问 <a href="https://platform.deepseek.com" target="_blank">DeepSeek 开放平台</a></li>
            <li>注册并获取 API Key</li>
            <li>在"设置"页面填入 API Key</li>
            <li>点击"保存设置"</li>
          </ol>
        </div>
      </n-collapse-item>
      
      <n-collapse-item title="📚 关于词库" name="dictionary">
        <div class="help-content">
          <p><strong>内置词库</strong></p>
          <p>目前包含：</p>
          <ul>
            <li>古汉语词典：52,759 词条（康熙字典 + 古汉语常用字）</li>
            <li>英汉词典：768,739 词条（ECDICT）</li>
          </ul>
          
          <p><strong>离线查询</strong></p>
          <p>输入单词会优先从本地词库查询，秒级响应，无需联网。</p>
        </div>
      </n-collapse-item>
      
      <n-collapse-item title="ℹ️ 关于易译" name="about">
        <div class="help-content">
          <p><strong>易译 v0.1.0</strong></p>
          <p>古文/英语划词翻译助手</p>
          <p style="color: #999; font-size: 12px; margin-top: 12px;">
            基于 Tauri 2.0 + Vue 3 开发<br>
            使用 DeepSeek API 进行 AI 翻译
          </p>
        </div>
      </n-collapse-item>
    </n-collapse>
  </div>
</template>

<script setup lang="ts">
import { NCollapse, NCollapseItem } from 'naive-ui'
</script>

<style scoped>
.help-panel {
  padding: 8px;
}

.help-content {
  padding: 8px;
  font-size: 14px;
  line-height: 1.8;
}

.help-content p {
  margin-bottom: 8px;
}

.help-content ul, .help-content ol {
  padding-left: 20px;
  margin-bottom: 12px;
}

.help-content a {
  color: #6366f1;
}

.shortcut-table {
  width: 100%;
  border-collapse: collapse;
}

.shortcut-table td {
  padding: 8px;
  border-bottom: 1px solid #eee;
}

.shortcut-table kbd {
  background: #f5f5f5;
  padding: 2px 8px;
  border-radius: 4px;
  font-family: monospace;
  font-size: 12px;
}
</style>