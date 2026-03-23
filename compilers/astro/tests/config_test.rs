//! 配置模块测试

use astro::config::{AstroConfig, ConfigManager};
use std::fs;
use tempfile::tempdir;

#[test]
fn test_config_manager_creation() {
    // 创建配置管理器
    let _config_manager = ConfigManager::new();

    // 验证配置管理器创建成功
    assert!(true);
}

#[test]
fn test_config_manager_default() {
    // 创建默认配置管理器
    let _config_manager = ConfigManager::default();

    // 验证默认配置管理器创建成功
    assert!(true);
}

#[test]
fn test_config_manager_load_from_project() {
    // 创建临时目录
    let temp_dir = tempdir().unwrap();

    // 创建配置文件
    let config_file = temp_dir.path().join("astro.config.json");
    let config_content = r#"
    {
        "site": "https://example.com",
        "base": "/blog",
        "output": "static"
    }
    "#;
    fs::write(&config_file, config_content).unwrap();

    // 创建配置管理器
    let mut config_manager = ConfigManager::new();

    // 从项目目录加载配置
    let config = config_manager.load_from_project(temp_dir.path()).unwrap();

    // 验证配置
    assert_eq!(config.site, Some("https://example.com".to_string()));
    assert_eq!(config.base, Some("/blog".to_string()));
    assert_eq!(config.output, "static");
}

#[test]
fn test_config_manager_load_from_project_no_config() {
    // 创建临时目录
    let temp_dir = tempdir().unwrap();

    // 创建配置管理器
    let mut config_manager = ConfigManager::new();

    // 从项目目录加载配置（没有配置文件）
    let config = config_manager.load_from_project(temp_dir.path()).unwrap();

    // 验证默认配置
    assert_eq!(config.output, "static");
    assert_eq!(config.src_dir, "./src");
    assert_eq!(config.public_dir, "./public");
    assert_eq!(config.out_dir, "dist");
}

#[test]
fn test_astro_config_default() {
    // 创建默认 Astro 配置
    let config = AstroConfig::default();

    // 验证默认配置值
    assert_eq!(config.trailing_slash, "ignore");
    assert_eq!(config.output, "static");
    assert_eq!(config.src_dir, "./src");
    assert_eq!(config.public_dir, "./public");
    assert_eq!(config.out_dir, "dist");
    assert_eq!(config.cache_dir, "./node_modules/.astro");
    assert_eq!(config.compress_html, true);
    assert_eq!(config.scoped_style_strategy, "attribute");
    assert_eq!(config.build.format, "directory");
    assert_eq!(config.build.client, "./client");
    assert_eq!(config.build.server, "./server");
    assert_eq!(config.build.assets, "_astro");
    assert_eq!(config.build.server_entry, "entry.mjs");
    assert_eq!(config.build.redirects, true);
    assert_eq!(config.build.inline_stylesheets, "auto");
    assert_eq!(config.build.concurrency, 1);
    assert_eq!(config.server.port, 4321);
    assert_eq!(config.dev_toolbar.enabled, true);
    assert_eq!(config.dev_toolbar.placement, "bottom-center");
    assert_eq!(config.prefetch.default_strategy, "hover");
    assert_eq!(config.image.endpoint.route, "/_image");
    assert_eq!(config.image.service.entrypoint, "astro/assets/services/sharp");
    assert_eq!(config.image.responsive_styles, false);
    assert_eq!(config.image.fit, "cover");
    assert_eq!(config.image.object_position, "center");
    assert_eq!(config.markdown.gfm, true);
    assert_eq!(config.markdown.smartypants, true);
    assert_eq!(config.env.validate_secrets, false);
}
