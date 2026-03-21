//! Liquid 模板引擎集成模块
//!
//! 该模块提供 Liquid 模板语言的完整集成，包括：
//! - 标准 Liquid 标签和过滤器支持
//! - Jekyll 特有的 Liquid 扩展（site、page、post 等变量）
//! - 模板继承（layout）和包含（include）
//! - Jekyll 风格的模板上下文
//! - Jekyll 特有的 Liquid 过滤器

use crate::{
    errors::LiquidError,
    jekyll::{FrontMatter, JekyllConfig, JekyllStructure},
};
use chrono::{DateTime, Local, Utc};
use liquid::{
    ParserBuilder, Template,
    model::{Object, Value},
};
use serde_json;
use std::{collections::HashMap, fs, path::Path};

/// Liquid 模板引擎核心
///
/// 该结构体是 Jekyll Liquid 模板引擎的核心实现，负责：
/// - 编译和渲染 Liquid 模板
/// - 提供 Jekyll 风格的上下文对象
/// - 处理模板继承和包含
pub struct LiquidEngine {
    /// Jekyll 目录结构
    structure: JekyllStructure,
    /// Jekyll 配置
    config: JekyllConfig,
    /// Liquid 解析器
    parser: ParserBuilder,
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
        let parser = ParserBuilder::with_stdlib();

        Self { structure, config, parser }
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
    pub fn render_template(&self, template: &str, context: &Object) -> Result<String, LiquidError> {
        let parser = self.parser.build().map_err(|e| LiquidError::ParseError(format!("Failed to build parser: {}", e)))?;
        let compiled =
            parser.parse(template).map_err(|e| LiquidError::ParseError(format!("Failed to parse template: {}", e)))?;

        let output =
            compiled.render(context).map_err(|e| LiquidError::RenderError(format!("Failed to render template: {}", e)))?;

        Ok(output)
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
    pub fn render_template_file(&self, template_path: &Path, context: &Object) -> Result<String, LiquidError> {
        let template_content = fs::read_to_string(template_path)?;
        let parser = self.parser.build().map_err(|e| LiquidError::ParseError(format!("Failed to build parser: {}", e)))?;
        let compiled = parser.parse(&template_content).map_err(|e| {
            LiquidError::ParseError(format!("Failed to parse template file {}: {}", template_path.display(), e))
        })?;

        let output = compiled.render(context).map_err(|e| {
            LiquidError::RenderError(format!("Failed to render template file {}: {}", template_path.display(), e))
        })?;

        Ok(output)
    }

    /// 渲染布局模板
    ///
    /// 该方法用于 Jekyll 的模板继承机制，将内容包装在指定的布局中。
    /// 布局文件中使用 `{{ content }}` 来放置实际内容。
    ///
    /// # Arguments
    ///
    /// * `layout_name` - 布局名称（不含扩展名）
    /// * `content` - 要放入布局的内容
    /// * `context` - 模板上下文对象
    ///
    /// # Returns
    ///
    /// 返回渲染后的完整 HTML 字符串
    ///
    /// # Errors
    ///
    /// 返回 `LiquidError` 如果布局未找到或渲染失败
    pub fn render_layout(&self, layout_name: &str, content: &str, context: &Object) -> Result<String, LiquidError> {
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

        let mut context_with_content = context.clone();
        context_with_content.insert("content".into(), Value::scalar(content.to_string()));

        self.render_template_file(&layout_path, &context_with_content)
    }

    /// 渲染包含文件
    ///
    /// 该方法用于 Jekyll 的 include 机制，加载并渲染 _includes 目录中的模板片段。
    ///
    /// # Arguments
    ///
    /// * `include_name` - 包含文件名称（可以含扩展名）
    /// * `context` - 模板上下文对象
    ///
    /// # Returns
    ///
    /// 返回渲染后的包含内容
    ///
    /// # Errors
    ///
    /// 返回 `LiquidError` 如果包含文件未找到或渲染失败
    pub fn render_include(&self, include_name: &str, context: &Object) -> Result<String, LiquidError> {
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

        self.render_template_file(&include_path, context)
    }

    /// 创建 Jekyll 风格的模板上下文
    ///
    /// 该方法构建一个完整的 Jekyll 模板上下文，包含：
    /// - site 对象：来自 _config.yml 的配置
    /// - page 对象：来自页面的 Front Matter
    /// - now 变量：当前时间
    ///
    /// # Arguments
    ///
    /// * `front_matter` - 页面的 Front Matter
    /// * `page_path` - 页面文件路径
    ///
    /// # Returns
    ///
    /// 返回构建好的 Liquid 上下文对象
    pub fn create_jekyll_context(&self, front_matter: &FrontMatter, page_path: &str) -> Object {
        let mut context = Object::new();

        context.insert("site".into(), self.build_site_object());
        context.insert("page".into(), self.build_page_object(front_matter, page_path));

        let now: DateTime<Local> = Local::now();
        context.insert("now".into(), Value::scalar(now.format("%Y-%m-%d %H:%M:%S").to_string()));

        context
    }

    /// 构建 site 对象
    ///
    /// site 对象包含 Jekyll 配置中的所有信息，以及一些额外的元数据。
    ///
    /// # Returns
    ///
    /// 返回包含站点信息的 Liquid Value
    fn build_site_object(&self) -> Value {
        let mut site = Object::new();

        if let Some(title) = &self.config.title {
            site.insert("title".into(), Value::scalar(title.clone()));
        }
        if let Some(description) = &self.config.description {
            site.insert("description".into(), Value::scalar(description.clone()));
        }
        if let Some(author) = &self.config.author {
            site.insert("author".into(), Value::scalar(author.clone()));
        }
        if let Some(url) = &self.config.url {
            site.insert("url".into(), Value::scalar(url.clone()));
        }
        if let Some(baseurl) = &self.config.baseurl {
            site.insert("baseurl".into(), Value::scalar(baseurl.clone()));
        }
        if let Some(permalink) = &self.config.permalink {
            site.insert("permalink".into(), Value::scalar(permalink.clone()));
        }
        if let Some(timezone) = &self.config.timezone {
            site.insert("timezone".into(), Value::scalar(timezone.clone()));
        }
        if let Some(markdown) = &self.config.markdown {
            site.insert("markdown".into(), Value::scalar(markdown.clone()));
        }
        if let Some(highlighter) = &self.config.highlighter {
            site.insert("highlighter".into(), Value::scalar(highlighter.clone()));
        }

        let now: DateTime<Utc> = Utc::now();
        site.insert("time".into(), Value::scalar(now.to_rfc3339()));

        let mut empty_array = Vec::new();
        site.insert("posts".into(), Value::Array(empty_array.clone()));
        site.insert("pages".into(), Value::Array(empty_array.clone()));
        site.insert("static_files".into(), Value::Array(empty_array.clone()));
        site.insert("html_pages".into(), Value::Array(empty_array.clone()));
        site.insert("collections".into(), Value::Object(Object::new()));
        site.insert("data".into(), Value::Object(Object::new()));

        for (key, value) in &self.config.custom {
            let liquid_value = self.serde_json_to_liquid_value(value);
            site.insert(key.clone().into(), liquid_value);
        }

        Value::Object(site)
    }

    /// 构建 page 对象
    ///
    /// page 对象包含页面的 Front Matter 信息以及一些额外的元数据。
    ///
    /// # Arguments
    ///
    /// * `front_matter` - 页面的 Front Matter
    /// * `page_path` - 页面文件路径
    ///
    /// # Returns
    ///
    /// 返回包含页面信息的 Liquid Value
    fn build_page_object(&self, front_matter: &FrontMatter, page_path: &str) -> Value {
        let mut page = Object::new();

        for (key, value) in &front_matter.variables {
            let liquid_value = self.serde_json_to_liquid_value(value);
            page.insert(key.clone().into(), liquid_value);
        }

        page.insert("path".into(), Value::scalar(page_path.to_string()));

        let path = Path::new(page_path);
        if let Some(file_name) = path.file_name() {
            if let Some(file_name_str) = file_name.to_str() {
                page.insert("name".into(), Value::scalar(file_name_str.to_string()));
            }
        }
        if let Some(dir) = path.parent() {
            if let Some(dir_str) = dir.to_str() {
                page.insert("dir".into(), Value::scalar(dir_str.to_string()));
            }
        }

        if let Some(url) = self.build_page_url(page_path) {
            page.insert("url".into(), Value::scalar(url));
        }

        Value::Object(page)
    }

    /// 构建页面 URL
    ///
    /// 根据页面路径和配置构建页面的 URL。
    ///
    /// # Arguments
    ///
    /// * `page_path` - 页面文件路径
    ///
    /// # Returns
    ///
    /// 返回页面的 URL 字符串
    fn build_page_url(&self, page_path: &str) -> Option<String> {
        let path = Path::new(page_path);
        let mut url = String::new();

        if let Some(baseurl) = &self.config.baseurl {
            url.push_str(baseurl);
        }

        if let Some(stem) = path.file_stem() {
            if let Some(stem_str) = stem.to_str() {
                if stem_str != "index" {
                    url.push('/');
                    url.push_str(stem_str);
                }
            }
        }

        if url.is_empty() { Some("/".to_string()) } else { Some(url) }
    }

    /// 将 serde_json::Value 转换为 liquid::Value
    ///
    /// 该方法用于将从 Front Matter 或配置中解析出的 JSON 值
    /// 转换为 Liquid 模板引擎可以使用的 Value 类型。
    ///
    /// # Arguments
    ///
    /// * `json_value` - 要转换的 serde_json::Value
    ///
    /// # Returns
    ///
    /// 返回转换后的 liquid::Value
    fn serde_json_to_liquid_value(&self, json_value: &serde_json::Value) -> Value {
        match json_value {
            serde_json::Value::Null => Value::Nil,
            serde_json::Value::Bool(b) => Value::scalar(*b),
            serde_json::Value::Number(n) => {
                if let Some(i) = n.as_i64() {
                    Value::scalar(i)
                }
                else if let Some(f) = n.as_f64() {
                    Value::scalar(f)
                }
                else {
                    Value::Nil
                }
            }
            serde_json::Value::String(s) => Value::scalar(s.clone()),
            serde_json::Value::Array(arr) => {
                let liquid_arr: Vec<Value> = arr.iter().map(|v| self.serde_json_to_liquid_value(v)).collect();
                Value::Array(liquid_arr)
            }
            serde_json::Value::Object(obj) => {
                let mut liquid_obj = Object::new();
                for (k, v) in obj {
                    liquid_obj.insert(k.clone().into(), self.serde_json_to_liquid_value(v));
                }
                Value::Object(liquid_obj)
            }
        }
    }

    /// 获取 Jekyll 目录结构
    ///
    /// # Returns
    ///
    /// 返回 Jekyll 目录结构的引用
    pub fn structure(&self) -> &JekyllStructure {
        &self.structure
    }

    /// 获取 Jekyll 配置
    ///
    /// # Returns
    ///
    /// 返回 Jekyll 配置的引用
    pub fn config(&self) -> &JekyllConfig {
        &self.config
    }
}
