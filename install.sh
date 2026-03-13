#!/bin/bash
# 易译 - 一键安装脚本

set -e

echo "🍑 易译 - 划词翻译助手"
echo "=========================="
echo ""

# 检测操作系统
if [[ "$OSTYPE" == "linux-gnu"* ]]; then
    echo "检测到 Linux 系统"
    
    # 检测包管理器
    if command -v apt-get &> /dev/null; then
        echo "使用 apt-get 安装依赖..."
        sudo apt-get update
        sudo apt-get install -y \
            libgtk-3-dev \
            libwebkit2gtk-4.1-dev \
            libappindicator3-dev \
            librsvg2-dev \
            patchelf \
            build-essential \
            curl \
            wget
    elif command -v dnf &> /dev/null; then
        echo "使用 dnf 安装依赖..."
        sudo dnf install -y \
            gtk3-devel \
            webkit2gtk4.1-devel \
            libappindicator-gtk3-devel \
            librsvg2-devel \
            openssl-devel \
            curl \
            wget
    elif command -v pacman &> /dev/null; then
        echo "使用 pacman 安装依赖..."
        sudo pacman -S --noconfirm \
            gtk3 \
            webkit2gtk-4.1 \
            libappindicator-gtk3 \
            librsvg \
            openssl \
            curl \
            wget
    else
        echo "❌ 不支持的包管理器，请手动安装依赖"
        exit 1
    fi
    
elif [[ "$OSTYPE" == "darwin"* ]]; then
    echo "检测到 macOS 系统"
    
    if command -v brew &> /dev/null; then
        echo "使用 Homebrew 安装依赖..."
        brew install create-dmg
    else
        echo "❌ 请先安装 Homebrew: https://brew.sh"
        exit 1
    fi
    
else
    echo "❌ 不支持的操作系统: $OSTYPE"
    exit 1
fi

# 检查 Rust
if ! command -v rustc &> /dev/null; then
    echo ""
    echo "安装 Rust..."
    export RUSTUP_DIST_SERVER="https://rsproxy.cn"
    export RUSTUP_UPDATE_ROOT="https://rsproxy.cn/rustup"
    curl --proto '=https' --tlsv1.2 -sSf https://rsproxy.cn/rustup-init.sh | sh -s -- -y
    source "$HOME/.cargo/env"
else
    echo "✅ Rust 已安装: $(rustc --version)"
fi

# 检查 Node.js
if ! command -v node &> /dev/null; then
    echo "❌ 请先安装 Node.js >= 18"
    exit 1
else
    echo "✅ Node.js 已安装: $(node --version)"
fi

echo ""
echo "✅ 依赖安装完成！"
echo ""
echo "下一步："
echo "  cd yi-yi"
echo "  npm install"
echo "  npm run tauri dev"
echo ""
echo "🍑 开始使用易译吧！"