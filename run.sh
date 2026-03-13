#!/bin/bash
# 易译启动脚本 - 修复 Linux WebKit 渲染问题
export WEBKIT_DISABLE_DMABUF_RENDERER=1
cd "$(dirname "$0")/src-tauri"
./target/release/yi-yi "$@"