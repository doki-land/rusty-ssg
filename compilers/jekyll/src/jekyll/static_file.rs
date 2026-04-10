#![warn(missing_docs)]

//! 静态文件处理模块
//!
//! 提供静态文件的复制和处理功能

use crate::errors::{JekyllError, Result, StaticFileError};
use std::path::{Path, PathBuf};

/// 静态文件
#[derive(Debug, Clone)]
pub struct StaticFile {
    /// 源文件路径
    pub path: PathBuf,
    /// 目标文件路径
    pub dest_path: PathBuf,
    /// 文件修改时间
    pub modified_time: std::time::SystemTime,
    /// 文件相对路径
    pub relative_path: PathBuf,
    /// 文件类型
    pub file_type: String,
}

impl StaticFile {
    /// 从文件路径创建静态文件
    ///
    /// # Arguments
    ///
    /// * `path` - 文件路径
    /// * `dest_dir` - 目标目录
    /// * `base_dir` - 基础目录（用于计算相对路径）
    ///
    /// # Returns
    ///
    /// 静态文件或错误
    pub fn from_path<P: AsRef<Path>>(path: P, dest_dir: P, base_dir: P) -> Result<Self> {
        let path = path.as_ref();
        let dest_dir = dest_dir.as_ref();
        let base_dir = base_dir.as_ref();

        let modified_time = path.metadata().map_err(JekyllError::from)?.modified().map_err(JekyllError::from)?;

        // 计算相对路径
        let relative_path = path.strip_prefix(base_dir)
            .map_err(|_| JekyllError::DirectoryNotFound(base_dir.display().to_string()))?
            .to_path_buf();

        // 构建目标路径
        let dest_path = dest_dir.join(&relative_path);

        // 确定文件类型
        let file_type = path.extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("")
            .to_string();

        Ok(Self { 
            path: path.to_path_buf(), 
            dest_path, 
            modified_time,
            relative_path,
            file_type,
        })
    }

    /// 从文件路径创建静态文件（无基础目录）
    ///
    /// # Arguments
    ///
    /// * `path` - 文件路径
    /// * `dest_dir` - 目标目录
    ///
    /// # Returns
    ///
    /// 静态文件或错误
    pub fn from_path_simple<P: AsRef<Path>>(path: P, dest_dir: P) -> Result<Self> {
        let path = path.as_ref();
        let dest_dir = dest_dir.as_ref();

        let modified_time = path.metadata().map_err(JekyllError::from)?.modified().map_err(JekyllError::from)?;

        let dest_path = dest_dir.join(path.file_name().unwrap());
        let relative_path = PathBuf::from(path.file_name().unwrap_or_default());
        let file_type = path.extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("")
            .to_string();

        Ok(Self { 
            path: path.to_path_buf(), 
            dest_path, 
            modified_time,
            relative_path,
            file_type,
        })
    }

    /// 复制文件到目标位置
    ///
    /// # Returns
    ///
    /// 成功或错误
    pub fn copy(&self) -> Result<()> {
        if let Some(parent) = self.dest_path.parent() {
            if !parent.exists() {
                std::fs::create_dir_all(parent).map_err(JekyllError::from)?;
            }
        }

        std::fs::copy(&self.path, &self.dest_path).map_err(JekyllError::from)?;
        Ok(())
    }

    /// 获取相对路径
    ///
    /// # Arguments
    ///
    /// * `base` - 基础目录
    ///
    /// # Returns
    ///
    /// 相对路径
    pub fn relative_path<P: AsRef<Path>>(&self, base: P) -> Result<PathBuf> {
        self.path
            .strip_prefix(&base)
            .map(|p| p.to_path_buf())
            .map_err(|_| JekyllError::DirectoryNotFound(base.as_ref().display().to_string()).into())
    }
}

/// 静态文件管理器
#[derive(Debug)]
pub struct StaticFileManager {
    /// 静态文件列表
    files: Vec<StaticFile>,
}

impl StaticFileManager {
    /// 创建新的静态文件管理器
    pub fn new() -> Self {
        Self { files: Vec::new() }
    }

    /// 添加静态文件
    ///
    /// # Arguments
    ///
    /// * `file` - 静态文件
    pub fn add_file(&mut self, file: StaticFile) {
        self.files.push(file);
    }

    /// 从目录添加静态文件
    ///
    /// # Arguments
    ///
    /// * `dir` - 源目录
    /// * `dest_dir` - 目标目录
    ///
    /// # Returns
    ///
    /// 添加的文件数量或错误
    pub fn add_files_from_dir<P: AsRef<Path>>(&mut self, dir: P, dest_dir: P) -> Result<usize> {
        let dir = dir.as_ref();
        let dest_dir = dest_dir.as_ref();

        if !dir.exists() {
            return Ok(0);
        }

        let mut count = 0;

        for entry in walkdir::WalkDir::new(dir).into_iter().filter_map(|e| e.ok()).filter(|e| e.file_type().is_file()) {
            let path = entry.path();

            match StaticFile::from_path(path, dest_dir, dir) {
                Ok(file) => {
                    self.add_file(file);
                    count += 1;
                }
                Err(e) => {
                    eprintln!("Warning: Failed to add static file {}: {}", path.display(), e);
                }
            }
        }

        Ok(count)
    }

    /// 从多个目录添加静态文件
    ///
    /// # Arguments
    ///
    /// * `dirs` - 源目录列表
    /// * `dest_dir` - 目标目录
    ///
    /// # Returns
    ///
    /// 添加的文件数量或错误
    pub fn add_files_from_dirs<P: AsRef<Path>>(&mut self, dirs: &[P], dest_dir: P) -> Result<usize> {
        let dest_dir = dest_dir.as_ref();
        let mut count = 0;

        for dir in dirs {
            let dir = dir.as_ref();
            count += self.add_files_from_dir(dir, dest_dir)?;
        }

        Ok(count)
    }

    /// 按文件类型过滤静态文件
    ///
    /// # Arguments
    ///
    /// * `file_type` - 文件类型
    ///
    /// # Returns
    ///
    /// 过滤后的静态文件列表
    pub fn filter_by_file_type(&self, file_type: &str) -> Vec<&StaticFile> {
        self.files
            .iter()
            .filter(|file| file.file_type == file_type)
            .collect()
    }

    /// 按文件类型分组静态文件
    ///
    /// # Returns
    ///
    /// 按文件类型分组的静态文件
    pub fn group_by_file_type(&self) -> std::collections::HashMap<String, Vec<&StaticFile>> {
        let mut groups = std::collections::HashMap::new();

        for file in &self.files {
            groups.entry(file.file_type.clone())
                .or_insert(Vec::new())
                .push(file);
        }

        groups
    }

    /// 复制所有静态文件
    ///
    /// # Returns
    ///
    /// 复制的文件数量或错误
    pub fn copy_all(&self) -> Result<usize> {
        let mut count = 0;

        for file in &self.files {
            if file.copy().is_ok() {
                count += 1;
            }
        }

        Ok(count)
    }

    /// 获取所有静态文件
    pub fn files(&self) -> &[StaticFile] {
        &self.files
    }

    /// 清除所有静态文件
    pub fn clear(&mut self) {
        self.files.clear();
    }

    /// 获取文件数量
    pub fn count(&self) -> usize {
        self.files.len()
    }
}

impl Default for StaticFileManager {
    fn default() -> Self {
        Self::new()
    }
}
