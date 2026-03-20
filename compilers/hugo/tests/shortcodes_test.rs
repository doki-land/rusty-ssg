use hugo::compiler::shortcodes::*;

#[test]
fn test_shortcode_params_creation() {
    let mut params = ShortcodeParams::new();
    params.add_positional("pos1".to_string());
    params.add_positional("pos2".to_string());
    params.add_named("key1".to_string(), "value1".to_string());
    params.add_named("key2".to_string(), "value2".to_string());

    assert_eq!(params.get_positional(0), Some("pos1"));
    assert_eq!(params.get_positional(1), Some("pos2"));
    assert_eq!(params.get_positional(2), None);
    assert_eq!(params.get_named("key1"), Some("value1"));
    assert_eq!(params.get_named("key2"), Some("value2"));
    assert_eq!(params.get("key1", 0), Some("value1"));
    assert_eq!(params.get("nonexistent", 0), Some("pos1"));
}

#[test]
fn test_shortcode_context() {
    let mut context = ShortcodeContext::new("/test/path.md".to_string());
    context.add_extra("key".to_string(), "value".to_string());

    assert_eq!(context.document_path, "/test/path.md");
    assert_eq!(context.get_extra("key"), Some("value"));
    assert_eq!(context.get_extra("nonexistent"), None);
}

#[test]
fn test_shortcode_registry() {
    let registry = ShortcodeRegistry::default();

    assert!(registry.has("highlight"));
    assert!(registry.has("figure"));
    assert!(registry.has("ref"));
    assert!(registry.has("relref"));
    assert!(registry.has("alert"));
    assert!(registry.has("notice"));
    assert!(registry.has("tip"));
    assert!(registry.has("warning"));
    assert!(registry.has("error"));
    assert!(registry.has("info"));
    assert!(registry.has("details"));
    assert!(registry.has("blockquote"));
    assert!(registry.has("tabs"));
    assert!(registry.has("tab"));
    assert!(registry.has("code"));
    assert!(registry.has("gist"));
    assert!(registry.has("instagram"));
    assert!(registry.has("tiktok"));
    assert!(registry.has("twitter"));
    assert!(registry.has("vimeo"));
    assert!(registry.has("youtube"));
}

#[test]
fn test_parser_simple_shortcode() {
    let parser = ShortcodeParser::new();
    let result = parser.parse_text("{{< figure src=\"image.jpg\" >}}").unwrap();
    
    assert_eq!(result.len(), 1);
    if let TextFragment::Shortcode(sc) = &result[0] {
        assert_eq!(sc.name, "figure");
        assert_eq!(sc.shortcode_type, ShortcodeType::Raw);
        assert_eq!(sc.params.get_named("src"), Some("image.jpg"));
        assert!(sc.inner.is_none());
    }
}

#[test]
fn test_parser_markdown_shortcode() {
    let parser = ShortcodeParser::new();
    let result = parser.parse_text("{{% alert %}}Hello{{% /alert %}}").unwrap();
    
    assert_eq!(result.len(), 1);
    if let TextFragment::Shortcode(sc) = &result[0] {
        assert_eq!(