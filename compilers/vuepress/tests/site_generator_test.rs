//! 站点生成器测试

use std::{collections::HashMap, fs, path::PathBuf};
use tempfile::tempdir;
use vuepress::{
    Document,
    tools::{ConfigLoader, StaticSiteGenerator},
    types::VuePressConfig,
};

#[test]
fn test_site_generator() {
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

    let doc2 = Document {
        meta: nargo_types::document::DocumentMeta {
            path: "guide/getting-started.md".to_string(),
            title: Some("Getting Started".to_string()),
            lang: None,
            last_updated: None,
            extra: std::collections::HashMap::new(),
        },
        frontmatter: nargo_types::document::FrontMatter::default(),
        content: "# Getting Started\n\nThis is the getting started guide.".to_string(),
        rendered_content: Some("<h1>Getting Started</h1><p>This is the getting started guide.</p>".to_string()),
        span: Default::default(),
    };

    documents.insert("index.md".to_string(), doc1);
    documents.insert("guide/getting-started.md".to_string(), doc2);

    // 创建配置
    let config = VuePressConfig::new();

    // 创建站点生成器
    let mut generator = StaticSiteGenerator::new(config).unwrap();

    // 生成站点
    let result = generator.generate(&documents, &output_dir);
    assert!(result.is_ok());

    // 验证生成的文件
    let index_path = output_dir.join("zh-hans").join("index.html");
    assert!(index_path.exists());

    let guide_path = output_dir.join("zh-hans").join("guide").join("getting-started.html");
    assert!(guide_path.exists());

    // 验证路由配置文件
    let routes_path = output_dir.join(".vuepress").join("routes.json");
    assert!(routes_path.exists());
    let routes_content = fs::read_to_string(routes_path).unwrap();
    assert!(routes_content.contains("/index"));
    assert!(routes_content.contains("/guide/getting-started"));

    // 验证搜索索引文件
    let search_path = output_dir.join(".vuepress").join("search-index.json");
    assert!(search_path.exists());
    let search_content = fs::read_to_string(search_path).unwrap();
    assert!(search_content.contains("Home Page"));
    assert!(search_content.contains("Getting Started"));

    // 验证站点地图文件
    let sitemap_path = output_dir.join("sitemap.xml");
    assert!(sitemap_path.exists());
    let sitemap_content = fs::read_to_string(sitemap_path).unwrap();
    assert!(sitemap_content.contains("/zh-hans/index.html"));
    assert!(sitemap_content.contains("/zh-hans/guide/getting-started.html"));

    // 验证根索引文件
    let root_index_path = output_dir.join("index.html");
    assert!(root_index_path.exists());
}
