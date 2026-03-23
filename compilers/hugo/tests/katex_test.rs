use hugo::plugin::katex::*;
use hugo::plugin::VutexPlugin;
use hugo::plugin::PluginContext;
use hugo::NargoValue;

#[test]
fn test_before_render() {
    let plugin = KaTeXPlugin::new();
    let context = PluginContext::new("Text with $inline$ and $$block$$ formulas.".to_string(), std::collections::HashMap::new(), "test.md".to_string());
    let result = plugin.before_render(context);
    assert!(result.content.contains("inline"));
    assert!(result.content.contains("block"));
}
