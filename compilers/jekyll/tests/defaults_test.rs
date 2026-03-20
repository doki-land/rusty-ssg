
//! 默认值配置模块测试
//!
//! 测试 Jekyll defaults 配置的解析和应用功能

use jekyll::*;
use serde_json::json;
use std::collections::HashMap;
use std::path::Path;

#[test]
fn test_default_config_from_map() {
    let map = HashMap::from_iter(vec![
        (
            "scope".to_string(),
            json!({
                "path": "posts/",
                "collection": "posts",
                "layout": "post"
            }),
        ),
        (
            "values".to_string(),
            json!({
                "layout": "post",
                "author": "Test Author"
            }),
        ),
    ]);

    let default_config = DefaultConfig::from_map(&amp;map).unwrap();
    assert_eq!(default_config.scope.path, Some("posts/".to_string()));
    assert_eq!(default_config.scope.collection, Some("posts".to_string()));
    assert_eq!(default_config.scope.layout, Some("post".to_string()));
    assert_eq!(default_config.values.get("layout"), Some(&amp;json!("post")));
    assert_eq!(default_config.values.get("author"), Some(&amp;json!("Test Author")));
}

#[test]
fn test_scope_matches_path_glob() {
    let scope = Scope::new().with_path("posts/**/*.md".to_string());
    assert!(scope.matches(Path::new("posts/2024-01-01-test.md"), None, None));
    assert!(scope.matches(Path::new("posts/subdir/test.md"), None, None));
    assert!(!scope.matches(Path::new("drafts/2024-01-01-test.md"), None, None));
}

#[test]
fn test_scope_matches_collection_and_layout() {
    let scope = Scope::new()
        .with_collection("posts".to_string())
        .with_layout("post".to_string());

    let path = Path::new("any/path.md");
    assert!(scope.matches(path, Some("posts"), Some("post")));
    assert!(!scope.matches(path, Some("drafts"), Some("post")));
    assert!(!scope.matches(path, Some("posts"), Some("page")));
}

#[test]
fn test_apply_multiple_defaults() {
    let mut manager = DefaultsManager::new();

    manager.add_default(DefaultConfig::new(
        Scope::new(),
        HashMap::from_iter(vec![
            ("layout".to_string(), json!("default")),
            ("category".to_string(), json!("general")),
        ]),
    ));

    manager.add_default(DefaultConfig::new(
        Scope::new().with_path("posts/".to_string()),
        HashMap::from_iter(vec![
            ("layout".to_string(), json!("post")),
            ("author".to_string(), json!("Test Author")),
        ]),
    ));

    let mut front_matter = FrontMatter::new(String::new(), HashMap::new(), String::new());
    manager.apply_defaults(Path::new("posts/test.md"), None, &amp;mut front_matter);

    assert_eq!(front_matter.get_str("layout"), Some("post"));
    assert_eq!(front_matter.get_str("category"), Some("general"));
    assert_eq!(front_matter.get_str("author"), Some("Test Author"));
}

#[test]
fn test_defaults_from_config() {
    let mut config = JekyllConfig::new();
    config.defaults = Some(vec![
        HashMap::from_iter(vec![
            (
                "scope".to_string(),
                json!({ "path": "posts/" }),
            ),
            (
                "values".to_string(),
                json!({ "layout": "post" }),
            ),
        ]),
    ]);

    let manager = DefaultsManager::from_config(&amp;config).unwrap();
    assert_eq!(manager.defaults().len(), 1);
}

#[test]
fn test_merge_values_no_override() {
    let mut target = HashMap::from_iter(vec![
        ("layout".to_string(), json!("custom")),
    ]);

    let source = HashMap::from_iter(vec![
        ("layout".to_string(), json!("default")),
        ("author".to_string(), json!("Test Author")),
    ]);

    let mut front_matter = FrontMatter::new(String::new(), target, String::new());
    let mut manager = DefaultsManager::new();
    manager.add_default(DefaultConfig::new(Scope::new(), source));
    
    manager.apply_defaults(Path::new("test.md"), None, &amp;mut front_matter);

    assert_eq!(front_matter.get_str("layout"), Some("custom"));
    assert_eq!(front_matter.get_str("author"), Some("Test Author"));
}
