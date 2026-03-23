use vitepress::plugin::{PluginContext, VitePressPlugin, katex::KaTeXPlugin};

#[test]
fn test_before_render() {
    let plugin = KaTeXPlugin::new();
    let context = PluginContext::from_content("Text with $inline$ and $$block$$ formulas.".to_string(), "test.md".to_string());
    let result = plugin.before_render(context);
    assert!(result.content.contains("katex"));
}
