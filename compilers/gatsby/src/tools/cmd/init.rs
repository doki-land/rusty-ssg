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
        Self::create_example_pages(&pages_dir)?;

        // 创建示例博客文章
        Self::create_example_blog_post(&blog_dir)?;

        // 创建 package.json
        Self::create_package_json(&project_dir)?;

        // 创建 .gitignore
        Self::create_gitignore(&project_dir)?;

        println!("  {} Created project structure", style("✓").green());
        println!("  {} Created configuration files", style("✓").green());
        println!("  {} Created example pages", style("✓").green());

        println!("{}", style("Gatsby project initialized successfully!").green());
        println!();
        println!("To start developing:");
        println!("  cd {}", project_name);
        println!("  gatsby develop");
        println!();
        println!("To build for production:");
        println!("  cd {}", project_name);
        println!("  gatsby build");

        Ok(())
    }

    /// 创建配置文件
    fn create_config_file(project_dir: &PathBuf) -> Result<()> {
        let config_content = r#"{
  "siteMetadata": {
    "title": "Gatsby Site",
    "description": "A modern static site generated with Gatsby",
    "author": "Your Name",
    "siteUrl": "https://example.com"
  },
  "plugins": [
    "gatsby-plugin-react-helmet",
    "gatsby-plugin-sharp",
    "gatsby-transformer-sharp"
  ]
}"#;

        let config_path = project_dir.join("gatsby-config.json");
        fs::write(config_path, config_content)?;

        Ok(())
    }

    /// 创建示例页面
    fn create_example_pages(pages_dir: &PathBuf) -> Result<()> {
        // 创建 index.md
        let index_content = r#"---
title: "Home"
date: 2024-01-01
description: "Welcome to my Gatsby site"
---

# Welcome to My Gatsby Site

This is the home page of my Gatsby site.

## Features

- Fast static site generation
- Markdown support
- GraphQL data layer
- Modern web technologies
- Easy deployment

## About This Site

This site was generated using the Rust implementation of Gatsby, which provides the same great features as the original JavaScript version but with improved performance and reliability.

## Getting Started

1. **Edit content**: Add or modify files in the `content` directory
2. **Customize**: Update the site configuration in `gatsby-config.json`
3. **Build**: Run `gatsby build` to generate static files
4. **Deploy**: Deploy the `public` directory to your hosting provider

Happy building! 🚀
"#;

        let index_path = pages_dir.join("index.md");
        fs::write(index_path, index_content)?;

        // 创建 about.md
        let about_content = r#"---
title: "About"
date: 2024-01-01
description: "About this site"
---

# About This Site

This site was built using Gatsby, a modern static site generator.

## Who Am I?

I'm a web developer passionate about modern web technologies and static site generators.

## Why Gatsby?

Gatsby offers:

- Blazing fast performance
- Rich ecosystem of plugins
- Powerful GraphQL data layer
- Modern React-based architecture
- Excellent developer experience

## Contact

Feel free to reach out if you have any questions or suggestions!
"#;

        let about_path = pages_dir.join("about.md");
        fs::write(about_path, about_content)?;

        Ok(())
    }

    /// 创建示例博客文章
    fn create_example_blog_post(blog_dir: &PathBuf) -> Result<()> {
        let post_content = r#"---
title: "First Blog Post"
date: 2024-01-01
description: "My first blog post with Gatsby"
author: "Your Name"
tags: ["gatsby", "blog", "getting-started"]
categories: ["tutorial"]
---

# First Blog Post

Welcome to my first blog post using Gatsby!

## What I'm Doing

I'm learning how to use Gatsby to build a modern static blog.

## Why Gatsby?

- It's fast
- It's flexible
- It has a great ecosystem
- It supports Markdown
- It has a powerful GraphQL data layer

## Next Steps

1. Write more blog posts
2. Customize the site design
3. Add more features
4. Deploy to production

Stay tuned for more content! 🚀
"#;

        let post_path = blog_dir.join("first-post.md");
        fs::write(post_path, post_content)?;

        Ok(())
    }

    /// 创建 package.json
    fn create_package_json(project_dir: &PathBuf) -> Result<()> {
        let package_content = r#"{
  "name": "gatsby-site",
  "private": true,
  "description": "A modern static site generated with Gatsby",
  "version": "0.1.0",
  "license": "0BSD",
  "scripts": {
    "build": "gatsby build",
    "develop": "gatsby develop",
    "start": "gatsby develop",
    "serve": "gatsby serve",
    "clean": "gatsby clean"
  },
  "dependencies": {
    "gatsby": "^5.13.3",
    "react": "^18.2.0",
    "react-dom": "^18.2.0"
  }
}"#;

        let package_path = project_dir.join("package.json");
        fs::write(package_path, package_content)?;

        Ok(())
    }

    /// 创建 .gitignore
    fn create_gitignore(project_dir: &PathBuf) -> Result<()> {
        let gitignore_content = r#"# Logs
logs
*.log
npm-debug.log*
yarn-debug.log*
yarn-error.log*
pnpm-debug.log*
lerna-debug.log*

# Dependencies
node_modules
.pnp
.pnp.js

# Testing
coverage
*.lcov
.nyc_output

# Production
dist
dist-ssr
*.local

# Editor directories and files
.vscode/*
!.vscode/extensions.json
.idea
.DS_Store
*.suo
*.ntvs*
*.njsproj
*.sln
*.sw?

# Environment variables
.env
.env.local
.env.development.local
.env.test.local
.env.production.local

# Gatsby files
.cache
public

# Temporary files
*.tmp
*.temp
"#;

        let gitignore_path = project_dir.join(".gitignore");
        fs::write(gitignore_path, gitignore_content)?;

        Ok(())
    }
}
