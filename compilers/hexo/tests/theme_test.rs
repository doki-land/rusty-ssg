//! 主题系统测试

use hexo::theme::{
    ThemeManager, generate_variables, render_theme,
    variables::{Category, Page, Post, Site, Tag},
};
use std::path::Path;

#[test]
fn test_theme_manager() {
    let mut theme_manager = ThemeManager::new();

    // 添加主题搜索路径
    let test_theme_path = Path::new("../../examples/hexo-mvp/themes");
    theme_manager.add_search_path(test_theme_path);

    // 尝试加载主题
    // 注意：这里我们没有实际的主题文件，所以会失败
    // 但我们可以测试错误处理
    let result = theme_manager.load_theme("landscape");
    assert!(result.is_err());
}

#[test]
fn test_generate_variables() {
    // 创建站点信息
    let site = Site {
        title: "Test Site".to_string(),
        description: "A test site".to_string(),
        author: "Test Author".to_string(),
        url: "http://example.com".to_string(),
        root: "/".to_string(),
        categories: vec![],
        tags: vec![],
        posts: vec![],
        pages: vec![],
    };

    // 创建文章信息
    let post = Post {
        title: "Test Post".to_string(),
        date: "2026-03-20".to_string(),
        updated: None,
        path: "2026/03/20/test-post/".to_string(),
        content: "<h1>Test Post</h1><p>This is a test post.</p>".to_string(),
        excerpt: "This is a test post excerpt.".to_string(),
        categories: vec![],
        tags: vec![],
        front_matter: serde_json::Value::Object(serde_json::Map::new()),
    };

    // 生成主题变量
    let variables = generate_variables(
        site,
        Some(post),
        serde_json::Value::Object(serde_json::Map::new()),
        serde_json::Value::Object(serde_json::Map::new()),
    );

    assert_eq!(variables.site.title, "Test Site");
    assert!(variables.page.is_some());
    assert_eq!(variables.page.unwrap().title, "Test Post");
}

#[test]
fn test_render_theme() {
    // 创建一个简单的模板文件
    let template_content = "<!DOCTYPE html>
<html>
<head>
    <title>{{ site.title }}</title>
</head>
<body>
    {% if page %}
    <h1>{{ page.title }}</h1>
    {{ page.content }}
    {% endif %}
</body>
</html>";

    // 写入临时文件
    use std::{fs::File, io::Write};
    use tempfile::tempdir;

    let dir = tempdir().unwrap();
    let template_path = dir.path().join("test.ejs");

    let mut file = File::create(&template_path).unwrap();
    file.write_all(template_content.as_bytes()).unwrap();

    // 创建站点信息
    let site = Site {
        title: "Test Site".to_string(),
        description: "A test site".to_string(),
        author: "Test Author".to_string(),
        url: "http://example.com".to_string(),
        root: "/".to_string(),
        categories: vec![],
        tags: vec![],
        posts: vec![],
        pages: vec![],
    };

    // 创建文章信息
    let post = Post {
        title: "Test Post".to_string(),
        date: "2026-03-20".to_string(),
        updated: None,
        path: "2026/03/20/test-post/".to_string(),
        content: "<h1>Test Post</h1><p>This is a test post.</p>".to_string(),
        excerpt: "This is a test post excerpt.".to_string(),
        categories: vec![],
        tags: vec![],
        front_matter: serde_json::Value::Object(serde_json::Map::new()),
    };

    // 生成主题变量
    let variables = generate_variables(
        site,
        Some(post),
        serde_json::Value::Object(serde_json::Map::new()),
        serde_json::Value::Object(serde_json::Map::new()),
    );

    // 渲染模板
    let result = render_theme(&template_path, serde_json::to_value(variables).unwrap());

    assert!(result.is_ok());
    let html = result.unwrap();
    assert!(html.contains("Test Site"));
    assert!(html.contains("Test Post"));
    assert!(html.contains("This is a test post"));
}
