//! 依赖分析模块

use hashbrown::{HashMap, HashSet};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::{
    path::{Path, PathBuf},
    sync::RwLock,
};

/// 依赖关系
#[derive(Debug)]
pub struct Dependency {
    /// 依赖的源文件路径
    pub source: PathBuf,
    /// 依赖的目标文件路径
    pub target: PathBuf,
    /// 依赖类型（import 或 export）
    pub dep_type: DependencyType,
}

/// 依赖类型
#[derive(Debug, PartialEq, Eq)]
pub enum DependencyType {
    /// 导入依赖
    Import,
    /// 导出依赖
    Export,
}

/// 依赖图
#[derive(Debug)]
pub struct DependencyGraph {
    /// 依赖关系映射
    dependencies: RwLock<HashMap<PathBuf, HashSet<PathBuf>>>,
    /// 反向依赖关系映射
    reverse_dependencies: RwLock<HashMap<PathBuf, HashSet<PathBuf>>>,
}

impl DependencyGraph {
    /// 创建新的依赖图
    pub fn new() -> Self {
        Self { dependencies: RwLock::new(HashMap::new()), reverse_dependencies: RwLock::new(HashMap::new()) }
    }

    /// 添加依赖关系
    pub fn add_dependency(&self, source: PathBuf, target: PathBuf) {
        // 添加正向依赖
        let mut dependencies = self.dependencies.write().unwrap();
        dependencies.entry(source.clone()).or_insert_with(HashSet::new).insert(target.clone());

        // 添加反向依赖
        let mut reverse_dependencies = self.reverse_dependencies.write().unwrap();
        reverse_dependencies.entry(target).or_insert_with(HashSet::new).insert(source);
    }

    /// 获取文件的所有依赖
    pub fn get_dependencies(&self, file: &Path) -> Option<HashSet<PathBuf>> {
        let dependencies = self.dependencies.read().unwrap();
        dependencies.get(file).cloned()
    }

    /// 获取文件的所有反向依赖
    pub fn get_reverse_dependencies(&self, file: &Path) -> Option<HashSet<PathBuf>> {
        let reverse_dependencies = self.reverse_dependencies.read().unwrap();
        reverse_dependencies.get(file).cloned()
    }

    /// 检查是否存在依赖关系
    pub fn has_dependency(&self, source: &Path, target: &Path) -> bool {
        let dependencies = self.dependencies.read().unwrap();
        if let Some(deps) = dependencies.get(source) { deps.contains(target) } else { false }
    }

    /// 获取所有文件路径
    pub fn all_files(&self) -> HashSet<PathBuf> {
        let dependencies = self.dependencies.read().unwrap();
        let mut files = HashSet::new();
        for (source, targets) in &*dependencies {
            files.insert(source.clone());
            for target in targets {
                files.insert(target.clone());
            }
        }
        files
    }
}

/// 依赖分析器
pub struct DependencyAnalyzer {
    /// 依赖图
    graph: DependencyGraph,
}

impl DependencyAnalyzer {
    /// 创建新的依赖分析器
    pub fn new() -> Self {
        Self { graph: DependencyGraph::new() }
    }

    /// 分析文件依赖
    pub fn analyze_file(&self, file_path: &Path, content: &str) -> Result<(), String> {
        // 解析 import 语句
        self.parse_imports(file_path, content);

        // 解析 export 语句
        self.parse_exports(file_path, content);

        Ok(())
    }

    /// 解析 import 语句
    fn parse_imports(&self, file_path: &Path, content: &str) {
        // 简单的 import 语句解析
        // 实际应该使用更复杂的解析器
        for line in content.lines() {
            let line = line.trim();
            if line.starts_with("import") && line.contains("from") {
                if let Some(path_str) = self.extract_import_path(line) {
                    if let Some(target_path) = self.resolve_path(file_path, path_str) {
                        self.graph.add_dependency(file_path.to_path_buf(), target_path);
                    }
                }
            }
        }
    }

    /// 解析 export 语句
    fn parse_exports(&self, file_path: &Path, content: &str) {
        // 简单的 export 语句解析
        for line in content.lines() {
            let line = line.trim();
            if line.starts_with("export") && line.contains("from") {
                if let Some(path_str) = self.extract_export_path(line) {
                    if let Some(target_path) = self.resolve_path(file_path, path_str) {
                        self.graph.add_dependency(file_path.to_path_buf(), target_path);
                    }
                }
            }
        }
    }

    /// 从 import 语句中提取路径
    fn extract_import_path<'a>(&self, line: &'a str) -> Option<&'a str> {
        // 简单实现，实际应该使用正则表达式或解析器
        if let Some(from_idx) = line.find("from") {
            let path_part = &line[from_idx + 4..].trim();
            if path_part.starts_with('"') && path_part.ends_with('"') {
                Some(&path_part[1..path_part.len() - 1])
            }
            else if path_part.starts_with("'") && path_part.ends_with("'") {
                Some(&path_part[1..path_part.len() - 1])
            }
            else {
                None
            }
        }
        else {
            None
        }
    }

    /// 从 export 语句中提取路径
    fn extract_export_path<'a>(&self, line: &'a str) -> Option<&'a str> {
        // 简单实现，实际应该使用正则表达式或解析器
        if let Some(from_idx) = line.find("from") {
            let path_part = &line[from_idx + 4..].trim();
            if path_part.starts_with('"') && path_part.ends_with('"') {
                Some(&path_part[1..path_part.len() - 1])
            }
            else if path_part.starts_with("'") && path_part.ends_with("'") {
                Some(&path_part[1..path_part.len() - 1])
            }
            else {
                None
            }
        }
        else {
            None
        }
    }

    /// 解析路径
    fn resolve_path(&self, base_path: &Path, path_str: &str) -> Option<PathBuf> {
        // 处理相对路径
        if path_str.starts_with('.') {
            let mut resolved_path = base_path.parent()?.to_path_buf();
            resolved_path.push(path_str);

            // 添加扩展名
            if !resolved_path.extension().is_some() {
                resolved_path.set_extension("js");
            }

            Some(resolved_path)
        }
        else {
            // 处理绝对路径和模块路径
            // 这里简化处理，实际应该根据模块解析规则来处理
            None
        }
    }

    /// 批量分析文件依赖
    pub fn analyze_files(&self, files: &[(PathBuf, String)]) {
        files.par_iter().for_each(|(path, content)| {
            if let Err(err) = self.analyze_file(path, content) {
                eprintln!("Error analyzing dependencies for file {}: {}", path.display(), err);
            }
        });
    }

    /// 获取依赖图
    pub fn graph(&self) -> &DependencyGraph {
        &self.graph
    }
}

impl Default for DependencyAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}
