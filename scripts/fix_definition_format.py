#!/usr/bin/env python3
import sqlite3
import json
import re
from pathlib import Path

def convert_definition_to_json():
    """将 ancient_dict 表中的 definition 从纯文本转换为 JSON 格式"""
    
    db_path = Path(__file__).parent.parent / "src-tauri" / "dict_data.db"
    
    conn = sqlite3.connect(str(db_path))
    cursor = conn.cursor()
    
    cursor.execute("SELECT id, word, definition FROM ancient_dict")
    rows = cursor.fetchall()
    
    print(f"正在转换 {len(rows)} 条记录...")
    
    updated = 0
    for row_id, word, definition in rows:
        if not definition:
            continue
        
        if definition.startswith('['):
            continue
        
        definitions = [{"pos": "", "definition": definition}]
        json_str = json.dumps(definitions, ensure_ascii=False)
        
        cursor.execute("UPDATE ancient_dict SET definition = ? WHERE id = ?", (json_str, row_id))
        updated += 1
        
        if updated % 1000 == 0:
            conn.commit()
            print(f"\r已转换: {updated}", end='', flush=True)
    
    conn.commit()
    print(f"\n✅ 转换完成: {updated} 条记录")
    
    cursor.execute("SELECT word, definition FROM ancient_dict WHERE word='哀' LIMIT 1")
    result = cursor.fetchone()
    if result:
        print(f"\n验证 '哀': {result[1][:100]}...")
    
    conn.close()

if __name__ == "__main__":
    convert_definition_to_json()