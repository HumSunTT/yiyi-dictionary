#!/bin/bash

APP_DIR="/home/supertaotao/.openclaw/workspace/yi-yi"
LOG_FILE="/tmp/yi-yi-startup.log"

echo "========================================" | tee "$LOG_FILE"
echo "易译翻译应用启动" | tee -a "$LOG_FILE"
echo "$(date)" | tee -a "$LOG_FILE"
echo "========================================" | tee -a "$LOG_FILE"

cd "$APP_DIR"

echo "[1/5] 停止旧进程..." | tee -a "$LOG_FILE"
pkill -f "target/debug/yi-yi" 2>/dev/null || true
pkill -f "target/release/yi-yi" 2>/dev/null || true
pkill -f "node.*vite" 2>/dev/null || true
pkill -f "node.*tauri" 2>/dev/null || true
sleep 2

echo "[2/5] 检查端口..." | tee -a "$LOG_FILE"
if lsof -i :1420 >/dev/null 2>&1; then
    lsof -ti :1420 | xargs kill -9 2>/dev/null || true
    sleep 1
fi

echo "[3/5] 检查数据库..." | tee -a "$LOG_FILE"
DB_PATH="$HOME/.local/share/com.yi-yi.translate/yi_yi.db"
if [ ! -f "$DB_PATH" ] || [ $(stat -c%s "$DB_PATH" 2>/dev/null || echo 0) -lt 10000000 ]; then
    mkdir -p "$(dirname "$DB_PATH")"
    cp "$APP_DIR/src-tauri/dict_data.db" "$DB_PATH"
fi
echo "数据库: $(ls -lh "$DB_PATH" | awk '{print $5}')" | tee -a "$LOG_FILE"

echo "[4/5] 编译检查..." | tee -a "$LOG_FILE"
cd "$APP_DIR/src-tauri"
cargo build 2>&1 | tail -3 | tee -a "$LOG_FILE"
cd "$APP_DIR"

echo "[5/5] 启动应用..." | tee -a "$LOG_FILE"
echo "========================================" | tee -a "$LOG_FILE"

exec npm run tauri dev 2>&1 | tee -a "$LOG_FILE"