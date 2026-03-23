use hugo::compiler::hugo_template::context::*;

#[test]
fn test_site_creation() {
    let site = HugoSite::new().with_title("Test Site".to_string()).with_base_url("https://example.com".to_string());

    assert_eq!(site.title, Some("Test Site".to_string()));
    assert_eq!(site.base_url, Some("https://example.com".to_string()));
}

#[test]
fn test_page_creation() {
    let page = HugoPage::new()
        .with_title("Test Page".to_string())
        .with_content("<p>Hello World</p>".to_string())
        .add_tag("test".to_string())
        .add_tag("example".to_string());

    assert_eq!(page.title, Some("Test Page".to_string()));
    assert_eq!(page.tags.len(), 2);
}

#[test]
fn test_params() {
    let mut site_params = SiteParams::new().with_param("foo".to_string(), "bar".to_string());

    assert_eq!(site_params.get("foo"), Some(&"bar".to_string()));
}
