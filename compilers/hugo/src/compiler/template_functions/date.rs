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

    /// time - 解析时间字符串
    ///
    /// # Arguments
    ///
    /// * `args[0]` - 时间字符串
    pub fn time(&self, args: &[Value]) -> Result<Value, String> {
        if args.is_empty() {
            return Err("time requires at least 1 argument".to_string());
        }

        let date_str = args[0].as_str().ok_or("Argument must be a string")?;
        let dt = Self::parse_date(date_str)?;

        Ok(Value::String(dt.to_rfc3339()))
    }

    /// addDate - 日期加减
    ///
    /// # Arguments
    ///
    /// * `args[0]` - 日期字符串
    /// * `args[1]` - 年数
    /// * `args[2]` - 月数
    /// * `args[3]` - 天数
    pub fn add_date(&self, args: &[Value]) -> Result<Value, String> {
        if args.len() < 4 {
            return Err("addDate requires 4 arguments".to_string());
        }

        let date_str = args[0].as_str().ok_or("First argument must be a string")?;
        let years = args[1].as_i64().ok_or("Second argument must be an integer")?;
        let months = args[2].as_i64().ok_or("Third argument must be an integer")?;
        let days = args[3].as_i64().ok_or("Fourth argument must be an integer")?;

        let mut dt = Self::parse_date(date_str)?;

        // 先加减年和月
        if years != 0 || months != 0 {
            dt = dt.checked_add_months(chrono::Months::new((years * 12 + months) as u32))
                .ok_or("Date calculation overflow")?;
        }

        // 再加减天
        if days != 0 {
            dt = dt.checked_add_days(chrono::Days::new(days.abs() as u64))
                .ok_or("Date calculation overflow")?;
        }

        Ok(Value::String(dt.to_rfc3339()))
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
