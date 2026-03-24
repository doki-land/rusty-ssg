//! 链接验证模块
//! 提供链接验证功能，确保站点中的链接都能正常工作

use crate::types::{MkDocsConfig, ValidationConfig, ValidationLevel};
use regex;
use std::{
    collections::HashSet,
    path::{Path, PathBuf},
    result::Result,
};

/// 链接验证错误
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LinkError {
    /// 链接未找到
    NotFound(String),
    /// 锚点未找到
    AnchorNotFound(String, String),
    /// 绝对链接错误
    AbsoluteLinkError(String),
    /// 未识别的链接
    UnrecognizedLink(String),
}

/// 链接验证结果
#[derive(Debug, Clone)]
pub struct ValidationResult {
    /// 错误列表
    pub errors: Vec<LinkError>,
    /// 警告列表
    pub warnings: Vec<LinkError>,
    /// 信息列表
    pub info: Vec<LinkError>,
}

impl ValidationResult {
    /// 创建新的验证结果
    pub fn new() -> Self {
        Self { errors: Vec::new(), warnings: Vec::new(), info: Vec::new() }
    }

    /// 添加错误
    pub fn add_error(&mut self, error: LinkError) {
        self.errors.push(error);
    }

    /// 添加警告
    pub fn add_warning(&mut self, warning: LinkError) {
        self.warnings.push(warning);
    }

    /// 添加信息
    pub fn add_info(&mut self, info: LinkError) {
        self.info.push(info);
    }

    /// 检查是否有错误
    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }

    /// 检查是否有警告
    pub fn has_warnings(&self) -> bool {
        !self.warnings.is_empty()
    }

    /// 检查是否有信息
    pub fn has_info(&self) -> bool {
        !self.info.is_empty()
    }

    /// 打印验证结果
    pub fn print(&self) {
        if self.has_errors() {
            println!("\nErrors:");
            for error in &self.errors {
                println!("  - {:?}", error);
            }
        }

        if self.has_warnings() {
            println!("\nWarnings:");
            for warning in &self.warnings {
                println!("  - {:?}", warning);
            }
        }

        if self.has_info() {
            println!("\nInfo:");
            for info in &self.info {
                println!("  - {:?}", info);
            }
        }

        if !self.has_errors() && !self.has_warnings() && !self.has_info() {
            println!("\nAll links are valid!");
        }
    }
}

/// 链接验证器
pub struct LinkValidator {
    /// 配置
    config: MkDocsConfig,
    /// 已发现的文件
    files: HashSet<String>,
    /// 验证结果
    result: ValidationResult,
}

impl LinkValidator {
    /// 创建新的链接验证器
    pub fn new(config: MkDocsConfig) -> Self {
        Self { config, files: HashSet::new(), result: ValidationResult::new() }
    }

    /// 添加文件
    pub fn add_file(&mut self, path: &str) {
        self.files.insert(path.to_string());
    }

    /// 验证导航链接
    pub fn validate_nav(&mut self) {
        let nav_items = self.config.nav().to_vec();
        let not_found_level = self.config.validation().nav.not_found.clone();

        for item in nav_items {
            self.validate_nav_item(&item, &not_found_level);
        }
    }

    /// 验证导航项
    fn validate_nav_item(&mut self, item: &crate::types::NavItem, not_found_level: &crate::types::ValidationLevel) {
        match item {
            crate::types::NavItem::String(_) => {
                // 字符串类型的导航项不需要验证
            }
            crate::types::NavItem::Map(map) => {
                for (_, value) in map {
                    match value {
                        crate::types::NavValue::String(path) => {
                            self.validate_link(path, not_found_level);
                        }
                        crate::types::NavValue::List(items) => {
                            for sub_item in items {
                                self.validate_nav_item(sub_item, not_found_level);
                            }
                        }
                    }
                }
            }
        }
    }

    /// 验证链接
    pub fn validate_link(&mut self, link: &str, level: &ValidationLevel) {
        if link.starts_with("http://") || link.starts_with("https://") {
            // 绝对链接，根据配置处理
            match level {
                ValidationLevel::Ignore => {}
                ValidationLevel::Info => {
                    self.result.add_info(LinkError::AbsoluteLinkError(link.to_string()));
                }
                ValidationLevel::Warn => {
                    self.result.add_warning(LinkError::AbsoluteLinkError(link.to_string()));
                }
            }
        }
        else if link.starts_with("#") {
            // 锚点链接，由 validate_anchors 处理
        }
        else if link.ends_with(".md") || link.ends_with(".html") {
            // 本地文件链接
            let md_path = link.replace(".html", ".md");
            if !self.files.contains(&md_path) {
                match level {
                    ValidationLevel::Ignore => {}
                    ValidationLevel::Info => {
                        self.result.add_info(LinkError::NotFound(link.to_string()));
                    }
                    ValidationLevel::Warn => {
                        self.result.add_warning(LinkError::NotFound(link.to_string()));
                    }
                }
            }
        }
        else if link.ends_with("/") {
            // 目录 URL（如 page/）
            let md_path = format!("{}{}", link.trim_end_matches("/"), ".md");
            if !self.files.contains(&md_path) {
                match level {
                    ValidationLevel::Ignore => {}
                    ValidationLevel::Info => {
                        self.result.add_info(LinkError::NotFound(link.to_string()));
                    }
                    ValidationLevel::Warn => {
                        self.result.add_warning(LinkError::NotFound(link.to_string()));
                    }
                }
            }
        }
        else if !link.contains(".") {
            // 可能是目录 URL 或没有扩展名的文件
            let md_path = format!("{}{}", link, ".md");
            if !self.files.contains(&md_path) {
                match level {
                    ValidationLevel::Ignore => {}
                    ValidationLevel::Info => {
                        self.result.add_info(LinkError::NotFound(link.to_string()));
                    }
                    ValidationLevel::Warn => {
                        self.result.add_warning(LinkError::NotFound(link.to_string()));
                    }
                }
            }
        }
        else {
            // 其他链接类型（如图片、CSS、JS 等）
            // 暂时只添加信息级别的提示
            self.result.add_info(LinkError::UnrecognizedLink(link.to_string()));
        }
    }

    /// 验证所有链接
    pub fn validate_all(&mut self, documents: &std::collections::HashMap<String, String>) {
        // 验证导航链接
        self.validate_nav();

        // 验证文档中的链接
        let not_found_level = self.config.validation().links.not_found.clone();
        for (path, content) in documents {
            self.validate_document_links(path, content, &not_found_level);
        }
    }

    /// 验证文档中的链接
    fn validate_document_links(&mut self, path: &str, content: &str, not_found_level: &crate::types::ValidationLevel) {
        // 提取 Markdown 中的链接和图片
        let link_regex = regex::Regex::new(r#"\[([^\]]+)\]\(([^\)]+)\)"#).unwrap();
        let image_regex = regex::Regex::new(r#"!\[([^\]]*)\]\(([^\)]+)\)"#).unwrap();

        // 验证普通链接
        for capture in link_regex.captures_iter(content) {
            if let Some(link) = capture.get(2) {
                let link_str = link.as_str().trim();
                self.validate_link(link_str, not_found_level);
            }
        }

        // 验证图片链接
        for capture in image_regex.captures_iter(content) {
            if let Some(link) = capture.get(2) {
                let link_str = link.as_str().trim();
                self.validate_link(link_str, not_found_level);
            }
        }

        // 验证锚点链接
        self.validate_anchors(path, content);
    }

    /// 验证锚点链接
    fn validate_anchors(&mut self, _path: &str, content: &str) {
        // 提取文档中的锚点（标题）
        let header_regex = regex::Regex::new(r#"^#{1,6}\s+(.+)$"#).unwrap();
        let mut anchors = HashSet::new();

        for line in content.lines() {
            if let Some(capture) = header_regex.captures(line) {
                if let Some(header) = capture.get(1) {
                    let anchor = self.generate_anchor(header.as_str());
                    anchors.insert(anchor);
                }
            }
        }

        // 提取文档中的锚点链接
        let anchor_regex = regex::Regex::new(r#"\[([^\]]+)\]\((#[^\)]+)\)"#).unwrap();
        for capture in anchor_regex.captures_iter(content) {
            if let Some(anchor) = capture.get(2) {
                let anchor_str = anchor.as_str();
                // 移除 # 前缀
                let anchor_name = &anchor_str[1..];
                if !anchors.contains(anchor_name) {
                    self.result.add_warning(LinkError::AnchorNotFound(anchor_name.to_string(), anchor_str.to_string()));
                }
            }
        }
    }

    /// 生成锚点名称（根据 Markdown 标题生成）
    fn generate_anchor(&self, header: &str) -> String {
        // 转换为小写
        let mut anchor = header.to_lowercase();
        // 移除特殊字符
        anchor = anchor.chars().filter(|c| c.is_alphanumeric() || *c == ' ').collect();
        // 空格替换为连字符
        anchor = anchor.replace(' ', "-");
        anchor
    }

    /// 获取验证结果
    pub fn result(&self) -> &ValidationResult {
        &self.result
    }

    /// 获取可变的验证结果
    pub fn result_mut(&mut self) -> &mut ValidationResult {
        &mut self.result
    }
}
