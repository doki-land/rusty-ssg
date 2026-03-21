//! 框架特定的组件解析器

use std::path::Path;
use crate::compiler::component::{Component, Framework};

/// 框架组件解析器
pub trait FrameworkParser {
    /// 解析组件文件
    /// 
    /// # 参数
    /// - `file_path`: 组件文件路径
    /// - `content`: 组件文件内容
    /// 
    /// # 返回值
    /// 解析后的组件
    fn parse_component(&self, file_path: &Path, content: &str) -> Result<Component, String>;
    
    /// 检测文件是否为该框架的组件
    /// 
    /// # 参数
    /// - `file_path`: 文件路径
    /// 
    /// # 返回值
    /// 是否为该框架的组件
    fn is_framework_component(&self, file_path: &Path) -> bool;
    
    /// 获取框架类型
    fn framework(&self) -> Framework;
}

/// React 组件解析器
pub struct ReactParser;

impl FrameworkParser for ReactParser {
    fn parse_component(&self, file_path: &Path, content: &str) -> Result<Component, String> {
        // 从文件路径中提取组件名称
        let component_name = file_path.file_stem()
            .and_then(|os_str| os_str.to_str())
            .unwrap_or("UnknownComponent");
        
        // 创建 React 组件
        Ok(Component::new_react(component_name, content))
    }
    
    fn is_framework_component(&self, file_path: &Path) -> bool {
        // 检查文件扩展名是否为 .jsx 或 .tsx
        let ext = file_path.extension().and_then(|os_str| os_str.to_str());
        matches!(ext, Some("jsx") | Some("tsx"))
    }
    
    fn framework(&self) -> Framework {
        Framework::React
    }
}

/// Vue 组件解析器
pub struct VueParser;

impl FrameworkParser for VueParser {
    fn parse_component(&self, file_path: &Path, content: &str) -> Result<Component, String> {
        // 从文件路径中提取组件名称
        let component_name = file_path.file_stem()
            .and_then(|os_str| os_str.to_str())
            .unwrap_or("UnknownComponent");
        
        // 创建 Vue 组件
        Ok(Component::new_vue(component_name, content))
    }
    
    fn is_framework_component(&self, file_path: &Path) -> bool {
        // 检查文件扩展名是否为 .vue
        let ext = file_path.extension().and_then(|os_str| os_str.to_str());
        matches!(ext, Some("vue"))
    }
    
    fn framework(&self) -> Framework {
        Framework::Vue
    }
}

/// Svelte 组件解析器
pub struct SvelteParser;

impl FrameworkParser for SvelteParser {
    fn parse_component(&self, file_path: &Path, content: &str) -> Result<Component, String> {
        // 从文件路径中提取组件名称
        let component_name = file_path.file_stem()
            .and_then(|os_str| os_str.to_str())
            .unwrap_or("UnknownComponent");
        
        // 创建 Svelte 组件
        Ok(Component::new_svelte(component_name, content))
    }
    
    fn is_framework_component(&self, file_path: &Path) -> bool {
        // 检查文件扩展名是否为 .svelte
        let ext = file_path.extension().and_then(|os_str| os_str.to_str());
        matches!(ext, Some("svelte"))
    }
    
    fn framework(&self) -> Framework {
        Framework::Svelte
    }
}

/// Solid 组件解析器
pub struct SolidParser;

impl FrameworkParser for SolidParser {
    fn parse_component(&self, file_path: &Path, content: &str) -> Result<Component, String> {
        // 从文件路径中提取组件名称
        let component_name = file_path.file_stem()
            .and_then(|os_str| os_str.to_str())
            .unwrap_or("UnknownComponent");
        
        // 创建 Solid 组件
        Ok(Component::new_solid(component_name, content))
    }
    
    fn is_framework_component(&self, file_path: &Path) -> bool {
        // 检查文件扩展名是否为 .jsx 或 .tsx
        let ext = file_path.extension().and_then(|os_str| os_str.to_str());
        matches!(ext, Some("jsx") | Some("tsx"))
    }
    
    fn framework(&self) -> Framework {
        Framework::Solid
    }
}

/// Preact 组件解析器
pub struct PreactParser;

impl FrameworkParser for PreactParser {
    fn parse_component(&self, file_path: &Path, content: &str) -> Result<Component, String> {
        // 从文件路径中提取组件名称
        let component_name = file_path.file_stem()
            .and_then(|os_str| os_str.to_str())
            .unwrap_or("UnknownComponent");
        
        // 创建 Preact 组件
        Ok(Component::new_preact(component_name, content))
    }
    
    fn is_framework_component(&self, file_path: &Path) -> bool {
        // 检查文件扩展名是否为 .jsx 或 .tsx
        let ext = file_path.extension().and_then(|os_str| os_str.to_str());
        matches!(ext, Some("jsx") | Some("tsx"))
    }
    
    fn framework(&self) -> Framework {
        Framework::Preact
    }
}

/// Lit 组件解析器
pub struct LitParser;

impl FrameworkParser for LitParser {
    fn parse_component(&self, file_path: &Path, content: &str) -> Result<Component, String> {
        // 从文件路径中提取组件名称
        let component_name = file_path.file_stem()
            .and_then(|os_str| os_str.to_str())
            .unwrap_or("UnknownComponent");
        
        // 创建 Lit 组件
        Ok(Component::new_lit(component_name, content))
    }
    
    fn is_framework_component(&self, file_path: &Path) -> bool {
        // 检查文件扩展名是否为 .js 或 .ts
        let ext = file_path.extension().and_then(|os_str| os_str.to_str());
        matches!(ext, Some("js") | Some("ts"))
    }
    
    fn framework(&self) -> Framework {
        Framework::Lit
    }
}

/// 框架解析器管理器
pub struct FrameworkParserManager {
    parsers: Vec<Box<dyn FrameworkParser>>,
}

impl FrameworkParserManager {
    /// 创建新的框架解析器管理器
    pub fn new() -> Self {
        let mut parsers: Vec<Box<dyn FrameworkParser>> = Vec::new();
        
        // 添加各种框架的解析器
        parsers.push(Box::new(ReactParser));
        parsers.push(Box::new(VueParser));
        parsers.push(Box::new(SvelteParser));
        parsers.push(Box::new(SolidParser));
        parsers.push(Box::new(PreactParser));
        parsers.push(Box::new(LitParser));
        
        Self {
            parsers,
        }
    }
    
    /// 解析组件文件
    /// 
    /// # 参数
    /// - `file_path`: 组件文件路径
    /// - `content`: 组件文件内容
    /// 
    /// # 返回值
    /// 解析后的组件
    pub fn parse_component(&self, file_path: &Path, content: &str) -> Result<Component, String> {
        // 尝试使用每个解析器解析组件
        for parser in &self.parsers {
            if parser.is_framework_component(file_path) {
                return parser.parse_component(file_path, content);
            }
        }
        
        // 如果没有找到匹配的解析器，返回错误
        Err(format!("No framework parser found for file: {:?}", file_path))
    }
    
    /// 获取文件对应的框架类型
    /// 
    /// # 参数
    /// - `file_path`: 文件路径
    /// 
    /// # 返回值
    /// 框架类型，如果没有找到匹配的框架，返回 None
    pub fn get_framework(&self, file_path: &Path) -> Option<Framework> {
        for parser in &self.parsers {
            if parser.is_framework_component(file_path) {
                return Some(parser.framework());
            }
        }
        None
    }
}

impl Default for FrameworkParserManager {
    fn default() -> Self {
        Self::new()
    }
}