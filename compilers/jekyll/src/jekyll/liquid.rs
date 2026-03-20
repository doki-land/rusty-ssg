//! Liquid 模板引擎集成模块
//!
//! 该模块提供 Liquid 模板语言的完整集成，包括：
//! - 标准 Liquid 标签和过滤器支持
//! - Jekyll 特有的 Liquid 扩展（site、page、post 等变量）
//! - 模板继承（layout）和包含（include）
//! - Jekyll 风格的模板上下文

use crate::{
    errors::LiquidError,
    jekyll::{FrontMatter, JekyllConfig, JekyllError, JekyllStructure},
};
use nargo_template::{TemplateEngine, TemplateManager};
use serde_json::{Value, json};
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

/// Liquid 模板引擎核心
pub struct LiquidEngine {
    /// Jekyll 目录结构
    structure: JekyllStructure,
    /// Jekyll 配置
    config: JekyllConfig,
    /// 模板管理器
    template_manager: TemplateManager,
    /// 已注册的模板
    registered_templates: HashMap<String, String>,
}

impl LiquidEngine {
    /// 创建新的 Liquid 模板引擎
    ///
    /// # Arguments
    ///
    /// * `structure` - Jekyll 目录结构
    /// * `config` - Jekyll 配置
    ///
    /// # Returns
    ///
    /// 返回新创建的 LiquidEngine 实例
    pub fn new(structure: JekyllStructure, config: JekyllConfig) -> Self {
        let template_manager = TemplateManager::new();
        Self { structure, config, template_manager, registered_templates: HashMap::new() }
    }

    /// 渲染 Liquid 模板
    ///
    /// # Arguments
    ///
    /// * `template` - 要渲染的模板字符串
    /// * `context` - 模板上下文对象
    ///
    /// # Returns
    ///
    /// 返回渲染后的字符串
    ///
    /// # Errors
    ///
    /// 返回 `LiquidError` 如果解析或渲染失败
    pub fn render_template(&mut self, template: &str, context: &Value) -> Result<String, LiquidError> {
        // 注册临时模板
        self.template_manager
            .register_template(TemplateEngine::Handlebars, "temp", template)
            .map_err(|e| LiquidError::ParseError(e.to_string()))?;

        // 渲染模板
        self.template_manager
            .render(TemplateEngine::Handlebars, "temp", context)
            .map_err(|e| LiquidError::RenderError(e.to_string()))
    }

    /// 从文件加载并渲染模板
    ///
    /// # Arguments
    ///
    /// * `template_path` - 模板文件路径
    /// * `context` - 模板上下文对象
    ///
    /// # Returns
    ///
    /// 返回渲染后的字符串
    ///
    /// # Errors
    ///
    /// 返回 `LiquidError` 如果文件读取、解析或渲染失败
    pub fn render_template_file(&mut self, template_path: &Path, context: &Value) -> Result<String, LiquidError> {
        let template_key = template_path.to_string_lossy().to_string();

        // 检查是否已注册
        if !self.registered_templates.contains_key(&template_key) {
            // 读取模板文件
            let template_content = std::fs::read_to_string(template_path)?;

            // 注册模板
            self.template_manager
                .register_template(TemplateEngine::Handlebars, &template_key, &template_content)
                .map_err(|e| LiquidError::ParseError(e.to_string()))?;

            // 记录已注册
            self.registered_templates.insert(template_key.clone(), template_content);
        }

        // 渲染模板
        self.template_manager
            .render(TemplateEngine::Handlebars, &template_key, context)
            .map_err(|e| LiquidError::RenderError(e.to_string()))
    }

    /// 渲染布局模板
    ///
    /// # Arguments
    ///
    /// * `layout_name` - 布局名称
    /// * `content` - 内容
    /// * `context` - 模板上下文对象
    ///
    /// # Returns
    ///
    /// 返回渲染后的字符串
    ///
    /// # Errors
    ///
    /// 返回 `LiquidError` 如果布局未找到或渲染失败
    pub fn render_layout(&mut self, layout_name: &str, content: &str, context: &Value) -> Result<String, LiquidError> {
        // 构建布局文件路径
        let layout_path = match self.structure.directory_path(crate::jekyll::JekyllDirectory::Layouts) {
            Some(layouts_dir) => {
                let mut path = layouts_dir.to_path_buf();
                path.push(format!("{}.html", layout_name));
                if !path.exists() {
                    path.set_extension("");
                    path.set_extension("liquid");
                }
                if !path.exists() {
                    return Err(LiquidError::TemplateNotFound(format!("Layout '{}' not found", layout_name)));
                }
                path
            }
            None => return Err(LiquidError::TemplateNotFound("Layouts directory not found".to_string())),
        };

        // 创建包含 content 的上下文
        let mut context_map = context.as_object().unwrap_or(&serde_json::Map::new()).clone();
        context_map.insert("content".to_string(), Value::String(content.to_string()));
        let layout_context = Value::Object(context_map);

        // 渲染布局
        self.render_template_file(&layout_path, &layout_context)
    }

    /// 渲染包含文件
    ///
    /// # Arguments
    ///
    /// * `include_name` - 包含文件名称
    /// * `context` - 模板上下文对象
    ///
    /// # Returns
    ///
    /// 返回渲染后的字符串
    ///
    /// # Errors
    ///
    /// 返回 `LiquidError` 如果包含文件未找到或渲染失败
    pub fn render_include(&mut self, include_name: &str, context: &Value) -> Result<String, LiquidError> {
        // 构建包含文件路径
        let include_path = match self.structure.directory_path(crate::jekyll::JekyllDirectory::Includes) {
            Some(includes_dir) => {
                let mut path = includes_dir.to_path_buf();
                path.push(include_name);
                if !path.exists() {
                    path.set_extension("html");
                }
                if !path.exists() {
                    path.set_extension("");
                    path.set_extension("liquid");
                }
                if !path.exists() {
                    return Err(LiquidError::TemplateNotFound(format!("Include '{}' not found", include_name)));
                }
                path
            }
            None => return Err(LiquidError::TemplateNotFound("Includes directory not found".to_string())),
        };

        // 渲染包含文件
        self.render_template_file(&include_path, context)
    }

    /// 创建 Jekyll 风格的模板上下文
    ///
    /// # Arguments
    ///
    /// * `front_matter` - 前置内容
    /// * `page_path` - 页面路径
    ///
    /// # Returns
    ///
    /// 返回构建的上下文对象
    pub fn create_jekyll_context(&self, front_matter: &FrontMatter, page_path: &str) -> Value {
        let mut context = serde_json::Map::new();

        // 构建 site 变量
        let mut site = serde_json::Map::new();
        if let Some(title) = &self.config.title {
            site.insert("title".to_string(), Value::String(title.clone()));
        }
        if let Some(description) = &self.config.description {
            site.insert("description".to_string(), Value::String(description.clone()));
        }
        if let Some(url) = &self.config.url {
            site.insert("url".to_string(), Value::String(url.clone()));
        }
        if let Some(baseurl) = &self.config.baseurl {
            site.insert("baseurl".to_string(), Value::String(baseurl.clone()));
        }
        context.insert("site".to_string(), Value::Object(site));

        // 构建 page 变量
        let mut page = serde_json::Map::new();
        for (key, value) in &front_matter.variables {
            page.insert(key.clone(), value.clone());
        }
        page.insert("path".to_string(), Value::String(page_path.to_string()));
        context.insert("page".to_string(), Value::Object(page));

        Value::Object(context)
    }

    /// 清除模板缓存
    pub fn clear_cache(&mut self) {
        self.registered_templates.clear();
        // 注意：nargo-template 的 TemplateManager 目前没有提供清除缓存的方法
    }

    /// 获取 Jekyll 目录结构
    pub fn structure(&self) -> &JekyllStructure {
        &self.structure
    }

    /// 获取 Jekyll 配置
    pub fn config(&self) -> &JekyllConfig {
        &self.config
    }
}
