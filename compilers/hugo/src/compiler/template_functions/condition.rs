//! 条件函数
//! 提供 Hugo 兼容的条件处理函数

use serde_json::Value;

/// 条件函数集合
pub struct ConditionFunctions;

impl ConditionFunctions {
    /// 创建新的条件函数集合
    pub fn new() -> Self {
        Self
    }

    /// cond - 条件表达式
    ///
    /// # Arguments
    ///
    /// * `args[0]` - 条件表达式
    /// * `args[1]` - 条件为真时返回的值
    /// * `args[2]` - 条件为假时返回的值
    pub fn cond(&self, args: &[Value]) -> Result<Value, String> {
        if args.len() < 3 {
            return Err("cond requires at least 3 arguments".to_string());
        }

        let condition = args[0].as_bool().unwrap_or(false);
        
        Ok(if condition {
            args[1].clone()
        } else {
            args[2].clone()
        })
    }

    /// default - 默认值
    ///
    /// # Arguments
    ///
    /// * `args[0]` - 要检查的值
    /// * `args[1]` - 默认值
    pub fn default(&self, args: &[Value]) -> Result<Value, String> {
        if args.len() < 2 {
            return Err("default requires 2 arguments".to_string());
        }

        let value = &args[0];
        let default_value = &args[1];

        if Self::is_empty(value) {
            Ok(default_value.clone())
        } else {
            Ok(value.clone())
        }
    }

    /// isset - 检查变量是否存在
    ///
    /// # Arguments
    ///
    /// * `args[0]` - 要检查的变量
    pub fn isset(&self, args: &[Value]) -> Result<Value, String> {
        if args.is_empty() {
            return Err("isset requires at least 1 argument".to_string());
        }

        let value = &args[0];
        
        // 在 serde_json 中，null 表示不存在或未设置
        Ok(Value::Bool(!value.is_null()))
    }

    /// empty - 检查是否为空
    ///
    /// # Arguments
    ///
    /// * `args[0]` - 要检查的值
    pub fn empty(&self, args: &[Value]) -> Result<Value, String> {
        if args.is_empty() {
            return Err("empty requires at least 1 argument".to_string());
        }

        let value = &args[0];
        Ok(Value::Bool(Self::is_empty(value)))
    }

    /// 检查值是否为空
    fn is_empty(value: &Value) -> bool {
        match value {
            Value::Null => true,
            Value::Bool(false) => true,
            Value::Number(n) if n.as_f64().unwrap_or(0.0) == 0.0 => true,
            Value::String(s) => s.is_empty(),
            Value::Array(arr) => arr.is_empty(),
            Value::Object(obj) => obj.is_empty(),
            _ => false,
        }
    }
}

impl Default for ConditionFunctions {
    fn default() -> Self {
        Self::new()
    }
}
