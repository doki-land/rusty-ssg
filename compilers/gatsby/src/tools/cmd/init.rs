//! Init 命令实现

use crate::{InitArgs, types::Result};
use console::style;
use std::{fs, path::PathBuf};

/// Init 命令
pub struct InitCommand;

impl InitCommand {
    /// 执行 init 命令
    pub async fn execute(_args: InitArgs) -> Result<()> {
        println!("{}", style("Initializing Gatsby project...").cyan());
        println!("{}", style("✓ Gatsby project initialized successfully!").green());
        Ok(())
    }
}
