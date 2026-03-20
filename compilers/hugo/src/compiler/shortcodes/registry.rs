//! 短代码注册表
//!
//! 用于管理和注册短代码处理器。

use crate::compiler::shortcodes::types::{Shortcode, ShortcodeContext, ShortcodeError, ShortcodeHandler, ShortcodeResult};
use std::collections::HashMap;

/// 短代码注册表
pub struct ShortcodeRegistry {
    /// 已注册的短代码处理器
    handlers: HashMap<String, Box<dyn ShortcodeHandler>>,
}

impl ShortcodeRegistry {
    /// 创建新的短代码注册表
    pub fn new() -> Self {
        Self { handlers: HashMap::new() }
    }

    /// 注册短代码处理器
    ///
    /// # Arguments
    ///
    /// * `name` - 短代码名称
    /// * `handler` - 短代码处理器
    pub fn register<H>(&mut self, name: &str, handler: H)
    where
        H: ShortcodeHandler + 'static,
    {
        self.handlers.insert(name.to_string(), Box::new(handler));
    }

    /// 检查短代码是否已注册
    ///
    /// # Arguments
    ///
    /// * `name` - 短代码名称
    pub fn has(&self, name: &str) -> bool {
        self.handlers.contains_key(name)
    }

    /// 执行短代码
    ///
    /// # Arguments
    ///
    /// * `shortcode` - 短代码定义
    /// * `context` - 短代码上下文
    ///
    /// # Returns
    ///
    /// 短代码执行结果
    pub fn execute(&self, shortcode: &Shortcode, context: &ShortcodeContext) -> ShortcodeResult<String> {
        if let Some(handler) = self.handlers.get(&shortcode.name) {
            handler.handle(shortcode, context)
        }
        else {
            Err(ShortcodeError::ShortcodeNotFound { name: shortcode.name.clone() })
        }
    }

    /// 获取所有已注册的短代码名称
    pub fn registered_names(&self) -> Vec<String> {
        self.handlers.keys().cloned().collect()
    }
}

impl Default for ShortcodeRegistry {
    fn default() -> Self {
        let mut registry = Self::new();
        crate::compiler::shortcodes::builtin::register_builtins(&mut registry);
        registry
    }
}
