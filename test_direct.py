#!/usr/bin/env python3
"""
直接测试 Rust Tauri 后端
"""

import subprocess
import json
import sys

def test_tauri_backend():
    """测试 Tauri 后端 API"""
    
    # 测试查询哀
    try:
        result = subprocess.run([
            'cargo', 'run', '--release', '-p', 'yi-yi', '--bin', 'yi-yi-test'
        ], capture_output=True, text=True, cwd='/home/supertaotao/.openclaw/workspace/yi-yi/src-tauri')
        
        print("Tauri backend test result:", result.stdout)
        print("Errors:", result.stderr)
    except Exception as e:
        print("Error running test:", e)

def manual_test():
    """手动测试数据库和 API"""
    print("=== 手动测试 ===")
    
    # 检查数据库
    import sqlite3
    db_path = "/home/supertaotao/.local/share/com.yi-yi.app/yi_yi.db"
    
    conn = sqlite3.connect(db_path)
    cursor = conn.cursor()
    
    # 测试古汉语
    cursor.execute("SELECT word, definition FROM ancient_dict WHERE word='哀' LIMIT 1")
    result = cursor.fetchone()
    if result:
        print("✅ 古汉语 '哀' 存在")
        print("   Definition length:", len(result[1]))
    else:
        print("❌ 古汉语 '哀' 不存在")
    
    # 测试英文
    cursor.execute("SELECT word, definition FROM english_dict WHERE word='test' COLLATE NOCASE LIMIT 1")
    result = cursor.fetchone()
    if result:
        print("✅ 英文 'test' 存在")
        print("   Definition length:", len(result[1]))
    else:
        print("❌ 英文 'test' 不存在")
    
    conn.close()

if __name__ == "__main__":
    manual_test()