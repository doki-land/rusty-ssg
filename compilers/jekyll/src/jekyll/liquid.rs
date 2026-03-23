#![warn(missing_docs)]

//! Jekyll Liquid 模板引擎模块
//!
//! 提供 Liquid 模板引擎的集成和渲染功能

use std::collections::HashMap;
use std::path::Path;

use serde_json::Value;

use crate::errors::{LiquidError, Result};

use super::{FrontMatterParser, JekyllConfig, JekyllStructure};

/// Liquid 过滤器
#[derive(Debug, Clone)]
pub struct LiquidFilter {
    /// 过滤器名称
    name: String,
    /// 过滤器函数
    filter_fn: Box<dyn Fn(&[Value]) -> Value + Send + Sync>,
}

impl LiquidFilter {
    /// 创建新的过滤器
    pub fn new<F>(name: &str, filter_fn: F) -> Self
    where
        F: Fn(&[Value]) -> Value + Send + Sync + 'static,
    {
        Self { name: name.to_string(), filter_fn: Box::new(filter_fn) }
    }

    /// 获取过滤器名称
    pub fn name(&self) -> &str {
        &self.name
    }

    /// 应用过滤器
    pub fn apply(&self, args: &[Value]) -> Value {
        (self.filter_fn)(args)
    }
}

/// Liquid 标签
#[derive(Debug, Clone)]
pub struct LiquidTag {
    /// 标签名称
    name: String,
    /// 标签内容
    content: String,
    /// 标签属性
    attributes: HashMap<String, String>,
}

impl LiquidTag {
    /// 创建新的标签
    pub fn new(name: &str, content: &str) -> Self {
        Self { name: name.to_string(), content: content.to_string(), attributes: HashMap::new() }
    }

    /// 获取标签名称
    pub fn name(&self) -> &str {
        &self.name
    }

    /// 获取标签内容
    pub fn content(&self) -> &str {
        &self.content
    }

    /// 获取属性
    pub fn get_attr(&self, key: &str) -> Option<&String> {
        self.attributes.get(key)
    }

    /// 设置属性
    pub fn set_attr(&mut self, key: &str, value: &str) {
        self.attributes.insert(key.to_string(), value.to_string());
    }
}

/// Liquid 模板引擎
#[derive(Debug)]
pub struct LiquidEngine {
    /// Jekyll 目录结构
    structure: JekyllStructure,
    /// Jekyll 配置
    config: JekyllConfig,
    /// 布局缓存
    layouts: HashMap<String, String>,
    /// 包含文件缓存
    includes: HashMap<String, String>,
    /// 自定义过滤器
    filters: HashMap<String, LiquidFilter>,
}

impl LiquidEngine {
    /// 创建新的 Liquid 引擎
    ///
    /// # Arguments
    ///
    /// * `structure` - Jekyll 目录结构
    /// * `config` - Jekyll 配置
    ///
    /// # Returns
    ///
    /// 返回 Liquid 引擎实例
    pub fn new(structure: JekyllStructure, config: JekyllConfig) -> Self {
        let mut engine = Self { structure, config, layouts: HashMap::new(), includes: HashMap::new(), filters: HashMap::new() };
        engine.register_default_filters();
        engine
    }

    /// 注册默认过滤器
    fn register_default_filters(&mut self) {
        self.filters.insert(
            "upcase".to_string(),
            LiquidFilter::new("upcase", |args| {
                if let Some(Value::String(s)) = args.first() {
                    Value::String(s.to_uppercase())
                }
                else {
                    Value::Null
                }
            }),
        );

        self.filters.insert(
            "downcase".to_string(),
            LiquidFilter::new("downcase", |args| {
                if let Some(Value::String(s)) = args.first() {
                    Value::String(s.to_lowercase())
                }
                else {
                    Value::Null
                }
            }),
        );

        self.filters.insert(
            "capitalize".to_string(),
            LiquidFilter::new("capitalize", |args| {
                if let Some(Value::String(s)) = args.first() {
                    let mut chars = s.chars();
                    match chars.next() {
                        None => Value::String(String::new()),
                        Some(c) => Value::String(c.to_uppercase().collect::<String>() + chars.as_str()),
                    }
                }
                else {
                    Value::Null
                }
            }),
        );

        self.filters.insert(
            "strip".to_string(),
            LiquidFilter::new("strip", |args| {
                if let Some(Value::String(s)) = args.first() {
                    Value::String(s.trim().to_string())
                }
                else {
                    Value::Null
                }
            }),
        );

        self.filters.insert(
            "escape".to_string(),
            LiquidFilter::new("escape", |args| {
                if let Some(Value::String(s)) = args.first() {
                    Value::String(html_escape(s))
                }
                else {
                    Value::Null
                }
            }),
        );
    }

    /// 加载布局文件
    pub fn load_layouts(&mut self) -> Result<()> {
        if let Some(layouts_dir) = self.structure.layouts_dir() {
            self.load_templates_from_dir(layouts_dir, &mut self.layouts)?;
        }
        Ok(())
    }

    /// 加载包含文件
    pub fn load_includes(&mut self) -> Result<()> {
        if let Some(includes_dir) = self.structure.includes_dir() {
            self.load_templates_from_dir(includes_dir, &mut self.includes)?;
        }
        Ok(())
    }

    /// 从目录加载模板
    fn load_templates_from_dir(&self, dir: &Path, cache: &mut HashMap<String, String>) -> Result<()> {
        if !dir.exists() {
            return Ok(());
        }

        for entry in walkdir::WalkDir::new(dir)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
        {
            let path = entry.path();

            if let Some(ext) = path.extension() {
                if ext == "html" || ext == "liquid" {
                    let name = path.file_stem().and_then(|s| s.to_str()).unwrap_or("").to_string();
                    let content = std::fs::read_to_string(path).map_err(LiquidError::from)?;
                    cache.insert(name, content);
                }
            }
        }

        Ok(())
    }

    /// 创建 Jekyll 上下文
    ///
    /// # Arguments
    ///
    /// * `front_matter` - Front Matter
    /// * `page_path` - 页面路径
    ///
    /// # Returns
    ///
    /// 返回上下文对象
    pub fn create_jekyll_context(&self, front_matter: &super::FrontMatter, page_path: &str) -> Value {
        let mut context = serde_json::Map::new();

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

        let mut page = serde_json::Map::new();
        page.insert("path".to_string(), Value::String(page_path.to_string()));

        for (key, value) in front_matter.variables().as_object().unwrap_or(&serde_json::Map::new()) {
            page.insert(key.clone(), value.clone());
        }
        context.insert("page".to_string(), Value::Object(page));

        let mut content_map = serde_json::Map::new();
        content_map.insert("".to_string(), Value::String(front_matter.content().to_string()));
        context.insert("content".to_string(), Value::Object(content_map));

        Value::Object(context)
    }

    /// 渲染模板
    ///
    /// # Arguments
    ///
    /// * `template` - 模板内容
    /// * `context` - 渲染上下文
    ///
    /// # Returns
    ///
    /// 返回渲染结果或错误
    pub fn render_template(&self, template: &str, context: &Value) -> Result<String> {
        let mut result = template.to_string();

        result = self.render_variables(&result, context)?;
        result = self.render_filters(&result, context)?;
        result = self.render_tags(&result, context)?;

        Ok(result)
    }

    /// 渲染变量
    fn render_variables(&self, template: &str, context: &Value) -> Result<String> {
        let mut result = template.to_string();
        let var_regex = regex::Regex::new(r"\{\{\s*(\w+(?:\.\w+)*)\s*\}\}").map_err(|e| LiquidError::parse_error(e.to_string()))?;

        for cap in var_regex.captures_iter(template) {
            if let Some(full_match) = cap.get(0) {
                if let Some(var_path) = cap.get(1) {
                    let var_name = var_path.as_str();
                    let value = self.get_variable(var_name, context);

                    let value_str = value_to_string(&value);
                    result = result.replace(full_match.as_str(), &value_str);
                }
            }
        }

        Ok(result)
    }

    /// 渲染过滤器
    fn render_filters(&self, template: &str, context: &Value) -> Result<String> {
        let mut result = template.to_string();
        let filter_regex = regex::Regex::new(r"\{\{\s*(\w+)(?:\s*\|\s*(\w+(?::[^}]+)?))*\s*\}\}").map_err(|e| LiquidError::parse_error(e.to_string()))?;

        for cap in filter_regex.captures_iter(template) {
            if let Some(full_match) = cap.get(0) {
                if let Some(var_name) = cap.get(1) {
                    let var_value = self.get_variable(var_name.as_str(), context);

                    let filter_pattern = regex::Regex::new(r"\|\s*(\w+)(?::([^}]+))?").map_err(|e| LiquidError::parse_error(e.to_string()))?;
                    let mut final_value = var_value;

                    for filter_cap in filter_pattern.captures_iter(full_match.as_str()) {
                        if let Some(filter_name) = filter_cap.get(1) {
                            let filter_name = filter_name.as_str();

                            if let Some(filter) = self.filters.get(filter_name) {
                                let args = vec![final_value];
                                final_value = filter.apply(&args);
                            }
                        }
                    }

                    result = result.replace(full_match.as_str(), &value_to_string(&final_value));
                }
            }
        }

        Ok(result)
    }

    /// 渲染标签
    fn render_tags(&self, template: &str, context: &Value) -> Result<String> {
        let mut result = template.to_string();

        result = self.render_if_tags(&result, context)?;
        result = self.render_for_tags(&result, context)?;

        Ok(result)
    }

    /// 渲染 {% if %} 标签
    fn render_if_tags(&self, template: &str, _context: &Value) -> Result<String> {
        let mut result = template.to_string();

        let if_regex = regex::Regex::new(r"\{%\s*if\s+(\w+(?:\.\w+)*)\s*%\}").map_err(|e| LiquidError::parse_error(e.to_string()))?;
        let endif_regex = regex::Regex::new(r"\{%\s*endif\s*%\}").map_err(|e| LiquidError::parse_error(e.to_string()))?;
        let else_regex = regex::Regex::new(r"\{%\s*else\s*%\}").map_err(|e| LiquidError::parse_error(e.to_string()))?;

        let if_matches: Vec<_> = if_regex.find_iter(template).collect();
        let endif_matches: Vec<_> = endif_regex.find_iter(template).collect();

        if if_matches.len() != endif_matches.len() {
            return Ok(result);
        }

        for (i, if_match) in if_matches.iter().enumerate() {
            let start = if_match.start();
            let end = endif_matches[i].end();

            let tag_content = &template[start..end];

            let var_match = if_regex.captures(tag_content).unwrap();
            let var_name = var_match.get(1).map(|m| m.as_str()).unwrap_or("");

            let has_else = else_regex.is_match(tag_content);

            if has_else {
                let parts: Vec<&str> = tag_content.split("{% else %}").collect();
                if parts.len() == 2 {
                    let true_part = parts[0].replace(if_match.as_str(), "").replace("{% endif %}", "");
                    let false_part = parts[1].replace("{% endif %}", "");

                    if !var_name.is_empty() {
                        result = result.replace(tag_content, &true_part);
                    }
                    else {
                        result = result.replace(tag_content, &false_part);
                    }
                }
            }
            else {
                let content = tag_content.replace(if_match.as_str(), "").replace("{% endif %}", "");

                if !var_name.is_empty() {
                    result = result.replace(tag_content, &content);
                }
                else {
                    result = result.replace(tag_content, "");
                }
            }
        }

        Ok(result)
    }

    /// 渲染 {% for %} 标签
    fn render_for_tags(&self, template: &str, _context: &Value) -> Result<String> {
        let mut result = template.to_string();

        let for_regex = regex::Regex::new(r"\{%\s*for\s+(\w+)\s+in\s+(\w+(?:\.\w+)*)\s*%\}").map_err(|e| LiquidError::parse_error(e.to_string()))?;
        let endfor_regex = regex::Regex::new(r"\{%\s*endfor\s*%\}").map_err(|e| LiquidError::parse_error(e.to_string()))?;

        let for_matches: Vec<_> = for_regex.find_iter(template).collect();
        let endfor_matches: Vec<_> = endfor_regex.find_iter(template).collect();

        if for_matches.len() != endfor_matches.len() {
            return Ok(result);
        }

        for (i, for_match) in for_matches.iter().enumerate() {
            let start = for_match.start();
            let end = endfor_matches[i].end();

            let tag_content = &template[start..end];

            let cap = for_regex.captures(tag_content).unwrap();
            let item_var = cap.get(1).map(|m| m.as_str()).unwrap_or("");
            let list_var = cap.get(2).map(|m| m.as_str()).unwrap_or("");

            let content = tag_content.replace(for_match.as_str(), "").replace("{% endfor %}", "");

            if let Value::Array(arr) = self.get_variable(list_var, &Value::Null) {
                let mut rendered_items = Vec::new();

                for item in arr {
                    let mut item_content = content.replace(&format!("{{ {} }}", item_var), &value_to_string(item));
                    rendered_items.push(item_content);
                }

                result = result.replace(tag_content, &rendered_items.join(""));
            }
        }

        Ok(result)
    }

    /// 获取变量值
    fn get_variable(&self, name: &str, context: &Value) -> Value {
        if name.is_empty() {
            return Value::Null;
        }

        if let Value::Object(obj) = context {
            let parts: Vec<&str> = name.split('.').collect();

            let mut current: Option<&Value> = None;

            for (i, part) in parts.iter().enumerate() {
                if i == 0 {
                    current = obj.get(*part);
                }
                else if let Some(c) = current {
                    if let Value::Object(map) = c {
                        current = map.get(*part);
                    }
                    else if let Value::Array(arr) = c {
                        if let Ok(idx) = part.parse::<usize>() {
                            current = arr.get(idx);
                        }
                        else {
                            return Value::Null;
                        }
                    }
                    else {
                        return Value::Null;
                    }
                }

                if current.is_none() {
                    return Value::Null;
                }
            }

            current.cloned().unwrap_or(Value::Null)
        }
        else {
            Value::Null
        }
    }

    /// 渲染布局
    ///
    /// # Arguments
    ///
    /// * `layout_name` - 布局名称
    /// * `content` - 内容
    /// * `context` - 上下文
    ///
    /// # Returns
    ///
    /// 返回渲染结果或错误
    pub fn render_layout(&mut self, layout_name: &str, content: &str, context: &Value) -> Result<String> {
        if self.layouts.is_empty() {
            self.load_layouts()?;
        }

        let layout_content = self.layouts.get(layout_name).ok_or_else(|| LiquidError::template_not_found(layout_name.to_string()))?;

        let mut full_context = context.clone();

        if let Value::Object(map) = &mut full_context {
            let mut content_map = serde_json::Map::new();
            content_map.insert("".to_string(), Value::String(content.to_string()));
            map.insert("content".to_string(), Value::Object(content_map));
        }

        self.render_template(layout_content, &full_context)
    }

    /// 渲染包含文件
    ///
    /// # Arguments
    ///
    /// * `include_name` - 包含文件名称
    /// * `context` - 上下文
    ///
    /// # Returns
    ///
    /// 返回渲染结果或错误
    pub fn render_include(&mut self, include_name: &str, context: &Value) -> Result<String> {
        if self.includes.is_empty() {
            self.load_includes()?;
        }

        let include_content = self.includes.get(include_name).ok_or_else(|| LiquidError::template_not_found(include_name.to_string()))?;

        self.render_template(include_content, context)
    }

    /// 注册自定义过滤器
    pub fn register_filter(&mut self, filter: LiquidFilter) {
        self.filters.insert(filter.name().to_string(), filter);
    }
}

fn value_to_string(value: &Value) -> String {
    match value {
        Value::String(s) => s.clone(),
        Value::Number(n) => n.to_string(),
        Value::Bool(b) => b.to_string(),
        Value::Null => String::new(),
        Value::Array(arr) => {
            let items: Vec<String> = arr.iter().map(value_to_string).collect();
            items.join(", ")
        }
        Value::Object(obj) => {
            let items: Vec<String> = obj.values().map(value_to_string).collect();
            items.join(", ")
        }
    }
}

fn html_escape(s: &str) -> String {
    let mut result = String::with_capacity(s.len() * 2);
    for c in s.chars() {
        match c {
            '<' => result.push_str("&lt;"),
            '>' => result.push_str("&gt;"),
            '&' => result.push_str("&amp;"),
            '"' => result.push_str("&quot;"),
            '\'' => result.push_str("&#39;"),
            _ => result.push(c),
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_render_template() {
        let temp_dir = tempdir().unwrap();
        let structure = JekyllStructure::new(temp_dir.path()).unwrap();
        let config = JekyllConfig::new();
        let engine = LiquidEngine::new(structure, config);

        let template = "Hello {{ name }}!";
        let mut context = serde_json::Map::new();
        context.insert("name".to_string(), Value::String("World".to_string()));

        let result = engine.render_template(template, &Value::Object(context)).unwrap();
        assert_eq!(result, "Hello World!");
    }

    #[test]
    fn test_render_template_with_filters() {
        let temp_dir = tempdir().unwrap();
        let structure = JekyllStructure::new(temp_dir.path()).unwrap();
        let config = JekyllConfig::new();
        let engine = LiquidEngine::new(structure, config);

        let template = "Hello {{ name | upcase }}!";
        let mut context = serde_json::Map::new();
        context.insert("name".to_string(), Value::String("world".to_string()));

        let result = engine.render_template(template, &Value::Object(context)).unwrap();
        assert_eq!(result, "Hello WORLD!");
    }

    #[test]
    fn test_create_jekyll_context() {
        let temp_dir = tempdir().unwrap();
        let structure = JekyllStructure::new(temp_dir.path()).unwrap();
        let config = JekyllConfig::new()
            .with_title("Test Site".to_string())
            .with_description("A test site".to_string())
            .with_url("https://example.com".to_string());
        let engine = LiquidEngine::new(structure, config);

        let content = r#"---
title: Test Page
layout: post
date: 2024-01-01
---
Content here."#;
        let front_matter = FrontMatterParser::parse(content).unwrap();

        let context = engine.create_jekyll_context(&front_matter, "test-page.md");

        let site = context.get("site").unwrap().as_object().unwrap();
        assert_eq!(site.get("title").unwrap().as_str().unwrap(), "Test Site");

        let page = context.get("page").unwrap().as_object().unwrap();
        assert_eq!(page.get("title").unwrap().as_str().unwrap(), "Test Page");
    }

    #[test]
    fn test_render_layout() {
        let temp_dir = tempdir().unwrap();

        let layouts_dir = temp_dir.path().join("_layouts");
        fs::create_dir_all(&layouts_dir).unwrap();

        let layout_content = r#"<!DOCTYPE html>
<html>
<head>
    <title>{{ page.title }}</title>
</head>
<body>
    {{ content }}
</body>
</html>"#;
        fs::write(layouts_dir.join("default.html"), layout_content).unwrap();

        let structure = JekyllStructure::new(temp_dir.path()).unwrap();
        let config = JekyllConfig::new();
        let mut engine = LiquidEngine::new(structure, config);

        let content = "<h1>Hello World</h1><p>This is the content.</p>";
        let mut context = serde_json::Map::new();
        let mut page = serde_json::Map::new();
        page.insert("title".to_string(), Value::String("Test Page".to_string()));
        context.insert("page".to_string(), Value::Object(page));

        let result = engine.render_layout("default", content, &Value::Object(context)).unwrap();
        assert!(result.contains("<!DOCTYPE html>"));
        assert!(result.contains("<title>Test Page</title>"));
        assert!(result.contains("<h1>Hello World</h1>"));
    }

    #[test]
    fn test_render_include() {
        let temp_dir = tempdir().unwrap();

        let includes_dir = temp_dir.path().join("_includes");
        fs::create_dir_all(&includes_dir).unwrap();

        let include_content = r#"<div class="footer">
    <p>Footer content</p>
</div>"#;
        fs::write(includes_dir.join("footer.html"), include_content).unwrap();

        let structure = JekyllStructure::new(temp_dir.path()).unwrap();
        let config = JekyllConfig::new();
        let mut engine = LiquidEngine::new(structure, config);

        let context = serde_json::Map::new();
        let result = engine.render_include("footer.html", &Value::Object(context)).unwrap();
        assert!(result.contains("<div class=\"footer\">"));
    }
}
