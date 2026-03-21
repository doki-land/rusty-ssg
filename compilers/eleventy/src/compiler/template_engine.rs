//! 模板引擎模块

use handlebars::{Context, Handlebars, Helper, HelperResult, Output, RenderContext};
use liquid::{ParserBuilder, Template, model::Value as LiquidValue};
use pulldown_cmark::{Options, Parser, html::push_html};
use std::collections::HashMap;

/// 模板引擎错误类型
#[derive(Debug)]
pub enum TemplateError {
    /// 模板解析错误
    ParseError(String),
    /// 模板渲染错误
    RenderError(String),
    /// 模板未找到错误
    TemplateNotFoundError(String),
}

/// 模板引擎 trait
pub trait TemplateEngine: Send + Sync {
    /// 渲染模板
    fn render(&self, template: &str, data: &serde_json::Value) -> Result<String, TemplateError>;

    /// 注册过滤器
    fn register_filter(&mut self, name: &str, filter: fn(&str) -> String);

    /// 注册全局变量
    fn register_global(&mut self, name: &str, value: serde_json::Value);
}

/// 简单模板引擎实现
pub struct SimpleTemplateEngine {
    /// 过滤器
    filters: HashMap<String, fn(&str) -> String>,

    /// 全局变量
    globals: HashMap<String, serde_json::Value>,
}

impl SimpleTemplateEngine {
    /// 创建新的简单模板引擎
    pub fn new() -> Self {
        Self { filters: HashMap::new(), globals: HashMap::new() }
    }
}

impl TemplateEngine for SimpleTemplateEngine {
    fn render(&self, template: &str, data: &serde_json::Value) -> Result<String, TemplateError> {
        let mut result = template.to_string();

        // 替换变量
        if let Some(obj) = data.as_object() {
            for (key, value) in obj {
                let placeholder = format!("{{{{ {} }}}}", key);
                result = result.replace(&placeholder, &value.to_string());
            }
        }

        // 替换全局变量
        for (key, value) in &self.globals {
            let placeholder = format!("{{{{ {} }}}}", key);
            result = result.replace(&placeholder, &value.to_string());
        }

        Ok(result)
    }

    fn register_filter(&mut self, name: &str, filter: fn(&str) -> String) {
        self.filters.insert(name.to_string(), filter);
    }

    fn register_global(&mut self, name: &str, value: serde_json::Value) {
        self.globals.insert(name.to_string(), value);
    }
}

/// Liquid 模板引擎实现
pub struct LiquidTemplateEngine {
    /// Liquid 模板
    template: Template,

    /// 全局变量
    globals: HashMap<String, serde_json::Value>,
}

impl LiquidTemplateEngine {
    /// 创建新的 Liquid 模板引擎
    pub fn new(template: &str) -> Result<Self, TemplateError> {
        let parser = ParserBuilder::with_stdlib().build().map_err(|e| TemplateError::ParseError(e.to_string()))?;
        let template = parser.parse(template).map_err(|e| TemplateError::ParseError(e.to_string()))?;

        Ok(Self { template, globals: HashMap::new() })
    }
}

impl TemplateEngine for LiquidTemplateEngine {
    fn render(&self, template: &str, data: &serde_json::Value) -> Result<String, TemplateError> {
        // 这里使用传入的 template，而不是预编译的模板
        let parser = ParserBuilder::with_stdlib().build().map_err(|e| TemplateError::ParseError(e.to_string()))?;
        let template = parser.parse(template).map_err(|e| TemplateError::ParseError(e.to_string()))?;

        let mut globals = liquid::object!({});

        // 添加全局变量
        for (key, value) in &self.globals {
            globals.insert(key.into(), self.convert_value(value));
        }

        // 添加数据
        if let Some(obj) = data.as_object() {
            for (key, value) in obj {
                globals.insert(key.into(), self.convert_value(value));
            }
        }

        let result = template.render(&globals).map_err(|e| TemplateError::RenderError(e.to_string()))?;
        Ok(result)
    }

    fn register_filter(&mut self, _name: &str, _filter: fn(&str) -> String) {
        // 暂时不支持注册过滤器
    }

    fn register_global(&mut self, name: &str, value: serde_json::Value) {
        self.globals.insert(name.to_string(), value);
    }
}

impl LiquidTemplateEngine {
    /// 将 serde_json::Value 转换为 liquid::model::Value
    fn convert_value(&self, value: &serde_json::Value) -> LiquidValue {
        match value {
            serde_json::Value::Null => LiquidValue::Nil,
            serde_json::Value::Bool(b) => LiquidValue::scalar(*b),
            serde_json::Value::Number(n) => {
                if n.is_i64() {
                    LiquidValue::scalar(n.as_i64().unwrap())
                }
                else if n.is_f64() {
                    LiquidValue::scalar(n.as_f64().unwrap())
                }
                else {
                    LiquidValue::scalar(n.to_string())
                }
            }
            serde_json::Value::String(s) => LiquidValue::scalar(s.clone()),
            serde_json::Value::Array(arr) => {
                let mut liquid_arr = Vec::new();
                for item in arr {
                    liquid_arr.push(self.convert_value(item));
                }
                LiquidValue::array(liquid_arr)
            }
            serde_json::Value::Object(obj) => {
                let mut liquid_obj = liquid::object!({});
                for (key, value) in obj {
                    liquid_obj.insert(key.into(), self.convert_value(value));
                }
                LiquidValue::Object(liquid_obj)
            }
        }
    }
}

/// Handlebars 模板引擎实现
pub struct HandlebarsTemplateEngine {
    /// Handlebars 实例
    handlebars: Handlebars<'static>,
}

impl HandlebarsTemplateEngine {
    /// 创建新的 Handlebars 模板引擎
    pub fn new() -> Self {
        let mut handlebars = Handlebars::new();
        handlebars.set_strict_mode(true);

        Self { handlebars }
    }
}

impl TemplateEngine for HandlebarsTemplateEngine {
    fn render(&self, template: &str, data: &serde_json::Value) -> Result<String, TemplateError> {
        let result = self.handlebars.render_template(template, data).map_err(|e| TemplateError::RenderError(e.to_string()))?;
        Ok(result)
    }

    fn register_filter(&mut self, name: &str, filter: fn(&str) -> String) {
        self.handlebars.register_helper(
            name,
            Box::new(
                move |h: &Helper, _: &Handlebars, _: &Context, _: &mut RenderContext, out: &mut dyn Output| -> HelperResult {
                    let param = h.param(0).unwrap().value().as_str().unwrap_or("");
                    let result = filter(param);
                    out.write(&result)?;
                    Ok(())
                },
            ),
        );
    }

    fn register_global(&mut self, name: &str, value: serde_json::Value) {
        // Handlebars doesn't have a direct register_data method, so we'll handle this differently
        // For now, we'll just ignore this as it's not critical for basic functionality
    }
}

/// Markdown 模板引擎实现
pub struct MarkdownTemplateEngine {
    /// 解析选项
    options: Options,
}

impl MarkdownTemplateEngine {
    /// 创建新的 Markdown 模板引擎
    pub fn new() -> Self {
        let mut options = Options::empty();
        options.insert(Options::ENABLE_TABLES);
        options.insert(Options::ENABLE_FOOTNOTES);
        options.insert(Options::ENABLE_STRIKETHROUGH);
        options.insert(Options::ENABLE_TASKLISTS);

        Self { options }
    }
}

impl TemplateEngine for MarkdownTemplateEngine {
    fn render(&self, template: &str, _data: &serde_json::Value) -> Result<String, TemplateError> {
        let parser = Parser::new_ext(template, self.options);
        let mut html_output = String::new();
        push_html(&mut html_output, parser);
        Ok(html_output)
    }

    fn register_filter(&mut self, _name: &str, _filter: fn(&str) -> String) {
        // Markdown 不支持过滤器
    }

    fn register_global(&mut self, _name: &str, _value: serde_json::Value) {
        // Markdown 不支持全局变量
    }
}

/// 模板引擎工厂
pub struct TemplateEngineFactory {
    /// 引擎映射
    engines: HashMap<String, Box<dyn TemplateEngine>>,

    /// 文件扩展名到引擎的映射
    extension_map: HashMap<String, String>,
}

impl TemplateEngineFactory {
    /// 创建新的模板引擎工厂
    pub fn new() -> Self {
        let mut factory = Self { engines: HashMap::new(), extension_map: HashMap::new() };

        // 注册默认模板引擎
        factory.register_default_engines();
        factory
    }

    /// 注册默认模板引擎
    fn register_default_engines(&mut self) {
        // 注册简单模板引擎
        self.register_engine("simple", Box::new(SimpleTemplateEngine::new()));

        // 注册 Liquid 模板引擎
        self.register_engine("liquid", Box::new(LiquidTemplateEngine::new("").unwrap()));

        // 注册 Handlebars 模板引擎
        self.register_engine("handlebars", Box::new(HandlebarsTemplateEngine::new()));

        // 注册 Markdown 模板引擎
        self.register_engine("markdown", Box::new(MarkdownTemplateEngine::new()));

        // 注册文件扩展名映射
        self.extension_map.insert("liquid".to_string(), "liquid".to_string());
        self.extension_map.insert("hbs".to_string(), "handlebars".to_string());
        self.extension_map.insert("md".to_string(), "markdown".to_string());
        self.extension_map.insert("markdown".to_string(), "markdown".to_string());
    }

    /// 注册模板引擎
    pub fn register_engine(&mut self, name: &str, engine: Box<dyn TemplateEngine>) {
        self.engines.insert(name.to_string(), engine);
    }

    /// 注册文件扩展名映射
    pub fn register_extension(&mut self, extension: &str, engine_name: &str) {
        self.extension_map.insert(extension.to_string(), engine_name.to_string());
    }

    /// 获取模板引擎
    pub fn get_engine(&self, name: &str) -> Option<&Box<dyn TemplateEngine>> {
        self.engines.get(name)
    }

    /// 获取可变模板引擎
    pub fn get_engine_mut(&mut self, name: &str) -> Option<&mut Box<dyn TemplateEngine>> {
        self.engines.get_mut(name)
    }

    /// 根据文件扩展名获取模板引擎
    pub fn get_engine_by_extension(&self, extension: &str) -> Option<&Box<dyn TemplateEngine>> {
        if let Some(engine_name) = self.extension_map.get(extension) {
            self.engines.get(engine_name)
        }
        else {
            // 默认使用简单模板引擎
            self.engines.get("simple")
        }
    }

    /// 渲染模板
    pub fn render(&self, engine_name: &str, template: &str, data: &serde_json::Value) -> Result<String, TemplateError> {
        if let Some(engine) = self.engines.get(engine_name) {
            engine.render(template, data)
        }
        else {
            Err(TemplateError::TemplateNotFoundError(format!("Template engine not found: {}", engine_name)))
        }
    }

    /// 根据文件扩展名渲染模板
    pub fn render_by_extension(
        &self,
        extension: &str,
        template: &str,
        data: &serde_json::Value,
    ) -> Result<String, TemplateError> {
        if let Some(engine) = self.get_engine_by_extension(extension) {
            engine.render(template, data)
        }
        else {
            Err(TemplateError::TemplateNotFoundError(format!("No template engine found for extension: {}", extension)))
        }
    }
}
