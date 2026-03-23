//! 配置模块
//! 
//! 负责加载和解析 MkDocs 配置文件

use crate::types::MkDocsConfig;
use std::path::PathBuf;

/// 加载 MkDocs 配置
///
/// # Arguments
///
/// * `path` - 配置文件路径
///
/// # Returns
///
/// 加载的配置对象
pub fn load_config(path: &PathBuf) -> crate::types::Result<MkDocsConfig> {
    MkDocsConfig::load_from_file(path)
}

/// 从目录加载 MkDocs 配置
///
/// # Arguments
///
/// * `dir` - 目录路径
///
/// # Returns
///
/// 加载的配置对象
pub fn load_config_from_dir(dir: &PathBuf) -> crate::types::Result<MkDocsConfig> {
    MkDocsConfig::load_from_dir(dir)
}

/// 创建默认配置
///
/// # Returns
///
/// 默认配置对象
pub fn create_default_config() -> MkDocsConfig {
    MkDocsConfig::new()
}
