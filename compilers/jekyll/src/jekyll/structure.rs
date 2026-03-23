#![warn(missing_docs)]

//! Jekyll 目录结构模块
//!
//! 提供 Jekyll 项目目录结构的识别和管理功能

use std::path::{Path, PathBuf};

use crate::errors::{JekyllError, Result};

/// Jekyll 标准目录类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum JekyllDirectory {
    /// 帖子目录 _posts
    Posts,
    /// 布局目录 _layouts
    Layouts,
    /// 包含文件目录 _includes
    Includes,
    /// 数据目录 _data
    Data,
    /// 草稿目录 _drafts
    Drafts,
    /// Sass 目录 _sass
    Sass,
    /// 输出目录 _site
    Site,
    /// 资源目录 assets
    Assets,
}

impl JekyllDirectory {
    /// 获取目录名称
    pub fn name(&self) -> &'static str {
        match self {
            JekyllDirectory::Posts => "_posts",
            JekyllDirectory::Layouts => "_layouts",
            JekyllDirectory::Includes => "_includes",
            JekyllDirectory::Data => "_data",
            JekyllDirectory::Drafts => "_drafts",
            JekyllDirectory::Sass => "_sass",
            JekyllDirectory::Site => "_site",
            JekyllDirectory::Assets => "assets",
        }
    }

    /// 从目录名称创建
    pub fn from_name(name: &str) -> Option<Self> {
        match name {
            "_posts" => Some(JekyllDirectory::Posts),
            "_layouts" => Some(JekyllDirectory::Layouts),
            "_includes" => Some(JekyllDirectory::Includes),
            "_data" => Some(JekyllDirectory::Data),
            "_drafts" => Some(JekyllDirectory::Drafts),
            "_sass" => Some(JekyllDirectory::Sass),
            "_site" => Some(JekyllDirectory::Site),
            "assets" => Some(JekyllDirectory::Assets),
            _ => None,
        }
    }

    /// 检查是否为下划线开头的特殊目录
    pub fn is_special(&self) -> bool {
        matches!(
            self,
            JekyllDirectory::Posts
                | JekyllDirectory::Layouts
                | JekyllDirectory::Includes
                | JekyllDirectory::Data
                | JekyllDirectory::Drafts
                | JekyllDirectory::Sass
                | JekyllDirectory::Site
        )
    }
}

/// Jekyll 项目目录结构
#[derive(Debug, Clone)]
pub struct JekyllStructure {
    /// 项目根目录
    root: PathBuf,
    /// 已发现的目录
    directories: std::collections::HashMap<JekyllDirectory, PathBuf>,
    /// 集合目录（_collections 或自定义集合）
    collections: Vec<String>,
}

impl JekyllStructure {
    /// 创建新的 Jekyll 目录结构
    ///
    /// # Arguments
    ///
    /// * `root` - 项目根目录路径
    ///
    /// # Returns
    ///
    /// 返回目录结构实例或错误
    pub fn new<P: AsRef<Path>>(root: P) -> Result<Self> {
        let root = root.as_ref().to_path_buf();

        if !root.exists() {
            return Err(JekyllError::DirectoryNotFound(root.display().to_string()).into());
        }

        let mut structure = Self { root, directories: std::collections::HashMap::new(), collections: Vec::new() };

        structure.discover_directories()?;

        Ok(structure)
    }

    /// 发现所有 Jekyll 目录
    fn discover_directories(&mut self) -> Result<()> {
        let entries = std::fs::read_dir(&self.root).map_err(JekyllError::from)?;

        for entry in entries {
            let entry = entry.map_err(JekyllError::from)?;
            let path = entry.path();

            if path.is_dir() {
                if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                    if let Some(dir_type) = JekyllDirectory::from_name(name) {
                        self.directories.insert(dir_type, path);
                    }
                    else if name.starts_with('_') && name != "_site" {
                        self.collections.push(name[1..].to_string());
                    }
                }
            }
        }

        Ok(())
    }

    /// 检查是否存在指定类型的目录
    ///
    /// # Arguments
    ///
    /// * `dir_type` - 目录类型
    ///
    /// # Returns
    ///
    /// 如果目录存在返回 true
    pub fn has_directory(&self, dir_type: JekyllDirectory) -> bool {
        self.directories.contains_key(&dir_type)
    }

    /// 获取指定类型目录的路径
    ///
    /// # Arguments
    ///
    /// * `dir_type` - 目录类型
    ///
    /// # Returns
    ///
    /// 返回目录路径，如果不存在返回 None
    pub fn get_directory(&self, dir_type: JekyllDirectory) -> Option<&PathBuf> {
        self.directories.get(&dir_type)
    }

    /// 获取项目根目录
    pub fn root(&self) -> &Path {
        &self.root
    }

    /// 获取帖子目录路径
    pub fn posts_dir(&self) -> Option<&PathBuf> {
        self.get_directory(JekyllDirectory::Posts)
    }

    /// 获取布局目录路径
    pub fn layouts_dir(&self) -> Option<&PathBuf> {
        self.get_directory(JekyllDirectory::Layouts)
    }

    /// 获取包含文件目录路径
    pub fn includes_dir(&self) -> Option<&PathBuf> {
        self.get_directory(JekyllDirectory::Includes)
    }

    /// 获取数据目录路径
    pub fn data_dir(&self) -> Option<&PathBuf> {
        self.get_directory(JekyllDirectory::Data)
    }

    /// 获取草稿目录路径
    pub fn drafts_dir(&self) -> Option<&PathBuf> {
        self.get_directory(JekyllDirectory::Drafts)
    }

    /// 获取 Sass 目录路径
    pub fn sass_dir(&self) -> Option<&PathBuf> {
        self.get_directory(JekyllDirectory::Sass)
    }

    /// 获取输出目录路径
    pub fn site_dir(&self) -> PathBuf {
        self.get_directory(JekyllDirectory::Site).cloned().unwrap_or_else(|| self.root.join("_site"))
    }

    /// 获取资源目录路径
    pub fn assets_dir(&self) -> Option<&PathBuf> {
        self.get_directory(JekyllDirectory::Assets)
    }

    /// 获取所有集合名称
    pub fn collections(&self) -> &[String] {
        &self.collections
    }

    /// 获取集合目录路径
    ///
    /// # Arguments
    ///
    /// * `name` - 集合名称
    ///
    /// # Returns
    ///
    /// 返回集合目录路径
    pub fn collection_dir(&self, name: &str) -> PathBuf {
        self.root.join(format!("_{}", name))
    }

    /// 获取所有已发现的目录
    pub fn all_directories(&self) -> &std::collections::HashMap<JekyllDirectory, PathBuf> {
        &self.directories
    }

    /// 创建目录结构（用于初始化新项目）
    ///
    /// # Arguments
    ///
    /// * `create_posts` - 是否创建 _posts 目录
    /// * `create_layouts` - 是否创建 _layouts 目录
    /// * `create_includes` - 是否创建 _includes 目录
    /// * `create_data` - 是否创建 _data 目录
    pub fn create_structure(
        &self,
        create_posts: bool,
        create_layouts: bool,
        create_includes: bool,
        create_data: bool,
    ) -> Result<()> {
        if create_posts {
            std::fs::create_dir_all(self.root.join("_posts")).map_err(JekyllError::from)?;
        }
        if create_layouts {
            std::fs::create_dir_all(self.root.join("_layouts")).map_err(JekyllError::from)?;
        }
        if create_includes {
            std::fs::create_dir_all(self.root.join("_includes")).map_err(JekyllError::from)?;
        }
        if create_data {
            std::fs::create_dir_all(self.root.join("_data")).map_err(JekyllError::from)?;
        }

        Ok(())
    }
}
