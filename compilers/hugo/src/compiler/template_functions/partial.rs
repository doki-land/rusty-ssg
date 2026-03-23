//! Partial 模板函数
//! 提供 Hugo 兼容的 Partial 模板系统

use crate::compiler::hugo_template::{HugoTemplateEngine, TemplateResolver};
use serde_json::Value;
use std::{collections::HashMap, path::Path, sync::RwLock};

/// Partial 模板函数集合
pub struct PartialFunctions {
    /// 模板缓存
    cache: RwLock<HashMap<String, String>>,
    /// 模板解析器
    resolver: Option<TemplateResolver>,
    /// 模板引擎
    engine: Option<HugoTemplateEngine>,
}

impl PartialFunctions {
    /// 创建新的 Partial 函数集合
    pub fn new() -> Self {
        Self { cache: RwLock::new(HashMap::new()), resolver: None, engine: None }
    }

    /// 设置模板解析器
    ///
    /// # Arguments
    ///
    /// * `resolver` - 模板解析器
    pub fn with_resolver(mut self, resolver: TemplateResolver) -> Self {
        self.resolver = Some(resolver);
        self
    }

    /// 设置模板引擎
    ///
    /// # Arguments
    ///
    /// * `engine` - 模板引擎
    pub fn with_engine(mut self, engine: HugoTemplateEngine) -> Self {
        self.engine = Some(engine);
        self
    }

    /// partial - 渲染 Partial 模板
    ///
    /// # Arguments
    ///
    /// * `args[0]` - 模板名称
    /// * `args[1]` - 上下文数据（可选）
    pub fn partial(&self, args: &[Value]) -> Result<Value, String> {
        if args.is_empty() {
            return Err("partial requires at least 1 argument".to_string());
        }

        let template_name = args[0].as_str().ok_or("First argument must be a string")?;

        // 构建 Partial 模板路径
        let partial_path = if template_name.starts_with("partials/") {
            template_name.to_string()
        }
        else {
            format!("partials/{}", template_name)
        };

        // 检查模板是否存在
        if let Some(resolver) = &self.resolver {
            if !resolver.template_exists(&partial_path) {
                return Err(format!("Partial template not found: {}", partial_path));
            }
        }

        // 渲染模板
        let result = self.render_partial(&partial_path, args.get(1))?;

        Ok(Value::String(result))
    }

    /// partialCached - 渲染 Partial 模板（带缓存）
    ///
    /// # Arguments
    ///
    /// * `args[0]` - 模板名称
    /// * `args[1]` - 缓存键（可选）
    /// * `args[2]` - 上下文数据（可选）
    pub fn partial_cached(&self, args: &[Value]) -> Result<Value, String> {
        if args.is_empty() {
            return Err("partialCached requires at least 1 argument".to_string());
        }

        let template_name = args[0].as_str().ok_or("First argument must be a string")?;

        // 构建 Partial 模板路径
        let partial_path = if template_name.starts_with("partials/") {
            template_name.to_string()
        }
        else {
            format!("partials/{}", template_name)
        };

        // 生成缓存键
        let cache_key = if args.len() > 1 { format!("{}:{}", partial_path, args[1]) } else { partial_path.clone() };

        // 检查缓存
        if let Ok(cache) = self.cache.read() {
            if let Some(cached) = cache.get(&cache_key) {
                return Ok(Value::String(cached.clone()));
            }
        }

        // 渲染模板
        let result = self.render_partial(&partial_path, args.get(2))?;

        // 存入缓存
        if let Ok(mut cache) = self.cache.write() {
            cache.insert(cache_key, result.clone());
        }

        Ok(Value::String(result))
    }

    /// 渲染 Partial 模板
    ///
    /// # Arguments
    ///
    /// * `partial_path` - Partial 模板路径
    /// * `context` - 上下文数据（可选）
    fn render_partial(&self, partial_path: &str, context: Option<&Value>) -> Result<String, String> {
        // 这里需要实际的模板渲染逻辑
        // 暂时返回一个模拟的渲染结果
        Ok(format!("<div class=\"partial\">Rendered partial: {}</div>", partial_path))
    }

    /// 清除缓存
    pub fn clear_cache(&self) {
        if let Ok(mut cache) = self.cache.write() {
            cache.clear();
        }
    }

    /// 获取缓存大小
    pub fn cache_size(&self) -> usize {
        if let Ok(cache) = self.cache.read() { cache.len() } else { 0 }
    }
}

impl Default for PartialFunctions {
    fn default() -> Self {
        Self::new()
    }
}
