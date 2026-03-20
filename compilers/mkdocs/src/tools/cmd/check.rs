//! Check 命令实现 - 检查文档

use crate::{CheckArgs, types::Result};
use console::style;
use std::path::PathBuf;

/// Check 命令
pub struct CheckCommand;

impl CheckCommand {
    /// 执行 check 命令
    pub async fn execute(args: CheckArgs) -> Result<()> {
        println!("{}", style("Checking MkDocs project...").cyan());

        let source_dir = args.source.unwrap_or_else(|| PathBuf::from("."));

        println!("  Source directory: {}", source_dir.display());
        println!("\n{}", style("Check feature is under development").yellow());

        Ok(())
    }
}
