/// 静态文件处理模块
///
/// 该模块负责处理 Jekyll 项目中的静态文件，包括收集、过滤
/// 和复制静态文件到输出目录。支持 exclude 和 include 配置。
use super::{JekyllConfig, JekyllStructure, StaticFileError};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

/// 静态文件处理器
///
/// 负责管理 Jekyll 项目中的静态文件，包括收集、过滤和复制。
pub struct StaticFileProcessor {
    /// Jekyll 目录结构
    structure: JekyllStructure,
    /// Jekyll 配置
    config: JekyllConfig,
}

/// 静态文件信息
///
/// 包含静态文件的源路径和目标路径信息。
#[derive(Debug, Clone)]
pub struct StaticFile {
    /// 源文件路径
    source_path: PathBuf,
    /// 相对源目录的路径
    relative_path: PathBuf,
}

impl StaticFile {
    /// 创建新的 StaticFile 实例
    ///
    /// # Arguments
    ///
    /// * `source_path` - 源文件的绝对路径
    /// * `root_path` - 源目录的绝对路径，用于计算相对路径
    pub fn new<P: AsRef<Path>, Q: AsRef<Path>>(source_path: P, root_path: Q) -> Self {
        let source_path = source_path.as_ref().to_path_buf();
        let root_path = root_path.as_ref();
        let relative_path = source_path.strip_prefix(root_path).unwrap_or_else(|_| source_path.as_path()).to_path_buf();

        Self { source_path, relative_path }
    }

    /// 获取源文件路径
    pub fn source_path(&self) -> &Path {
        &self.source_path
    }

    /// 获取相对路径
    pub fn relative_path(&self) -> &Path {
        &self.relative_path
    }

    /// 计算目标路径
    ///
    /// # Arguments
    ///
    /// * `destination` - 输出目录路径
    ///
    /// # Returns
    ///
    /// 返回在输出目录中的目标路径
    pub fn destination_path<P: AsRef<Path>>(&self, destination: P) -> PathBuf {
        destination.as_ref().join(&self.relative_path)
    }
}

impl StaticFileProcessor {
    /// 创建新的静态文件处理器
    ///
    /// # Arguments
    ///
    /// * `structure` - Jekyll 目录结构
    /// * `config` - Jekyll 配置
    pub fn new(structure: JekyllStructure, config: JekyllConfig) -> Self {
        Self { structure, config }
    }

    /// 获取 Jekyll 目录结构
    pub fn structure(&self) -> &JekyllStructure {
        &self.structure
    }

    /// 获取 Jekyll 配置
    pub fn config(&self) -> &JekyllConfig {
        &self.config
    }

    /// 收集所有静态文件
    ///
    /// 会根据 exclude 和 include 配置进行过滤。
    ///
    /// # Returns
    ///
    /// 返回所有符合条件的静态文件列表
    ///
    /// # Errors
    ///
    /// 返回 `StaticFileError` 如果文件系统操作失败
    pub fn collect_static_files(&self) -> Result<Vec<StaticFile>, StaticFileError> {
        let mut files = Vec::new();
        let root = self.structure.root();

        for entry in WalkDir::new(root) {
            let entry = entry.map_err(|e| StaticFileError::FileSystemError(e.into()))?;
            let path = entry.path();

            if path.is_file() {
                if self.should_include_file(path) {
                    files.push(StaticFile::new(path, root));
                }
            }
        }

        Ok(files)
    }

    /// 复制所有静态文件到目标目录
    ///
    /// # Arguments
    ///
    /// * `destination` - 目标输出目录
    ///
    /// # Returns
    ///
    /// 返回成功复制的文件数量
    ///
    /// # Errors
    ///
    /// 返回 `StaticFileError` 如果文件复制或目录创建失败
    pub fn copy_static_files<P: AsRef<Path>>(&self, destination: P) -> Result<usize, StaticFileError> {
        let destination = destination.as_ref();
        let files = self.collect_static_files()?;
        let mut copied = 0;

        for file in &files {
            self.copy_file(file, destination)?;
            copied += 1;
        }

        Ok(copied)
    }

    /// 复制单个文件到目标目录
    ///
    /// # Arguments
    ///
    /// * `file` - 要复制的静态文件
    /// * `destination` - 目标输出目录
    ///
    /// # Errors
    ///
    /// 返回 `StaticFileError` 如果文件复制或目录创建失败
    fn copy_file<P: AsRef<Path>>(&self, file: &StaticFile, destination: P) -> Result<(), StaticFileError> {
        let dest_path = file.destination_path(destination);
        let dest_dir =
            dest_path.parent().ok_or_else(|| StaticFileError::PathMatchError("Invalid destination path".to_string()))?;

        if !dest_dir.exists() {
            std::fs::create_dir_all(dest_dir).map_err(|e| {
                StaticFileError::DirectoryCreateError(format!("Failed to create directory {}: {}", dest_dir.display(), e))
            })?;
        }

        std::fs::copy(&file.source_path, &dest_path).map_err(|e| {
            StaticFileError::CopyError(format!(
                "Failed to copy {} to {}: {}",
                file.source_path.display(),
                dest_path.display(),
                e
            ))
        })?;

        Ok(())
    }

    /// 检查文件是否应该被包含
    ///
    /// 会根据 exclude 和 include 配置进行判断。
    ///
    /// # Arguments
    ///
    /// * `path` - 文件路径
    ///
    /// # Returns
    ///
    /// 如果文件应该被包含返回 true，否则返回 false
    fn should_include_file(&self, path: &Path) -> bool {
        let relative_path = match path.strip_prefix(self.structure.root()) {
            Ok(p) => p,
            Err(_) => return false,
        };

        let relative_str = relative_path.to_string_lossy();

        if self.is_excluded(&relative_str) {
            if self.is_included(&relative_str) {
                return true;
            }
            return false;
        }

        true
    }

    /// 检查路径是否在排除列表中
    ///
    /// # Arguments
    ///
    /// * `path` - 相对路径字符串
    ///
    /// # Returns
    ///
    /// 如果路径被排除返回 true
    fn is_excluded(&self, path: &str) -> bool {
        if let Some(exclude) = &self.config.exclude {
            for pattern in exclude {
                if self.matches_pattern(path, pattern) {
                    return true;
                }
            }
        }

        Self::default_excluded().iter().any(|p| self.matches_pattern(path, p))
    }

    /// 检查路径是否在包含列表中
    ///
    /// # Arguments
    ///
    /// * `path` - 相对路径字符串
    ///
    /// # Returns
    ///
    /// 如果路径被包含返回 true
    fn is_included(&self, path: &str) -> bool {
        if let Some(include) = &self.config.include {
            for pattern in include {
                if self.matches_pattern(path, pattern) {
                    return true;
                }
            }
        }
        false
    }

    /// 简单的路径模式匹配
    ///
    /// 支持基本的通配符匹配：
    /// - `*` 匹配任意字符（除路径分隔符）
    /// - `**` 匹配任意路径
    ///
    /// # Arguments
    ///
    /// * `path` - 要匹配的路径
    /// * `pattern` - 匹配模式
    ///
    /// # Returns
    ///
    /// 如果路径匹配模式返回 true
    fn matches_pattern(&self, path: &str, pattern: &str) -> bool {
        if pattern == "**" {
            return true;
        }

        let pattern_normalized = pattern.replace('\\', "/");
        let path_normalized = path.replace('\\', "/");

        if pattern_normalized.contains("**") {
            let parts: Vec<&str> = pattern_normalized.split("**").collect();
            if parts.len() == 2 {
                let prefix = parts[0];
                let suffix = parts[1];
                return path_normalized.starts_with(prefix) && path_normalized.ends_with(suffix);
            }
        }

        if pattern_normalized.contains('*') {
            let regex_pattern = pattern_normalized.replace('.', "\\.").replace('*', ".*");
            if let Ok(re) = regex::Regex::new(&format!("^{}$", regex_pattern)) {
                return re.is_match(&path_normalized);
            }
        }

        path_normalized == pattern_normalized || path_normalized.starts_with(&format!("{}/", pattern_normalized))
    }

    /// 获取默认的排除列表
    ///
    /// 这些是 Jekyll 默认会排除的文件和目录。
    fn default_excluded() -> Vec<&'static str> {
        vec![
            "_site",
            ".git",
            ".gitignore",
            ".svn",
            "node_modules",
            "vendor",
            "Gemfile",
            "Gemfile.lock",
            "package.json",
            "package-lock.json",
            "yarn.lock",
            "Cargo.toml",
            "Cargo.lock",
            "target",
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{fs::File, io::Write};
    use tempfile::tempdir;

    #[test]
    fn test_static_file_new() {
        let root = Path::new("/test/root");
        let source = Path::new("/test/root/assets/image.jpg");
        let file = StaticFile::new(source, root);

        assert_eq!(file.source_path(), source);
        assert_eq!(file.relative_path(), Path::new("assets/image.jpg"));
    }

    #[test]
    fn test_static_file_destination_path() {
        let root = Path::new("/test/root");
        let source = Path::new("/test/root/assets/image.jpg");
        let file = StaticFile::new(source, root);
        let dest = Path::new("/test/output");

        assert_eq!(file.destination_path(dest), Path::new("/test/output/assets/image.jpg"));
    }

    #[test]
    fn test_matches_pattern() {
        let config = JekyllConfig::default();
        let structure = JekyllStructure::new(tempdir().unwrap().path()).unwrap();
        let processor = StaticFileProcessor::new(structure, config);

        assert!(processor.matches_pattern("test.txt", "test.txt"));
        assert!(processor.matches_pattern("dir/test.txt", "dir/test.txt"));
        assert!(processor.matches_pattern("any.txt", "*.txt"));
        assert!(processor.matches_pattern("dir/subdir/file.txt", "**/*.txt"));
        assert!(!processor.matches_pattern("test.jpg", "*.txt"));
    }

    #[test]
    fn test_collect_static_files() {
        let temp_dir = tempdir().unwrap();
        let root = temp_dir.path();

        std::fs::create_dir_all(root.join("assets")).unwrap();
        let mut file1 = File::create(root.join("assets/image.jpg")).unwrap();
        writeln!(file1, "test image").unwrap();

        let mut file2 = File::create(root.join("style.css")).unwrap();
        writeln!(file2, "test css").unwrap();

        std::fs::create_dir_all(root.join("_site")).unwrap();
        let mut excluded_file = File::create(root.join("_site/index.html")).unwrap();
        writeln!(excluded_file, "excluded").unwrap();

        let structure = JekyllStructure::new(root).unwrap();
        let config = JekyllConfig::default();
        let processor = StaticFileProcessor::new(structure, config);

        let files = processor.collect_static_files().unwrap();

        let relative_paths: Vec<_> = files.iter().map(|f| f.relative_path().to_string_lossy()).collect();
        assert!(relative_paths.contains(&"assets/image.jpg".into()));
        assert!(relative_paths.contains(&"style.css".into()));
        assert!(!relative_paths.contains(&"_site/index.html".into()));
    }
}
