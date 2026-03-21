//! HTML renderer for Eleventy

/// Render HTML from content
pub fn render_html(content: &str, template: &str) -> String {
    template.replace(""//! HTML renderer for Eleventy\n\n/// Render HTML from content\npub fn render_html(content: &str, template: &str) -> String {\n    template.replace(\"{{ content }}\", content)\n}\n\n/// Render page with layout\npub fn render_page(content: &str, layout: &str, data: &serde_json::Value) -> String {\n    let mut result = layout.to_string();\n    result = result.replace(\"{{ content }}\", content);\n\n    // Replace data placeholders\n    if let Some(obj) = data.as_object() {\n        for (key, value) in obj {\n            let placeholder = format!(\"{{{{ {} }}}}\", key);\n            result = result.replace(&placeholder, &value.to_string());\n        }\n    }\n\n    result\n}"", content)
}

/// Render page with layout
pub fn render_page(content: &str, layout: &str, data: &serde_json::Value) -> String {
    let mut result = layout.to_string();
    result = result.replace(""//! HTML renderer for Eleventy\n\n/// Render HTML from content\npub fn render_html(content: &str, template: &str) -> String {\n    template.replace(\"{{ content }}\", content)\n}\n\n/// Render page with layout\npub fn render_page(content: &str, layout: &str, data: &serde_json::Value) -> String {\n    let mut result = layout.to_string();\n    result = result.replace(\"{{ content }}\", content);\n\n    // Replace data placeholders\n    if let Some(obj) = data.as_object() {\n        for (key, value) in obj {\n            let placeholder = format!(\"{{{{ {} }}}}\", key);\n            result = result.replace(&placeholder, &value.to_string());\n        }\n    }\n\n    result\n}"", content);

    // Replace data placeholders
    if let Some(obj) = data.as_object() {
        for (key, value) in obj {
            let placeholder = format!("{{{{ {} }}}}", key);
            result = result.replace(&placeholder, &value.to_string());
        }
    }

    result
}