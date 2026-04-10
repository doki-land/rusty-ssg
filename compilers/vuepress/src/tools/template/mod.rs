//! 模板管理模块
//! 提供统一的模板引擎管理接口

use std::path::Path;

/// 统一模板管理器
pub struct UnifiedTemplateManager {
    templates: std::collections::HashMap<String, String>,
}

impl UnifiedTemplateManager {
    /// 创建新的统一模板管理器
    pub fn new() -> Self {
        Self { templates: std::collections::HashMap::new() }
    }

    /// 注册模板
    pub fn register_template(&mut self, name: &str, content: &str) -> Result<(), std::io::Error> {
        self.templates.insert(name.to_string(), content.to_string());
        Ok(())
    }

    /// 注册模板文件
    pub fn register_template_file(&mut self, name: &str, path: &Path) -> Result<(), std::io::Error> {
        let content = std::fs::read_to_string(path)?;
        self.templates.insert(name.to_string(), content);
        Ok(())
    }

    /// 从目录注册模板
    pub fn register_templates_from_dir(&mut self, dir: &Path) -> Result<(), std::io::Error> {
        if dir.exists() {
            for entry in walkdir::WalkDir::new(dir) {
                let entry = entry?;
                let path = entry.path();
                if path.is_file() {
                    if let Some(name) = path.file_stem().and_then(|s| s.to_str()) {
                        let content = std::fs::read_to_string(path)?;
                        self.templates.insert(name.to_string(), content);
                    }
                }
            }
        }
        Ok(())
    }

    /// 渲染模板
    pub fn render(&self, template_name: &str, _context: &serde_json::Value) -> Result<String, std::io::Error> {
        match self.templates.get(template_name) {
            Some(content) => Ok(content.clone()),
            None => Err(std::io::Error::new(std::io::ErrorKind::NotFound, format!("Template not found: {}", template_name))),
        }
    }

    /// 渲染模板（使用 serde_json::Value）
    pub fn render_json(&self, template_name: &str, context: &serde_json::Value) -> Result<String, std::io::Error> {
        self.render(template_name, context)
    }
}

impl Default for UnifiedTemplateManager {
    fn default() -> Self {
        Self::new()
    }
}
