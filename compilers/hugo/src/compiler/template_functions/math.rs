//! 数学函数
//! 提供 Hugo 兼容的数学函数

use serde_json::Value;

/// 数学函数集合
#[derive(Clone)]
pub struct MathFunctions;

impl MathFunctions {
    /// 创建新的数学函数集合
    pub fn new() -> Self {
        Self
    }

    /// add - 加法
    pub fn add(&self, args: &[Value]) -> Result<Value, String> {
        if args.len() < 2 {
            return Err("add requires at least 2 arguments".to_string());
        }

        let mut result = 0.0;
        for arg in args {
            match arg {
                Value::Number(n) => {
                    result += n.as_f64().unwrap_or(0.0);
                }
                _ => {
                    return Err("All arguments must be numbers".to_string());
                }
            }
        }

        Ok(Value::Number(serde_json::Number::from_f64(result).unwrap()))
    }

    /// sub - 减法
    pub fn sub(&self, args: &[Value]) -> Result<Value, String> {
        if args.len() < 2 {
            return Err("sub requires at least 2 arguments".to_string());
        }

        let first = args[0].as_f64().ok_or("First argument must be a number")?;
        let mut result = first;

        for arg in &args[1..] {
            let num = arg.as_f64().ok_or("All arguments must be numbers")?;
            result -= num;
        }

        Ok(Value::Number(serde_json::Number::from_f64(result).unwrap()))
    }

    /// mul - 乘法
    pub fn mul(&self, args: &[Value]) -> Result<Value, String> {
        if args.len() < 2 {
            return Err("mul requires at least 2 arguments".to_string());
        }

        let mut result = 1.0;
        for arg in args {
            match arg {
                Value::Number(n) => {
                    result *= n.as_f64().unwrap_or(0.0);
                }
                _ => {
                    return Err("All arguments must be numbers".to_string());
                }
            }
        }

        Ok(Value::Number(serde_json::Number::from_f64(result).unwrap()))
    }

    /// div - 除法
    pub fn div(&self, args: &[Value]) -> Result<Value, String> {
        if args.len() < 2 {
            return Err("div requires at least 2 arguments".to_string());
        }

        let first = args[0].as_f64().ok_or("First argument must be a number")?;
        let mut result = first;

        for arg in &args[1..] {
            let num = arg.as_f64().ok_or("All arguments must be numbers")?;
            if num == 0.0 {
                return Err("Division by zero".to_string());
            }
            result /= num;
        }

        Ok(Value::Number(serde_json::Number::from_f64(result).unwrap()))
    }

    /// mod - 取模
    pub fn mod_(&self, args: &[Value]) -> Result<Value, String> {
        if args.len() < 2 {
            return Err("mod requires at least 2 arguments".to_string());
        }

        let first = args[0].as_f64().ok_or("First argument must be a number")?;
        let second = args[1].as_f64().ok_or("Second argument must be a number")?;

        if second == 0.0 {
            return Err("Division by zero".to_string());
        }

        let result = first % second;
        Ok(Value::Number(serde_json::Number::from_f64(result).unwrap()))
    }

    /// math.Max - 最大值
    pub fn max(&self, args: &[Value]) -> Result<Value, String> {
        if args.is_empty() {
            return Err("max requires at least 1 argument".to_string());
        }

        let mut max_val = None;
        for arg in args {
            let num = arg.as_f64().ok_or("All arguments must be numbers")?;
            if max_val.is_none() || num > max_val.unwrap() {
                max_val = Some(num);
            }
        }

        Ok(Value::Number(serde_json::Number::from_f64(max_val.unwrap()).unwrap()))
    }

    /// math.Min - 最小值
    pub fn min(&self, args: &[Value]) -> Result<Value, String> {
        if args.is_empty() {
            return Err("min requires at least 1 argument".to_string());
        }

        let mut min_val = None;
        for arg in args {
            let num = arg.as_f64().ok_or("All arguments must be numbers")?;
            if min_val.is_none() || num < min_val.unwrap() {
                min_val = Some(num);
            }
        }

        Ok(Value::Number(serde_json::Number::from_f64(min_val.unwrap()).unwrap()))
    }

    /// math.Abs - 绝对值
    pub fn abs(&self, args: &[Value]) -> Result<Value, String> {
        if args.is_empty() {
            return Err("abs requires at least 1 argument".to_string());
        }

        let num = args[0].as_f64().ok_or("Argument must be a number")?;
        let result = num.abs();

        Ok(Value::Number(serde_json::Number::from_f64(result).unwrap()))
    }
}

impl Default for MathFunctions {
    fn default() -> Self {
        Self::new()
    }
}
