#!/bin/bash
# 易译 - 打包脚本

set -e

echo "🍑 易译 - 打包脚本"
echo "===================="

# 颜色定义
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# 检查 Rust
if ! command -v rustc &> /dev/null; then
    echo -e "${RED}❌ Rust 未安装${NC}"
    echo "请先运行 install.sh"
    exit 1
fi

# 检查 Node.js
if ! command -v node &> /dev/null; then
    echo -e "${RED}❌ Node.js 未安装${NC}"
    exit 1
fi

# 检查系统依赖
check_deps() {
    echo "检查系统依赖..."
    
    local missing=()
    
    if ! pkg-config --exists gtk+-3.0 2>/dev/null; then
        missing+=("libgtk-3-dev")
    fi
    
    if ! pkg-config --exists webkit2gtk-4.1 2>/dev/null; then
        missing+=("libwebkit2gtk-4.1-dev")
    fi
    
    if ! pkg-config --exists librsvg-2.0 2>/dev/null; then
        missing+=("librsvg2-dev")
    fi
    
    if [ ${#missing[@]} -ne 0 ]; then
        echo -e "${YELLOW}⚠️ 缺少系统依赖:${NC}"
        for dep in "${missing[@]}"; do
            echo "  - $dep"
        done
        echo ""
        echo "请运行以下命令安装:"
        echo "  sudo apt-get update && sudo apt-get install -y ${missing[*]}"
        exit 1
    fi
    
    echo -e "${GREEN}✅ 系统依赖检查通过${NC}"
}

# 检查依赖
check_deps

# 安装 npm 依赖
echo ""
echo "安装 npm 依赖..."
npm install

# 构建
echo ""
echo "开始构建..."
npm run tauri build

echo ""
echo -e "${GREEN}✅ 构建完成！${NC}"
echo ""
echo "安装包位于:"
echo "  src-tauri/target/release/bundle/"
echo ""
echo "🍑 易译 打包成功！"