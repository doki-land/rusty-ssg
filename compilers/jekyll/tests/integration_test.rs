//! 集成测试套件
//!
//! 测试 Jekyll 核心功能的集成测试

use jekyll::{FrontMatterParser, JekyllConfigLoader, JekyllStructure, LiquidEngine, MarkdownConverter, PostManager, JekyllDirectory};
use chrono::Datelike;
use std::{fs, path::Path};
use tempfile::tempdir;

#[test]
fn test_jekyll_structure_discovery() {
    let temp_dir = tempdir().unwrap();
    let root = temp_dir.path();

    // 创建 Jekyll 标准目录
    fs::create_dir_all(root.join("_posts")).unwrap();
    fs::create_dir_all(root.join("_layouts")).unwrap();
    fs::create_dir_all(root.join("_includes")).unwrap();
    fs::create_dir_all(root.join("_data")).unwrap();

    let structure = JekyllStructure::new(root).unwrap();

    assert!(structure.has_directory(JekyllDirectory::Posts));
    assert!(structure.has_directory(JekyllDirectory::Layouts));
    assert!(structure.has_directory(JekyllDirectory::Includes));
    assert!(structure.has_directory(JekyllDirectory::Data));
    assert!(!structure.has_directory(JekyllDirectory::Drafts));
}

#[test]
fn test_front_matter_parser() {
    let content = r#"---
title: Test Post
layout: post
date: 2024-01-01
categories:
  - programming
  - rust
tags: [jekyll, test]
---

This is the content.
"#;

    let front_matter = FrontMatterParser::parse(content).unwrap();

    assert_eq!(front_matter.get("title").unwrap().as_str().unwrap(), "Test Post");
    assert_eq!(front_matter.get("layout").unwrap().as_str().unwrap(), "post");
    assert_eq!(front_matter.get("date").unwrap().as_str().unwrap(), "2024-01-01");

    let categories_value = front_matter.get("categories").unwrap();
    let categories = categories_value.as_array().unwrap();
    assert_eq!(categories.len(), 2);
    assert_eq!(categories[0].as_str().unwrap(), "programming");
    assert_eq!(categories[1].as_str().unwrap(), "rust");

    let tags_value = front_matter.get("tags").unwrap();
    let tags = tags_value.as_array().unwrap();
    assert_eq!(tags.len(), 2);
    assert_eq!(tags[0].as_str().unwrap(), "jekyll");
    assert_eq!(tags[1].as_str().unwrap(), "test");

    assert!(front_matter.content().contains("This is the content"));
}

#[test]
fn test_post_creation() {
    let temp_dir = tempdir().unwrap();
    let root = temp_dir.path();

    // 创建 _posts 目录
    fs::create_dir_all(root.join("_posts")).unwrap();

    // 创建测试帖子
    let post_content = r#"---
title: Test Post
layout: post
date: 2024-01-01
categories: programming
---

This is a test post.
"#;
    fs::write(root.join("_posts").join("2024-01-01-test-post.md"), post_content).unwrap();

    // 加载配置
    let config = JekyllConfigLoader::load_from_dir(root).unwrap();

    // 加载帖子
    let structure = JekyllStructure::new(root).unwrap();
    let mut post_manager = PostManager::new(structure, config);
    let post_count = post_manager.load_posts().unwrap();

    assert_eq!(post_count, 1);
    let posts = post_manager.posts();
    assert_eq!(posts.len(), 1);

    let post = &posts[0];
    assert_eq!(post.title, "test post");
    assert_eq!(post.date.year(), 2024);
    assert_eq!(post.date.month(), 1);
    assert_eq!(post.date.day(), 1);
    assert_eq!(post.categories, vec!["programming"]);
    assert!(post.permalink.contains("2024/01/01/test-post"));
}

#[test]
fn test_markdown_conversion() {
    let markdown = r#"# Hello World

This is **bold** and this is *italic*.

## Code Example

```rust
fn main() {
    println!("Hello, world!");
}
```
"#;

    let converter = MarkdownConverter::new();
    let html = converter.convert(markdown).unwrap();

    assert!(html.contains("<h1>Hello World</h1>"));
    assert!(html.contains("<strong>bold</strong>"));
    assert!(html.contains("<em>italic</em>"));
    assert!(html.contains("<h2>Code Example</h2>"));
    assert!(html.contains("<pre"));
    assert!(html.contains("<code"));
    assert!(html.contains("fn main()"));
}

#[test]
fn test_liquid_template() {
    let temp_dir = tempdir().unwrap();
    let root = temp_dir.path();

    // 创建 _layouts 目录和布局文件
    fs::create_dir_all(root.join("_layouts")).unwrap();
    let layout_content = r#"<!DOCTYPE html>
<html>
<head>
    <title>{{ page.title }}</title>
</head>
<body>
    {{ content }}
</body>
</html>"#;
    fs::write(root.join("_layouts").join("default.html"), layout_content).unwrap();

    // 加载配置和结构
    let config = JekyllConfigLoader::load_from_dir(root).unwrap();
    let structure = JekyllStructure::new(root).unwrap();
    let mut liquid_engine = LiquidEngine::new(structure, config);

    // 创建上下文
    let content = r#"---
title: Test Page
---

<h1>Hello World</h1>"#;
    let front_matter = FrontMatterParser::parse(content).unwrap();
    let context = liquid_engine.create_jekyll_context(&front_matter, "test-page.md");

    // 渲染布局
    let result = liquid_engine.render_layout("default", front_matter.content(), &context).unwrap();

    assert!(result.contains("<!DOCTYPE html>"));
    assert!(result.contains("<title>Test Page</title>"));
    assert!(result.contains("<h1>Hello World</h1>"));
}

#[test]
fn test_config_loader() {
    let temp_dir = tempdir().unwrap();
    let root = temp_dir.path();

    // 创建 _config.yml 文件
    let config_content = r#"title: Test Site
description: A test site
author: Test Author
url: http://localhost:4000
permalink: /:categories/:year/:month/:day/:title/
"#;
    fs::write(root.join("_config.yml"), config_content).unwrap();

    // 加载配置
    let config = JekyllConfigLoader::load_from_dir(root).unwrap();

    assert_eq!(config.title, Some("Test Site".to_string()));
    assert_eq!(config.description, Some("A test site".to_string()));
    assert_eq!(config.author, Some("Test Author".to_string()));
    assert_eq!(config.url, Some("http://localhost:4000".to_string()));
    assert_eq!(config.permalink, Some("/:categories/:year/:month/:day/:title/".to_string()));
}

#[test]
fn test_post_manager() {
    let temp_dir = tempdir().unwrap();
    let root = temp_dir.path();

    // 创建 _posts 目录
    fs::create_dir_all(root.join("_posts")).unwrap();

    // 创建多个测试帖子
    let post1_content = r#"---
title: First Post
layout: post
date: 2024-01-01
categories: programming
---
First post content.
"#;
    fs::write(root.join("_posts").join("2024-01-01-first-post.md"), post1_content).unwrap();

    let post2_content = r#"---
title: Second Post
layout: post
date: 2024-01-02
categories: programming
---
Second post content.
"#;
    fs::write(root.join("_posts").join("2024-01-02-second-post.md"), post2_content).unwrap();

    let post3_content = r#"---
title: Third Post
layout: post
date: 2024-01-03
categories: design
---
Third post content.
"#;
    fs::write(root.join("_posts").join("2024-01-03-third-post.md"), post3_content).unwrap();

    // 加载配置和帖子
    let config = JekyllConfigLoader::load_from_dir(root).unwrap();
    let structure = JekyllStructure::new(root).unwrap();
    let mut post_manager = PostManager::new(structure, config);
    let post_count = post_manager.load_posts().unwrap();

    assert_eq!(post_count, 3);

    // 检查帖子排序（按日期降序）
    let posts = post_manager.posts();
    assert!(posts[0].date >= posts[1].date);
    assert!(posts[1].date >= posts[2].date);

    // 检查分类分组
    let programming_posts = post_manager.get_posts_by_category("programming");
    assert_eq!(programming_posts.len(), 2);

    let design_posts = post_manager.get_posts_by_category("design");
    assert_eq!(design_posts.len(), 1);

    // 检查最新帖子
    let latest_posts = post_manager.get_latest_posts(2);
    assert_eq!(latest_posts.len(), 2);
    assert_eq!(latest_posts[0].title, "third post");
    assert_eq!(latest_posts[1].title, "second post");
}
