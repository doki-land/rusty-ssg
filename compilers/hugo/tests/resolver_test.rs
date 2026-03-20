use crate::compiler::hugo_template::resolver::*;
use tempfile::tempdir;

#[test]
fn test_resolver_creation() {
    let dir = tempdir().unwrap();
    let resolver = TemplateResolver::new(dir.path());
    assert_eq!(resolver.root_dir(), dir.path());
    assert!(resolver.theme().is_none());
}

#[test]
fn test_set_theme() {
    let dir = tempdir().unwrap();
    let mut resolver = TemplateResolver::new(dir.path());
    resolver.set_theme("my-theme".to_string());
    assert_eq!(resolver.theme(), Some(&"my-theme".to_string()));
}

#[test]
fn test_template_not_found() {
    let dir = tempdir().unwrap();
    let resolver = TemplateResolver::new(dir.path());
    let result = resolver.resolve_template("nonexistent.html");
    assert!(matches!(result, Err(TemplateResolverError::TemplateNotFound(_))));
}

#[test]
fn test_list_templates_empty() {
    let dir = tempdir().unwrap();
    let resolver = TemplateResolver::new(dir.path());
    let templates = resolver.list_templates();
    assert!(templates.is_empty());
}
