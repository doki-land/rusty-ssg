use hugo::compiler::hugo_template::*;
use tempfile::tempdir;

#[test]
fn test_engine_creation() {
    let dir = tempdir().unwrap();
    let site = HugoSite::new();
    let engine = HugoTemplateEngine::new(dir.path(), site);
    assert!(engine.is_ok());
}

#[test]
fn test_add_and_render_template() {
    let dir = tempdir().unwrap();
    let site = HugoSite::new().with_title("Test Site".to_string());
    let mut engine = HugoTemplateEngine::new(dir.path(), site).unwrap();

    engine.add_template("test", "Hello, {{ site.title }}!").unwrap();

    let page = HugoPage::new();
    let result = engine.render("test", page).unwrap();
    assert_eq!(result, "Hello, Test Site!");
}
