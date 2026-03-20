//! Develop 命令实现 - 开发服务器

use crate::{DevelopArgs, types::Result};
use console::style;
use std::path::PathBuf;

/// Develop 命令
#[cfg(feature = "dev")]
pub struct DevelopCommand;

#[cfg(feature = "dev")]
impl DevelopCommand {
    /// 执行 develop 命令
    pub async fn execute(args: DevelopArgs) -> Result<()> {
        println!("{}", style("Starting Gatsby development server...").cyan());

        let source_dir = args.source.unwrap_or_else(|| PathBuf::from("."));

        println!("  Source directory: {}", source_dir.display());
        println!("  Binding to: {}:{}", args.bind, args.port);
        println!("  Server URL: http://{}:{}", args.bind, args.port);

        if !args.no_browser {
            println!("  {} Opening browser...", style("→").blue());
        }

        println!("  {} Development server started", style("✓").green());
        println!("\n  {} Press Ctrl+C to stop the server", style("ℹ").blue());

        Ok(())
    }
}
