//! 集合处理函数
//! 提供 Hugo 兼容的集合处理函数

use serde_json::Value;
use std::collections::HashMap;

/// 集合处理函数集合
pub struct CollectionFunctions;

impl CollectionFunctions {
    /// 创建新的集合函数集合
    pub fn new() -> Self {
        Self
    }

    /// slice - 创建切片
    ///
    /// # Arguments
    ///
    /// * `args` - 可变数量的参数，用于创建切片
    pub fn slice(&self, args: &[Value]) -> Result<Value, String> {
        Ok(Value::Array(args.to_vec()))
    }

    /// dict - 创建字典
    ///
    /// # Arguments
    ///
    /// * `args` - 键值对参数（key1, value1, key2, value2, ...）
    pub fn dict(&self, args: &[Value]) -> Result<Value, String> {
        if args.len() % 2 != 0 {
            return Err("dict requires an even number of arguments".to_string());
        }

        let mut map = HashMap::new();
        
        for chunk in args.chunks(2) {
            let key = chunk[0].as_str().ok_or("Dictionary keys must be strings")?;
            map.insert(key.to_string(), chunk[1].clone());
        }

        Ok(Value::Object(map.into_iter().collect()))
    }

    /// first - 获取集合的前 N 个元素
    ///
    /// # Arguments
    ///
    /// * `args[0]` - 数量 N
    /// * `args[1]` - 集合
    pub fn first(&self, args: &[Value]) -> Result<Value, String> {
        if args.len() < 2 {
            return Err("first requires 2 arguments".to_string());
        }

        let n = args[0].as_i64().ok_or("First argument must be an integer")? as usize;
        let arr = args[1].as_array().ok_or("Second argument must be an array")?;

        let result: Vec<Value> = arr.iter().take(n).cloned().collect();
        Ok(Value::Array(result))
    }

    /// last - 获取集合的最后 N 个元素
    ///
    /// # Arguments
    ///
    /// * `args[0]` - 数量 N
    /// * `args[1]` - 集合
    pub fn last(&self, args: &[Value]) -> Result<Value, String> {
        if args.len() < 2 {
            return Err("last requires 2 arguments".to_string());
        }

        let n = args[0].as_i64().ok_or("First argument must be an integer")? as usize;
        let arr = args[1].as_array().ok_or("Second argument must be an array")?;

        let skip = if arr.len() > n { arr.len() - n } else { 0 };
        let result: Vec<Value> = arr.iter().skip(skip).cloned().collect();
        Ok(Value::Array(result))
    }

    /// after - 获取集合中指定位置之后的所有元素
    ///
    /// # Arguments
    ///
    /// * `args[0]` - 起始位置
    /// * `args[1]` - 集合
    pub fn after(&self, args: &[Value]) -> Result<Value, String> {
        if args.len() < 2 {
            return Err("after requires 2 arguments".to_string());
        }

        let n = args[0].as_i64().ok_or("First argument must be an integer")? as usize;
        let arr = args[1].as_array().ok_or("Second argument must be an array")?;

        let result: Vec<Value> = arr.iter().skip(n).cloned().collect();
        Ok(Value::Array(result))
    }

    /// before - 获取集合中指定位置之前的所有元素
    ///
    /// # Arguments
    ///
    /// * `args[0]` - 结束位置
    /// * `args[1]` - 集合
    pub fn before(&self, args: &[Value]) -> Result<Value, String> {
        if args.len() < 2 {
            return Err("before requires 2 arguments".to_string());
        }

        let n = args[0].as_i64().ok_or("First argument must be an integer")? as usize;
        let arr = args[1].as_array().ok_or("Second argument must be an array")?;

        let result: Vec<Value> = arr.iter().take(n).cloned().collect();
        Ok(Value::Array(result))
    }

    /// append - 在集合末尾添加元素
    ///
    /// # Arguments
    ///
    /// * `args[0]` - 集合
    /// * `args[1..]` - 要添加的元素
    pub fn append(&self, args: &[Value]) -> Result<Value, String> {
        if args.is_empty() {
            return Err("append requires at least 1 argument".to_string());
        }

        let mut arr = args[0].as_array().ok_or("First argument must be an array")?.clone();
        
        for item in &args[1..] {
            arr.push(item.clone());
        }

        Ok(Value::Array(arr))
    }

    /// prepend - 在集合开头添加元素
    ///
    /// # Arguments
    ///
    /// * `args[0]` - 集合
    /// * `args[1..]` - 要添加的元素
    pub fn prepend(&self, args: &[Value]) -> Result<Value, String> {
        if args.is_empty() {
            return Err("prepend requires at least 1 argument".to_string());
        }

        let arr = args[0].as_array().ok_or("First argument must be an array")?.clone();
        let mut result = args[1..].to_vec();
        result.extend(arr);

        Ok(Value::Array(result))
    }
}

impl Default for CollectionFunctions {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_slice() {
        let funcs = CollectionFunctions::new();
        
        assert_eq!(
            funcs.slice(&[json!(1), json!(2), json!(3)]).unwrap(),
            json!([1, 2, 3])
        );
    }

    #[test]
    fn test_dict() {
        let funcs = CollectionFunctions::new();
        
        let result = funcs.dict(&[json!("key1"), json!("value1"), json!("key2"), json!("value2")]).unwrap();
        
        assert_eq!(result["key1"], json!("value1"));
        assert_eq!(result["key2"], json!("value2"));
    }

    #[test]
    fn test_first() {
        let funcs = CollectionFunctions::new();
        
        assert_eq!(
            funcs.first(&[json!(2), json!([1, 2, 3, 4, 5])]).unwrap(),
            json!([1, 2])
        );
    }

    #[test]
    fn test_last() {
        let funcs = CollectionFunctions::new();
        
        assert_eq!(
            funcs.last(&[json!(2), json!([1, 2, 3, 4, 5])]).unwrap(),
            json!([4, 5])
        );
    }
}
