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

        Ok(self.create_number_value(result))
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

        Ok(self.create_number_value(result))
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

        Ok(self.create_number_value(result))
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

        Ok(self.create_number_value(result))
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
        Ok(self.create_number_value(result))
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

        Ok(self.create_number_value(max_val.unwrap()))
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

        Ok(self.create_number_value(min_val.unwrap()))
    }

    /// math.Abs - 绝对值
    pub fn abs(&self, args: &[Value]) -> Result<Value, String> {
        if args.is_empty() {
            return Err("abs requires at least 1 argument".to_string());
        }

        let num = args[0].as_f64().ok_or("Argument must be a number")?;
        let result = num.abs();

        Ok(self.create_number_value(result))
    }

    /// 创建数字值，优先使用整数表示
    fn create_number_value(&self, value: f64) -> Value {
        // 检查是否为整数
        if value.fract() == 0.0 {
            // 尝试转换为 i64
            let int_value = value as i64;
            if int_value as f64 == value {
                Value::Number(serde_json::Number::from(int_value))
            } else {
                // 如果超出 i64 范围，使用 f64
                Value::Number(serde_json::Number::from_f64(value).unwrap())
            }
        } else {
            // 非整数使用 f64
            Value::Number(serde_json::Number::from_f64(value).unwrap())
        }
    }
}

impl Default for MathFunctions {
    fn default() -> Self {
        Self::new()
    }
}
