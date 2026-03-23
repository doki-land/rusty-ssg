//! 内容处理函数
//! 提供 Hugo 兼容的内容处理函数

use serde_json::Value;
use oak_markdown::Parser;

/// 内容处理函数集合
pub struct ContentFunctions;

impl ContentFunctions {
    /// 创建新的内容处理函数集合
    pub fn new() -> Self {
        Self
    }

    /// markdownify - 将 Markdown 转换为 HTML
    pub fn markdownify(&self, args: &[Value]) -> Result<Value, String> {
        if args.is_empty() {
            return Err("markdownify requires at least 1 argument".to_string());
        }

        let input = args[0].as_str().ok_or("Argument must be a string")?;
        
        let parser = Parser::default();
        let html = parser.parse(input);

        Ok(Value::String(html))
    }

    /// plainify - 移除 HTML 标签，返回纯文本
    pub fn plainify(&self, args: &[Value]) -> Result<Value, String> {
        if args.is_empty() {
            return Err("plainify requires at least 1 argument".to_string());
        }

        let input = args[0].as_str().ok_or("Argument must be a string")?;
        
        // 简单的 HTML 标签移除
        let plain = input
            .replace('<', "<")
            .replace('>', ">")
            .replace(|c: char| c.is_control(), "")
            .trim()
            .to_string();

        Ok(Value::String(plain))
    }

    /// highlight - 代码高亮
    ///
    /// # Arguments
    ///
    /// * `args[0]` - 代码内容
    /// * `args[1]` - 语言（可选）
    /// * `args[2]` - 选项（可选）
    pub fn highlight(&self, args: &[Value]) -> Result<Value, String> {
        if args.is_empty() {
            return Err("highlight requires at least 1 argument".to_string());
        }

        let code = args[0].as_str().ok_or("First argument must be a string")?;
        let language = if args.len() > 1 {
            args[1].as_str().unwrap_or("")
        } else {
            ""
        };

        // 简单的代码高亮实现
        let html = if language.is_empty() {
            format!("<pre><code>{}</code></pre>", code)
        } else {
            format!("<pre><code class=\"language-{}\">{}</code></pre>", language, code)
        };

        Ok(Value::String(html))
    }

    /// emojify - 处理 Emoji
    ///
    /// # Arguments
    ///
    /// * `args[0]` - 包含 Emoji 代码的字符串
    pub fn emojify(&self, args: &[Value]) -> Result<Value, String> {
        if args.is_empty() {
            return Err("emojify requires at least 1 argument".to_string());
        }

        let input = args[0].as_str().ok_or("Argument must be a string")?;
        
        // 简单的 Emoji 处理
        let result = input
            .replace(":smile:", "😊")
            .replace(":laugh:", "😂")
            .replace(":heart:", "❤️")
            .replace(":thumbsup:", "👍")
            .replace(":rocket:", "🚀")
            .replace(":star:", "⭐")
            .replace(":check:", "✅")
            .replace(":x:", "❌");

        Ok(Value::String(result))
    }
}

impl Default for ContentFunctions {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_markdownify() {
        let funcs = ContentFunctions::new();
        let result = funcs.markdownify(&[json!("# Hello")]).unwrap();
        assert!(result.is_string());
    }

    #[test]
    fn test_plainify() {
        let funcs = ContentFunctions::new();
        assert_eq!(
            funcs.plainify(&[json!("<p>Hello <b>World</b></p>")]).unwrap(),
            json!("Hello World")
        );
    }

    #[test]
    fn test_highlight() {
        let funcs = ContentFunctions::new();
        let result = funcs.highlight(&[json!("let x = 1;")]).unwrap();
        assert!(result.is_string());
    }

    #[test]
    fn test_emojify() {
        let funcs = ContentFunctions::new();
        assert_eq!(
            funcs.emojify(&[json!("Hello :smile:")]).unwrap(),
            json!("Hello 😊")
        );
    }
}
