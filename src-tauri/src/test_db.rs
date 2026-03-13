use std::sync::Mutex;
use tauri::Manager;
use yi_yi_lib::{api::DeepSeekClient, commands, database::Database, models::AppSettings};

fn main() {
    let settings = AppSettings::default();
    let db_path = "/home/supertaotao/.local/share/com.yi-yi.app/yi_yi.db";

    // 直接测试数据库查询
    match Database::new_with_path(db_path) {
        Ok(db) => {
            println!("✅ 数据库打开成功");

            // 测试古汉语查询
            match db.query_ancient("哀") {
                Some(result) => {
                    println!("✅ 古汉语查询成功: {:?}", result.word);
                }
                None => {
                    println!("❌ 古汉语查询失败");
                }
            }

            // 测试英文查询
            match db.query_english("test") {
                Some(result) => {
                    println!("✅ 英文查询成功: {:?}", result.word);
                }
                None => {
                    println!("❌ 英文查询失败");
                }
            }
        }
        Err(e) => {
            println!("❌ 数据库打开失败: {}", e);
        }
    }
}
