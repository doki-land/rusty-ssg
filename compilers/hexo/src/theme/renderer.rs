//! 主题渲染器

use crate::types::Result;
use std::{fs::File, io::Read, path::Path};

/// 主题渲染器
pub struct ThemeRenderer {
    /// 主题变量
    variables: serde_json::Value,
}

impl ThemeRenderer {
    /// 创建主题渲染器
    pub fn new(variables: serde_json::Value) -> Self {
        Self { variables }
    }

    /// 渲染模板
    pub fn render_template(&self, template_path: &Path) -> Result<String> {
        // 读取模板文件
        let mut file = File::open(template_path).map_err(|e| {
            crate::types::HexoError::io_error(
                Some(template_path.to_string_lossy().to_string()),
                format!("Failed to open template file: {}", e),
            )
        })?;
        let mut content = String::new();
        file.read_to_string(&mut content).map_err(|e| {
            crate::types::HexoError::io_error(
                Some(template_path.to_string_lossy().to_string()),
                format!("Failed to read template file: {}", e),
            )
        })?;

        // 简单的EJS语法转换
        let transformed_content = self.transform_ejs_to_askama(&content);

        // 使用简单的变量替换渲染
        Ok(self.render_with_simple_replace(&transformed_content)?)
    }

    /// 将EJS语法转换为Askama语法
    fn transform_ejs_to_askama(&self, content: &str) -> String {
        // 简单的EJS语法转换
        // <% code %> -> {% code %}
        // <%= expression %> -> {{ expression }}
        // <%# comment %> -> {# comment #}
        let mut result = content.to_string();
        result = result.replace("<%=", "{{");
        result = result.replace("%>", "}}");
        result = result.replace("<%", "{%");
        result = result.replace("%>", "%}");
        result = result.replace("<%#", "{#");
        result = result.replace("%>", "#}");
        result
    }

    /// 使用简单的变量替换渲染模板
    fn render_with_simple_replace(&self, content: &str) -> Result<String> {
        let mut result = content.to_string();

        // 简单的变量替换
        self.replace_variables(&mut result, &self.variables, "");

        Ok(result)
    }

    /// 递归替换变量
    fn replace_variables(&self, result: &mut String, value: &serde_json::Value, prefix: &str) {
        match value {
            serde_json::Value::Object(map) => {
                for (key, val) in map {
                    let new_prefix = if prefix.is_empty() { key.clone() } else { format!("{}.{}", prefix, key) };
                    self.replace_variables(result, val, &new_prefix);
                }
            }
            serde_json::Value::String(s) => {
                let placeholder = format!("{{{{ {} }}}}", prefix);
                *result = result.replace(&placeholder, s);
            }
            _ => {
                let placeholder = format!("{{{{ {} }}}}", prefix);
                let value_str = value.to_string();
                *result = result.replace(&placeholder, &value_str);
            }
        }
    }
}

/// 渲染主题
pub fn render_theme(template_path: &Path, variables: serde_json::Value) -> Result<String> {
    let renderer = ThemeRenderer::new(variables);
    renderer.render_template(template_path)
}
