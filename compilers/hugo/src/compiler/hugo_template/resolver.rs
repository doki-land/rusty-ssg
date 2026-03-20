//! Hugo 模板解析器模块
//! 实现 Hugo 的模板查找规则，支持 layouts/ 目录和主题继承

use std::{
    error::Error,
    fmt,
    path::{Path, PathBuf},
};

/// 模板解析器错误类型
#[derive(Debug)]
pub enum TemplateResolverError {
    /// 模板文件未找到
    TemplateNotFound {
        /// 模板名称
        name: String,
    },

    /// 文件系统错误
    IoError {
        /// 底层 IO 错误
        source: std::io::Error,
    },
}

impl TemplateResolverError {
    /// 获取错误的 i18n 键
    pub fn i18n_key(&self) -> &'static str {
        match self {
            TemplateResolverError::TemplateNotFound { .. } => "hugo.error.template.not_found",
            TemplateResolverError::IoError { .. } => "hugo.error.template.io",
        }
    }

    /// 获取错误的参数
    pub fn params(&self) -> Vec<(String, String)> {
        match self {
            TemplateResolverError::TemplateNotFound { name } => vec![("name".to_string(), name.clone())],
            TemplateResolverError::IoError { source } => vec![("message".to_string(), source.to_string())],
        }
    }
}

impl fmt::Display for TemplateResolverError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TemplateResolverError::TemplateNotFound { name } => write!(f, "Template not found: {}", name),
            TemplateResolverError::IoError { source } => write!(f, "File system error: {}", source),
        }
    }
}

impl Error for TemplateResolverError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            TemplateResolverError::IoError { source } => Some(source),
            _ => None,
        }
    }
}

impl From<std::io::Error> for TemplateResolverError {
    fn from(source: std::io::Error) -> Self {
        TemplateResolverError::IoError { source }
    }
}

/// Hugo 模板解析器
///
/// 负责根据 Hugo 的模板查找规则定位和加载模板文件
pub struct TemplateResolver {
    /// 项目根目录
    root_dir: PathBuf,
    /// 当前主题名称
    theme: Option<String>,
}

impl TemplateResolver {
    /// 创建新的模板解析器
    ///
    /// # Arguments
    ///
    /// * `root_dir` - 项目根目录
    pub fn new(root_dir: impl AsRef<Path>) -> Self {
        Self { root_dir: root_dir.as_ref().to_path_buf(), theme: None }
    }

    /// 设置主题
    ///
    /// # Arguments
    ///
    /// * `theme` - 主题名称
    pub fn set_theme(&mut self, theme: String) {
        self.theme = Some(theme);
    }

    /// 清除主题设置
    pub fn clear_theme(&mut self) {
        self.theme = None;
    }

    /// 获取根目录
    pub fn root_dir(&self) -> &PathBuf {
        &self.root_dir
    }

    /// 获取当前主题
    pub fn theme(&self) -> Option<&String> {
        self.theme.as_ref()
    }

    /// 解析模板，返回模板名称和内容
    ///
    /// 按照 Hugo 的查找顺序查找模板：
    /// 1. 项目根目录的 layouts/ 文件夹
    /// 2. 主题目录的 layouts/ 文件夹（如果设置了主题）
    ///
    /// # Arguments
    ///
    /// * `template_name` - 模板名称（如 "baseof.html" 或 "partials/header.html"）
    ///
    /// # Returns
    ///
    /// 返回 (模板名称, 模板内容) 元组
    pub fn resolve_template(&self, template_name: &str) -> Result<(String, String), TemplateResolverError> {
        let possible_paths = self.get_possible_paths(template_name);

        for path in possible_paths {
            if path.exists() {
                let content = std::fs::read_to_string(&path)?;
                return Ok((template_name.to_string(), content));
            }
        }

        Err(TemplateResolverError::TemplateNotFound { name: template_name.to_string() })
    }

    /// 获取可能的模板文件路径列表
    ///
    /// 按照优先级排序：项目 layouts/ 目录优先，然后是主题 layouts/ 目录
    ///
    /// # Arguments
    ///
    /// * `template_name` - 模板名称
    fn get_possible_paths(&self, template_name: &str) -> Vec<PathBuf> {
        let mut paths = Vec::new();

        // 1. 项目根目录的 layouts/ 文件夹
        let project_layouts = self.root_dir.join("layouts").join(template_name);
        paths.push(project_layouts);

        // 2. 主题目录的 layouts/ 文件夹（如果设置了主题）
        if let Some(theme) = &self.theme {
            let theme_layouts = self.root_dir.join("themes").join(theme).join("layouts").join(template_name);
            paths.push(theme_layouts);
        }

        paths
    }

    /// 检查模板是否存在
    ///
    /// # Arguments
    ///
    /// * `template_name` - 模板名称
    pub fn template_exists(&self, template_name: &str) -> bool {
        self.get_possible_paths(template_name).iter().any(|path| path.exists())
    }

    /// 列出所有可用的模板
    ///
    /// 扫描项目和主题的 layouts/ 目录，返回所有找到的模板名称
    pub fn list_templates(&self) -> Vec<String> {
        let mut templates = Vec::new();

        // 扫描项目 layouts/ 目录
        let project_layouts = self.root_dir.join("layouts");
        if project_layouts.exists() {
            self.scan_directory(&project_layouts, &project_layouts, &mut templates);
        }

        // 扫描主题 layouts/ 目录
        if let Some(theme) = &self.theme {
            let theme_layouts = self.root_dir.join("themes").join(theme).join("layouts");
            if theme_layouts.exists() {
                self.scan_directory(&theme_layouts, &theme_layouts, &mut templates);
            }
        }

        // 去重（项目模板优先）
        let mut seen = std::collections::HashSet::new();
        templates.retain(|t| seen.insert(t.clone()));

        templates
    }

    /// 递归扫描目录查找模板文件
    ///
    /// # Arguments
    ///
    /// * `base_dir` - 基础目录（用于计算相对路径）
    /// * `current_dir` - 当前扫描目录
    /// * `templates` - 收集到的模板列表
    fn scan_directory(&self, base_dir: &Path, current_dir: &Path, templates: &mut Vec<String>) {
        if let Ok(entries) = std::fs::read_dir(current_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    self.scan_directory(base_dir, &path, templates);
                }
                else if let Some(ext) = path.extension() {
                    if ext == "html" || ext == "tmpl" {
                        if let Ok(rel_path) = path.strip_prefix(base_dir) {
                            if let Some(template_name) = rel_path.to_str() {
                                templates.push(template_name.replace('\\', "/"));
                            }
                        }
                    }
                }
            }
        }
    }
}
