//! 综合测试

use mkdocs::{CompileResult, compile_batch, compile_single, compiler::HtmlRenderer, types::MkDocsConfig};
use oak_yaml;
use std::collections::HashMap;

#[test]
fn test_full_workflow() {
    let yaml_config = r#"
site_name: Test Workflow Site
site_description: A comprehensive test
"#;

    let config = MkDocsConfig::from_yaml(yaml_config).unwrap();
    assert_eq!(config.site_name(), "Test Workflow Site");

    let renderer = HtmlRenderer::new();

    let markdown = "# ".to_string() + &config.site_name() + "\n\n" + &config.site_description.clone().unwrap_or_default();
    let html = renderer.render(&markdown);
    assert!(!html.is_empty());
}

#[test]
fn test_compile_result() {
    let result = CompileResult::success(HashMap::new(), 100);
    assert!(result.success);
    assert_eq!(result.compile_time_ms, 100);
    assert!(result.errors.is_empty());

    let errors = vec!["Error 1".to_string(), "Error 2".to_string()];
    let result = CompileResult::failure(errors, 50);
    assert!(!result.success);
    assert_eq!(result.compile_time_ms, 50);
    assert_eq!(result.errors.len(), 2);
}

#[test]
fn test_compile_result_json() {
    let result = CompileResult::success(HashMap::new(), 42);
    let json = result.to_json().unwrap();
    assert!(!json.is_empty());

    let json_pretty = result.to_json_pretty().unwrap();
    assert!(!json_pretty.is_empty());
    assert!(json_pretty.len() > json.len());
}

#[test]
fn test_batch_with_real_example() {
    let mut documents = HashMap::new();
    documents.insert("home.md".to_string(), "# Home\nWelcome to home".to_string());
    documents.insert("docs/intro.md".to_string(), "## Introduction\nThis is intro".to_string());
    documents.insert("docs/api.md".to_string(), "## API Reference\nAPI docs".to_string());

    let result = compile_batch(&documents);

    assert!(result.success);
    assert_eq!(result.documents.len(), 3);

    for (path, doc) in &result.documents {
        assert!(doc.content.len() > 0);
        assert_eq!(doc.meta.path, *path);
    }
}

#[test]
fn test_error_handling() {
    let invalid_config = MkDocsConfig { site_name: "".to_string(), ..Default::default() };

    let validation = invalid_config.validate();
    assert!(validation.is_err());
}

#[test]
fn test_config_serde_roundtrip() {
    let original = MkDocsConfig {
        site_name: "Roundtrip Test".to_string(),
        site_description: Some("Test description".to_string()),
        ..Default::default()
    };

    let yaml = oak_yaml::language::to_string(&original).unwrap();
    let parsed = MkDocsConfig::from_yaml(&yaml).unwrap();

    assert_eq!(original.site_name, parsed.site_name);
    assert_eq!(original.site_description, parsed.site_description);
}
