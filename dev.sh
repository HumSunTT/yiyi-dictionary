#!/bin/bash
# 易译 - 开发启动脚本

set -e

echo "🍑 易译 - 开发模式启动"
echo "========================"

cd "$(dirname "$0")"

# 检查 node_modules
if [ ! -d "node_modules" ]; then
    echo "安装依赖..."
    npm install
fi

# 启动开发服务器
echo ""
echo "启动开发服务器..."
echo "前端地址: http://localhost:1420"
echo ""

npm run tauri dev