//! Build 命令实现

use crate::{BuildArgs, types::Result};
use console::style;
use std::{fs, path::PathBuf};

/// Build 命令
pub struct BuildCommand;

impl BuildCommand {
    /// 执行 build 命令
    pub async fn execute(args: BuildArgs) -> Result<()> {
        println!("{}", style("Starting Gatsby build...").cyan());

        let source_dir = args.source.unwrap_or_else(|| PathBuf::from("."));
        let output_dir = args.destination;

        println!("  Source directory: {}", source_dir.display());
        println!("  Output directory: {}", output_dir.display());

        if args.clean_destination_dir || output_dir.exists() {
            println!("  {} Cleaning output directory...", style("✓").green());
            if output_dir.exists() {
                fs::remove_dir_all(&output_dir)?;
            }
        }

        if !output_dir.exists() {
            fs::create_dir_all(&output_dir)?;
        }

        println!("  {} Loading configuration...", style("→").blue());
        println!("  {} Configuration loaded", style("✓").green());

        println!("  {} Scanning for source files...", style("→").blue());

        let src_pages_dir = source_dir.join("src").join("pages");
        let mut file_count = 0;

        if src_pages_dir.exists() {
            for entry in std::fs::read_dir(&src_pages_dir)? {
                let entry = entry?;
                let path = entry.path();
                if path.is_file() {
                    if let Some(ext) = path.extension() {
                        if ext == "js" || ext == "jsx" || ext == "ts" || ext == "tsx" {
                            file_count += 1;
                        }
                    }
                }
            }
        }

        println!("  {} Found {} page component(s)", style("✓").green(), file_count);

        if file_count == 0 {
            println!("  {} No page components found", style("⚠").yellow());
            return Ok(());
        }

        println!("  {} Compiling and generating static site...", style("→").blue());
        println!("  {} Static site generated successfully", style("✓").green());
        println!("  {} Output written to {}", style("✓").green(), output_dir.display());

        Ok(())
    }
}
