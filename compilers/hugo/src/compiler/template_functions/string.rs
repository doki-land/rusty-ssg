//! 字符串处理函数
//! 提供 Hugo 兼容的字符串处理函数

use serde_json::Value;
use slug::Slugify;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref WHITESPACE_RE: Regex = Regex::new(r"\s+").unwrap();
    static ref NON_ALPHANUMERIC_RE: Regex = Regex::new(r"[^a-zA-Z0-9\s]").unwrap();
}

/// 字符串处理函数集合
pub struct StringFunctions;

impl StringFunctions {
    /// 创建新的字符串函数集合
    pub fn new() -> Self {
        Self
    }

    /// lower - 转换为小写
    pub fn lower(&self, args: &[Value]) -> Result<Value, String> {
        if args.is_empty() {
            return Err("lower requires at least 1 argument".to_string());
        }

        let input = args[0].as_str().ok_or("Argument must be a string")?;
        Ok(Value::String(input.to_lowercase()))
    }

    /// upper - 转换为大写
    pub fn upper(&self, args: &[Value]) -> Result<Value, String> {
        if args.is_empty() {
            return Err("upper requires at least 1 argument".to_string());
        }

        let input = args[0].as_str().ok_or("Argument must be a string")?;
        Ok(Value::String(input.to_uppercase()))
    }

    /// title - 转换为标题格式（每个单词首字母大写）
    pub fn title(&self, args: &[Value]) -> Result<Value, String> {
        if args.is_empty() {
            return Err("title requires at least 1 argument".to_string());
        }

        let input = args[0].as_str().ok_or("Argument must be a string")?;
        
        let title = input
            .split_whitespace()
            .map(|word| {
                let mut chars = word.chars();
                match chars.next() {
                    None => String::new(),
                    Some(first) => {
                        first.to_uppercase().collect::<String>() + &chars.as_str().to_lowercase()
                    }
                }
            })
            .collect::<Vec<_>>()
            .join(" ");

        Ok(Value::String(title))
    }

    /// slug - 转换为 URL slug 格式
    pub fn slug(&self, args: &[Value]) -> Result<Value, String> {
        if args.is_empty() {
            return Err("slug requires at least 1 argument".to_string());
        }

        let input = args[0].as_str().ok_or("Argument must be a string")?;
        
        let mut slug = String::new();
        for c in input.slugify() {
            if c.is_alphanumeric() || c == '-' {
                slug.push(c);
            }
        }
        
        let re = Regex::new(r"-+").unwrap();
        let slug = re.replace_all(&slug, "-");
        
        Ok(Value::String(slug.trim_matches('-').to_string()))
    }

    /// truncate - 截断字符串
    ///
    /// # Arguments
    ///
    /// * `args[0]` - 最大长度
    /// * `args[1]` - 要截断的字符串（可选，默认为空）
    /// * `args[2]` - 省略符（可选，默认为 "..."）
    pub fn truncate(&self, args: &[Value]) -> Result<Value, String> {
        if args.is_empty() {
            return Err("truncate requires at least 1 argument".to_string());
        }

        let length = args[0].as_i64().ok_or("First argument must be an integer")? as usize;
        
        let input = if args.len() > 1 {
            args[1].as_str().unwrap_or("")
        } else {
            ""
        };
        
        let suffix = if args.len() > 2 {
            args[2].as_str().unwrap_or("...")
        } else {
            "..."
        };

        if input.chars().count() <= length {
            return Ok(Value::String(input.to_string()));
        }

        let mut result = String::new();
        let mut count = 0;
        
        for c in input.chars() {
            if count >= length {
                break;
            }
            result.push(c);
            count += 1;
        }
        
        result.push_str(suffix);
        
        Ok(Value::String(result))
    }

    /// replace - 简单字符串替换
    ///
    /// # Arguments
    ///
    /// * `args[0]` - 要替换的字符串
    /// * `args[1]` - 查找的字符串
    /// * `args[2]` - 替换为的字符串
    pub fn replace(&self, args: &[Value]) -> Result<Value, String> {
        if args.len() < 3 {
            return Err("replace requires 3 arguments".to_string());
        }

        let input = args[0].as_str().ok_or("First argument must be a string")?;
        let find = args[1].as_str().ok_or("Second argument must be a string")?;
        let replace = args[2].as_str().ok_or("Third argument must be a string")?;

        Ok(Value::String(input.replace(find, replace)))
    }

    /// substr - 获取子字符串
    ///
    /// # Arguments
    ///
    /// * `args[0]` - 字符串
    /// * `args[1]` - 起始位置
    /// * `args[2]` - 长度（可选）
    pub fn substr(&self, args: &[Value]) -> Result<Value, String> {
        if args.len() < 2 {
            return Err("substr requires at least 2 arguments".to_string());
        }

        let input = args[0].as_str().ok_or("First argument must be a string")?;
        let start = args[1].as_i64().ok_or("Second argument must be an integer")? as isize;

        let chars: Vec<char> = input.chars().collect();
        let len = chars.len() as isize;
        
        let start = if start < 0 { len + start } else { start };
        
        if start < 0 || start >= len {
            return Ok(Value::String(String::new()));
        }

        let result = if args.len() > 2 {
            let count = args[2].as_i64().ok_or("Third argument must be an integer")? as usize;
            let end = (start as usize).saturating_add(count).min(chars.len());
            chars[start as usize..end].iter().collect()
        } else {
            chars[start as usize..].iter().collect()
        };

        Ok(Value::String(result))
    }

    /// split - 分割字符串
    ///
    /// # Arguments
    ///
    /// * `args[0]` - 字符串
    /// * `args[1]` - 分隔符
    pub fn split(&self, args: &[Value]) -> Result<Value, String> {
        if args.len() < 2 {
            return Err("split requires 2 arguments".to_string());
        }

        let input = args[0].as_str().ok_or("First argument must be a string")?;
        let sep = args[1].as_str().ok_or("Second argument must be a string")?;

        let parts: Vec<Value> = input
            .split(sep)
            .map(|s| Value::String(s.to_string()))
            .collect();

        Ok(Value::Array(parts))
    }

    /// trim - 去除首尾空白
    ///
    /// # Arguments
    ///
    /// * `args[0]` - 字符串
    /// * `args[1]` - 要去除的字符集（可选）
    pub fn trim(&self, args: &[Value]) -> Result<Value, String> {
        if args.is_empty() {
            return Err("trim requires at least 1 argument".to_string());
        }

        let input = args[0].as_str().ok_or("First argument must be a string")?;

        let result = if args.len() > 1 {
            let chars = args[1].as_str().unwrap_or(" ");
            let chars: Vec<char> = chars.chars().collect();
            input.trim_matches(chars.as_slice())
        } else {
            input.trim()
        };

        Ok(Value::String(result.to_string()))
    }

    /// trimSuffix - 去除后缀
    ///
    /// # Arguments
    ///
    /// * `args[0]` - 字符串
    /// * `args[1]` - 要去除的后缀
    pub fn trim_suffix(&self, args: &[Value]) -> Result<Value, String> {
        if args.len() < 2 {
            return Err("trimSuffix requires 2 arguments".to_string());
        }

        let input = args[0].as_str().ok_or("First argument must be a string")?;
        let suffix = args[1].as_str().ok_or("Second argument must be a string")?;

        let result = if input.ends_with(suffix) {
            &input[..input.len() - suffix.len()]
        } else {
            input
        };

        Ok(Value::String(result.to_string()))
    }

    /// trimPrefix - 去除前缀
    ///
    /// # Arguments
    ///
    /// * `args[0]` - 字符串
    /// * `args[1]` - 要去除的前缀
    pub fn trim_prefix(&self, args: &[Value]) -> Result<Value, String> {
        if args.len() < 2 {
            return Err("trimPrefix requires 2 arguments".to_string());
        }

        let input = args[0].as_str().ok_or("First argument must be a string")?;
        let prefix = args[1].as_str().ok_or("Second argument must be a string")?;

        let result = if input.starts_with(prefix) {
            &input[prefix.len()..]
        } else {
            input
        };

        Ok(Value::String(result.to_string()))
    }

    /// replaceRE - 使用正则表达式替换
    ///
    /// # Arguments
    ///
    /// * `args[0]` - 正则表达式
    /// * `args[1]` - 替换为的字符串
    /// * `args[2]` - 要处理的字符串
    pub fn replace_re(&self, args: &[Value]) -> Result<Value, String> {
        if args.len() < 3 {
            return Err("replaceRE requires 3 arguments".to_string());
        }

        let pattern = args[0].as_str().ok_or("First argument must be a string")?;
        let replacement = args[1].as_str().ok_or("Second argument must be a string")?;
        let input = args[2].as_str().ok_or("Third argument must be a string")?;

        let re = Regex::new(pattern).map_err(|e| format!("Invalid regex: {}", e))?;
        let result = re.replace_all(input, replacement);

        Ok(Value::String(result.to_string()))
    }
}

impl Default for StringFunctions {
    fn default() -> Self {
        Self::new()
    }
}
