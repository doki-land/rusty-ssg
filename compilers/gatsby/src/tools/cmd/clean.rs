#![warn(missing_docs)]

//! Clean 命令实现
//! 清理缓存和构建文件

use crate::tools::CleanArgs;
use std::{fs, path::Path};

/// Clean 命令
pub struct CleanCommand;

impl CleanCommand {
    /// 执行 clean 命令
    ///
    /// # Arguments
    ///
    /// * `args` - 命令参数
    ///
    /// # Returns
    ///
    /// 执行结果
    pub async fn execute(args: CleanArgs) -> crate::types::Result<()> {
        println!("🧹 Cleaning Gatsby cache and build files...");

        // 清理 public 目录
        if Path::new("public").exists() {
            fs::remove_dir_all("public").map_err(|e| crate::types::GatsbyError::ConfigError {
                message: format!("Failed to remove public directory: {:?}", e),
            })?;
            println!("✅ Removed public directory");
        }

        // 清理 .cache 目录
        if Path::new(".cache").exists() {
            fs::remove_dir_all(".cache").map_err(|e| crate::types::GatsbyError::ConfigError {
                message: format!("Failed to remove .cache directory: {:?}", e),
            })?;
            println!("✅ Removed .cache directory");
        }

        println!("🎉 Clean completed successfully");
        Ok(())
    }
}
