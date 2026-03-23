//! 日期处理函数
//! 提供 Hugo 兼容的日期处理函数

use serde_json::Value;
use chrono::{DateTime, Utc, NaiveDateTime, TimeZone};

/// 日期处理函数集合
pub struct DateFunctions;

impl DateFunctions {
    /// 创建新的日期函数集合
    pub fn new() -> Self {
        Self
    }

    /// now - 获取当前时间
    ///
    /// # Arguments
    ///
    /// 无参数
    pub fn now(&self, _args: &[Value]) -> Result<Value, String> {
        let now = Utc::now();
        Ok(Value::String(now.to_rfc3339()))
    }

    /// dateFormat - 格式化日期
    ///
    /// # Arguments
    ///
    /// * `args[0]` - 格式字符串
    /// * `args[1]` - 日期字符串（可选，默认为当前时间）
    pub fn date_format(&self, args: &[Value]) -> Result<Value, String> {
        if args.is_empty() {
            return Err("dateFormat requires at least 1 argument".to_string());
        }

        let format = args[0].as_str().ok_or("First argument must be a string")?;

        let dt = if args.len() > 1 {
            let date_str = args[1].as_str().ok_or("Second argument must be a string")?;
            Self::parse_date(date_str)?
        } else {
            Utc::now()
        };

        let formatted = dt.format(format).to_string();
        Ok(Value::String(formatted))
    }

    /// 解析日期字符串
    fn parse_date(date_str: &str) -> Result<DateTime<Utc>, String> {
        if let Ok(dt) = DateTime::parse_from_rfc3339(date_str) {
            return Ok(dt.with_timezone(&Utc));
        }

        let formats = [
            "%Y-%m-%d %H:%M:%S",
            "%Y-%m-%d %H:%M",
            "%Y-%m-%d",
            "%Y/%m/%d",
            "%d/%m/%Y",
            "%m/%d/%Y",
        ];

        for format in &formats {
            if let Ok(ndt) = NaiveDateTime::parse_from_str(date_str, format) {
                if let Some(dt) = Utc.from_utc_datetime(&ndt).single() {
                    return Ok(dt);
                }
            }
            if let Ok(nd) = chrono::NaiveDate::parse_from_str(date_str, format) {
                if let Some(dt) = Utc.from_utc_datetime(&nd.and_hms_opt(0, 0, 0).unwrap()).single() {
                    return Ok(dt);
                }
            }
        }

        Err(format!("Unable to parse date: {}", date_str))
    }
}

impl Default for DateFunctions {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_now() {
        let funcs = DateFunctions::new();
        let result = funcs.now(&[]).unwrap();
        assert!(result.is_string());
    }

    #[test]
    fn test_date_format() {
        let funcs = DateFunctions::new();
        
        assert_eq!(
            funcs.date_format(&[json!("%Y-%m-%d"), json!("2024-01-15")]).unwrap(),
            json!("2024-01-15")
        );
    }
}
