//! 页面处理函数
//! 提供 Hugo 兼容的页面处理函数

use serde_json::Value;

/// 页面处理函数集合
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
        let permalink = format!("/{}