//! 页面处理函数
//! 提供 Hugo 兼容的页面处理函数

use serde_json::Value;

/// 页面处理函数集合
#[derive(Clone)]
pub struct PageFunctions;

impl PageFunctions {
    /// 创建新的页面处理函数集合
    pub fn new() -> Self {
        Self
    }

    /// ref - 获取页面的永久链接
    ///
    /// # Arguments
    ///
    /// * `args[0]` - 页面路径或标识符
    pub fn ref_(&self, args: &[Value]) -> Result<Value, String> {
        if args.is_empty() {
            return Err("ref requires at least 1 argument".to_string());
        }

        let path = args[0].as_str().ok_or("Argument must be a string")?;

        // 简单实现：返回基于路径的永久链接
        let permalink = format!("/{}", path.trim_start_matches('/'));
        Ok(Value::String(permalink))
    }

    /// relref - 获取页面的相对链接
    ///
    /// # Arguments
    ///
    /// * `args[0]` - 页面路径或标识符
    pub fn relref(&self, args: &[Value]) -> Result<Value, String> {
        if args.is_empty() {
            return Err("relref requires at least 1 argument".to_string());
        }

        let path = args[0].as_str().ok_or("Argument must be a string")?;

        // 简单实现：返回基于路径的相对链接
        let rel_link = format!("/{}", path.trim_start_matches('/'));
        Ok(Value::String(rel_link))
    }

    /// getPage - 获取页面对象
    ///
    /// # Arguments
    ///
    /// * `args[0]` - 页面路径或标识符
    pub fn get_page(&self, args: &[Value]) -> Result<Value, String> {
        if args.is_empty() {
            return Err("getPage requires at least 1 argument".to_string());
        }

        let path = args[0].as_str().ok_or("Argument must be a string")?;

        // 简单实现：返回一个模拟的页面对象
        let page = serde_json::json!({
            "Title": path.split('/').last().unwrap_or(path),
            "Permalink": format!("/{}", path.trim_start_matches('/')),
            "RelPermalink": format!("/{}", path.trim_start_matches('/')),
            "Content": "",
            "Summary": "",
            "Date": "2024-01-01T00:00:00Z"
        });
        Ok(page)
    }

    /// pages - 获取所有页面
    pub fn pages(&self, _args: &[Value]) -> Result<Value, String> {
        // 简单实现：返回一个空数组
        Ok(Value::Array(Vec::new()))
    }

    /// site - 获取站点信息
    pub fn site(&self, _args: &[Value]) -> Result<Value, String> {
        // 简单实现：返回一个模拟的站点对象
        let site = serde_json::json!({
            "Title": "My Site",
            "BaseURL": "https://example.com",
            "Language": "en",
            "Pages": [],
            "Params": {}
        });
        Ok(site)
    }
}

impl Default for PageFunctions {
    fn default() -> Self {
        Self::new()
    }
}
