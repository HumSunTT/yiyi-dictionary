#!/usr/bin/env python3
import sqlite3
import re
import json
from collections import defaultdict

def main():
    db_path = "/home/supertaotao/.local/share/com.yi-yi.app/yi_yi.db"
    conn = sqlite3.connect(db_path)
    cursor = conn.cursor()
    
    cursor.execute("SELECT word, phonetic, pos, definition FROM english_dict")
    rows = cursor.fetchall()
    
    print(f"读取 {len(rows)} 条英汉词典数据")
    
    chinese_to_english = defaultdict(list)
    
    for word, phonetic, pos, definition in rows:
        if ' ' in word:
            continue
        
        # 匹配词性标注后的中文词，如 "n. 家庭, 家人"
        pos_pattern = r'^([a-z]+\.?)\s*([\u4e00-\u9fff]+)'
        match = re.search(pos_pattern, definition)
        
        if match:
            pos_tag = match.group(1)
            chinese_word = match.group(2)
            if 1 <= len(chinese_word) <= 4:
                chinese_to_english[chinese_word].append({
                    'word': word,
                    'phonetic': phonetic or '',
                    'pos': pos_tag,
                    'definition': definition,
                    'score': 100
                })
        
        # 匹配逗号分隔的中文词，如 "家庭, 家人, 族"
        comma_pattern = r'[,，]\s*([\u4e00-\u9fff]+)'
        for match in re.finditer(comma_pattern, definition):
            chinese_word = match.group(1)
            if 1 <= len(chinese_word) <= 4:
                chinese_to_english[chinese_word].append({
                    'word': word,
                    'phonetic': phonetic or '',
                    'pos': pos or '',
                    'definition': definition,
                    'score': 80
                })
    
    print(f"生成 {len(chinese_to_english)} 个中文词条")
    
    cursor.execute("DELETE FROM chinese_dict")
    
    insert_count = 0
    for chinese_word, entries in chinese_to_english.items():
        # 按分数和单词长度排序
        entries.sort(key=lambda x: (-x['score'], len(x['word'])))
        
        # 去重，保留前10个不同单词
        seen_words = set()
        top_entries = []
        for entry in entries:
            if entry['word'] not in seen_words:
                seen_words.add(entry['word'])
                top_entries.append(entry)
                if len(top_entries) >= 10:
                    break
        
        definitions = []
        examples = []
        
        for entry in top_entries:
            definitions.append({
                'pos': entry['pos'],
                'definition': entry['word']
            })
            
            if len(examples) < 3:
                trans = entry['definition'].split('\n')[0][:80]
                examples.append({
                    'text': entry['word'],
                    'translation': trans
                })
        
        if definitions:
            cursor.execute(
                "INSERT INTO chinese_dict (word, pinyin, definition, examples, frequency) VALUES (?, ?, ?, ?, ?)",
                (
                    chinese_word,
                    None,
                    json.dumps(definitions, ensure_ascii=False),
                    json.dumps(examples, ensure_ascii=False),
                    len(entries)
                )
            )
            insert_count += 1
    
    conn.commit()
    conn.close()
    
    print(f"成功插入 {insert_count} 条汉英词典数据")

if __name__ == "__main__":
    main()
