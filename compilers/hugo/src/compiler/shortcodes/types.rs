//! 短代码系统类型定义

use std::{collections::HashMap, error::Error, fmt};

/// 短代码错误类型
#[derive(Debug)]
pub enum ShortcodeError {
    /// 解析错误
    ParseError {
        /// 错误消息
        message: String,
    },
    /// 未找到短代码
    ShortcodeNotFound {
        /// 短代码名称
        name: String,
    },
    /// 参数错误
    ParameterError {
        /// 错误消息
        message: String,
    },
    /// 渲染错误
    RenderError {
        /// 错误消息
        message: String,
    },
}

impl ShortcodeError {
    /// 获取错误的 i18n 键
    pub fn i18n_key(&self) -> &'static str {
        match self {
            ShortcodeError::ParseError { .. } => "hugo.error.shortcode.parse",
            ShortcodeError::ShortcodeNotFound { .. } => "hugo.error.shortcode.not_found",
            ShortcodeError::ParameterError { .. } => "hugo.error.shortcode.parameter",
            ShortcodeError::RenderError { .. } => "hugo.error.shortcode.render",
        }
    }

    /// 获取错误的参数
    pub fn params(&self) -> Vec<(String, String)> {
        match self {
            ShortcodeError::ParseError { message } => vec![("message".to_string(), message.clone())],
            ShortcodeError::ShortcodeNotFound { name } => vec![("name".to_string(), name.clone())],
            ShortcodeError::ParameterError { message } => vec![("message".to_string(), message.clone())],
            ShortcodeError::RenderError { message } => vec![("message".to_string(), message.clone())],
        }
    }
}

impl fmt::Display for ShortcodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ShortcodeError::ParseError { message } => write!(f, "Parse error: {}", message),
            ShortcodeError::ShortcodeNotFound { name } => write!(f, "Shortcode not found: {}", name),
            ShortcodeError::ParameterError { message } => write!(f, "Parameter error: {}", message),
            ShortcodeError::RenderError { message } => write!(f, "Render error: {}", message),
        }
    }
}

impl Error for ShortcodeError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

/// 短代码结果类型
pub type ShortcodeResult<T> = Result<T, ShortcodeError>;

/// 短代码调用类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShortcodeType {
    /// 原始 HTML 输出，不经过 Markdown 处理（{{< ... >}}）
    Raw,
    /// Markdown 输出，经过 Markdown 处理（{{% ... %}}）
    Markdown,
}

/// 短代码参数
#[derive(Debug, Clone)]
pub struct ShortcodeParams {
    /// 位置参数
    pub positional: Vec<String>,
    /// 命名参数
    pub named: HashMap<String, String>,
}

impl ShortcodeParams {
    /// 创建新的短代码参数
    pub fn new() -> Self {
        Self { positional: Vec::new(), named: HashMap::new() }
    }

    /// 获取位置参数
    pub fn get_positional(&self, index: usize) -> Option<&str> {
        self.positional.get(index).map(|s| s.as_str())
    }

    /// 获取命名参数
    pub fn get_named(&self, name: &str) -> Option<&str> {
        self.named.get(name).map(|s| s.as_str())
    }

    /// 获取参数，优先尝试命名参数，如果没有则尝试位置参数
    pub fn get(&self, name: &str, pos: usize) -> Option<&str> {
        self.get_named(name).or_else(|| self.get_positional(pos))
    }

    /// 添加位置参数
    pub fn add_positional(&mut self, value: String) {
        self.positional.push(value);
    }

    /// 添加命名参数
    pub fn add_named(&mut self, name: String, value: String) {
        self.named.insert(name, value);
    }
}

impl Default for ShortcodeParams {
    fn default() -> Self {
        Self::new()
    }
}

/// 短代码定义
#[derive(Debug, Clone)]
pub struct Shortcode {
    /// 短代码名称
    pub name: String,
    /// 短代码类型
    pub shortcode_type: ShortcodeType,
    /// 短代码参数
    pub params: ShortcodeParams,
    /// 短代码内部内容（自闭合短代码可能没有）
    pub inner: Option<String>,
}

/// 短代码上下文
#[derive(Debug, Clone)]
pub struct ShortcodeContext {
    /// 当前文档路径
    pub document_path: String,
    /// 额外的上下文信息
    pub extra: HashMap<String, String>,
}

impl ShortcodeContext {
    /// 创建新的短代码上下文
    pub fn new(document_path: String) -> Self {
        Self { document_path, extra: HashMap::new() }
    }

    /// 添加额外的上下文信息
    pub fn add_extra(&mut self, key: String, value: String) {
        self.extra.insert(key, value);
    }

    /// 获取额外的上下文信息
    pub fn get_extra(&self, key: &str) -> Option<&str> {
        self.extra.get(key).map(|s| s.as_str())
    }
}

/// 短代码处理器 trait
pub trait ShortcodeHandler: Send + Sync {
    /// 处理短代码
    fn handle(&self, shortcode: &Shortcode, context: &ShortcodeContext) -> ShortcodeResult<String>;
}

impl<F> ShortcodeHandler for F
where
    F: Fn(&Shortcode, &ShortcodeContext) -> ShortcodeResult<String> + Send + Sync,
{
    fn handle(&self, shortcode: &Shortcode, context: &ShortcodeContext) -> ShortcodeResult<String> {
        self(shortcode, context)
    }
}
