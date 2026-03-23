//! 框架解析器模块

use crate::compiler::component::Framework;
use std::path::Path;

/// 框架解析器 trait
pub trait FrameworkParser {
    /// 检查给定路径是否为框架组件
    fn is_framework_component(&self, path: &Path) -> bool;
    
    /// 获取框架类型
    fn framework(&self) -> Framework;
}

/// 框架解析器管理器
pub struct FrameworkParserManager {
    /// 框架解析器列表
    parsers: Vec<Box<dyn FrameworkParser>>,
}

impl FrameworkParserManager {
    /// 创建新的框架解析器管理器
    pub fn new() -> Self {
        let mut parsers: Vec<Box<dyn FrameworkParser>> = Vec::new();
        parsers.push(Box::new(ReactParser) as Box<dyn FrameworkParser>);
        parsers.push(Box::new(VueParser) as Box<dyn FrameworkParser>);
        parsers.push(Box::new(SvelteParser) as Box<dyn FrameworkParser>);
        Self { parsers }
    }
    
    /// 根据文件路径获取框架类型
    pub fn get_framework(&self, path: &Path) -> Option<Framework> {
        for parser in &self.parsers {
            if parser.is_framework_component(path) {
                return Some(parser.framework());
            }
        }
        None
    }
}

/// React 解析器
pub struct ReactParser;

impl FrameworkParser for ReactParser {
    fn is_framework_component(&self, path: &Path) -> bool {
        if let Some(ext) = path.extension() {
            matches!(ext.to_str(), Some("jsx") | Some("tsx"))
        } else {
            false
        }
    }
    
    fn framework(&self) -> Framework {
        Framework::React
    }
}

/// Vue 解析器
pub struct VueParser;

impl FrameworkParser for VueParser {
    fn is_framework_component(&self, path: &Path) -> bool {
        if let Some(ext) = path.extension() {
            ext == "vue"
        } else {
            false
        }
    }
    
    fn framework(&self) -> Framework {
        Framework::Vue
    }
}

/// Svelte 解析器
pub struct SvelteParser;

impl FrameworkParser for SvelteParser {
    fn is_framework_component(&self, path: &Path) -> bool {
        if let Some(ext) = path.extension() {
            ext == "svelte"
        } else {
            false
        }
    }
    
    fn framework(&self) -> Framework {
        Framework::Svelte
    }
}
