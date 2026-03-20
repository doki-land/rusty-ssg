//! Check 命令实现

use crate::{CheckArgs, types::Result};
use console::style;
use std::path::PathBuf;

/// Check 命令
pub struct CheckCommand;

impl CheckCommand {
    /// 执行 check 命令
    pub async fn execute(args: CheckArgs) -> Result<()> {
        println!("{}", style("Checking Gatsby project...").cyan());

        let source_dir = args.source.unwrap_or_else(|| PathBuf::from("."));
        println!("  Source directory: {}", source_dir.display());

        let mut errors = Vec::new();
        let mut warnings = Vec::new();

        Self::check_config(&source_dir, &mut errors, &mut warnings)?;
        Self::check_pages(&source_dir, &mut errors, &mut warnings)?;

        if !warnings.is_empty() {
            println!("\n  {} Warnings ({}):", style("⚠").yellow(), warnings.len());
            for warning in &warnings {
                println!("    {}", style(warning).yellow());
            }
        }

        if !errors.is_empty() {
            println!("\n  {} Errors ({}):", style("✗").red(), errors.len());
            for error in &errors {
                println!("    {}", style(error).red());
            }
            println!("\n  {} Check failed with {} errors", style("✗").red(), errors.len());
        }
        else {
            println!("\n  {} All checks passed!", style("✓").green());
        }

        Ok(())
    }

    /// 检查配置文件
    fn check_config(source_dir: &PathBuf, _errors: &mut Vec<String>, warnings: &mut Vec<String>) -> Result<()> {
        println!("  {} Checking configuration...", style("→").blue());

        let config_path = source_dir.join("gatsby-config.js");
        if !config_path.exists() {
            warnings.push("gatsby-config.js not found".to_string());
        }
        else {
            println!("    {} Configuration file exists", style("✓").green());
        }

        Ok(())
    }

    /// 检查页面文件
    fn check_pages(source_dir: &PathBuf, _errors: &mut Vec<String>, warnings: &mut Vec<String>) -> Result<()> {
        println!("  {} Checking pages...", style("→").blue());

        let src_pages_dir = source_dir.join("src").join("pages");
        if !src_pages_dir.exists() {
            warnings.push("src/pages directory not found".to_string());
            return Ok(());
        }

        let mut file_count = 0;

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

        if file_count == 0 {
            warnings.push("No page components found in src/pages".to_string());
        }
        else {
            println!("    {} Found {} page component(s)", style("✓").green(), file_count);
        }

        Ok(())
    }
}
