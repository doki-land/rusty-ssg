use chrono::{Datelike, NaiveDate};
use jekyll::{FrontMatterParser, JekyllConfig, JekyllStructure, Post, PostManager};
use std::{fs, path::Path};
use tempfile::tempdir;

#[test]
fn test_parse_filename() {
    let temp_dir = tempdir().unwrap();
    let post_path = temp_dir.path().join("2024-01-01-test-post.md");

    let (title, date) = Post::parse_filename(&post_path).unwrap();
    assert_eq!(title, "test post");
    assert_eq!(date.year(), 2024);
    assert_eq!(date.month(), 1);
    assert_eq!(date.day(), 1);
}

#[test]
fn test_extract_categories_from_front_matter() {
    let content = r#"---
title: Test Post
categories:
  - Programming
  - Rust
---
Content here."#;
    let front_matter = FrontMatterParser::parse(content).unwrap();
    let path = Path::new("_posts/test.md");

    // 调试：打印 front matter 内容
    println!("Front matter variables: {:?}", front_matter.variables());

    // 调试：检查 categories 字段
    if let Some(categories) = front_matter.get("categories") {
        println!("Categories field: {:?}", categories);
        println!("Categories type: {:?}", categories);
    }
    else {
        println!("Categories field not found");
    }

    let categories = Post::extract_categories(&front_matter, path);
    println!("Extracted categories: {:?}", categories);

    assert_eq!(categories, vec!["Programming", "Rust"]);
}

#[test]
fn test_extract_categories_from_path() {
    let content = r#"---
title: Test Post
---
Content here."#;
    let front_matter = FrontMatterParser::parse(content).unwrap();
    let path = Path::new("_posts/programming/rust/test.md");

    let categories = Post::extract_categories(&front_matter, path);
    assert_eq!(categories, vec!["programming", "rust"]);
}

#[test]
fn test_extract_tags() {
    let content = r#"---
title: Test Post
tags: rust, programming, web
---
Content here."#;
    let front_matter = FrontMatterParser::parse(content).unwrap();

    let tags = Post::extract_tags(&front_matter);
    assert_eq!(tags, vec!["rust", "programming", "web"]);
}

#[test]
fn test_generate_permalink() {
    let config = JekyllConfig::new().with_permalink("/:categories/:year/:month/:day/:title/".to_string());
    let title = "Test Post";
    let date = NaiveDate::from_ymd(2024, 1, 1);
    let categories = vec!["programming".to_string(), "rust".to_string()];
    let permalink = Post::generate_permalink(title, &date, &categories, &config).unwrap();
    assert_eq!(permalink, "/programming/rust/2024/01/01/test-post/");
}

#[test]
fn test_slugify() {
    let title = "Test Post With Special Characters!";
    let slug = Post::slugify(title);
    assert_eq!(slug, "test-post-with-special-characters");
}

#[test]
fn test_post_manager() {
    let temp_dir = tempdir().unwrap();

    // 创建 _posts 目录
    let posts_dir = temp_dir.path().join("_posts");
    fs::create_dir_all(&posts_dir).unwrap();

    // 创建测试帖子
    let post_content1 = r#"---
title: First Post
categories: programming
---
First post content."#;
    fs::write(posts_dir.join("2024-01-01-first-post.md"), post_content1).unwrap();

    let post_content2 = r#"---
title: Second Post
categories: programming
---
Second post content."#;
    fs::write(posts_dir.join("2024-01-02-second-post.md"), post_content2).unwrap();

    // 创建结构和配置
    let structure = JekyllStructure::new(temp_dir.path()).unwrap();
    let config = JekyllConfig::new();

    // 创建帖子管理器并加载帖子
    let mut manager = PostManager::new(structure, config);
    let count = manager.load_posts().unwrap();

    assert_eq!(count, 2);
    assert_eq!(manager.posts().len(), 2);

    // 检查帖子排序（按日期降序）
    assert!(manager.posts()[0].date >= manager.posts()[1].date);

    // 检查分类分组
    let programming_posts = manager.get_posts_by_category("programming");
    assert_eq!(programming_posts.len(), 2);
}
