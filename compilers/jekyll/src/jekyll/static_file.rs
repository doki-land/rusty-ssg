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
}

impl StaticFile {
    /// 从文件路径创建静态文件
    /// 
    /// # Arguments
    /// 
    /// * `path` - 文件路径
    /// * `dest_dir` - 目标目录
    /// 
    /// # Returns
    /// 
    /// 静态文件或错误
    pub fn from_path<P: AsRef<Path>>(path: P, dest_dir: P) -> Result<Self> {
        let path = path.as_ref();
        let dest_dir = dest_dir.as_ref();
        
        let modified_time = path.metadata()
            .map_err(JekyllError::from)?
            .modified()
            .map_err(JekyllError::from)?;
        
        let dest_path = dest_dir.join(path.file_name().unwrap());
        
        Ok(Self {
            path: path.to_path_buf(),
            dest_path,
            modified_time,
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
        self.path.strip_prefix(base)
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
        
        for entry in walkdir::WalkDir::new(dir)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
        {
            let path = entry.path();
            let relative_path = path.strip_prefix(dir).unwrap();
            let file_dest_dir = dest_dir.join(relative_path.parent().unwrap_or_else(|| Path::new("")));
            
            match StaticFile::from_path(path, &file_dest_dir) {
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
