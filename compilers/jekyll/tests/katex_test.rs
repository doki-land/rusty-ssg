use nargo_document::{KaTeXPlugin, PluginContext, DocumentPlugin};

#[test]
fn test_before_render() {
    let plugin = KaTeXPlugin::new();
    let context = PluginContext::from_content("Text with $inline$ and $$block$$ formulas.".to_string(), "test.md".to_string());
    let result = plugin.before_render(context);
    assert!(result.content.contains("inline"));
    assert!(result.content.contains("block"));
}

#[test]
fn test_after_render() {
    let plugin = KaTeXPlugin::new();
    let context = PluginContext::from_content("Text with $inline$ and $$block$$ formulas.".to_string(), "test.md".to_string());
    let result = plugin.after_render(context);
    assert!(result.content.contains("inline"));
    assert!(result.content.contains("block"));
}
