use vitepress::plugin::katex::KaTeXPlugin;
use vitepress::plugin::{PluginContext, VitePressPlugin};

#[test]
fn test_after_render() {
    let plugin = KaTeXPlugin::new();
    let context = PluginContext::from_content("Text with $inline$ and $$block$$ formulas.".to_string(), "test.md".to_string());
    let result = plugin.after_render(context);
    assert!(result.content.contains("katex"));
}
