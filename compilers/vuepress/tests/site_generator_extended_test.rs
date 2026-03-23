//! 站点生成器扩展测试
//! 测试 404 页面、robots.txt 文件等新功能

use std::{collections::HashMap, fs, path::PathBuf};
use tempfile::tempdir;
use vuepress::{
    Document,
    tools::{ConfigLoader, StaticSiteGenerator},
    types::VuePressConfig,
};

#[test]
fn test_site_generator_404_page() {
    // 创建临时目录
    let temp_dir = tempdir().unwrap();
    let output_dir = temp_dir.path().join("output");

    // 创建测试文档
    let mut documents = HashMap::new();

    let doc1 = Document {
        meta: nargo_types::document::DocumentMeta {
            path: "index.md".to_string(),
            title: Some("Home Page".to_string()),
            lang: None,
            last_updated: None,
            extra: std::collections::HashMap::new(),
        },
        frontmatter: nargo_types::document::FrontMatter::default(),
        content: "# Home Page\n\nThis is the home page.".to_string(),
        rendered_content: Some("<h1>Home Page</h1><p>This is the home page.</p>".to_string()),
        span: Default::default(),
    };

    documents.insert("index.md".to_string(), doc1);

    // 创建配置
    let config = VuePressConfig::new();

    // 创建站点生成器
    let mut generator = StaticSiteGenerator::new(config).unwrap();

    // 生成站点
    let result = generator.generate(&documents, &output_dir);
    assert!(result.is_ok());

    // 验证 404 页面
    let page_404_path = output_dir.join("404.html");
    assert!(page_404_path.exists());
    let page_404_content = fs::read_to_string(page_404_path).unwrap();
    assert!(page_404_content.contains("404"));
    assert!(page_404_content.contains("Page not found"));
}

#[test]
fn test_site_generator_robots_txt() {
    // 创建临时目录
    let temp_dir = tempdir().unwrap();
    let output_dir = temp_dir.path().join("output");

    // 创建测试文档
    let mut documents = HashMap::new();

    let doc1 = Document {
        meta: nargo_types::document::DocumentMeta {
            path: "index.md".to_string(),
            title: Some("Home Page".to_string()),
            lang: None,
            last_updated: None,
            extra: std::collections::HashMap::new(),
        },
        frontmatter: nargo_types::document::FrontMatter::default(),
        content: "# Home Page\n\nThis is the home page.".to_string(),
        rendered_content: Some("<h1>Home Page</h1><p>This is the home page.</p>".to_string()),
        span: Default::default(),
    };

    documents.insert("index.md".to_string(), doc1);

    // 创建配置
    let config = VuePressConfig::new();

    // 创建站点生成器
    let mut generator = StaticSiteGenerator::new(config).unwrap();

    // 生成站点
    let result = generator.generate(&documents, &output_dir);
    assert!(result.is_ok());

    // 验证 robots.txt 文件
    let robots_path = output_dir.join("robots.txt");
    assert!(robots_path.exists());
    let robots_content = fs::read_to_string(robots_path).unwrap();
    assert!(robots_content.contains("User-agent: *"));
    assert!(robots_content.contains("Allow: /"));
    assert!(robots_content.contains("Sitemap: /sitemap.xml"));
}

#[test]
fn test_vue_component_sfc_format() {
    // 创建编译器实例
    let config = VuePressConfig::new();
    let mut compiler = vuepress::compiler::VuePressCompiler::with_config(config);

    // 测试 SFC 格式的 Vue 组件
    let markdown = r#"
# Test Page

This is a test page with SFC Vue components.

```vue
<template>
  <div class="welcome">
    <h2>Welcome to VuePress!</h2>
    <p>This is a Vue component embedded in Markdown.</p>
  </div>
</template>

<style scoped>
.welcome {
  padding: 2rem;
  background-color: #f0f0f0;
  border-radius: 8px;
}
</style>
```
"#;

    // 编译文档
    let result = compiler.compile_document(markdown, "test.md");
    assert!(result.is_ok());

    let doc = result.unwrap();
    assert!(doc.rendered_content.is_some());

    let rendered_content = doc.rendered_content.unwrap();
    println!("Rendered content: {}", rendered_content);

    // 验证 SFC 格式的 Vue 组件是否被正确处理
    assert!(rendered_content.contains("<template>"));
    assert!(rendered_content.contains("<div class=\"welcome\">"));
    assert!(rendered_content.contains("<h2>Welcome to VuePress!</h2>"));
    assert!(rendered_content.contains("<style scoped>"));
    assert!(rendered_content.contains(".welcome {"));
}
