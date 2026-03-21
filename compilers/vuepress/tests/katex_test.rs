use vuepress::plugin::katex::KaTeXPlugin;
use vuepress::plugin::{PluginContext, VutexPlugin};

#[test]
fn test_process_block_math() {
    let plugin = KaTeXPlugin::new();
    let input = "Some text $$\\sum_{i=1}^n i = \\frac{n(n+1)}{2}$$ more text";
    let output = plugin.process_block_math(input);
    assert!(output.contains("<div class=\"katex-block\">"));
    assert!(output.contains("\\sum_{i=1}^n i = \\frac{n(n+1)}{2}"));
}

#[test]
fn test_process_inline_math() {
    let plugin = KaTeXPlugin::new();
    let input = "Einstein's equation $E=mc^2$ is famous.";
    let output = plugin.process_inline_math(input);
    assert!(output.contains("<span class=\"katex-inline\">"));
    assert!(output.contains("E=mc^2"));
}

#[test]
fn test_before_render() {
    let plugin = KaTeXPlugin::new();
    let context = PluginContext::from_content("Text with $inline$ and $$block$$ formulas.".to_string(), "test.md".to_string());
    let result = plugin.before_render(context);
    assert!(result.content.contains("<span class=\"katex-inline\">inline</span>"));
    assert!(result.content.contains("<div class=\"katex-block\">block</div>"));
}
