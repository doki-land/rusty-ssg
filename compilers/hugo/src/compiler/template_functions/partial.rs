//! Partial 模板函数
//! 提供 Hugo 兼容的 Partial 模板系统

use serde_json::Value;
use std::collections::HashMap;
use std::sync::RwLock;

/// Partial 模板函数集合
pub struct PartialFunctions {
    /// 模板缓存
    cache: RwLock<HashMap<String, String>>,
}

impl PartialFunctions {
    /// 创建新的 Partial 函数集合
    pub fn new() -> Self {
        Self {
            cache: RwLock::new(HashMap::new()),
        }
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
        
        // 这里简化实现，实际应该从模板管理器中获取模板
        // 暂时返回模板名称作为结果
        Ok(Value::String(format!("[Partial: {}]", template_name)))
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
        
        // 生成缓存键
        let cache_key = if args.len() > 1 {
            format!("{}:{}", template_name, args[1])
        } else {
            template_name.to_string()
        };

        // 检查缓存
        if let Ok(cache) = self.cache.read() {
            if let Some(cached) = cache.get(&cache_key) {
                return Ok(Value::String(cached.clone()));
            }
        }

        // 渲染模板（简化实现）
        let result = format!("[PartialCached: {}]", template_name);

        // 存入缓存
        if let Ok(mut cache) = self.cache.write() {
            cache.insert(cache_key, result.clone());
        }

        Ok(Value::String(result))
    }

    /// 清除缓存
    pub fn clear_cache(&self) {
        if let Ok(mut cache) = self.cache.write() {
            cache.clear();
        }
    }

    /// 获取缓存大小
    pub fn cache_size(&self) -> usize {
        if let Ok(cache) = self.cache.read() {
            cache.len()
        } else {
            0
        }
    }
}

impl Default for PartialFunctions {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_partial() {
        let funcs = PartialFunctions::new();
        
        let result = funcs.partial(&[json!("header.html")]).unwrap();
        assert!(result.as_str().unwrap().contains("header.html"));
    }

    #[test]
    fn test_partial_cached() {
        let funcs = PartialFunctions::new();
        
        // 第一次调用，应该生成新内容
        let result1 = funcs.partial_cached(&[json!("footer.html")]).unwrap();
        assert!(result1.as_str().unwrap().contains("footer.html"));
        assert_eq!(funcs.cache_size(), 1);

        // 第二次调用，应该返回缓存的内容
        let result2 = funcs.partial_cached(&[json!("footer.html")]).unwrap();
        assert_eq!(result1, result2);
        assert_eq!(funcs.cache_size(), 1);

        // 清除缓存
        funcs.clear_cache();
        assert_eq!(funcs.cache_size(), 0);
    }

    #[test]
    fn test_partial_cached_with_key() {
        let funcs = PartialFunctions::new();
        
        // 使用不同的缓存键
        let result1 = funcs.partial_cached(&[json!("sidebar.html"), json!("key1")]).unwrap();
        let result2 = funcs.partial_cached(&[json!("sidebar.html"), json!("key2")]).unwrap();
        
        assert!(result1.as_str().unwrap().contains("sidebar.html"));
        assert!(result2.as_str().unwrap().contains("sidebar.html"));
        assert_eq!(funcs.cache_size(), 2);
    }
}
