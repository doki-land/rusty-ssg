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
        let content_path = source_dir.join("content").join(&args.path);

        println!("  Source directory: {}", source_dir.display());
        println!("  Content path: {}", content_path.display());

        if let Some(kind) = &args.kind {
            println!("  Archetype: {}", kind);
        }

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

        let archetype_content = Self::load_archetype(&source_dir, args.kind.as_deref())?;
        let content = Self::process_archetype(&archetype_content, &args.path)?;

        fs::write(&content_path, content)?;
        println!("  {} Created content: {}", style("✓").green(), content_path.display());

        Ok(())
    }

    /// 加载 archetype 模板
    fn load_archetype(source_dir: &PathBuf, kind: Option<&str>) -> Result<String> {
        let archetypes_dir = source_dir.join("archetypes");

        let template_path = if let Some(kind) = kind {
            let specific_path = archetypes_dir.join(format!("{}.md", kind));
            if specific_path.exists() { Some(specific_path) } else { None }
        }
        else {
            let default_path = archetypes_dir.join("default.md");
            if default_path.exists() { Some(default_path) } else { None }
        };

        if let Some(path) = template_path {
            println!("  {} Using archetype: {}", style("→").blue(), path.display());
            Ok(fs::read_to_string(&path)?)
        }
        else {
            println!("  {} No archetype found, using default template", style("ℹ").blue());
            Ok(Self::default_archetype())
        }
    }

    /// 处理 archetype 模板，替换占位符
    fn process_archetype(template: &str, path: &PathBuf) -> Result<String> {
        let now = Local::now();
        let date_str = now.format("%Y-%m-%d").to_string();
        let time_str = now.format("%H:%M:%S%:z").to_string();
        let datetime_str = now.format("%Y-%m-%dT%H:%M:%S%:z").to_string();

        let file_stem = path.file_stem().and_then(|s| s.to_str()).unwrap_or("new-post");

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

        let content = template
            .replace("{{ .Date }}", &date_str)
            .replace("{{ .Time }}", &time_str)
            .replace("{{ .DateTime }}", &datetime_str)
            .replace("{{ .Title }}", &title)
            .replace("{{ .Name }}", file_stem);

        Ok(content)
    }

    /// 默认 archetype 模板
    fn default_archetype() -> String {
        r#"---
title: "{{ .Title }}"
date: {{ .DateTime }}
draft: true
---

# {{ .Title }}

Write your content here.
"#
        .to_string()
    }
}
