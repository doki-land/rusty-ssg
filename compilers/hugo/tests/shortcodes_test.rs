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
        assert_eq!(sc.name, "alert");
        assert_eq!(sc.shortcode_type, ShortcodeType::Markdown);
        assert_eq!(sc.inner.as_deref(), Some("Hello"));
    }
}

#[test]
fn test_parser_self_closing() {
    let parser = ShortcodeParser::new();
    let result = parser.parse_text("{{< youtube id=\"123\" / >}}").unwrap();

    assert_eq!(result.len(), 1);
    if let TextFragment::Shortcode(sc) = &result[0] {
        assert_eq!(sc.name, "youtube");
        assert_eq!(sc.params.get_named("id"), Some("123"));
        assert!(sc.inner.is_none());
    }
}

#[test]
fn test_parser_mixed_text_and_shortcodes() {
    let parser = ShortcodeParser::new();
    let result = parser.parse_text("Before {{< tip >}}Inside{{< /tip >}} After").unwrap();

    assert_eq!(result.len(), 3);
    if let TextFragment::Text(t) = &result[0] {
        assert_eq!(t, "Before ");
    }
    if let TextFragment::Shortcode(sc) = &result[1] {
        assert_eq!(sc.name, "tip");
        assert_eq!(sc.inner.as_deref(), Some("Inside"));
    }
    if let TextFragment::Text(t) = &result[2] {
        assert_eq!(t, " After");
    }
}

#[test]
fn test_execute_highlight() {
    let registry = ShortcodeRegistry::default();
    let mut params = ShortcodeParams::new();
    params.add_positional("rust".to_string());

    let shortcode = Shortcode {
        name: "highlight".to_string(),
        shortcode_type: ShortcodeType::Raw,
        params,
        inner: Some("fn main() {}".to_string()),
    };

    let context = ShortcodeContext::new("test.md".to_string());
    let result = registry.execute(&shortcode, &context).unwrap();

    assert!(result.contains("language-rust"));
    assert!(result.contains("fn main() {}"));
}

#[test]
fn test_execute_figure() {
    let registry = ShortcodeRegistry::default();
    let mut params = ShortcodeParams::new();
    params.add_named("src".to_string(), "image.jpg".to_string());
    params.add_named("alt".to_string(), "Test Image".to_string());

    let shortcode = Shortcode { name: "figure".to_string(), shortcode_type: ShortcodeType::Raw, params, inner: None };

    let context = ShortcodeContext::new("test.md".to_string());
    let result = registry.execute(&shortcode, &context).unwrap();

    assert!(result.contains("<figure>"));
    assert!(result.contains("src=\"image.jpg\""));
    assert!(result.contains("alt=\"Test Image\""));
}

#[test]
fn test_execute_ref() {
    let registry = ShortcodeRegistry::default();
    let mut params = ShortcodeParams::new();
    params.add_positional("docs/page.md".to_string());

    let shortcode = Shortcode {
        name: "ref".to_string(),
        shortcode_type: ShortcodeType::Raw,
        params,
        inner: Some("Click here".to_string()),
    };

    let context = ShortcodeContext::new("test.md".to_string());
    let result = registry.execute(&shortcode, &context).unwrap();

    assert!(result.contains("<a href=\"/docs/page.md\">Click here</a>"));
}

#[test]
fn test_execute_relref() {
    let registry = ShortcodeRegistry::default();
    let mut params = ShortcodeParams::new();
    params.add_positional("page.md".to_string());

    let shortcode = Shortcode { name: "relref".to_string(), shortcode_type: ShortcodeType::Raw, params, inner: None };

    let context = ShortcodeContext::new("test.md".to_string());
    let result = registry.execute(&shortcode, &context).unwrap();

    assert!(result.contains("<a href=\"page.md\">page.md</a>"));
}

#[test]
fn test_execute_alert() {
    let registry = ShortcodeRegistry::default();
    let mut params = ShortcodeParams::new();
    params.add_named("title".to_string(), "Warning!".to_string());

    let shortcode = Shortcode {
        name: "alert".to_string(),
        shortcode_type: ShortcodeType::Markdown,
        params,
        inner: Some("This is a warning".to_string()),
    };

    let context = ShortcodeContext::new("test.md".to_string());
    let result = registry.execute(&shortcode, &context).unwrap();

    assert!(result.contains("alert-warning"));
    assert!(result.contains("Warning!"));
    assert!(result.contains("This is a warning"));
}

#[test]
fn test_execute_details() {
    let registry = ShortcodeRegistry::default();
    let mut params = ShortcodeParams::new();
    params.add_named("summary".to_string(), "Click to expand".to_string());
    params.add_named("open".to_string(), "true".to_string());

    let shortcode = Shortcode {
        name: "details".to_string(),
        shortcode_type: ShortcodeType::Raw,
        params,
        inner: Some("Hidden content".to_string()),
    };

    let context = ShortcodeContext::new("test.md".to_string());
    let result = registry.execute(&shortcode, &context).unwrap();

    assert!(result.contains("<details open>"));
    assert!(result.contains("<summary>Click to expand</summary>"));
    assert!(result.contains("Hidden content"));
}

#[test]
fn test_execute_code() {
    let registry = ShortcodeRegistry::default();
    let mut params = ShortcodeParams::new();
    params.add_positional("python".to_string());
    params.add_named("linenos".to_string(), "true".to_string());

    let shortcode = Shortcode {
        name: "code".to_string(),
        shortcode_type: ShortcodeType::Raw,
        params,
        inner: Some("print('Hello')".to_string()),
    };

    let context = ShortcodeContext::new("test.md".to_string());
    let result = registry.execute(&shortcode, &context).unwrap();

    assert!(result.contains("language-python"));
    assert!(result.contains("line-numbers"));
    assert!(result.contains("print(&#39;Hello&#39;)"));
}

#[test]
fn test_execute_youtube() {
    let registry = ShortcodeRegistry::default();
    let mut params = ShortcodeParams::new();
    params.add_positional("dQw4w9WgXcQ".to_string());

    let shortcode = Shortcode { name: "youtube".to_string(), shortcode_type: ShortcodeType::Raw, params, inner: None };

    let context = ShortcodeContext::new("test.md".to_string());
    let result = registry.execute(&shortcode, &context).unwrap();

    assert!(result.contains("youtube-video"));
    assert!(result.contains("dQw4w9WgXcQ"));
}

#[test]
fn test_execute_tabs_and_tab() {
    let registry = ShortcodeRegistry::default();

    let mut tab_params = ShortcodeParams::new();
    tab_params.add_positional("Tab 1".to_string());
    tab_params.add_named("active".to_string(), "true".to_string());

    let tab_shortcode = Shortcode {
        name: "tab".to_string(),
        shortcode_type: ShortcodeType::Raw,
        params: tab_params,
        inner: Some("Tab 1 content".to_string()),
    };

    let context = ShortcodeContext::new("test.md".to_string());
    let tab_result = registry.execute(&tab_shortcode, &context).unwrap();

    assert!(tab_result.contains("tab active"));
    assert!(tab_result.contains("data-tab-name=\"Tab 1\""));
    assert!(tab_result.contains("Tab 1 content"));

    let mut tabs_params = ShortcodeParams::new();
    tabs_params.add_named("id".to_string(), "my-tabs".to_string());

    let tabs_shortcode = Shortcode {
        name: "tabs".to_string(),
        shortcode_type: ShortcodeType::Raw,
        params: tabs_params,
        inner: Some(tab_result),
    };

    let tabs_result = registry.execute(&tabs_shortcode, &context).unwrap();
    assert!(tabs_result.contains("tabs"));
    assert!(tabs_result.contains("id=\"my-tabs\""));
}

#[test]
fn test_shortcode_not_found() {
    let registry = ShortcodeRegistry::default();
    let shortcode = Shortcode {
        name: "nonexistent".to_string(),
        shortcode_type: ShortcodeType::Raw,
        params: ShortcodeParams::new(),
        inner: None,
    };

    let context = ShortcodeContext::new("test.md".to_string());
    let result = registry.execute(&shortcode, &context);

    assert!(result.is_err());
    if let Err(ShortcodeError::ShortcodeNotFound { name }) = result {
        assert_eq!(name, "nonexistent");
    }
}
