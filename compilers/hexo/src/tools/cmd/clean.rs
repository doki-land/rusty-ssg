//! 清理命令实现

use super::super::CleanArgs;
use crate::types::Result;
use std::{fs, path::PathBuf};

/// 清理命令
pub struct CleanCommand;

impl CleanCommand {
    /// 执行清理命令
    pub async fn execute(args: CleanArgs) -> Result<()> {
        // 清理public目录
        let public_dir = PathBuf::from("public");
        if public_dir.exists() {
            fs::remove_dir_all(&public_dir)?;
            println!("Cleaned public directory");
        }

        // 清理数据库（如果需要）
        if args.database {
            let db_file = PathBuf::from("db.json");
            if db_file.exists() {
                fs::remove_file(&db_file)?;
                println!("Cleaned database");
            }
        }

        println!("Cleaned successfully!");
        Ok(())
    }
}
