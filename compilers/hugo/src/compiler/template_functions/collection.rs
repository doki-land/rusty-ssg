//! 集合处理函数
//! 提供 Hugo 兼容的集合处理函数

use serde_json::Value;
use std::collections::HashMap;
use rand::seq::SliceRandom;
use rand::thread_rng;

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

    /// shuffle - 随机打乱集合
    ///
    /// # Arguments
    ///
    /// * `args[0]` - 集合
    pub fn shuffle(&self, args: &[Value]) -> Result<Value, String> {
        if args.is_empty() {
            return Err("shuffle requires at least 1 argument".to_string());
        }

        let mut arr = args[0].as_array().ok_or("First argument must be an array")?.clone();
        let mut rng = rand::thread_rng();
        arr.shuffle(&mut rng);

        Ok(Value::Array(arr))
    }

    /// seq - 生成数字序列
    ///
    /// # Arguments
    ///
    /// * `args[0]` - 结束值
    /// * `args[1]` - 起始值（可选，默认为 1）
    /// * `args[2]` - 步长（可选，默认为 1）
    pub fn seq(&self, args: &[Value]) -> Result<Value, String> {
        if args.is_empty() {
            return Err("seq requires at least 1 argument".to_string());
        }

        let end = args[0].as_i64().ok_or("First argument must be an integer")?;
        let start = if args.len() > 1 {
            args[1].as_i64().ok_or("Second argument must be an integer")?
        } else {
            1
        };
        let step = if args.len() > 2 {
            args[2].as_i64().ok_or("Third argument must be an integer")?
        } else {
            1
        };

        if step == 0 {
            return Err("Step cannot be zero".to_string());
        }

        let mut result = Vec::new();
        let mut current = start;

        if step > 0 {
            while current <= end {
                result.push(Value::Number(current.into()));
                current += step;
            }
        } else {
            while current >= end {
                result.push(Value::Number(current.into()));
                current += step;
            }
        }

        Ok(Value::Array(result))
    }

    /// union - 集合合并（去重）
    ///
    /// # Arguments
    ///
    /// * `args[0]` - 第一个集合
    /// * `args[1]` - 第二个集合
    pub fn union(&self, args: &[Value]) -> Result<Value, String> {
        if args.len() < 2 {
            return Err("union requires 2 arguments".to_string());
        }

        let arr1 = args[0].as_array().ok_or("First argument must be an array")?;
        let arr2 = args[1].as_array().ok_or("Second argument must be an array")?;

        let mut seen = std::collections::HashSet::new();
        let mut result = Vec::new();

        for item in arr1 {
            if seen.insert(item.clone()) {
                result.push(item.clone());
            }
        }

        for item in arr2 {
            if seen.insert(item.clone()) {
                result.push(item.clone());
            }
        }

        Ok(Value::Array(result))
    }

    /// intersection - 集合交集
    ///
    /// # Arguments
    ///
    /// * `args[0]` - 第一个集合
    /// * `args[1]` - 第二个集合
    pub fn intersection(&self, args: &[Value]) -> Result<Value, String> {
        if args.len() < 2 {
            return Err("intersection requires 2 arguments".to_string());
        }

        let arr1 = args[0].as_array().ok_or("First argument must be an array")?;
        let arr2 = args[1].as_array().ok_or("Second argument must be an array")?;

        let set1: std::collections::HashSet<_> = arr1.iter().cloned().collect();
        let mut result = Vec::new();

        for item in arr2 {
            if set1.contains(item) {
                result.push(item.clone());
            }
        }

        Ok(Value::Array(result))
    }

    /// difference - 集合差集
    ///
    /// # Arguments
    ///
    /// * `args[0]` - 第一个集合
    /// * `args[1]` - 第二个集合
    pub fn difference(&self, args: &[Value]) -> Result<Value, String> {
        if args.len() < 2 {
            return Err("difference requires 2 arguments".to_string());
        }

        let arr1 = args[0].as_array().ok_or("First argument must be an array")?;
        let arr2 = args[1].as_array().ok_or("Second argument must be an array")?;

        let set2: std::collections::HashSet<_> = arr2.iter().cloned().collect();
        let mut result = Vec::new();

        for item in arr1 {
            if !set2.contains(item) {
                result.push(item.clone());
            }
        }

        Ok(Value::Array(result))
    }

    /// apply - 对集合中的每个元素应用函数
    ///
    /// # Arguments
    ///
    /// * `args[0]` - 函数名称
    /// * `args[1]` - 集合
    pub fn apply(&self, args: &[Value]) -> Result<Value, String> {
        if args.len() < 2 {
            return Err("apply requires 2 arguments".to_string());
        }

        let func_name = args[0].as_str().ok_or("First argument must be a string")?;
        let arr = args[1].as_array().ok_or("Second argument must be an array")?;

        // 这里简化实现，实际应该调用对应的函数
        // 由于函数注册表在 TemplateFunctions 中，这里需要更复杂的实现
        // 暂时返回原集合
        Ok(Value::Array(arr.clone()))
    }
}

impl Default for CollectionFunctions {
    fn default() -> Self {
        Self::new()
    }
}
