#!/usr/bin/env python3
import sqlite3
import json
import re
from pathlib import Path

def import_kangxi(kangxi_db_path: str, target_db_path: str):
    """从康熙字典导入古汉语词典数据"""
    print("正在导入康熙字典数据...")
    
    source_conn = sqlite3.connect(kangxi_db_path)
    source_cursor = source_conn.cursor()
    
    target_conn = sqlite3.connect(target_db_path)
    target_cursor = target_conn.cursor()
    
    target_cursor.execute('''
        CREATE TABLE IF NOT EXISTS ancient_dict (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            word TEXT NOT NULL,
            pinyin TEXT,
            definition TEXT NOT NULL,
            examples TEXT,
            source TEXT,
            frequency INTEGER DEFAULT 0
        )
    ''')
    target_cursor.execute('CREATE INDEX IF NOT EXISTS idx_ancient_word ON ancient_dict(word)')
    
    target_cursor.execute('DELETE FROM ancient_dict WHERE source = "康熙字典"')
    
    source_cursor.execute('SELECT word, content FROM dict WHERE word IS NOT NULL')
    
    count = 0
    batch = []
    
    for row in source_cursor:
        word = row[0]
        content = row[1]
        
        if not word or word.startswith('〈'):
            continue
        
        word = word.strip()
        if len(word) > 10:
            continue
        
        clean_def = re.sub(r'【[^】]+】', '', content)
        clean_def = re.sub(r'\s+', ' ', clean_def).strip()
        if len(clean_def) > 500:
            clean_def = clean_def[:500] + '...'
        
        batch.append((word, None, clean_def, None, '康熙字典', 300))
        count += 1
        
        if len(batch) >= 1000:
            target_cursor.executemany('''
                INSERT INTO ancient_dict (word, pinyin, definition, examples, source, frequency)
                VALUES (?, ?, ?, ?, ?, ?)
            ''', batch)
            target_conn.commit()
            batch = []
            print(f"\r已导入: {count} 词", end='', flush=True)
    
    if batch:
        target_cursor.executemany('''
            INSERT INTO ancient_dict (word, pinyin, definition, examples, source, frequency)
            VALUES (?, ?, ?, ?, ?, ?)
        ''', batch)
        target_conn.commit()
    
    print(f"\n康熙字典导入完成: {count} 词")
    
    source_conn.close()
    target_conn.close()
    return count


def import_guhanyu(guhanyu_json_path: str, target_db_path: str):
    """从古汉语常用字字典导入数据"""
    print("正在导入古汉语常用字字典...")
    
    with open(guhanyu_json_path, 'r', encoding='utf-8') as f:
        data = json.load(f)
    
    conn = sqlite3.connect(target_db_path)
    cursor = conn.cursor()
    
    cursor.execute('''
        CREATE TABLE IF NOT EXISTS ancient_dict (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            word TEXT NOT NULL,
            pinyin TEXT,
            definition TEXT NOT NULL,
            examples TEXT,
            source TEXT,
            frequency INTEGER DEFAULT 0
        )
    ''')
    cursor.execute('CREATE INDEX IF NOT EXISTS idx_ancient_word ON ancient_dict(word)')
    
    cursor.execute('DELETE FROM ancient_dict WHERE source = "古汉语常用字字典"')
    
    count = 0
    batch = []
    
    for entry in data:
        if not entry or len(entry) < 2:
            continue
        
        word = entry[0] if entry[0] else None
        if not word or len(word) > 5:
            continue
        
        definitions = []
        pinyin = None
        
        for item in entry[1:]:
            if not item:
                continue
            
            if item.startswith('㈠') or item.startswith('㈡') or item.startswith('㈢'):
                continue
            
            if re.match(r'^[āáǎàēéěèīíǐìōóǒòūúǔùǖǘǚǜ]+$', item):
                pinyin = item
            else:
                clean_item = re.sub(r'〖[^」]+〗', '', item)
                clean_item = re.sub(r'～', word, clean_item)
                if clean_item.strip():
                    definitions.append(clean_item.strip())
        
        if not definitions:
            continue
        
        definition_text = '\n'.join(definitions[:5])
        
        examples = []
        for line in definitions:
            matches = re.findall(r'《([^》]+)》[^《]*「([^」]+)」', line)
            for match in matches:
                examples.append({'source': match[0], 'text': match[1]})
        
        examples_json = json.dumps(examples, ensure_ascii=False) if examples else None
        
        batch.append((word, pinyin, definition_text, examples_json, '古汉语常用字字典', 800))
        count += 1
        
        if len(batch) >= 100:
            cursor.executemany('''
                INSERT INTO ancient_dict (word, pinyin, definition, examples, source, frequency)
                VALUES (?, ?, ?, ?, ?, ?)
            ''', batch)
            conn.commit()
            batch = []
    
    if batch:
        cursor.executemany('''
            INSERT INTO ancient_dict (word, pinyin, definition, examples, source, frequency)
            VALUES (?, ?, ?, ?, ?, ?)
        ''', batch)
        conn.commit()
    
    print(f"古汉语常用字字典导入完成: {count} 词")
    
    conn.close()
    return count


def main():
    base_dir = Path(__file__).parent.parent
    dicts_dir = base_dir / "dictionaries"
    target_db = base_dir / "src-tauri" / "dict_data.db"
    
    print("=" * 50)
    print("古汉语词典导入工具")
    print("=" * 50)
    
    kangxi_db = dicts_dir / "kangxi" / "kangxi.4w.db"
    if kangxi_db.exists():
        print(f"\n1. 导入康熙字典")
        import_kangxi(str(kangxi_db), str(target_db))
    else:
        print(f"找不到康熙字典: {kangxi_db}")
    
    guhanyu_json = dicts_dir / "guhanyu-master" / "guhanyu.json"
    if guhanyu_json.exists():
        print(f"\n2. 导入古汉语常用字字典")
        import_guhanyu(str(guhanyu_json), str(target_db))
    else:
        print(f"找不到古汉语常用字字典: {guhanyu_json}")
    
    conn = sqlite3.connect(str(target_db))
    total = conn.execute('SELECT COUNT(*) FROM ancient_dict').fetchone()[0]
    conn.close()
    
    print(f"\n{'=' * 50}")
    print(f"古汉语词典总词条: {total}")
    print(f"数据库: {target_db}")
    print("✅ 导入完成！")


if __name__ == "__main__":
    main()