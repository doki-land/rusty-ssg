//! Init 命令实现
//!
//! 提供 Jekyll 项目初始化功能，支持创建标准的 Jekyll 目录结构
//! 和配置文件，包含可选的示例内容。

use crate::{InitArgs, types::Result};
use console::style;
use std::{fs, path::PathBuf};

/// Init 命令执行器
///
/// 负责初始化新的 Jekyll 项目，创建标准目录结构和配置文件。
pub struct InitCommand;

impl InitCommand {
    /// 执行 init 命令
    ///
    /// 根据提供的参数初始化新的 Jekyll 项目。
    ///
    /// # Arguments
    ///
    /// * `args` - 初始化命令参数，包含项目名称、目标目录等配置
    ///
    /// # Returns
    ///
    /// 返回成功或错误结果
    pub async fn execute(args: InitArgs) -> Result<()> {
        println!("{}", style("Initializing Jekyll project...").cyan());

        let project_name = args.name.unwrap_or_else(|| "my-jekyll-site".to_string());

        let project_dir = if let Some(dest) = args.destination {
            dest
        }
        else if args.force {
            PathBuf::from(".")
        }
        else {
            PathBuf::from(project_name.clone())
        };

        println!("  Project directory: {}", project_dir.display());

        if project_dir.exists() && !args.force && project_dir != PathBuf::from(".") {
            println!("  {} Directory already exists, use --force to initialize in current directory", style("⚠").yellow());
            return Ok(());
        }

        if !project_dir.exists() {
            fs::create_dir_all(&project_dir)?;
            println!("  {} Project directory created", style("✓").green());
        }

        if args.blank {
            Self::create_blank_structure(&project_dir)?;
        }
        else {
            Self::create_standard_structure(&project_dir, !args.skip_example)?;
        }

        if !args.skip_git {
            Self::create_gitignore(&project_dir)?;
        }

        println!("{}", style("Project initialization complete!").green());
        if project_dir != PathBuf::from(".") {
            println!("  To build your site, run: cd {} && jekyll build", project_dir.display());
            println!("  To serve your site locally, run: cd {} && jekyll dev", project_dir.display());
        }
        else {
            println!("  To build your site, run: jekyll build");
            println!("  To serve your site locally, run: jekyll dev");
        }

        Ok(())
    }

    /// 创建空白项目结构
    ///
    /// 仅创建最基本的目录和配置文件。
    ///
    /// # Arguments
    ///
    /// * `project_dir` - 项目目录路径
    ///
    /// # Returns
    ///
    /// 返回成功或错误结果
    fn create_blank_structure(project_dir: &PathBuf) -> Result<()> {
        let dirs = ["_posts", "_layouts", "_includes"];

        for dir in &dirs {
            let dir_path = project_dir.join(dir);
            if !dir_path.exists() {
                fs::create_dir_all(&dir_path)?;
                println!("  {} Created directory: {}", style("✓").green(), dir);
            }
        }

        let config_path = project_dir.join("_config.yml");
        if !config_path.exists() {
            let config_content = r#"# Jekyll Configuration
title: My Jekyll Site
description: A simple Jekyll site
url: "http://localhost:4000"
"#;
            fs::write(&config_path, config_content)?;
            println!("  {} Created _config.yml", style("✓").green());
        }

        Ok(())
    }

    /// 创建标准项目结构
    ///
    /// 创建完整的 Jekyll 目录结构、配置文件和示例内容。
    ///
    /// # Arguments
    ///
    /// * `project_dir` - 项目目录路径
    /// * `include_example` - 是否包含示例内容
    ///
    /// # Returns
    ///
    /// 返回成功或错误结果
    fn create_standard_structure(project_dir: &PathBuf, include_example: bool) -> Result<()> {
        let dirs = ["_posts", "_layouts", "_includes", "_data", "_drafts", "_sass", "assets"];

        for dir in &dirs {
            let dir_path = project_dir.join(dir);
            if !dir_path.exists() {
                fs::create_dir_all(&dir_path)?;
                println!("  {} Created directory: {}", style("✓").green(), dir);
            }
        }

        Self::create_config_file(project_dir)?;
        Self::create_index_file(project_dir)?;
        Self::create_default_layout(project_dir)?;
        Self::create_stylesheet(project_dir)?;

        if include_example {
            Self::create_example_post(project_dir)?;
        }

        Ok(())
    }

    /// 创建配置文件 _config.yml
    ///
    /// # Arguments
    ///
    /// * `project_dir` - 项目目录路径
    ///
    /// # Returns
    ///
    /// 返回成功或错误结果
    fn create_config_file(project_dir: &PathBuf) -> Result<()> {
        let config_path = project_dir.join("_config.yml");
        if !config_path.exists() {
            let config_content = r#"# Jekyll Configuration
# https://jekyllrb.com/docs/configuration/

# Site settings
title: My Jekyll Site
description: A simple Jekyll site
author: Your Name
url: "http://localhost:4000"

# Build settings
permalink: /:categories/:year/:month/:day/:title/
markdown: kramdown

# Exclude files and directories
exclude:
  - Gemfile
  - Gemfile.lock
  - node_modules
  - vendor/bundle/
  - vendor/cache/
  - vendor/gems/
  - vendor/ruby/
"#;
            fs::write(&config_path, config_content)?;
            println!("  {} Created _config.yml", style("✓").green());
        }
        Ok(())
    }

    /// 创建首页文件 index.md
    ///
    /// # Arguments
    ///
    /// * `project_dir` - 项目目录路径
    ///
    /// # Returns
    ///
    /// 返回成功或错误结果
    fn create_index_file(project_dir: &PathBuf) -> Result<()> {
        let index_path = project_dir.join("index.md");
        if !index_path.exists() {
            let index_content = r#"---
layout: default
title: Home
---

# Welcome to My Jekyll Site

This is a simple Jekyll site created with Rusty Jekyll.

## Latest Posts

{% for post in site.posts %}
- [{{ post.title }}]({{ post.url }}) ({{ post.date | date: "%Y-%m-%d" }})
{% endfor %}
"#;
            fs::write(&index_path, index_content)?;
            println!("  {} Created index.md", style("✓").green());
        }
        Ok(())
    }

    /// 创建默认布局文件
    ///
    /// # Arguments
    ///
    /// * `project_dir` - 项目目录路径
    ///
    /// # Returns
    ///
    /// 返回成功或错误结果
    fn create_default_layout(project_dir: &PathBuf) -> Result<()> {
        let default_layout_path = project_dir.join("_layouts").join("default.html");
        if !default_layout_path.exists() {
            let default_layout_content = r#"<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <title>{{ page.title }} | {{ site.title }}</title>
    <link rel="stylesheet" href="/assets/css/style.css">
</head>
<body>
    <header>
        <h1><a href="/">{{ site.title }}</a></h1>
        <p>{{ site.description }}</p>
    </header>
    <main>
        {{ content }}
    </main>
    <footer>
        <p>&copy; {{ "now" | date: "%Y" }} {{ site.author }}</p>
    </footer>
</body>
</html>"#;
            fs::write(&default_layout_path, default_layout_content)?;
            println!("  {} Created default layout", style("✓").green());
        }
        Ok(())
    }

    /// 创建样式文件
    ///
    /// # Arguments
    ///
    /// * `project_dir` - 项目目录路径
    ///
    /// # Returns
    ///
    /// 返回成功或错误结果
    fn create_stylesheet(project_dir: &PathBuf) -> Result<()> {
        let css_dir = project_dir.join("assets").join("css");
        if !css_dir.exists() {
            fs::create_dir_all(&css_dir)?;
        }

        let style_path = css_dir.join("style.css");
        if !style_path.exists() {
            let style_content = r#"/* Basic styles */
body {
    font-family: Arial, sans-serif;
    line-height: 1.6;
    margin: 0;
    padding: 0;
    color: #333;
}

header {
    background: #f4f4f4;
    padding: 1rem;
    text-align: center;
}

main {
    padding: 1rem;
    max-width: 800px;
    margin: 0 auto;
}

footer {
    background: #f4f4f4;
    padding: 1rem;
    text-align: center;
    margin-top: 2rem;
}

a {
    color: #3366cc;
    text-decoration: none;
}

a:hover {
    text-decoration: underline;
}

.post-list {
    list-style: none;
    padding: 0;
}

.post-list li {
    margin-bottom: 1rem;
}
"#;
            fs::write(&style_path, style_content)?;
            println!("  {} Created style.css", style("✓").green());
        }
        Ok(())
    }

    /// 创建示例文章
    ///
    /// # Arguments
    ///
    /// * `project_dir` - 项目目录路径
    ///
    /// # Returns
    ///
    /// 返回成功或错误结果
    fn create_example_post(project_dir: &PathBuf) -> Result<()> {
        let example_post_path = project_dir.join("_posts").join("2024-01-01-welcome-to-jekyll.md");
        if !example_post_path.exists() {
            let example_post_content = r#"---
title: Welcome to Jekyll
layout: post
categories: [jekyll, tutorial]
tags: [welcome, getting-started]
date: 2024-01-01
---

# Welcome to Jekyll!

This is your first post. Edit or delete it, then start blogging!

## What is Jekyll?

Jekyll is a static site generator that takes Markdown files and turns them into a website.

## Getting Started

1. Add new posts in the `_posts` directory
2. Create layouts in the `_layouts` directory
3. Add includes in the `_includes` directory
4. Configure your site in `_config.yml`

Happy blogging!
"#;
            fs::write(&example_post_path, example_post_content)?;
            println!("  {} Created example post", style("✓").green());
        }
        Ok(())
    }

    /// 创建 .gitignore 文件
    ///
    /// # Arguments
    ///
    /// * `project_dir` - 项目目录路径
    ///
    /// # Returns
    ///
    /// 返回成功或错误结果
    fn create_gitignore(project_dir: &PathBuf) -> Result<()> {
        let gitignore_path = project_dir.join(".gitignore");
        if !gitignore_path.exists() {
            let gitignore_content = r#"# Jekyll
_site/
.sass-cache/
.jekyll-cache/
.jekyll-metadata

# Ruby
*.gem
*.rbc
Gemfile.lock

# IDE
.idea/
.vscode/
*.swp
*.swo

# OS
.DS_Store
Thumbs.db
"#;
            fs::write(&gitignore_path, gitignore_content)?;
            println!("  {} Created .gitignore", style("✓").green());
        }
        Ok(())
    }
}

/// 执行 init 命令的公开入口点
///
/// # Arguments
///
/// * `args` - 初始化命令参数
///
/// # Returns
///
/// 返回成功或错误结果
pub async fn execute(args: crate::InitArgs) -> crate::types::Result<()> {
    InitCommand::execute(args).await
}
