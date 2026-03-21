//! 模板引擎模块

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
        Self {
            filters: HashMap::new(),
            globals: HashMap::new(),
        }
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

/// 模板引擎工厂
pub struct TemplateEngineFactory {
    /// 引擎映射
    engines: HashMap<String, Box<dyn TemplateEngine>>,
}

impl TemplateEngineFactory {
    /// 创建新的模板引擎工厂
    pub fn new() -> Self {
        Self {
            engines: HashMap::new(),
        }
    }
    
    /// 注册模板引擎
    pub fn register_engine(&mut self, name: &str, engine: Box<dyn TemplateEngine>) {
        self.engines.insert(name.to_string(), engine);
    }
    
    /// 获取模板引擎
    pub fn get_engine(&self, name: &str) -> Option<&Box<dyn TemplateEngine>> {
        self.engines.get(name)
    }
    
    /// 获取可变模板引擎
    pub fn get_engine_mut(&mut self, name: &str) -> Option<&mut Box<dyn TemplateEngine>> {
        self.engines.get_mut(name)
    }
    
    /// 渲染模板
    pub fn render(&self, engine_name: &str, template: &str, data: &serde_json::Value) -> Result<String, TemplateError> {
        if let Some(engine) = self.engines.get(engine_name) {
            engine.render(template, data)
        } else {
            Err(TemplateError::TemplateNotFoundError(format!("Template engine not found: {}", engine_name)))
        }
    }
}