use crate::types::config::*;
use std::io::Write;
use tempfile::NamedTempFile;

#[test]
fn test_hugo_config_default() {
    let config = HugoConfig::default();
    assert!(config.base_url.is_none());
    assert!(config.title.is_none());
}

#[test]
fn test_hugo_config_builder() {
    let config = HugoConfig::new()
        .with_base_url("https://example.com/".to_string())
        .with_title("My Site".to_string())
        .with_theme("ananke".to_string());

    assert_eq!(config.base_url, Some("https://example.com/".to_string()));
    assert_eq!(config.title, Some("My Site".to_string()));
    assert_eq!(config.theme, Some("ananke".to_string()));
}

#[test]
fn test_taxonomy_validation() {
    let valid = TaxonomyConfig::new("tags".to_string());
    assert!(valid.validate().is_ok());

    let invalid = TaxonomyConfig::new("".to_string());
    assert!(invalid.validate().is_err());
}

#[test]
fn test_menu_item_validation() {
    let valid = MenuItem::new().with_name("Home".to_string()).with_url("/".to_string());
    assert!(valid.validate().is_ok());

    let invalid = MenuItem::new();
    assert!(invalid.validate().is_err());
}

#[test]
fn test_server_port_validation() {
    let mut server = ServerConfig::default();
    server.port = Some(8080);
    assert!(server.validate().is_ok());

    server.port = Some(0);
    assert!(server.validate().is_err());

    server.port = Some(65536);
    assert!(server.validate().is_err());
}

#[test]
fn test_config_validation() {
    let mut config = HugoConfig::new();
    config.base_url = Some("".to_string());
    assert!(config.validate().is_err());

    config.base_url = Some("https://example.com/".to_string());
    config.title = Some("".to_string());
    assert!(config.validate().is_err());

    config.title = Some("My Site".to_string());
    assert!(config.validate().is_ok());
}
