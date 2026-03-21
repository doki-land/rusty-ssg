use vuepress::plugin::{PluginContext, PluginRegistry, VutexPlugin, katex::KaTeXPlugin};

#[test]
fn test_katex_plugin_creation() {
    let plugin = KaTeXPlugin::new();
    assert_eq!(plugin.meta().name, "vutex-plugin-katex");
    assert_eq!(plugin.meta().version, "0.1.0");
}

#[test]
fn test_katex_process_block_math() {
    let plugin = KaTeXPlugin::new();
    let input = "Some text $$\\sum_{i=1}^n i = \\frac{n(n+1)}{2}$$ more text";
    let output = plugin.process_block_math(input);
    assert!(output.contains("<div class=\"katex-block\">"));
    assert!(output.contains("\\sum_{i=1}^n i = \\frac{n(n+1)}{2}"));
}

#[test]
fn test_katex_process_inline_math() {
    let plugin = KaTeXPlugin::new();
    let input = "Einstein's equation $E=mc^2$ is famous.";
    let output = plugin.process_inline_math(input);
    assert!(output.contains("<span class=\"katex-inline\">"));
    assert!(output.contains("E=mc^2"));
}

#[test]
fn test_katex_before_render() {
    let plugin = KaTeXPlugin::new();
    let context = PluginContext::from_content("Text with $inline$ and $$block$$ formulas.".to_string(), "test.md".to_string());
    let result = plugin.before_render(context);
    assert!(result.content.contains("<span class=\"katex-inline\">inline</span>"));
    assert!(result.content.contains("<div class=\"katex-block\">block</div>"));
}

#[test]
fn test_plugin_registry() {
    let mut registry = PluginRegistry::new();
    assert_eq!(registry.plugin_count(), 0);

    registry.register(KaTeXPlugin::new());
    assert_eq!(registry.plugin_count(), 1);

    let metas = registry.plugin_metas();
    assert_eq!(metas.len(), 1);
    assert_eq!(metas[0].name, "vutex-plugin-katex");
}

#[test]
fn test_plugin_registry_before_render_all() {
    let mut registry = PluginRegistry::new();
    registry.register(KaTeXPlugin::new());

    let context = PluginContext::from_content("Test $inline$ and $$block$$".to_string(), "test.md".to_string());
    let result = registry.before_render_all(context);

    assert!(result.content.contains("<span class=\"katex-inline\">inline</span>"));
    assert!(result.content.contains("<div class=\"katex-block\">block</div>"));
}
