//! New 命令实现 - 创建新内容

use crate::{NewArgs, types::Result};
use chrono::Local;
use console::style;
use std::{fs, path::PathBuf};

/// New 命令
pub struct NewCommand;

impl NewCommand {
    /// 执行 new 命令
    pub async fn execute(args: NewArgs) -> Result<()> {
        println!("{}", style("Creating new content...").cyan());

        let source_dir = args.source.unwrap_or_else(|| PathBuf::from("."));
        let docs_dir = source_dir.join("docs");
        let content_path = docs_dir.join(&args.path);

        println!("  Source directory: {}", source_dir.display());
        println!("  Content path: {}", content_path.display());

        if content_path.exists() {
            println!("  {} File already exists: {}", style("✗").red(), content_path.display());
            return Ok(());
        }

        if let Some(parent) = content_path.parent() {
            if !parent.exists() {
                fs::create_dir_all(parent)?;
                println!("  {} Created directory: {}", style("✓").green(), parent.display());
            }
        }

        let content = Self::generate_content(&args.path)?;

        fs::write(&content_path, content)?;
        println!("  {} Created content: {}", style("✓").green(), content_path.display());

        Ok(())
    }

    /// 生成内容文件
    fn generate_content(path: &PathBuf) -> Result<String> {
        let now = Local::now();
        let date_str = now.format("%Y-%m-%d").to_string();

        let file_stem = path.file_stem().and_then(|s| s.to_str()).unwrap_or("new-page");

        let title = file_stem
            .split(|c: char| c == '-' || c == '_')
            .map(|word| {
                let mut chars = word.chars();
                match chars.next() {
                    None => String::new(),
                    Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                }
            })
            .collect::<Vec<_>>()
            .join(" ");

        Ok(format!(
            r#"---
title: {}
date: {}
---

# {}

Write your content here.
"#,
            title, date_str, title
        ))
    }
}
