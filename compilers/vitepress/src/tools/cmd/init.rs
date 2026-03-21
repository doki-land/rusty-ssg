//! Init 命令实现

use crate::{tools::InitArgs, types::Result};
use console::style;
use std::{fs, path::PathBuf};

/// Init 命令
pub struct InitCommand;

impl InitCommand {
    /// 执行 init 命令
    pub async fn execute(args: InitArgs) -> Result<()> {
        println!("{}", style("Initializing VitePress project...").cyan());

        let project_name = args.name.unwrap_or_else(|| "vitepress-docs".to_string());
        let current_dir = std::env::current_dir()?;

        println!("  Project name: {}", style(&project_name).green());
        println!("  Directory: {}", style(current_dir.display()).green());

        Self::create_directory_structure()?;
        Self::create_config_file(&project_name)?;
        Self::create_index_document()?;
        Self::create_guide_documents()?;

        println!("{}", style("✓ VitePress project initialized successfully!").green());
        println!("\nNext steps:");
        println!("  1. {} - Start development server", style("vitepress dev").cyan());
        println!("  2. {} - Build static site", style("vitepress build").cyan());

        Ok(())
    }

    /// 创建目录结构
    fn create_directory_structure() -> Result<()> {
        let dirs = [".vitepress", "guide", "public/images"];

        for dir in &dirs {
            let path = PathBuf::from(dir);
            if !path.exists() {
                fs::create_dir_all(&path)?;
                println!("  {} Created directory: {}", style("✓").green(), dir);
            }
        }

        Ok(())
    }

    /// 创建配置文件
    fn create_config_file(project_name: &str) -> Result<()> {
        let config_content = format!(
            r#"# VitePress 配置文件

title = "{}"
description = "使用 VitePress 构建的文档站点"
base = "/"

[theme]
nav = [
    {{ text = "首页", link = "/" }},
    {{ text = "指南", link = "/guide/" }}
]

[theme.footer]
copyright = "Copyright © 2024"
message = "基于 VitePress 构建"

[build]
out_dir = "dist"
src_dir = "."
clean = true
minify = false
"#,
            project_name
        );

        fs::write(".vitepress/vitepress.config.toml", config_content)?;
        println!("  {} Created config file: .vitepress/vitepress.config.toml", style("✓").green());

        Ok(())
    }

    /// 创建首页文档
    fn create_index_document() -> Result<()> {
        let index_content = r#"# 欢迎使用 VitePress

VitePress 是一个高性能的文档生成系统，基于 Rust 构建，速度飞快！

## 快速开始

1. 编辑 `index.md` 开始你的文档
2. 运行 `vitepress dev` 启动开发服务器
3. 访问 http://localhost:5173 查看效果

## 功能特性

- 🚀 **极速构建** - 基于 Rust，编译速度极快
- 📝 **Markdown 支持** - 完整的 Markdown 语法支持
- 🌐 **多语言** - 内置多语言文档支持
- 🎨 **主题系统** - 灵活的主题定制
- 🔌 **插件系统** - 可扩展的功能插件

## 下一步

- 查看 [指南](./guide/) 了解更多功能
- 访问 GitHub 获取更多帮助
"#;

        fs::write("index.md", index_content)?;
        println!("  {} Created document: index.md", style("✓").green());

        Ok(())
    }

    /// 创建指南文档
    fn create_guide_documents() -> Result<()> {
        let guide_index_content = r#"# 指南

欢迎来到 VitePress 指南！这里将教你如何使用 VitePress 构建出色的文档站点。

## 章节

- [快速开始](./getting-started.md)
"#;

        let getting_started_content = r#"# 快速开始

本章节将帮助你快速上手 VitePress。

## 安装

VitePress 已经安装在你的系统中，可以直接使用。

## 创建项目

使用 `vitepress init` 命令可以快速创建一个新的 VitePress 项目。

## 开发模式

运行 `vitepress dev` 启动开发服务器，然后在浏览器中访问 http://localhost:5173。

## 构建

运行 `vitepress build` 生成静态站点文件。
"#;

        fs::write("guide/index.md", guide_index_content)?;
        fs::write("guide/getting-started.md", getting_started_content)?;

        println!("  {} Created guide documents", style("✓").green());

        Ok(())
    }
}
