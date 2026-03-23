//! 缓存模块

use hashbrown::HashMap;
use std::{
    path::{Path, PathBuf},
    sync::RwLock,
    time::SystemTime,
};

/// 文件缓存项
#[derive(Debug, Clone)]
pub struct FileCacheItem {
    /// 文件内容
    pub content: String,
    /// 文件修改时间
    pub modified_time: SystemTime,
}

/// 渲染缓存项
#[derive(Debug, Clone)]
pub struct RenderCacheItem {
    /// 渲染结果
    pub result: String,
    /// 依赖文件的修改时间
    pub dependencies: HashMap<PathBuf, SystemTime>,
}

/// 缓存管理器
#[derive(Debug)]
pub struct CacheManager {
    /// 文件内容缓存
    file_cache: RwLock<HashMap<PathBuf, FileCacheItem>>,
    /// 渲染结果缓存
    render_cache: RwLock<HashMap<String, RenderCacheItem>>,
}

impl CacheManager {
    /// 创建新的缓存管理器
    pub fn new() -> Self {
        Self { file_cache: RwLock::new(HashMap::new()), render_cache: RwLock::new(HashMap::new()) }
    }

    /// 获取文件内容缓存
    pub fn get_file(&self, path: &Path) -> Option<FileCacheItem> {
        let file_cache = self.file_cache.read().unwrap();
        file_cache.get(path).cloned()
    }

    /// 设置文件内容缓存
    pub fn set_file(&self, path: &Path, content: String) {
        let modified_time = path.metadata().ok().and_then(|m| m.modified().ok()).unwrap_or(SystemTime::now());
        let mut file_cache = self.file_cache.write().unwrap();
        file_cache.insert(path.to_path_buf(), FileCacheItem { content, modified_time });
    }

    /// 检查文件是否需要更新
    pub fn needs_update(&self, path: &Path) -> bool {
        if let Some(cache_item) = self.get_file(path) {
            if let Ok(metadata) = path.metadata() {
                if let Ok(modified_time) = metadata.modified() {
                    return modified_time > cache_item.modified_time;
                }
            }
            true
        }
        else {
            true
        }
    }

    /// 获取渲染结果缓存
    pub fn get_render(&self, key: &str) -> Option<RenderCacheItem> {
        let render_cache = self.render_cache.read().unwrap();
        render_cache.get(key).cloned()
    }

    /// 设置渲染结果缓存
    pub fn set_render(&self, key: &str, result: String, dependencies: HashMap<PathBuf, SystemTime>) {
        let mut render_cache = self.render_cache.write().unwrap();
        render_cache.insert(key.to_string(), RenderCacheItem { result, dependencies });
    }

    /// 检查渲染结果是否需要更新
    pub fn render_needs_update(&self, key: &str) -> bool {
        if let Some(cache_item) = self.get_render(key) {
            for (path, cached_time) in &cache_item.dependencies {
                if let Ok(metadata) = path.metadata() {
                    if let Ok(modified_time) = metadata.modified() {
                        if modified_time > *cached_time {
                            return true;
                        }
                    }
                    else {
                        return true;
                    }
                }
                else {
                    return true;
                }
            }
            false
        }
        else {
            true
        }
    }

    /// 清除所有缓存
    pub fn clear(&self) {
        let mut file_cache = self.file_cache.write().unwrap();
        file_cache.clear();
        let mut render_cache = self.render_cache.write().unwrap();
        render_cache.clear();
    }

    /// 清除指定文件的缓存
    pub fn clear_file(&self, path: &Path) {
        let mut file_cache = self.file_cache.write().unwrap();
        file_cache.remove(path);
    }

    /// 清除指定渲染结果的缓存
    pub fn clear_render(&self, key: &str) {
        let mut render_cache = self.render_cache.write().unwrap();
        render_cache.remove(key);
    }
}

impl Default for CacheManager {
    fn default() -> Self {
        Self::new()
    }
}
