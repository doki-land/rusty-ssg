//! Version 命令实现 - 显示版本信息

use crate::types::Result;
use console::style;

/// Version 命令
pub struct VersionCommand;

impl VersionCommand {
    /// 执行 version 命令
    pub async fn execute() -> Result<()> {
        println!("{}", style("MkDocs compiler").cyan());
        println!("  Version: {}", env!("CARGO_PKG_VERSION"));
        println!("  Rust edition: 2021");
        Ok(())
    }
}
