//! 模板管理模块
//! 提供统一的模板引擎管理接口

use nargo_template::{TemplateEngine, TemplateManager, ToJsonValue};
use serde_json::Value;
use std::path::Path;

/// 统一模板管理器
pub struct UnifiedTemplateManager {
    manager: TemplateManager,
}

impl UnifiedTemplateManager {
    /// 创建新的统一模板管理器
    pub fn new() -> Self {
        Self {
            manager: TemplateManager::new(),
        }
    }

    /// 注册模板
    pub fn register_template(
        &mut self,
        engine: TemplateEngine,
        name: &str,
        content: &str,
    ) -> Result<(), std::io::Error> {
        self.manager.register_template(engine, name, content)
    }

    /// 注册模板文件
    pub fn register_template_file(
        &mut self,
        engine: TemplateEngine,
        name: &str,
        path: &Path,
    ) -> Result<(), std::io::Error> {
        self.manager.register_template_file(engine, name, path)
    }

    /// 从目录注册模板
    pub fn register_templates_from_dir(
        &mut self,
        engine: TemplateEngine,
        dir: &Path,
    ) -> Result<(), std::io::Error> {
        self.manager.add_template_dir(dir);
        self.manager.load_templates(engine)
    }

    /// 渲染模板
    pub fn render<T: ToJsonValue>(
        &self,
        engine: TemplateEngine,
        template_name: &str,
        context: &T,
    ) -> Result<String, std::io::Error> {
        let json_context = context.to_json_value();
        self.manager.render(engine, template_name, &json_context)
    }

    /// 渲染模板（使用 serde_json::Value）
    pub fn render_json(
        &self,
        engine: TemplateEngine,
        template_name: &str,
        context: &Value,
    ) -> Result<String, std::io::Error> {
        self.manager.render(engine, template_name, context)
    }
}

impl Default for UnifiedTemplateManager {
    fn default() -> Self {
        Self::new()
    }
}
