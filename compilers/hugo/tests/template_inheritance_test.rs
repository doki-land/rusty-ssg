use hugo::compiler::hugo_template::{HugoPage, HugoSite, HugoTemplateEngine};

#[test]
fn test_template_inheritance() {
    // 创建站点配置
    let site = HugoSite::new()
        .with_title("Test Site".to_string())
        .with_language_code("en".to_string())
        .with_copyright("© 2024 Test Site".to_string());

    // 创建页面
    let page = HugoPage::new()
        .with_title("Home Page".to_string())
        .with_description("Welcome to the test site".to_string())
        .with_content("<p>This is the home page content</p>".to_string());

    // 创建模板引擎
    let mut engine = HugoTemplateEngine::new("e:\\\\rusty-ssg\\compilers\\hugo", site).unwrap();

    // 加载模板
    engine.load_template("templates/baseof.html").unwrap();
    engine.load_template("templates/index.html").unwrap();

    // 渲染模板
    let result = engine.render("templates/index.html", page).unwrap();

    // 验证渲染结果
    assert!(result.contains("<!DOCTYPE html>"));
    assert!(result.contains("<html lang=\"en\">"));
    assert!(result.contains("<title>Home Page - Test Site</title>"));
    assert!(result.contains("<meta name=\"description\" content=\"Welcome to the test site\">"));
    assert!(result.contains("<h1>Test Site</h1>"));
    assert!(result.contains("<h2>Home Page</h2>"));
    assert!(result.contains("<p>Welcome to the test site</p>"));
    assert!(result.contains("<p>This is the home page content</p>"));
    assert!(result.contains("<p>© 2024 Test Site</p>"));
}
