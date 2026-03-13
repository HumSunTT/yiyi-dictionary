// 数据库操作模块

use rusqlite::{Connection, Result as SqliteResult};
use std::path::PathBuf;
use std::sync::Mutex;
use tauri::AppHandle;
use tauri::Manager;

use crate::models::{DefinitionItem, DictionaryResult, ExampleItem, HistoryItem, VocabularyItem};

pub struct Database {
    conn: Mutex<Connection>,
}

impl Database {
    pub fn new(app_handle: &AppHandle) -> SqliteResult<Self> {
        let app_dir = app_handle
            .path()
            .app_data_dir()
            .expect("无法获取应用数据目录");
        std::fs::create_dir_all(&app_dir).ok();

        let db_path = PathBuf::from(&app_dir).join("yi_yi.db");

        // 尝试从打包资源复制词典数据
        let needs_init = !db_path.exists();
        if needs_init {
            // 尝试复制预置词典数据库
            if let Ok(resource_path) = app_handle
                .path()
                .resolve("dict_data.db", tauri::path::BaseDirectory::Resource)
            {
                if resource_path.exists() {
                    if let Err(e) = std::fs::copy(&resource_path, &db_path) {
                        log::warn!("无法复制词典资源: {}", e);
                    } else {
                        log::info!("词典数据库已从资源复制");
                    }
                }
            }
        }

        let conn = Connection::open(&db_path)?;

        let db = Database {
            conn: Mutex::new(conn),
        };

        db.initialize()?;
        Ok(db)
    }

    fn initialize(&self) -> SqliteResult<()> {
        let conn = self.conn.lock().unwrap();

        // 创建历史记录表
        conn.execute(
            "CREATE TABLE IF NOT EXISTS history (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                query TEXT NOT NULL,
                query_type TEXT,
                result TEXT,
                source TEXT,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP
            )",
            [],
        )?;

        // 创建生词本表
        conn.execute(
            "CREATE TABLE IF NOT EXISTS vocabulary (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                word TEXT NOT NULL,
                word_type TEXT,
                definition TEXT,
                note TEXT,
                review_count INTEGER DEFAULT 0,
                next_review DATE,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP
            )",
            [],
        )?;

        // 创建古汉语词典表
        conn.execute(
            "CREATE TABLE IF NOT EXISTS ancient_dict (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                word TEXT NOT NULL,
                pinyin TEXT,
                definition TEXT NOT NULL,
                examples TEXT,
                source TEXT,
                frequency INTEGER DEFAULT 0,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP
            )",
            [],
        )?;

        // 创建英汉词典表
        conn.execute(
            "CREATE TABLE IF NOT EXISTS english_dict (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                word TEXT NOT NULL,
                phonetic TEXT,
                pos TEXT,
                definition TEXT NOT NULL,
                examples TEXT,
                frequency INTEGER DEFAULT 0,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP
            )",
            [],
        )?;

        // 创建中英词典表
        conn.execute(
            "CREATE TABLE IF NOT EXISTS chinese_dict (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                word TEXT NOT NULL,
                pinyin TEXT,
                definition TEXT NOT NULL,
                examples TEXT,
                frequency INTEGER DEFAULT 0,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP
            )",
            [],
        )?;

        // 创建索引
        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_ancient_word ON ancient_dict(word)",
            [],
        )?;
        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_english_word ON english_dict(word)",
            [],
        )?;
        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_chinese_word ON chinese_dict(word)",
            [],
        )?;
        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_history_time ON history(created_at DESC)",
            [],
        )?;

        // 检查是否需要导入示例数据
        let ancient_count: i64 = conn
            .query_row("SELECT COUNT(*) FROM ancient_dict", [], |row| row.get(0))
            .unwrap_or(0);
        let english_count: i64 = conn
            .query_row("SELECT COUNT(*) FROM english_dict", [], |row| row.get(0))
            .unwrap_or(0);
        let chinese_count: i64 = conn
            .query_row("SELECT COUNT(*) FROM chinese_dict", [], |row| row.get(0))
            .unwrap_or(0);

        // 如果任何一个词典表为空，重新导入所有示例数据
        if ancient_count == 0 || english_count == 0 || chinese_count == 0 {
            drop(conn);
            self.import_sample_data()?;
        }

        Ok(())
    }

    /// 导入示例数据
    fn import_sample_data(&self) -> SqliteResult<()> {
        let conn = self.conn.lock().unwrap();

        // 古汉语词典示例数据
        let ancient_words = vec![
            (
                "哀",
                "āi",
                r#"[{"pos":"动","definition":"悲伤；哀悼"},{"pos":"形","definition":"悲哀的"}]"#,
                r#"[{"text":"哀民生之多艰","source":"《离骚》"},{"text":"哀而不伤","source":"《论语》"}]"#,
                "古汉语词典",
                300,
            ),
            (
                "学",
                "xué",
                r#"[{"pos":"动","definition":"学习"},{"pos":"名","definition":"学问"},{"pos":"名","definition":"学校"}]"#,
                r#"[{"text":"学而时习之，不亦说乎","source":"《论语》"},{"text":"学而不思则罔，思而不学则殆","source":"《论语》"}]"#,
                "古汉语词典",
                1000,
            ),
            (
                "之",
                "zhī",
                r#"[{"pos":"代","definition":"他/她/它"},{"pos":"助","definition":"的"},{"pos":"动","definition":"到...去"}]"#,
                r#"[{"text":"学而时习之","source":"《论语》"},{"text":"之子于归","source":"《诗经》"}]"#,
                "古汉语词典",
                900,
            ),
            (
                "者",
                "zhě",
                r#"[{"pos":"助","definition":"...的人"},{"pos":"助","definition":"...的事物"}]"#,
                r#"[{"text":"知者乐水，仁者乐山","source":"《论语》"}]"#,
                "古汉语词典",
                800,
            ),
            (
                "也",
                "yě",
                r#"[{"pos":"助","definition":"句末语气词，表示判断或解释"}]"#,
                r#"[{"text":"是吾剑之所从坠","source":"《吕氏春秋》"}]"#,
                "古汉语词典",
                850,
            ),
            (
                "乎",
                "hū",
                r#"[{"pos":"助","definition":"句末语气词，表示疑问"},{"pos":"助","definition":"句末语气词，表示感叹"}]"#,
                r#"[{"text":"学而时习之，不亦说乎","source":"《论语》"}]"#,
                "古汉语词典",
                700,
            ),
            (
                "矣",
                "yǐ",
                r#"[{"pos":"助","definition":"句末语气词，表示完成或肯定"}]"#,
                r#"[{"text":"温故而知新，可以为师矣","source":"《论语》"}]"#,
                "古汉语词典",
                650,
            ),
            (
                "曰",
                "yuē",
                r#"[{"pos":"动","definition":"说"}]"#,
                r#"[{"text":"子曰：学而时习之","source":"《论语》"}]"#,
                "古汉语词典",
                600,
            ),
            (
                "于",
                "yú",
                r#"[{"pos":"介","definition":"在"},{"pos":"介","definition":"向"},{"pos":"介","definition":"对于"}]"#,
                r#"[{"text":"己所不欲，勿施于人","source":"《论语》"}]"#,
                "古汉语词典",
                750,
            ),
            (
                "以",
                "yǐ",
                r#"[{"pos":"介","definition":"用"},{"pos":"介","definition":"因为"},{"pos":"连","definition":"而"}]"#,
                r#"[{"text":"以直报怨，以德报德","source":"《论语》"}]"#,
                "古汉语词典",
                720,
            ),
            (
                "而",
                "ér",
                r#"[{"pos":"连","definition":"并且"},{"pos":"连","definition":"却"},{"pos":"连","definition":"如果"}]"#,
                r#"[{"text":"学而时习之","source":"《论语》"}]"#,
                "古汉语词典",
                880,
            ),
            // 更多古汉语常用词
            (
                "吾",
                "wú",
                r#"[{"pos":"代","definition":"我，我的"}]"#,
                r#"[{"text":"吾日三省吾身","source":"《论语》"}]"#,
                "古汉语词典",
                400,
            ),
            (
                "汝",
                "rǔ",
                r#"[{"pos":"代","definition":"你，你的"}]"#,
                r#"[{"text":"吾与汝毕力平险","source":"《列子》"}]"#,
                "古汉语词典",
                380,
            ),
            (
                "何",
                "hé",
                r#"[{"pos":"代","definition":"什么"},{"pos":"副","definition":"多么"}]"#,
                r#"[{"text":"何陋之有","source":"《陋室铭》"}]"#,
                "古汉语词典",
                450,
            ),
            (
                "乃",
                "nǎi",
                r#"[{"pos":"副","definition":"于是，就"},{"pos":"副","definition":"是，为"},{"pos":"代","definition":"你，你的"}]"#,
                r#"[{"text":"乃不知有汉","source":"《桃花源记》"}]"#,
                "古汉语词典",
                360,
            ),
            (
                "若",
                "ruò",
                r#"[{"pos":"连","definition":"如果"},{"pos":"代","definition":"你"},{"pos":"动","definition":"像，如同"}]"#,
                r#"[{"text":"若为佣耕，何富贵也","source":"《史记》"}]"#,
                "古汉语词典",
                350,
            ),
            (
                "则",
                "zé",
                r#"[{"pos":"连","definition":"那么，就"},{"pos":"连","definition":"却"},{"pos":"名","definition":"法则，标准"}]"#,
                r#"[{"text":"学而不思则罔","source":"《论语》"}]"#,
                "古汉语词典",
                420,
            ),
            (
                "虽",
                "suī",
                r#"[{"pos":"连","definition":"虽然"},{"pos":"连","definition":"即使"}]"#,
                r#"[{"text":"虽人有百手，手有百指","source":"《口技》"}]"#,
                "古汉语词典",
                340,
            ),
            (
                "故",
                "gù",
                r#"[{"pos":"连","definition":"所以"},{"pos":"名","definition":"原因"},{"pos":"形","definition":"旧的"},{"pos":"副","definition":"特意"}]"#,
                r#"[{"text":"温故而知新","source":"《论语》"}]"#,
                "古汉语词典",
                440,
            ),
            (
                "既",
                "jì",
                r#"[{"pos":"副","definition":"已经"},{"pos":"连","definition":"既然"},{"pos":"副","definition":"不久"}]"#,
                r#"[{"text":"既来之，则安之","source":"《论语》"}]"#,
                "古汉语词典",
                320,
            ),
            (
                "且",
                "qiě",
                r#"[{"pos":"连","definition":"而且"},{"pos":"副","definition":"将要"},{"pos":"副","definition":"暂且"}]"#,
                r#"[{"text":"且壮士不死即已","source":"《史记》"}]"#,
                "古汉语词典",
                310,
            ),
            (
                "遂",
                "suì",
                r#"[{"pos":"副","definition":"于是，就"},{"pos":"副","definition":"终于"}]"#,
                r#"[{"text":"遂迷，不复得路","source":"《桃花源记》"}]"#,
                "古汉语词典",
                300,
            ),
            (
                "盖",
                "gài",
                r#"[{"pos":"副","definition":"大概"},{"pos":"连","definition":"因为"},{"pos":"动","definition":"超过"}]"#,
                r#"[{"text":"盖余之勤且艰若此","source":"《送东阳马生序》"}]"#,
                "古汉语词典",
                290,
            ),
            (
                "然",
                "rán",
                r#"[{"pos":"代","definition":"这样"},{"pos":"连","definition":"但是"},{"pos":"动","definition":"对，正确"}]"#,
                r#"[{"text":"然而不胜者，是天时不如地利也","source":"《孟子》"}]"#,
                "古汉语词典",
                330,
            ),
            (
                "或",
                "huò",
                r#"[{"pos":"代","definition":"有的"},{"pos":"副","definition":"或许"},{"pos":"连","definition":"或者"}]"#,
                r#"[{"text":"或以为死，或以为亡","source":"《史记》"}]"#,
                "古汉语词典",
                280,
            ),
        ];

        for (word, pinyin, definition, examples, source, frequency) in ancient_words {
            conn.execute(
                "INSERT INTO ancient_dict (word, pinyin, definition, examples, source, frequency) VALUES (?, ?, ?, ?, ?, ?)",
                rusqlite::params![word, pinyin, definition, examples, source, frequency],
            ).ok();
        }

        // 英汉词典示例数据
        let english_words = vec![
            (
                "hello",
                "/həˈləʊ/",
                "int.",
                "你好；喂",
                r#"[{"text":"Hello, how are you?","translation":"你好，你怎么样？"}]"#,
                1000,
            ),
            (
                "world",
                "/wɜːld/",
                "n.",
                "世界；地球",
                r#"[{"text":"The world is beautiful.","translation":"世界是美丽的。"}]"#,
                950,
            ),
            (
                "book",
                "/bʊk/",
                "n.",
                "书",
                r#"[{"text":"I am reading a book.","translation":"我正在读一本书。"}]"#,
                900,
            ),
            (
                "time",
                "/taɪm/",
                "n.",
                "时间；次",
                r#"[{"text":"Time flies.","translation":"光阴似箭。"}]"#,
                880,
            ),
            (
                "people",
                "/ˈpiːpl/",
                "n.",
                "人；人们",
                r#"[{"text":"People are friendly here.","translation":"这里的人们很友好。"}]"#,
                850,
            ),
            (
                "know",
                "/nəʊ/",
                "v.",
                "知道；认识",
                r#"[{"text":"I know him well.","translation":"我很了解他。"}]"#,
                800,
            ),
            (
                "think",
                "/θɪŋk/",
                "v.",
                "想；认为",
                r#"[{"text":"I think so.","translation":"我也这么认为。"}]"#,
                780,
            ),
            (
                "good",
                "/ɡʊd/",
                "adj.",
                "好的",
                r#"[{"text":"Good morning!","translation":"早上好！"}]"#,
                720,
            ),
            (
                "new",
                "/njuː/",
                "adj.",
                "新的",
                r#"[{"text":"Happy New Year!","translation":"新年快乐！"}]"#,
                700,
            ),
            (
                "great",
                "/ɡreɪt/",
                "adj.",
                "伟大的；极好的",
                r#"[{"text":"That's great!","translation":"太棒了！"}]"#,
                680,
            ),
            // 更多常用英文单词
            (
                "work",
                "/wɜːk/",
                "v./n.",
                "工作；起作用",
                r#"[{"text":"Hard work pays off.","translation":"努力工作会有回报。"}]"#,
                660,
            ),
            (
                "life",
                "/laɪf/",
                "n.",
                "生活；生命",
                r#"[{"text":"Life is short.","translation":"人生苦短。"}]"#,
                640,
            ),
            (
                "love",
                "/lʌv/",
                "v./n.",
                "爱；喜爱",
                r#"[{"text":"I love you.","translation":"我爱你。"}]"#,
                950,
            ),
            (
                "home",
                "/həʊm/",
                "n.",
                "家；家庭",
                r#"[{"text":"Home sweet home.","translation":"家是温馨的港湾。"}]"#,
                620,
            ),
            (
                "hand",
                "/hænd/",
                "n./v.",
                "手；传递",
                r#"[{"text":"Give me a hand.","translation":"帮我一下。"}]"#,
                600,
            ),
            (
                "part",
                "/pɑːt/",
                "n.",
                "部分；角色",
                r#"[{"text":"Part of the problem.","translation":"问题的一部分。"}]"#,
                580,
            ),
            (
                "child",
                "/tʃaɪld/",
                "n.",
                "孩子",
                r#"[{"text":"Every child is unique.","translation":"每个孩子都是独特的。"}]"#,
                560,
            ),
            (
                "eye",
                "/aɪ/",
                "n.",
                "眼睛",
                r#"[{"text":"Keep an eye on it.","translation":"留意一下。"}]"#,
                540,
            ),
            (
                "way",
                "/weɪ/",
                "n.",
                "方式；道路",
                r#"[{"text":"This is the way.","translation":"就是这样。"}]"#,
                700,
            ),
            (
                "day",
                "/deɪ/",
                "n.",
                "天；白天",
                r#"[{"text":"Have a nice day!","translation":"祝你有美好的一天！"}]"#,
                680,
            ),
            (
                "man",
                "/mæn/",
                "n.",
                "男人；人类",
                r#"[{"text":"Be a man.","translation":"做个男子汉。"}]"#,
                520,
            ),
            (
                "woman",
                "/ˈwʊmən/",
                "n.",
                "女人",
                r#"[{"text":"A strong woman.","translation":"一个坚强的女人。"}]"#,
                500,
            ),
            (
                "thing",
                "/θɪŋ/",
                "n.",
                "东西；事情",
                r#"[{"text":"One thing at a time.","translation":"一次做一件事。"}]"#,
                480,
            ),
            (
                "place",
                "/pleɪs/",
                "n./v.",
                "地方；放置",
                r#"[{"text":"This is my place.","translation":"这是我的地盘。"}]"#,
                460,
            ),
        ];

        for (word, phonetic, pos, definition, examples, frequency) in english_words {
            conn.execute(
                "INSERT INTO english_dict (word, phonetic, pos, definition, examples, frequency) VALUES (?, ?, ?, ?, ?, ?)",
                rusqlite::params![word, phonetic, pos, definition, examples, frequency],
            ).ok();
        }

        // 中英词典示例数据
        let chinese_words = vec![
            (
                "香蕉",
                "xiāng jiāo",
                r#"[{"pos":"n.","definition":"banana"}]"#,
                r#"[{"text":"我喜欢吃香蕉。","translation":"I like to eat bananas."}]"#,
                1000,
            ),
            (
                "苹果",
                "píng guǒ",
                r#"[{"pos":"n.","definition":"apple"}]"#,
                r#"[{"text":"每天一个苹果。","translation":"An apple a day."}]"#,
                950,
            ),
            (
                "你好",
                "nǐ hǎo",
                r#"[{"pos":"int.","definition":"hello"}]"#,
                r#"[{"text":"你好，很高兴见到你。","translation":"Hello, nice to meet you."}]"#,
                900,
            ),
            (
                "谢谢",
                "xiè xie",
                r#"[{"pos":"v.","definition":"thank you"}]"#,
                r#"[{"text":"谢谢你的帮助。","translation":"Thank you for your help."}]"#,
                880,
            ),
            (
                "再见",
                "zài jiàn",
                r#"[{"pos":"int.","definition":"goodbye"}]"#,
                r#"[{"text":"再见，明天见。","translation":"Goodbye, see you tomorrow."}]"#,
                850,
            ),
            (
                "是",
                "shì",
                r#"[{"pos":"v.","definition":"yes; to be"}]"#,
                r#"[{"text":"这是正确的。","translation":"This is correct."}]"#,
                800,
            ),
            (
                "不",
                "bù",
                r#"[{"pos":"adv.","definition":"no; not"}]"#,
                r#"[{"text":"我不明白。","translation":"I don't understand."}]"#,
                780,
            ),
            (
                "好",
                "hǎo",
                r#"[{"pos":"adj.","definition":"good"}]"#,
                r#"[{"text":"今天天气很好。","translation":"The weather is very good today."}]"#,
                720,
            ),
            (
                "人",
                "rén",
                r#"[{"pos":"n.","definition":"person; people"}]"#,
                r#"[{"text":"人人平等。","translation":"Everyone is equal."}]"#,
                700,
            ),
            (
                "大",
                "dà",
                r#"[{"pos":"adj.","definition":"big; large"}]"#,
                r#"[{"text":"这是一个大问题。","translation":"This is a big problem."}]"#,
                680,
            ),
            (
                "小",
                "xiǎo",
                r#"[{"pos":"adj.","definition":"small; little"}]"#,
                r#"[{"text":"小明是个好学生。","translation":"Xiao Ming is a good student."}]"#,
                660,
            ),
            (
                "中国",
                "zhōng guó",
                r#"[{"pos":"n.","definition":"China"}]"#,
                r#"[{"text":"我来自中国。","translation":"I am from China."}]"#,
                950,
            ),
            (
                "北京",
                "běi jīng",
                r#"[{"pos":"n.","definition":"Beijing"}]"#,
                r#"[{"text":"北京是中国的首都。","translation":"Beijing is the capital of China."}]"#,
                900,
            ),
            (
                "上海",
                "shàng hǎi",
                r#"[{"pos":"n.","definition":"Shanghai"}]"#,
                r#"[{"text":"上海是一个大城市。","translation":"Shanghai is a big city."}]"#,
                850,
            ),
            (
                "水",
                "shuǐ",
                r#"[{"pos":"n.","definition":"water"}]"#,
                r#"[{"text":"水是生命之源。","translation":"Water is the source of life."}]"#,
                640,
            ),
            (
                "火",
                "huǒ",
                r#"[{"pos":"n.","definition":"fire"}]"#,
                r#"[{"text":"小心火。","translation":"Be careful with fire."}]"#,
                620,
            ),
            (
                "山",
                "shān",
                r#"[{"pos":"n.","definition":"mountain"}]"#,
                r#"[{"text":"那座山很高。","translation":"That mountain is very high."}]"#,
                600,
            ),
            (
                "河",
                "hé",
                r#"[{"pos":"n.","definition":"river"}]"#,
                r#"[{"text":"黄河是中国的母亲河。","translation":"The Yellow River is China's mother river."}]"#,
                580,
            ),
            (
                "天",
                "tiān",
                r#"[{"pos":"n.","definition":"sky; heaven"}]"#,
                r#"[{"text":"今天的天气很好。","translation":"The weather is very good today."}]"#,
                560,
            ),
            (
                "地",
                "dì",
                r#"[{"pos":"n.","definition":"earth; ground"}]"#,
                r#"[{"text":"土地很肥沃。","translation":"The land is very fertile."}]"#,
                540,
            ),
        ];

        for (word, pinyin, definition, examples, frequency) in chinese_words {
            conn.execute(
                "INSERT INTO chinese_dict (word, pinyin, definition, examples, frequency) VALUES (?, ?, ?, ?, ?)",
                rusqlite::params![word, pinyin, definition, examples, frequency],
            ).ok();
        }

        log::info!("示例数据导入成功");
        Ok(())
    }

    /// 查询古汉语词典
    pub fn query_ancient(&self, word: &str) -> Option<DictionaryResult> {
        let conn = self.conn.lock().unwrap();

        let mut stmt = conn.prepare(
            "SELECT word, pinyin, definition, examples, source FROM ancient_dict WHERE word = ? LIMIT 1"
        ).ok()?;

        let result = stmt
            .query_row([word], |row| {
                Ok((
                    row.get::<_, String>(0)?,
                    row.get::<_, Option<String>>(1)?,
                    row.get::<_, String>(2)?,
                    row.get::<_, Option<String>>(3)?,
                    row.get::<_, Option<String>>(4)?,
                ))
            })
            .ok()?;

        let (w, pinyin, definition, examples, source) = result;

        // 解析释义 JSON
        let definitions: Vec<DefinitionItem> =
            serde_json::from_str(&definition).unwrap_or_default();

        // 解析例句 JSON
        let examples: Option<Vec<ExampleItem>> =
            examples.and_then(|e| serde_json::from_str(&e).ok());

        Some(DictionaryResult {
            r#type: "dictionary".to_string(),
            word: w,
            phonetic: pinyin,
            source,
            definitions,
            examples,
        })
    }

    /// 查询古汉语词典（返回所有来源的结果，古汉语常用字字典优先）
    pub fn query_ancient_all(&self, word: &str) -> Result<Vec<DictionaryResult>, rusqlite::Error> {
        let conn = self.conn.lock().unwrap();

        let mut stmt = conn.prepare(
            "SELECT word, pinyin, definition, examples, source FROM ancient_dict WHERE word = ?",
        )?;

        let results = stmt.query_map([word], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, Option<String>>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, Option<String>>(3)?,
                row.get::<_, Option<String>>(4)?,
            ))
        })?;

        let mut dictionary_results = Vec::new();
        for result in results {
            let (w, pinyin, definition, examples, source) = result?;

            let definitions: Vec<DefinitionItem> =
                serde_json::from_str(&definition).unwrap_or_default();

            let examples: Option<Vec<ExampleItem>> =
                examples.and_then(|e| serde_json::from_str(&e).ok());

            dictionary_results.push(DictionaryResult {
                r#type: "dictionary".to_string(),
                word: w,
                phonetic: pinyin,
                source,
                definitions,
                examples,
            });
        }

        dictionary_results.sort_by(|a, b| {
            fn source_priority(source: &Option<String>) -> u8 {
                match source.as_deref() {
                    Some("古汉语词典") => 0,
                    Some("古汉语常用字字典") => 1,
                    Some("康熙字典") => 2,
                    _ => 3,
                }
            }
            source_priority(&a.source).cmp(&source_priority(&b.source))
        });

        Ok(dictionary_results)
    }

    /// 查询英汉词典
    pub fn query_english(&self, word: &str) -> Option<DictionaryResult> {
        let conn = self.conn.lock().unwrap();

        let mut stmt = conn.prepare(
            "SELECT word, phonetic, pos, definition, examples FROM english_dict WHERE word = ? COLLATE NOCASE LIMIT 1"
        ).ok()?;

        let result = stmt
            .query_row([word], |row| {
                Ok((
                    row.get::<_, String>(0)?,
                    row.get::<_, Option<String>>(1)?,
                    row.get::<_, Option<String>>(2)?,
                    row.get::<_, String>(3)?,
                    row.get::<_, Option<String>>(4)?,
                ))
            })
            .ok()?;

        let (w, phonetic, pos, definition, examples) = result;

        // 解析释义 JSON
        let definitions: Vec<DefinitionItem> = if let Some(p) = pos {
            vec![DefinitionItem { pos: p, definition }]
        } else {
            serde_json::from_str(&definition).unwrap_or_default()
        };

        // 解析例句 JSON
        let examples: Option<Vec<ExampleItem>> =
            examples.and_then(|e| serde_json::from_str(&e).ok());

        Some(DictionaryResult {
            r#type: "dictionary".to_string(),
            word: w,
            phonetic,
            source: Some("英汉词典".to_string()),
            definitions,
            examples,
        })
    }

    /// 查询中英词典
    pub fn query_chinese(&self, word: &str) -> Option<DictionaryResult> {
        let conn = self.conn.lock().unwrap();

        let mut stmt = conn.prepare(
            "SELECT word, pinyin, definition, examples FROM chinese_dict WHERE word = ? LIMIT 1"
        ).ok()?;

        let result = stmt
            .query_row([word], |row| {
                Ok((
                    row.get::<_, String>(0)?,
                    row.get::<_, Option<String>>(1)?,
                    row.get::<_, String>(2)?,
                    row.get::<_, Option<String>>(3)?,
                ))
            })
            .ok()?;

        let (w, pinyin, definition, examples) = result;

        let definitions: Vec<DefinitionItem> =
            serde_json::from_str(&definition).unwrap_or_default();

        let examples: Option<Vec<ExampleItem>> =
            examples.and_then(|e| serde_json::from_str(&e).ok());

        Some(DictionaryResult {
            r#type: "dictionary".to_string(),
            word: w,
            phonetic: pinyin,
            source: Some("中英词典".to_string()),
            definitions,
            examples,
        })
    }

    /// 从英汉词典反向查询中文词
    pub fn query_english_by_chinese(&self, chinese_word: &str) -> Vec<DictionaryResult> {
        let conn = self.conn.lock().unwrap();

        let mut stmt = match conn.prepare(
            "SELECT word, phonetic, pos, definition, examples FROM english_dict WHERE definition LIKE ? OR definition LIKE ? LIMIT 5"
        ) {
            Ok(s) => s,
            Err(_) => return Vec::new(),
        };

        let pattern1 = format!("{}%", chinese_word);
        let pattern2 = format!("%\n{}%", chinese_word);

        let results: Vec<_> = stmt
            .query_map(rusqlite::params![&pattern1, &pattern2], |row| {
                Ok((
                    row.get::<_, String>(0)?,
                    row.get::<_, Option<String>>(1)?,
                    row.get::<_, Option<String>>(2)?,
                    row.get::<_, String>(3)?,
                    row.get::<_, Option<String>>(4)?,
                ))
            })
            .ok()
            .map(|r| r.collect::<Vec<_>>())
            .unwrap_or_default();

        let mut dictionary_results = Vec::new();
        let mut seen_definitions = std::collections::HashSet::new();

        for result in results.into_iter().flatten() {
            let (w, phonetic, pos, definition, examples) = result;

            let first_line = definition
                .split('\n')
                .next()
                .unwrap_or(&definition)
                .to_string();

            if seen_definitions.contains(&first_line) {
                continue;
            }
            seen_definitions.insert(first_line.clone());

            let definitions: Vec<DefinitionItem> = if let Some(p) = pos {
                vec![DefinitionItem {
                    pos: p,
                    definition: first_line,
                }]
            } else {
                vec![DefinitionItem {
                    pos: String::new(),
                    definition: first_line,
                }]
            };

            let examples: Option<Vec<ExampleItem>> =
                examples.and_then(|e| serde_json::from_str(&e).ok());

            dictionary_results.push(DictionaryResult {
                r#type: "dictionary".to_string(),
                word: w,
                phonetic,
                source: Some("英汉词典".to_string()),
                definitions,
                examples,
            });

            if dictionary_results.len() >= 3 {
                break;
            }
        }

        dictionary_results
    }

    /// 添加历史记录
    pub fn add_history(
        &self,
        query: &str,
        query_type: &str,
        result: &str,
        source: &str,
    ) -> SqliteResult<i64> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO history (query, query_type, result, source) VALUES (?, ?, ?, ?)",
            [query, query_type, result, source],
        )?;
        Ok(conn.last_insert_rowid())
    }

    /// 获取历史记录
    pub fn get_history(&self, limit: i32) -> SqliteResult<Vec<HistoryItem>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, query, query_type, result, source, created_at FROM history ORDER BY created_at DESC LIMIT ?"
        )?;

        let items = stmt
            .query_map([limit], |row| {
                Ok(HistoryItem {
                    id: row.get(0)?,
                    query: row.get(1)?,
                    query_type: row.get(2)?,
                    result: row.get(3)?,
                    source: row.get(4)?,
                    created_at: row.get(5)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(items)
    }

    /// 清空历史记录
    pub fn clear_history(&self) -> SqliteResult<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute("DELETE FROM history", [])?;
        Ok(())
    }

    /// 添加到生词本
    pub fn add_vocabulary(
        &self,
        word: &str,
        word_type: &str,
        definition: &str,
        note: Option<&str>,
    ) -> SqliteResult<i64> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO vocabulary (word, word_type, definition, note) VALUES (?, ?, ?, ?)",
            rusqlite::params![word, word_type, definition, note],
        )?;
        Ok(conn.last_insert_rowid())
    }

    /// 获取生词本
    pub fn get_vocabulary(&self) -> SqliteResult<Vec<VocabularyItem>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, word, word_type, definition, note, created_at FROM vocabulary ORDER BY created_at DESC"
        )?;

        let items = stmt
            .query_map([], |row| {
                Ok(VocabularyItem {
                    id: row.get(0)?,
                    word: row.get(1)?,
                    word_type: row.get(2)?,
                    definition: row.get(3)?,
                    note: row.get(4)?,
                    added_at: row.get(5)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(items)
    }

    /// 从生词本删除
    pub fn remove_vocabulary(&self, id: i64) -> SqliteResult<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute("DELETE FROM vocabulary WHERE id = ?", [id])?;
        Ok(())
    }

    /// 检查生词本是否已存在
    pub fn vocabulary_exists(&self, word: &str) -> bool {
        let conn = self.conn.lock().unwrap();
        let count: i64 = conn
            .query_row(
                "SELECT COUNT(*) FROM vocabulary WHERE word = ?",
                [word],
                |row| row.get(0),
            )
            .unwrap_or(0);
        count > 0
    }
}
