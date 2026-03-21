//! HTML renderer for Eleventy

/// Render HTML from content
pub fn render_html(content: &str, template: &str) -> String {
    template.replace("{{ content }}", content)
}

/// Render page with layout
pub fn render_page(content: &str, layout: &str, data: &serde_json::Value) -> String {
    let mut result = layout.to_string();
    result = result.replace("{{ content }}", content);

    // Replace data placeholders
    if let Some(obj) = data.as_object() {
        for (key, value) in obj {
            let placeholder = format!("{{{{ {} }}}}", key);
            result = result.replace(&placeholder, &value.to_string());
        }
    }

    result
}