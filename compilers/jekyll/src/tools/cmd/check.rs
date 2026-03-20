//! Check 命令实现

use crate::CheckArgs;
use console::style;
use std::{fs, path::PathBuf};
use crate::types::Result;
use crate::jekyll::{JekyllStructure, JekyllConfigLoader, PostManager, FrontMatterParser};

/// Check 命令
pub struct CheckCommand;

impl CheckCommand {
    /// 执行 check 命令
    pub async fn execute(args: CheckArgs) -> Result<()> {
        println!("{}", style("Checking Jekyll site...").cyan());

        let source_dir = args.source.unwrap_or_else(|| PathBuf::from("."));

        println!("  Source directory: {}", source_dir.display());

        println!("  {} Loading Jekyll structure...", style("→").blue());
        let structure = JekyllStructure::new(&source_dir)?;
        println!("  {} Jekyll structure loaded", style("✓").green());

        println!("  {} Loading configuration...", style("→").blue());
        let config = JekyllConfigLoader::load_from_dir(&source_dir)?;
        println!("  {} Configuration loaded", style("✓").green());

        println!("  {} Checking posts...", style("→").blue());
        let mut post_manager = PostManager::new(structure.clone(), config.clone());
        let post_count = post_manager.load_posts()?;
        println!("  {} Checked {} posts", style("✓").green(), post_count);

        println!("  {} Checking Markdown files...", style("→").blue());
        let markdown_files = structure.collect_markdown_files()?;
        let mut markdown_errors = 0;

        for file in &markdown_files {
            if let Err(e) = FrontMatterParser::parse_file(file) {
                println!("    {} Error in {}: {}", style("✗").red(), file.display(), e);
                markdown_errors += 1;
            }
        }

        if markdown_errors == 0 {
            println!("  {} All Markdown files are valid", style("✓").green());
        } else {
            println!("  {} Found {} errors in Markdown files", style("⚠").yellow(), markdown_errors);
        }

        println!("  {} Checking directory structure...", style("→").blue());

        // 检查必要的目录
        let required_dirs = ["_posts", "_layouts"];
        let mut missing_dirs = Vec::new();

        for dir in &required_dirs {
            let dir_path = source_dir.join(dir);
            if !dir_path.exists() {
                missing_dirs.push(dir);
            }
        }

        if missing_dirs.is_empty() {
            println!("  {} All required directories exist", style("✓").green());
        } else {
            println!("  {} Missing directories: {:?}", style("⚠").yellow(), missing_dirs);
        }

        println!("{}", style("Site check complete!").green());

        if markdown_errors > 0 || !missing_dirs.is_empty() {
            println!("  {} Some issues were found", style("⚠").yellow());
        } else {
            println!("  {} No issues found", style("✓").green());
        }

        Ok(())
    }
}
