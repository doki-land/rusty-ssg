//! 配置模块测试

use hexo::HexoConfig;
use std::path::PathBuf;

#[test]
fn test_load_config_from_file() {
    // 创建一个临时的 YAML 配置文件
    let config_content = r#"
# Hexo Configuration
## Docs: https://hexo.io/docs/configuration.html

# Site
site:
  title: Hexo Blog
  subtitle: A simple blog
  description: This is a test blog
  author: Test Author
  language: en
  timezone: UTC

# URL
url: https://example.com
root: /

# Theme
theme: landscape

# Deployment
deploy:
  type: git
  repo: https://github.com/example/example.github.io.git
  branch: main
  message: Updated

# Writing
writing:
  new_post_name: :title.md
  default_layout: post
  titlecase: false
  external_link:
    enable: true
    field: site
    exclude:
      - example.com
  filename_case: 0
  render_drafts: false
  permalink: :year/:month/:day/:title/
  asset_folder: true
  relative_link: false

# Server
server:
  port: 4000
  host: 0.0.0.0
  compress: true
  cache: true

# Pagination
pagination:
  per_page: 10
  dir: page

# Directory
directory:
  source_dir: source
  public_dir: public
  tag_dir: tags
  category_dir: categories

# Plugins
plugins:
  - hexo-generator-feed
  - hexo-generator-sitemap
"#;

    // 创建临时文件
    let temp_file = tempfile::Builder::new().suffix(".yml").tempfile().expect("Failed to create temp file");

    // 写入配置内容
    std::fs::write(temp_file.path(), config_content).expect("Failed to write config content");

    // 加载配置
    let config = HexoConfig::load_from_file(temp_file.path()).expect("Failed to load config");

    // 验证配置是否正确加载
    assert_eq!(config.site.as_ref().unwrap().title.as_ref().unwrap(), "Hexo Blog");
    assert_eq!(config.url.as_ref().unwrap(), "https://example.com");
    assert_eq!(config.root.as_ref().unwrap(), "/");
    assert_eq!(config.theme.as_ref().unwrap(), "landscape");
    assert_eq!(config.deploy.as_ref().unwrap().r#type.as_ref().unwrap(), "git");
    assert_eq!(config.deploy.as_ref().unwrap().repo.as_ref().unwrap(), "https://github.com/example/example.github.io.git");
    assert_eq!(config.deploy.as_ref().unwrap().branch.as_ref().unwrap(), "main");
    assert_eq!(config.writing.as_ref().unwrap().new_post_name.as_ref().unwrap(), ":title.md");
    assert_eq!(config.server.as_ref().unwrap().port.unwrap(), 4000);
    assert_eq!(config.pagination.as_ref().unwrap().per_page.unwrap(), 10);
    assert_eq!(config.directory.as_ref().unwrap().source_dir.as_ref().unwrap(), "source");
    assert_eq!(config.plugins.as_ref().unwrap().len(), 2);
    assert_eq!(config.plugins.as_ref().unwrap()[0], "hexo-generator-feed");
    assert_eq!(config.plugins.as_ref().unwrap()[1], "hexo-generator-sitemap");
}

#[test]
fn test_load_config_from_dir() {
    // 创建临时目录
    let temp_dir = tempfile::Builder::new().prefix("hexo-test-").tempdir().expect("Failed to create temp directory");

    // 创建 _config.yml 文件
    let config_path = temp_dir.path().join("_config.yml");
    let config_content = r#"
# Site
site:
  title: Test Blog
  author: Test Author

# URL
url: https://test.com
"#;

    std::fs::write(&config_path, config_content).expect("Failed to write config file");

    // 从目录加载配置
    let config = HexoConfig::load_from_dir(temp_dir.path()).expect("Failed to load config from directory");

    // 验证配置是否正确加载
    assert_eq!(config.site.as_ref().unwrap().title.as_ref().unwrap(), "Test Blog");
    assert_eq!(config.url.as_ref().unwrap(), "https://test.com");
}

#[test]
fn test_default_config() {
    // 创建临时目录，不包含配置文件
    let temp_dir = tempfile::Builder::new().prefix("hexo-test-").tempdir().expect("Failed to create temp directory");

    // 从目录加载配置（应该返回默认配置）
    let config = HexoConfig::load_from_dir(temp_dir.path()).expect("Failed to load default config");

    // 验证默认配置
    assert!(config.site.is_none());
    assert!(config.url.is_none());
    assert!(config.root.is_none());
    assert!(config.theme.is_none());
    assert!(config.deploy.is_none());
    assert!(config.writing.is_none());
    assert!(config.server.is_none());
    assert!(config.date_format.is_none());
    assert!(config.time_format.is_none());
    assert!(config.pagination.is_none());
    assert!(config.directory.is_none());
    assert!(config.plugins.is_none());
    assert!(config.theme_config.is_none());
}
