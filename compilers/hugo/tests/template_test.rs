use hugo::compiler::hugo_template::*;
use tempfile::tempdir;

#[test]
fn test_template_engine_creation() {
    let dir = tempdir().unwrap();
    let site = HugoSite::new().with_title("Test Site".to_string());
    let engine = HugoTemplateEngine::new(dir.path(), site).unwrap();
    assert_eq!(engine.site().title, Some("Test Site".to_string()));
}

#[test]
fn test_add_template() {
    let dir = tempdir().unwrap();
    let site = HugoSite::new();
    let mut engine = HugoTemplateEngine::new(dir.path(), site).unwrap();

    engine.add_template("test.html", "Hello {{site.title}}").unwrap();
}

#[test]
fn test_render_simple_template() {
    let dir = tempdir().unwrap();
    let site = HugoSite::new().with_title("My Blog".to_string());
    let mut engine = HugoTemplateEngine::new(dir.path(), site).unwrap();

    engine.add_template("simple.html", "<h1>{{site.title}}</h1>").unwrap();

    let page = HugoPage::new();
    let result = engine.render("simple.html", page).unwrap();

    assert_eq!(result, "<h1>My Blog</h1>");
}

#[test]
fn test_render_with_page_context() {
    let dir = tempdir().unwrap();
    let site = HugoSite::new();
    let mut engine = HugoTemplateEngine::new(dir.path(), site).unwrap();

    engine.add_template("page.html", "<h1>{{page.title}}</h1><div>{{page.content}}</div>").unwrap();

    let page = HugoPage::new().with_title("Test Page".to_string()).with_content("<p>Hello World</p>".to_string());

    let result = engine.render("page.html", page).unwrap();

    assert!(result.contains("Test Page"));
    assert!(result.contains("Hello World"));
}

#[test]
fn test_upper_function() {
    let dir = tempdir().unwrap();
    let site = HugoSite::new();
    let mut engine = HugoTemplateEngine::new(dir.path(), site).unwrap();

    engine.add_template("upper.html", "{{upper \"hello world\"}}").unwrap();

    let page = HugoPage::new();
    let result = engine.render("upper.html", page).unwrap();

    assert_eq!(result, "HELLO WORLD");
}

#[test]
fn test_lower_function() {
    let dir = tempdir().unwrap();
    let site = HugoSite::new();
    let mut engine = HugoTemplateEngine::new(dir.path(), site).unwrap();

    engine.add_template("lower.html", "{{lower \"HELLO WORLD\"}}").unwrap();

    let page = HugoPage::new();
    let result = engine.render("lower.html", page).unwrap();

    assert_eq!(result, "hello world");
}

#[test]
fn test_truncate_function() {
    let dir = tempdir().unwrap();
    let site = HugoSite::new();
    let mut engine = HugoTemplateEngine::new(dir.path(), site).unwrap();

    engine.add_template("truncate.html", "{{truncate \"Long text here\" 5}}").unwrap();

    let page = HugoPage::new();
    let result = engine.render("truncate.html", page).unwrap();

    assert!(result.contains("..."));
}

#[test]
fn test_default_function() {
    let dir = tempdir().unwrap();
    let site = HugoSite::new();
    let mut engine = HugoTemplateEngine::new(dir.path(), site).unwrap();

    engine.add_template("default.html", "{{default \"\" \"Default Value\"}}").unwrap();

    let page = HugoPage::new();
    let result = engine.render("default.html", page).unwrap();

    assert_eq!(result, "Default Value");
}

#[test]
fn test_template_partials() {
    let dir = tempdir().unwrap();
    let site = HugoSite::new().with_title("My Site".to_string());
    let mut engine = HugoTemplateEngine::new(dir.path(), site).unwrap();

    engine.add_template("partials/header.html", "<header>{{site.title}}</header>").unwrap();
    engine.add_template("page.html", "{{> partials/header}}<main>Content</main>").unwrap();

    let page = HugoPage::new();
    let result = engine.render("page.html", page).unwrap();

    assert!(result.contains("<header>My Site</header>"));
    assert!(result.contains("<main>Content</main>"));
}

#[test]
fn test_template_inheritance() {
    let dir = tempdir().unwrap();
    let site = HugoSite::new();
    let mut engine = HugoTemplateEngine::new(dir.path(), site).unwrap();

    engine
        .add_template("base.html", "<html><head>{{#> head}}{{/head}}</head><body>{{#> body}}{{/body}}</body></html>")
        .unwrap();
    engine.add_template("page.html", "{{#> base}}{{#*inline \"head\"}}<title>Page</title>{{/inline}}{{#*inline \"body\"}}<h1>Hello</h1>{{/inline}}{{/base}}").unwrap();

    let page = HugoPage::new();
    let result = engine.render("page.html", page).unwrap();

    assert!(result.contains("<title>Page</title>"));
    assert!(result.contains("<h1>Hello</h1>"));
}
