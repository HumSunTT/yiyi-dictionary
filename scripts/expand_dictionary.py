#!/usr/bin/env python3
"""
易译词典数据扩展脚本
- 扩展古汉语词典数据
- 导入 ECDICT 英汉词典
"""

import sqlite3
import json
import os
from pathlib import Path

# 古汉语词典扩展数据 - 常用文言词汇
ANCIENT_WORDS = [
    # 天文地理
    ("天", "tiān", [
        {"pos": "名", "definition": "天空"},
        {"pos": "名", "definition": "自然界"},
        {"pos": "名", "definition": "天帝，老天爷"}
    ], [("天行有常", "《荀子》"), ("天将降大任于是人也", "《孟子》")], "古汉语词典", 900),
    
    ("地", "dì", [
        {"pos": "名", "definition": "大地，土地"},
        {"pos": "名", "definition": "地面"},
        {"pos": "名", "definition": "地方，地点"}
    ], [("地势坤，君子以厚德载物", "《周易》")], "古汉语词典", 880),
    
    ("山", "shān", [
        {"pos": "名", "definition": "山峰，山岳"},
        {"pos": "名", "definition": "隐居之处"}
    ], [("采菊东篱下，悠然见南山", "陶渊明《饮酒》")], "古汉语词典", 600),
    
    ("水", "shuǐ", [
        {"pos": "名", "definition": "水，河流"},
        {"pos": "名", "definition": "水域"}
    ], [("上善若水", "《道德经》"), ("水能载舟，亦能覆舟", "《荀子》")], "古汉语词典", 650),
    
    ("日", "rì", [
        {"pos": "名", "definition": "太阳"},
        {"pos": "名", "definition": "白天"},
        {"pos": "名", "definition": "一昼夜"}
    ], [("日出而作，日入而息", "《击壤歌》")], "古汉语词典", 700),
    
    ("月", "yuè", [
        {"pos": "名", "definition": "月亮"},
        {"pos": "名", "definition": "月份"}
    ], [("明月几时有，把酒问青天", "苏轼《水调歌头》")], "古汉语词典", 680),
    
    ("风", "fēng", [
        {"pos": "名", "definition": "风"},
        {"pos": "名", "definition": "风俗，风气"},
        {"pos": "名", "definition": "景象"}
    ], [("春风又绿江南岸", "王安石《泊船瓜洲》")], "古汉语词典", 580),
    
    ("雨", "yǔ", [
        {"pos": "名", "definition": "雨水"},
        {"pos": "动", "definition": "下雨"}
    ], [("好雨知时节", "杜甫《春夜喜雨》")], "古汉语词典", 520),
    
    # 人物称谓
    ("王", "wáng", [
        {"pos": "名", "definition": "君主，帝王"},
        {"pos": "动", "definition": "称王，统治"}
    ], [("王侯将相宁有种乎", "《史记》")], "古汉语词典", 550),
    
    ("臣", "chén", [
        {"pos": "名", "definition": "臣子，官员"},
        {"pos": "名", "definition": "百姓"}
    ], [("臣心一片磁针石", "文天祥")], "古汉语词典", 480),
    
    ("民", "mín", [
        {"pos": "名", "definition": "人民，百姓"},
        {"pos": "名", "definition": "平民"}
    ], [("民为贵，社稷次之，君为轻", "《孟子》")], "古汉语词典", 560),
    
    ("父", "fù", [
        {"pos": "名", "definition": "父亲"},
        {"pos": "名", "definition": "对男性长辈的尊称"}
    ], [("父慈子孝", "《礼记》")], "古汉语词典", 450),
    
    ("母", "mǔ", [
        {"pos": "名", "definition": "母亲"},
        {"pos": "名", "definition": "女性长辈"}
    ], [("慈母手中线，游子身上衣", "孟郊《游子吟》")], "古汉语词典", 440),
    
    ("子", "zǐ", [
        {"pos": "名", "definition": "儿子，子女"},
        {"pos": "代", "definition": "你"},
        {"pos": "名", "definition": "先生（尊称）"}
    ], [("子不语怪力乱神", "《论语》")], "古汉语词典", 520),
    
    # 动作行为
    ("行", "xíng", [
        {"pos": "动", "definition": "走，行走"},
        {"pos": "动", "definition": "做，实行"},
        {"pos": "名", "definition": "行为，品行"}
    ], [("三人行，必有我师焉", "《论语》")], "古汉语词典", 780),
    
    ("言", "yán", [
        {"pos": "名", "definition": "言语，话语"},
        {"pos": "动", "definition": "说"}
    ], [("言必信，行必果", "《论语》")], "古汉语词典", 620),
    
    ("视", "shì", [
        {"pos": "动", "definition": "看，观看"},
        {"pos": "动", "definition": "看待，对待"}
    ], [("视死如归", "《史记》")], "古汉语词典", 430),
    
    ("听", "tīng", [
        {"pos": "动", "definition": "听，聆听"},
        {"pos": "动", "definition": "听从，接受"}
    ], [("兼听则明，偏信则暗", "《资治通鉴》")], "古汉语词典", 450),
    
    ("食", "shí", [
        {"pos": "名", "definition": "食物"},
        {"pos": "动", "definition": "吃"},
        {"pos": "动", "definition": "喂养"}
    ], [("民以食为天", "《汉书》")], "古汉语词典", 520),
    
    ("居", "jū", [
        {"pos": "动", "definition": "居住"},
        {"pos": "动", "definition": "处于，位于"},
        {"pos": "名", "definition": "住所"}
    ], [("居安思危", "《左传》")], "古汉语词典", 460),
    
    # 品德修养
    ("德", "dé", [
        {"pos": "名", "definition": "道德，品德"},
        {"pos": "名", "definition": "恩德"}
    ], [("厚德载物", "《周易》")], "古汉语词典", 650),
    
    ("义", "yì", [
        {"pos": "名", "definition": "正义，道义"},
        {"pos": "名", "definition": "情谊"}
    ], [("舍生取义", "《孟子》")], "古汉语词典", 580),
    
    ("仁", "rén", [
        {"pos": "名", "definition": "仁爱，仁德"},
        {"pos": "名", "definition": "仁人"}
    ], [("仁者爱人", "《孟子》")], "古汉语词典", 560),
    
    ("礼", "lǐ", [
        {"pos": "名", "definition": "礼节，礼仪"},
        {"pos": "名", "definition": "规矩"}
    ], [("克己复礼", "《论语》")], "古汉语词典", 520),
    
    ("信", "xìn", [
        {"pos": "名", "definition": "信用，诚信"},
        {"pos": "动", "definition": "相信，信任"}
    ], [("人无信不立", "《论语》")], "古汉语词典", 540),
    
    ("智", "zhì", [
        {"pos": "名", "definition": "智慧，才智"},
        {"pos": "形", "definition": "明智的"}
    ], [("智者千虑，必有一失", "《史记》")], "古汉语词典", 480),
    
    ("勇", "yǒng", [
        {"pos": "名", "definition": "勇气，胆量"},
        {"pos": "形", "definition": "勇敢的"}
    ], [("勇者无惧", "《论语》")], "古汉语词典", 460),
    
    # 程度数量
    ("大", "dà", [
        {"pos": "形", "definition": "大的，巨大的"},
        {"pos": "副", "definition": "非常，很"}
    ], [("大智若愚", "《老子》")], "古汉语词典", 700),
    
    ("小", "xiǎo", [
        {"pos": "形", "definition": "小的，细小的"},
        {"pos": "名", "definition": "小事"}
    ], [("勿以恶小而为之，勿以善小而不为", "《三国志》")], "古汉语词典", 650),
    
    ("多", "duō", [
        {"pos": "形", "definition": "数量大的"},
        {"pos": "副", "definition": "多么"}
    ], [("多行不义必自毙", "《左传》")], "古汉语词典", 550),
    
    ("少", "shǎo", [
        {"pos": "形", "definition": "数量少的"},
        {"pos": "动", "definition": "缺少"}
    ], [("少壮不努力，老大徒伤悲", "《长歌行》")], "古汉语词典", 520),
    
    # 时间方位
    ("上", "shàng", [
        {"pos": "名", "definition": "上面，高处"},
        {"pos": "动", "definition": "登上，上升"},
        {"pos": "形", "definition": "上级的"}
    ], [("上善若水", "《道德经》")], "古汉语词典", 600),
    
    ("下", "xià", [
        {"pos": "名", "definition": "下面，低处"},
        {"pos": "动", "definition": "下降，落下"},
        {"pos": "名", "definition": "下属"}
    ], [("不耻下问", "《论语》")], "古汉语词典", 580),
    
    ("前", "qián", [
        {"pos": "名", "definition": "前面，前方"},
        {"pos": "名", "definition": "以前，从前"}
    ], [("前事不忘，后事之师", "《战国策》")], "古汉语词典", 500),
    
    ("后", "hòu", [
        {"pos": "名", "definition": "后面，后方"},
        {"pos": "名", "definition": "后来，以后"},
        {"pos": "名", "definition": "君主"}
    ], [("后来居上", "《史记》")], "古汉语词典", 490),
    
    ("今", "jīn", [
        {"pos": "名", "definition": "现在，当今"},
        {"pos": "副", "definition": "现在"}
    ], [("今朝有酒今朝醉", "罗隐《自遣》")], "古汉语词典", 450),
    
    ("古", "gǔ", [
        {"pos": "名", "definition": "古代"},
        {"pos": "形", "definition": "古老的"}
    ], [("古往今来", "《淮南子》")], "古汉语词典", 440),
    
    # 否定疑问
    ("非", "fēi", [
        {"pos": "副", "definition": "不，不是"},
        {"pos": "动", "definition": "非难，责怪"},
        {"pos": "名", "definition": "错误"}
    ], [("非淡泊无以明志", "诸葛亮《诫子书》")], "古汉语词典", 520),
    
    ("莫", "mò", [
        {"pos": "副", "definition": "不要，不可"},
        {"pos": "副", "definition": "没有谁"},
        {"pos": "副", "definition": "不"}
    ], [("莫等闲，白了少年头", "岳飞《满江红》")], "古汉语词典", 500),
    
    ("岂", "qǐ", [
        {"pos": "副", "definition": "难道，怎么"},
        {"pos": "副", "definition": "是否"}
    ], [("岂能尽如人意，但求无愧我心", "刘伯温")], "古汉语词典", 420),
    
    ("安", "ān", [
        {"pos": "形", "definition": "安定的，平稳的"},
        {"pos": "动", "definition": "使安定"},
        {"pos": "代", "definition": "哪里，怎么"}
    ], [("既来之，则安之", "《论语》")], "古汉语词典", 480),
    
    # 其他常用词
    ("道", "dào", [
        {"pos": "名", "definition": "道路"},
        {"pos": "名", "definition": "道理，规律"},
        {"pos": "名", "definition": "道德"},
        {"pos": "动", "definition": "说"}
    ], [("道可道，非常道", "《道德经》")], "古汉语词典", 750),
    
    ("理", "lǐ", [
        {"pos": "名", "definition": "道理，事理"},
        {"pos": "动", "definition": "整理，治理"},
        {"pos": "名", "definition": "纹理"}
    ], [("天理昭昭", "《朱子语类》")], "古汉语词典", 480),
    
    ("心", "xīn", [
        {"pos": "名", "definition": "心脏"},
        {"pos": "名", "definition": "思想，心思"},
        {"pos": "名", "definition": "中心"}
    ], [("心旷神怡", "范仲淹《岳阳楼记》")], "古汉语词典", 580),
    
    ("意", "yì", [
        {"pos": "名", "definition": "心意，意思"},
        {"pos": "名", "definition": "意志"},
        {"pos": "名", "definition": "情趣"}
    ], [("意在笔先", "《晋书》")], "古汉语词典", 450),
    
    ("气", "qì", [
        {"pos": "名", "definition": "气体，气息"},
        {"pos": "名", "definition": "精神，气势"},
        {"pos": "名", "definition": "气质"}
    ], [("气壮山河", "《正气歌》")], "古汉语词典", 480),
    
    ("志", "zhì", [
        {"pos": "名", "definition": "志向，志愿"},
        {"pos": "动", "definition": "记住"},
        {"pos": "名", "definition": "记载"}
    ], [("有志者事竟成", "《后汉书》")], "古汉语词典", 500),
    
    ("学", "xué", [
        {"pos": "动", "definition": "学习"},
        {"pos": "名", "definition": "学问"},
        {"pos": "名", "definition": "学校"}
    ], [("学而时习之，不亦说乎", "《论语》")], "古汉语词典", 1000),
    
    ("思", "sī", [
        {"pos": "动", "definition": "思考，想"},
        {"pos": "名", "definition": "思想"},
        {"pos": "名", "definition": "情思"}
    ], [("学而不思则罔", "《论语》")], "古汉语词典", 550),
    
    ("用", "yòng", [
        {"pos": "动", "definition": "使用，运用"},
        {"pos": "名", "definition": "用处，作用"},
        {"pos": "介", "definition": "因为"}
    ], [("学以致用", "《论语》")], "古汉语词典", 520),
    
    ("事", "shì", [
        {"pos": "名", "definition": "事情，事务"},
        {"pos": "动", "definition": "侍奉"},
        {"pos": "动", "definition": "做"}
    ], [("事在人为", "《红楼梦》")], "古汉语词典", 550),
    
    ("物", "wù", [
        {"pos": "名", "definition": "事物，东西"},
        {"pos": "名", "definition": "万物"},
        {"pos": "名", "definition": "内容"}
    ], [("格物致知", "《大学》")], "古汉语词典", 480),
    
    ("利", "lì", [
        {"pos": "名", "definition": "利益，好处"},
        {"pos": "形", "definition": "锋利的"},
        {"pos": "动", "definition": "有利于"}
    ], [("君子喻于义，小人喻于利", "《论语》")], "古汉语词典", 500),
    
    ("害", "hài", [
        {"pos": "名", "definition": "害处，祸害"},
        {"pos": "动", "definition": "伤害"},
        {"pos": "动", "definition": "害怕"}
    ], [("趋利避害", "《荀子》")], "古汉语词典", 420),
]

# 英汉词典扩展数据 - 高频词汇
ENGLISH_WORDS = [
    # 高频动词
    ("be", "/biː/", "v.", "是；存在", [("To be or not to be", "生存还是毁灭")], 1000),
    ("have", "/hæv/", "v.", "有；吃；让", [("I have a dream", "我有一个梦想")], 980),
    ("do", "/duː/", "v.", "做；干", [("Just do it", "想做就做")], 950),
    ("say", "/seɪ/", "v.", "说，讲", [("Easier said than done", "说起来容易做起来难")], 900),
    ("get", "/ɡet/", "v.", "得到；变得", [("Get started", "开始吧")], 880),
    ("make", "/meɪk/", "v.", "制造；使", [("Make it happen", "让它发生")], 860),
    ("go", "/ɡəʊ/", "v.", "去；走", [("Let it go", "随它去吧")], 850),
    ("know", "/nəʊ/", "v.", "知道；认识", [("Knowledge is power", "知识就是力量")], 840),
    ("take", "/teɪk/", "v.", "拿；取", [("Take your time", "慢慢来")], 820),
    ("see", "/siː/", "v.", "看见；理解", [("See you later", "回头见")], 800),
    ("come", "/kʌm/", "v.", "来；出现", [("Come true", "实现")], 780),
    ("think", "/θɪŋk/", "v.", "想；认为", [("Think twice", "三思而后行")], 760),
    ("look", "/lʊk/", "v.", "看；看起来", [("Look forward", "期待")], 740),
    ("want", "/wɒnt/", "v.", "想要；需要", [("I want you", "我需要你")], 720),
    ("give", "/ɡɪv/", "v.", "给；给予", [("Give up", "放弃")], 700),
    ("use", "/juːz/", "v.", "使用；利用", [("Use your head", "动动脑筋")], 680),
    ("find", "/faɪnd/", "v.", "找到；发现", [("Find out", "查明")], 660),
    ("tell", "/tel/", "v.", "告诉；说", [("Tell me why", "告诉我为什么")], 640),
    ("ask", "/ɑːsk/", "v.", "问；请求", [("Ask for help", "寻求帮助")], 620),
    ("work", "/wɜːk/", "v./n.", "工作；起作用", [("Work hard", "努力工作")], 600),
    
    # 高频名词
    ("time", "/taɪm/", "n.", "时间；次", [("Time flies", "光阴似箭")], 950),
    ("year", "/jɪə/", "n.", "年；年龄", [("New Year", "新年")], 850),
    ("people", "/ˈpiːpl/", "n.", "人；人们", [("People power", "人民力量")], 820),
    ("way", "/weɪ/", "n.", "方式；道路", [("This is the way", "就是这样")], 800),
    ("thing", "/θɪŋ/", "n.", "东西；事情", [("First things first", "重要的事情先做")], 780),
    ("man", "/mæn/", "n.", "男人；人类", [("Be a man", "做个男子汉")], 750),
    ("world", "/wɜːld/", "n.", "世界；地球", [("World peace", "世界和平")], 740),
    ("life", "/laɪf/", "n.", "生活；生命", [("Life is beautiful", "生活是美好的")], 720),
    ("hand", "/hænd/", "n./v.", "手；传递", [("Give me a hand", "帮我一下")], 700),
    ("part", "/pɑːt/", "n.", "部分；角色", [("Part of", "一部分")], 680),
    ("child", "/tʃaɪld/", "n.", "孩子", [("Every child is unique", "每个孩子都是独特的")], 660),
    ("eye", "/aɪ/", "n.", "眼睛", [("Keep an eye on", "留意")], 640),
    ("place", "/pleɪs/", "n.", "地方；位置", [("Take place", "发生")], 620),
    ("week", "/wiːk/", "n.", "周；星期", [("Day of the week", "工作日")], 600),
    ("case", "/keɪs/", "n.", "情况；案例", [("In case of", "万一")], 580),
    
    # 高频形容词
    ("good", "/ɡʊd/", "adj.", "好的；优秀的", [("Good luck", "祝好运")], 900),
    ("new", "/njuː/", "adj.", "新的；新鲜的", [("New beginning", "新的开始")], 850),
    ("first", "/fɜːst/", "adj./adv.", "第一的；首先", [("First love", "初恋")], 800),
    ("last", "/lɑːst/", "adj./adv.", "最后的；上一个", [("Last but not least", "最后但同样重要")], 750),
    ("long", "/lɒŋ/", "adj./adv.", "长的；长久", [("Long time no see", "好久不见")], 720),
    ("great", "/ɡreɪt/", "adj.", "伟大的；极好的", [("Great minds think alike", "英雄所见略同")], 700),
    ("little", "/ˈlɪtl/", "adj.", "小的；少的", [("Little by little", "一点一点地")], 680),
    ("own", "/əʊn/", "adj./v.", "自己的；拥有", [("On my own", "靠我自己")], 660),
    ("other", "/ˈʌðə/", "adj./pron.", "其他的；别人", [("Each other", "彼此")], 640),
    ("old", "/əʊld/", "adj.", "旧的；年老的", [("Old friend", "老朋友")], 620),
    ("right", "/raɪt/", "adj./n.", "正确的；权利", [("Right choice", "正确的选择")], 600),
    ("big", "/bɪɡ/", "adj.", "大的；重要的", [("Big dream", "大梦想")], 580),
    ("high", "/haɪ/", "adj./adv.", "高的；高度", [("Aim high", "志存高远")], 560),
    ("different", "/ˈdɪfrənt/", "adj.", "不同的", [("Different strokes", "各有所好")], 540),
    ("small", "/smɔːl/", "adj.", "小的；少的", [("Small world", "世界真小")], 520),
    
    # 高频副词
    ("not", "/nɒt/", "adv.", "不", [("Not at all", "一点也不")], 980),
    ("only", "/ˈəʊnli/", "adv./adj.", "只；唯一的", [("Only you", "只有你")], 800),
    ("then", "/ðen/", "adv.", "那么；然后", [("Now and then", "偶尔")], 750),
    ("also", "/ˈɔːlsəʊ/", "adv.", "也；同样", [("Also known as", "也称为")], 700),
    ("very", "/ˈveri/", "adv.", "非常；很", [("Very much", "非常")], 680),
    ("even", "/ˈiːvn/", "adv.", "甚至；更加", [("Even better", "甚至更好")], 660),
    ("just", "/dʒʌst/", "adv.", "只是；刚才", [("Just do it", "想做就做")], 640),
    ("well", "/wel/", "adv./adj.", "好；健康的", [("Very well", "很好")], 620),
    ("back", "/bæk/", "adv./n.", "向后；后面", [("Back to back", "背靠背")], 600),
    ("still", "/stɪl/", "adv.", "仍然；静止", [("Still alive", "还活着")], 580),
    ("here", "/hɪə/", "adv.", "这里", [("Here you go", "给你")], 560),
    ("there", "/ðeə/", "adv.", "那里", [("Over there", "在那边")], 540),
    ("now", "/naʊ/", "adv.", "现在", [("Right now", "现在")], 520),
    ("too", "/tuː/", "adv.", "太；也", [("Me too", "我也是")], 500),
    ("again", "/əˈɡen/", "adv.", "再次", [("Try again", "再试一次")], 480),
]


def create_expanded_database(db_path: str):
    """创建扩展的词典数据库"""
    conn = sqlite3.connect(db_path)
    cursor = conn.cursor()
    
    # 创建表
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
    
    cursor.execute('''
        CREATE TABLE IF NOT EXISTS english_dict (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            word TEXT NOT NULL,
            phonetic TEXT,
            pos TEXT,
            definition TEXT NOT NULL,
            examples TEXT,
            frequency INTEGER DEFAULT 0
        )
    ''')
    
    # 创建索引
    cursor.execute('CREATE INDEX IF NOT EXISTS idx_ancient_word ON ancient_dict(word)')
    cursor.execute('CREATE INDEX IF NOT EXISTS idx_english_word ON english_dict(word)')
    
    # 插入古汉语词典数据
    for word, pinyin, definitions, examples, source, frequency in ANCIENT_WORDS:
        definition_json = json.dumps(definitions, ensure_ascii=False)
        examples_json = json.dumps(examples, ensure_ascii=False)
        
        cursor.execute('''
            INSERT OR REPLACE INTO ancient_dict (word, pinyin, definition, examples, source, frequency)
            VALUES (?, ?, ?, ?, ?, ?)
        ''', (word, pinyin, definition_json, examples_json, source, frequency))
    
    # 插入英汉词典数据
    for word, phonetic, pos, definition, examples, frequency in ENGLISH_WORDS:
        examples_json = json.dumps(examples, ensure_ascii=False)
        
        cursor.execute('''
            INSERT OR REPLACE INTO english_dict (word, phonetic, pos, definition, examples, frequency)
            VALUES (?, ?, ?, ?, ?, ?)
        ''', (word, phonetic, pos, definition, examples_json, frequency))
    
    conn.commit()
    
    # 统计
    ancient_count = cursor.execute('SELECT COUNT(*) FROM ancient_dict').fetchone()[0]
    english_count = cursor.execute('SELECT COUNT(*) FROM english_dict').fetchone()[0]
    
    conn.close()
    
    return ancient_count, english_count


def main():
    # 数据库路径
    script_dir = Path(__file__).parent
    db_path = script_dir.parent / "dictionaries" / "expanded_dict.db"
    
    print(f"创建扩展词典数据库: {db_path}")
    
    ancient_count, english_count = create_expanded_database(str(db_path))
    
    print(f"\n✅ 词典扩展完成!")
    print(f"   古汉语词典: {ancient_count} 词")
    print(f"   英汉词典: {english_count} 词")
    print(f"\n数据库文件: {db_path}")


if __name__ == "__main__":
    main()