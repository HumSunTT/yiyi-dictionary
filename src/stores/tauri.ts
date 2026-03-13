// Tauri 窗口和剪贴板工具函数

import type { LogicalSize, PhysicalPosition } from '@tauri-apps/api/window'

// 检测是否在 Tauri 环境中
export const isTauri = typeof window !== 'undefined' && '__TAURI__' in window

// 窗口控制
export const windowControls = {
  async minimize() {
    if (!isTauri) return
    const { getCurrentWindow } = await import('@tauri-apps/api/window')
    getCurrentWindow().minimize()
  },
  
  async maximize() {
    if (!isTauri) return
    const { getCurrentWindow } = await import('@tauri-apps/api/window')
    getCurrentWindow().toggleMaximize()
  },
  
  async close() {
    if (!isTauri) return
    const { getCurrentWindow } = await import('@tauri-apps/api/window')
    getCurrentWindow().close()
  },
  
  async show() {
    if (!isTauri) return
    const { getCurrentWindow } = await import('@tauri-apps/api/window')
    getCurrentWindow().show()
    getCurrentWindow().setFocus()
  },
  
  async hide() {
    if (!isTauri) return
    const { getCurrentWindow } = await import('@tauri-apps/api/window')
    getCurrentWindow().hide()
  },
  
  async setSize(width: number, height: number) {
    if (!isTauri) return
    const { getCurrentWindow } = await import('@tauri-apps/api/window')
    getCurrentWindow().setSize({ type: 'Logical', width, height } as LogicalSize)
  },
  
  async setPosition(x: number, y: number) {
    if (!isTauri) return
    const { getCurrentWindow } = await import('@tauri-apps/api/window')
    getCurrentWindow().setPosition({ type: 'Physical', x, y } as PhysicalPosition)
  }
}

// 剪贴板操作
export const clipboard = {
  async readText(): Promise<string | null> {
    if (!isTauri) {
      // 浏览器环境
      try {
        return await navigator.clipboard.readText()
      } catch {
        return null
      }
    }
    
    // Tauri 环境使用 navigator.clipboard
    try {
      return await navigator.clipboard.readText()
    } catch {
      return null
    }
  },
  
  async writeText(text: string): Promise<void> {
    if (!isTauri) {
      await navigator.clipboard.writeText(text)
      return
    }
    
    await navigator.clipboard.writeText(text)
  }
}

// 全局快捷键 - 需要在 Tauri 配置中启用
export const hotkey = {
  async register(_shortcut: string, _callback: () => void) {
    // Tauri 2.0 全局快捷键需要在 Rust 端配置
    console.log('全局快捷键需要在 Tauri 配置中启用')
  },
  
  async unregister(_shortcut: string) {
    console.log('全局快捷键需要在 Tauri 配置中启用')
  },
  
  async unregisterAll() {
    console.log('全局快捷键需要在 Tauri 配置中启用')
  }
}

// 自动检测剪贴板变化并翻译
export function watchClipboard(callback: (text: string) => void, interval = 500) {
  let lastText = ''
  
  const checkClipboard = async () => {
    const text = await clipboard.readText()
    if (text && text !== lastText) {
      lastText = text
      callback(text)
    }
  }
  
  const timer = setInterval(checkClipboard, interval)
  
  return () => clearInterval(timer)
}