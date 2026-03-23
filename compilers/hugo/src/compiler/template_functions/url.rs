//! URL 处理函数
//! 提供 Hugo 兼容的 URL 处理函数

use regex::Regex;
use serde_json::Value;

/// URL 处理函数集合
#[derive(Clone)]
pub struct UrlFunctions {
    /// 基础 URL
    base_url: Option<String>,
    /// 当前语言
    current_lang: Option<String>,
}

impl UrlFunctions {
    /// 创建新的 URL 函数集合
    pub fn new() -> Self {
        Self { base_url: None, current_lang: None }
    }

    /// 设置基础 URL
    pub fn with_base_url(mut self, base_url: String) -> Self {
        self.base_url = Some(base_url);
        self
    }

    /// 设置当前语言
    pub fn with_current_lang(mut self, lang: String) -> Self {
        self.current_lang = Some(lang);
        self
    }

    /// relURL - 生成相对 URL
    ///
    /// # Arguments
    ///
    /// * `args[0]` - URL 路径
    ///
    /// # Returns
    ///
    /// 相对 URL 字符串
    pub fn rel_url(&self, args: &[Value]) -> Result<Value, String> {
        if args.is_empty() {
            return Err("relURL requires at least 1 argument".to_string());
        }

        let path = args[0].as_str().ok_or("Argument must be a string")?;

        let url = if path.starts_with('/') { path.to_string() } else { format!("/{}", path) };

        Ok(Value::String(url))
    }

    /// absURL - 生成绝对 URL
    ///
    /// # Arguments
    ///
    /// * `args[0]` - URL 路径
    ///
    /// # Returns
    ///
    /// 绝对 URL 字符串
    pub fn abs_url(&self, args: &[Value]) -> Result<Value, String> {
        if args.is_empty() {
            return Err("absURL requires at least 1 argument".to_string());
        }

        let path = args[0].as_str().ok_or("Argument must be a string")?;

        let base = self.base_url.as_deref().unwrap_or("https://example.com");

        let url = if path.starts_with('/') {
            format!("{}{}", base.trim_end_matches('/'), path)
        }
        else {
            format!("{}/{}", base.trim_end_matches('/'), path)
        };

        Ok(Value::String(url))
    }

    /// urlize - 将字符串转换为 URL 友好格式
    ///
    /// # Arguments
    ///
    /// * `args[0]` - 要转换的字符串
    ///
    /// # Returns
    ///
    /// URL 友好的字符串
    pub fn urlize(&self, args: &[Value]) -> Result<Value, String> {
        if args.is_empty() {
            return Err("urlize requires at least 1 argument".to_string());
        }

        let input = args[0].as_str().ok_or("Argument must be a string")?;

        let slug = Self::create_slug(input);

        Ok(Value::String(slug))
    }

    /// absLangURL - 生成带语言前缀的绝对 URL
    ///
    /// # Arguments
    ///
    /// * `args[0]` - URL 路径
    ///
    /// # Returns
    ///
    /// 带语言前缀的绝对 URL 字符串
    pub fn abs_lang_url(&self, args: &[Value]) -> Result<Value, String> {
        if args.is_empty() {
            return Err("absLangURL requires at least 1 argument".to_string());
        }

        let path = args[0].as_str().ok_or("Argument must be a string")?;

        let base = self.base_url.as_deref().unwrap_or("https://example.com");
        let lang = self.current_lang.as_deref().unwrap_or("");

        let url = if lang.is_empty() {
            if path.starts_with('/') {
                format!("{}{}", base.trim_end_matches('/'), path)
            }
            else {
                format!("{}/{}", base.trim_end_matches('/'), path)
            }
        }
        else {
            if path.starts_with('/') {
                format!("{}/{}{}", base.trim_end_matches('/'), lang, path)
            }
            else {
                format!("{}/{}/{}  ", base.trim_end_matches('/'), lang, path)
            }
        };

        Ok(Value::String(url.trim().to_string()))
    }

    /// relLangURL - 生成带语言前缀的相对 URL
    ///
    /// # Arguments
    ///
    /// * `args[0]` - URL 路径
    ///
    /// # Returns
    ///
    /// 带语言前缀的相对 URL 字符串
    pub fn rel_lang_url(&self, args: &[Value]) -> Result<Value, String> {
        if args.is_empty() {
            return Err("relLangURL requires at least 1 argument".to_string());
        }

        let path = args[0].as_str().ok_or("Argument must be a string")?;

        let lang = self.current_lang.as_deref().unwrap_or("");

        let url = if lang.is_empty() {
            if path.starts_with('/') { path.to_string() } else { format!("/{}", path) }
        }
        else {
            if path.starts_with('/') { format!("/{}{}", lang, path) } else { format!("/{}/{}", lang, path) }
        };

        Ok(Value::String(url))
    }

    /// 创建 URL slug
    fn create_slug(input: &str) -> String {
        let re = Regex::new(r"[^a-zA-Z0-9\s-]").unwrap();

        let slug = input.to_lowercase().trim().replace(' ', "-");

        let slug = re.replace_all(&slug, "");

        let re_multi_dash = Regex::new(r"-+").unwrap();
        let slug = re_multi_dash.replace_all(&slug, "-");

        slug.trim_matches('-').to_string()
    }
}

impl Default for UrlFunctions {
    fn default() -> Self {
        Self::new()
    }
}
