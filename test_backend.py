#!/usr/bin/env python3
"""
测试 Tauri 后端接口
"""

import sqlite3
import json

def test_database_query():
    """测试数据库查询"""
    db_path = "/home/supertaotao/.local/share/com.yi-yi.app/yi_yi.db"
    
    conn = sqlite3.connect(db_path)
    cursor = conn.cursor()
    
    # 测试古汉语查询
    cursor.execute("SELECT word, pinyin, definition, examples, source FROM ancient_dict WHERE word = ? LIMIT 1", ("哀",))
    result = cursor.fetchone()
    
    if result:
        print("✅ 古汉语查询成功:")
        word, pinyin, definition, examples, source = result
        print(f"  词: {word}")
        print(f"  拼音: {pinyin}")
        print(f"  来源: {source}")
        
        # 解析 definition JSON
        try:
            defs = json.loads(definition)
            print(f"  释义条数: {len(defs)}")
        except json.JSONDecodeError:
            print(f"  释义 (纯文本): {definition[:100]}...")
    else:
        print("❌ 古汉语查询失败")
    
    # 测试英文查询
    cursor.execute("SELECT word, phonetic, pos, definition, examples FROM english_dict WHERE word = ? COLLATE NOCASE LIMIT 1", ("test",))
    result = cursor.fetchone()
    
    if result:
        print("✅ 英文查询成功:")
        word, phonetic, pos, definition, examples = result
        print(f"  词: {word}")
        print(f"  音标: {phonetic}")
        print(f"  词性: {pos}")
        print(f"  释义: {definition[:100]}...")
    else:
        print("❌ 英文查询失败")
    
    conn.close()

if __name__ == "__main__":
    test_database_query()