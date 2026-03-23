use gatsby::{ConfigError, ConfigValidation, GatsbyConfig, PluginConfig, SiteMetadata, DevelopConfig, TrailingSlash};
use std::collections::HashMap;

#[test]
fn test_config_creation() {
    let config = GatsbyConfig::new();
    assert!(config.site_metadata.is_none());
    assert!(config.path_prefix.is_none());
    assert!(config.plugins.is_none());
}

#[test]
fn test_config_with_site_metadata() {
    let site_metadata =
        SiteMetadata::new().with_title("Test Site".to_string()).with_description("Test Description".to_string());

    let config = GatsbyConfig::new().with_site_metadata(site_metadata);

    assert!(config.site_metadata.is_some());
    let metadata = config.site_metadata.as_ref().unwrap();
    assert_eq!(metadata.title, Some("Test Site".to_string()));
    assert_eq!(metadata.description, Some("Test Description".to_string()));
}

#[test]
fn test_config_validation_success() {
    let config = GatsbyConfig::new();
    let result = config.validate();
    assert!(result.is_ok());
}

#[test]
fn test_config_validation_empty_path_prefix() {
    let mut config = GatsbyConfig::new();
    config.path_prefix = Some("".to_string());
    let result = config.validate();
    assert!(result.is_err());
    match result {
        Err(ConfigError::ValidationError { cause }) => {
            assert!(cause.contains("empty"));
        }
        _ => panic!("Expected validation error for empty path prefix"),
    }
}

#[test]
fn test_plugin_config_name() {
    let plugin = PluginConfig::Name("gatsby-plugin-test".to_string());
    let result = plugin.validate();
    assert!(result.is_ok());
}

#[test]
fn test_plugin_config_with_options() {
    let plugin = PluginConfig::WithOptions { resolve: "gatsby-plugin-test".to_string(), options: None };
    let result = plugin.validate();
    assert!(result.is_ok());
}

#[test]
fn test_plugin_config_empty_name() {
    let plugin = PluginConfig::Name("".to_string());
    let result = plugin.validate();
    assert!(result.is_err());
}

#[test]
fn test_config_serialization_json() {
    let config = GatsbyConfig::new().with_site_metadata(SiteMetadata::new().with_title("Test".to_string()));
    let json = config.to_json();
    assert!(json.is_ok());
}

#[test]
fn test_config_serialization_yaml() {
    let config = GatsbyConfig::new().with_site_metadata(SiteMetadata::new().with_title("Test".to_string()));
    let yaml = config.to_yaml();
    assert!(yaml.is_ok());
}

#[test]
fn test_config_serialization_toml() {
    let config = GatsbyConfig::new().with_site_metadata(SiteMetadata::new().with_title("Test".to_string()));
    let toml = config.to_toml();
    assert!(toml.is_ok());
}

#[test]
fn test_load_json_config() {
    let json = r#"{
            "siteMetadata": {
                "title": "Test Site",
                "description": "A test site"
            },
            "plugins": ["gatsby-plugin-test"]
        }"#;

    let config = GatsbyConfig::load_from_json_str(json).unwrap();
    assert_eq!(config.site_title(), "Test Site");
    assert_eq!(config.site_description(), Some("A test site"));
}

#[test]
fn test_load_toml_config() {
    let toml = r#"
[siteMetadata]
title = "Test Site"
description = "A test site"

[[plugins]]
resolve = "gatsby-plugin-test"
"#;

    let config = GatsbyConfig::load_from_toml_str(toml).unwrap();
    assert_eq!(config.site_title(), "Test Site");
}

#[test]
fn test_plugin_config() {
    let plugin = PluginConfig::simple("gatsby-plugin-test".to_string());
    assert_eq!(plugin.name(), "gatsby-plugin-test");
    assert!(plugin.options().is_none());

    let mut options = HashMap::new();
    options.insert("key".to_string(), serde_json::json!("value"));
    let plugin_with_options = PluginConfig::with_options("gatsby-plugin-test".to_string(), options);
    assert_eq!(plugin_with_options.name(), "gatsby-plugin-test");
    assert!(plugin_with_options.options().is_some());
}

#[test]
fn test_develop_config_defaults() {
    let config = DevelopConfig::new();
    assert_eq!(config.get_host(), "localhost");
    assert_eq!(config.get_port(), 8000);
}
