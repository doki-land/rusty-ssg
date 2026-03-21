use crate::jekyll::defaults::{DefaultConfig, DefaultsManager, FrontMatter, Scope};
use serde_json::{Map, json};
use std::{collections::HashMap, path::Path};

#[test]
fn test_scope_from_map() {
    let map = Map::from_iter(vec![
        ("path".to_string(), json!("posts/")),
        ("collection".to_string(), json!("posts")),
        ("layout".to_string(), json!("post")),
    ]);

    let scope = Scope::from_map(&map).unwrap();
    assert_eq!(scope.path, Some("posts/".to_string()));
    assert_eq!(scope.collection, Some("posts".to_string()));
    assert_eq!(scope.layout, Some("post".to_string()));
}

#[test]
fn test_scope_matches_path() {
    let scope = Scope::new().with_path("posts/".to_string());
    let path = Path::new("posts/2024-01-01-test.md");
    assert!(scope.matches(path, None, None));
}

#[test]
fn test_scope_matches_collection() {
    let scope = Scope::new().with_collection("posts".to_string());
    let path = Path::new("any/path.md");
    assert!(scope.matches(path, Some("posts"), None));
    assert!(!scope.matches(path, Some("drafts"), None));
}

#[test]
fn test_apply_defaults() {
    let mut manager = DefaultsManager::new();

    let values =
        HashMap::from_iter(vec![("layout".to_string(), json!("default")), ("author".to_string(), json!("Test Author"))]);

    manager.add_default(DefaultConfig::new(Scope::new(), values));

    let mut front_matter = FrontMatter::new(String::new(), HashMap::new(), String::new());

    manager.apply_defaults(Path::new("test.md"), None, &mut front_matter);

    assert_eq!(front_matter.get_str("layout"), Some("default"));
    assert_eq!(front_matter.get_str("author"), Some("Test Author"));
}

#[test]
fn test_apply_defaults_no_override() {
    let mut manager = DefaultsManager::new();

    let values = HashMap::from_iter(vec![("layout".to_string(), json!("default"))]);

    manager.add_default(DefaultConfig::new(Scope::new(), values));

    let mut front_matter =
        FrontMatter::new(String::new(), HashMap::from_iter(vec![("layout".to_string(), json!("custom"))]), String::new());

    manager.apply_defaults(Path::new("test.md"), None, &mut front_matter);

    assert_eq!(front_matter.get_str("layout"), Some("custom"));
}
