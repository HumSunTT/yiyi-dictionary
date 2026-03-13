#!/usr/bin/env python3
"""创建小型示例数据库用于编译"""

import sqlite3
import os

def create_sample_db(db_path):
    """创建小型示例数据库"""
    
    # 删除旧文件
    if os.path.exists(db_path):
        os.remove(db_path)
    
    conn = sqlite3.connect(db_path)
    cursor = conn.cursor()
    
    # 创建古汉语词典表
    cursor.execute('''
        CREATE TABLE IF NOT EXISTS ancient_dict (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            word TEXT NOT NULL,
            pinyin TEXT,
            definition TEXT NOT NULL,
            examples TEXT,
            source TEXT,
            frequency INTEGER DEFAULT 0,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP
        )
    ''')
    
    # 创建英汉词典表
    cursor.execute('''
        CREATE TABLE IF NOT EXISTS english_dict (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            word TEXT NOT NULL,
            phonetic TEXT,
            pos TEXT,
            definition TEXT NOT NULL,
            examples TEXT,
            frequency INTEGER DEFAULT 0,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP
        )
    ''')
    
    # 创建索引
    cursor.execute('CREATE INDEX IF NOT EXISTS idx_ancient_word ON ancient_dict(word)')
    cursor.execute('CREATE INDEX IF NOT EXISTS idx_english_word ON english_dict(word)')
    
    # 插入少量古汉语示例数据
    ancient_words = [
        ('之', 'zhī', '[{"pos":"代","definition":"他/她/它"},{"pos":"助","definition":"的"}]', '[{"text":"学而时习之","source":"《论语》"}]', '古汉语常用字字典', 1000),
        ('者', 'zhě', '[{"pos":"助","definition":"...的人"}]', '[{"text":"知者乐水","source":"《论语》"}]', '古汉语常用字字典', 900),
        ('也', 'yě', '[{"pos":"助","definition":"句末语气词"}]', '[{"text":"是吾剑之所从坠","source":"《吕氏春秋》"}]', '古汉语常用字字典', 850),
        ('乎', 'hū', '[{"pos":"助","definition":"句末语气词，表示疑问"}]', '[{"text":"不亦说乎","source":"《论语》"}]', '古汉语常用字字典', 800),
        ('矣', 'yǐ', '[{"pos":"助","definition":"句末语气词，表示完成"}]', '[{"text":"可以为师矣","source":"《论语》"}]', '古汉语常用字字典', 750),
        ('曰', 'yuē', '[{"pos":"动","definition":"说"}]', '[{"text":"子曰","source":"《论语》"}]', '古汉语常用字字典', 700),
        ('于', 'yú', '[{"pos":"介","definition":"在；向；对于"}]', '[{"text":"勿施于人","source":"《论语》"}]', '古汉语常用字字典', 680),
        ('以', 'yǐ', '[{"pos":"介","definition":"用；因为"},{"pos":"连","definition":"而"}]', '[{"text":"以直报怨","source":"《论语》"}]', '古汉语常用字字典', 650),
        ('而', 'ér', '[{"pos":"连","definition":"并且；却"}]', '[{"text":"学而时习之","source":"《论语》"}]', '古汉语常用字字典', 620),
        ('学', 'xué', '[{"pos":"动","definition":"学习"},{"pos":"名","definition":"学问"}]', '[{"text":"学而时习之","source":"《论语》"}]', '古汉语常用字字典', 600),
        ('吾', 'wú', '[{"pos":"代","definition":"我，我的"}]', '[{"text":"吾日三省吾身","source":"《论语》"}]', '古汉语常用字字典', 550),
        ('何', 'hé', '[{"pos":"代","definition":"什么"},{"pos":"副","definition":"多么"}]', '[{"text":"何陋之有","source":"《陋室铭》"}]', '古汉语常用字字典', 500),
        ('则', 'zé', '[{"pos":"连","definition":"那么，就"}]', '[{"text":"学而不思则罔","source":"《论语》"}]', '古汉语常用字字典', 480),
        ('故', 'gù', '[{"pos":"连","definition":"所以"},{"pos":"名","definition":"原因"}]', '[{"text":"温故而知新","source":"《论语》"}]', '古汉语常用字字典', 450),
        ('然', 'rán', '[{"pos":"代","definition":"这样"},{"pos":"连","definition":"但是"}]', '[{"text":"然而不胜者","source":"《孟子》"}]', '古汉语常用字字典', 400),
    ]
    
    cursor.executemany(
        'INSERT INTO ancient_dict (word, pinyin, definition, examples, source, frequency) VALUES (?, ?, ?, ?, ?, ?)',
        ancient_words
    )
    
    # 插入少量英语示例数据
    english_words = [
        ('hello', '/həˈləʊ/', 'int.', '你好；喂', '[{"text":"Hello, how are you?","translation":"你好，你怎么样？"}]', 1000),
        ('world', '/wɜːld/', 'n.', '世界；地球', '[{"text":"The world is beautiful.","translation":"世界是美丽的。"}]', 950),
        ('book', '/bʊk/', 'n.', '书', '[{"text":"I am reading a book.","translation":"我正在读一本书。"}]', 900),
        ('time', '/taɪm/', 'n.', '时间；次', '[{"text":"Time flies.","translation":"光阴似箭。"}]', 850),
        ('people', '/ˈpiːpl/', 'n.', '人；人们', '[{"text":"People are friendly.","translation":"人们很友好。"}]', 800),
        ('know', '/nəʊ/', 'v.', '知道；认识', '[{"text":"I know him.","translation":"我认识他。"}]', 750),
        ('think', '/θɪŋk/', 'v.', '想；认为', '[{"text":"I think so.","translation":"我也这么认为。"}]', 700),
        ('good', '/ɡʊd/', 'adj.', '好的', '[{"text":"Good morning!","translation":"早上好！"}]', 680),
        ('new', '/njuː/', 'adj.', '新的', '[{"text":"Happy New Year!","translation":"新年快乐！"}]', 650),
        ('great', '/ɡreɪt/', 'adj.', '伟大的；极好的', '[{"text":"That\'s great!","translation":"太棒了！"}]', 600),
        ('work', '/wɜːk/', 'v./n.', '工作', '[{"text":"Hard work pays off.","translation":"努力工作会有回报。"}]', 550),
        ('life', '/laɪf/', 'n.', '生活；生命', '[{"text":"Life is short.","translation":"人生苦短。"}]', 500),
        ('love', '/lʌv/', 'v./n.', '爱；喜爱', '[{"text":"I love you.","translation":"我爱你。"}]', 480),
        ('way', '/weɪ/', 'n.', '方式；道路', '[{"text":"This is the way.","translation":"就是这样。"}]', 450),
        ('day', '/deɪ/', 'n.', '天；白天', '[{"text":"Have a nice day!","translation":"祝你有美好的一天！"}]', 400),
    ]
    
    cursor.executemany(
        'INSERT INTO english_dict (word, phonetic, pos, definition, examples, frequency) VALUES (?, ?, ?, ?, ?, ?)',
        english_words
    )
    
    conn.commit()
    conn.close()
    
    # 显示文件大小
    size = os.path.getsize(db_path)
    print(f"示例数据库已创建: {db_path}")
    print(f"文件大小: {size / 1024:.1f} KB")
    print(f"古汉语词汇: {len(ancient_words)} 条")
    print(f"英语词汇: {len(english_words)} 条")

if __name__ == '__main__':
    create_sample_db('src-tauri/dict_data.db')