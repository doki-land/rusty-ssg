//! Init 命令实现

use crate::InitArgs;
use console::style;
use std::{fs, path::PathBuf};
use crate::types::Result;

/// Init 命令
pub struct InitCommand;

impl InitCommand {
    /// 执行 init 命令
    pub async fn execute(args: InitArgs) -> Result<()> {
        println!("{}", style("Initializing Jekyll project...").cyan());

        let project_name = args.name.unwrap_or_else(|| "my-jekyll-site".to_string());
        let project_dir = PathBuf::from(project_name.clone());

        println!("  Creating project directory: {}", project_dir.display());

        if project_dir.exists() {
            println!("  {} Directory already exists", style("⚠").yellow());
        } else {
            fs::create_dir_all(&project_dir)?;
            println!("  {} Project directory created", style("✓").green());
        }

        // 创建 Jekyll 标准目录结构
        let dirs = ["_posts", "_layouts", "_includes", "_data", "_drafts", "_sass", "assets"];

        for dir in &dirs {
            let dir_path = project_dir.join(dir);
            if !dir_path.exists() {
                fs::create_dir_all(&dir_path)?;
                println!("  {} Created directory: {}", style("✓").green(), dir);
            } else {
                println!("  {} Directory already exists: {}", style("⚠").yellow(), dir);
            }
        }

        // 创建 _config.yml 文件
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
        } else {
            println!("  {} _config.yml already exists", style("⚠").yellow());
        }

        // 创建 index.md 文件
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
        } else {
            println!("  {} index.md already exists", style("⚠").yellow());
        }

        // 创建 default layout
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
        } else {
            println!("  {} Default layout already exists", style("⚠").yellow());
        }

        // 创建 assets/css 目录和 style.css 文件
        let css_dir = project_dir.join("assets").join("css");
        if !css_dir.exists() {
            fs::create_dir_all(&css_dir)?;
            println!("  {} Created assets/css directory", style("✓").green());
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
        } else {
            println!("  {} style.css already exists", style("⚠").yellow());
        }

        // 创建示例帖子
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
        } else {
            println!("  {} Example post already exists", style("⚠").yellow());
        }

        println!("{}", style("Project initialization complete!").green());
        println!("  To build your site, run: cd {} && jekyll build", project_name);
        println!("  To serve your site locally, run: cd {} && jekyll serve", project_name);

        Ok(())
    }
}
