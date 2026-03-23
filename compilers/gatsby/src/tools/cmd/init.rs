//! Init 命令实现

use crate::{InitArgs, types::Result};
use console::style;
use std::{fs, path::PathBuf};

/// Init 命令
pub struct InitCommand;

impl InitCommand {
    /// 执行 init 命令
    pub async fn execute(args: InitArgs) -> Result<()> {
        println!("{}", style("Initializing Gatsby project...").cyan());

        let project_name = args.name.unwrap_or_else(|| "gatsby-site".to_string());
        let project_dir = PathBuf::from(project_name);

        if project_dir.exists() {
            return Err(crate::types::GatsbyError::config(format!("Directory '{}' already exists", project_name)));
        }

        // 创建项目目录
        fs::create_dir_all(&project_dir)?;
        println!("  {} Created project directory: {}", style("→").blue(), project_dir.display());

        // 创建 src 目录结构
        let src_dir = project_dir.join("src");
        let pages_dir = src_dir.join("pages");
        let components_dir = src_dir.join("components");
        let templates_dir = src_dir.join("templates");

        fs::create_dir_all(&pages_dir)?;
        fs::create_dir_all(&components_dir)?;
        fs::create_dir_all(&templates_dir)?;

        // 创建 content 目录
        let content_dir = project_dir.join("content");
        let blog_dir = content_dir.join("blog");
        fs::create_dir_all(&blog_dir)?;

        // 创建 static 目录
        let static_dir = project_dir.join("static");
        fs::create_dir_all(&static_dir)?;

        // 创建配置文件
        Self::create_config_file(&project_dir)?;

        // 创建示例页面
        Self::create_example_pages(&pages