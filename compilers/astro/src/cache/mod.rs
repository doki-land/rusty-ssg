//! 缓存模块

use hashbrown::HashMap;
use serde::{Deserialize, Serialize};
use std::{
    fs,
    path::{Path, PathBuf},
    sync::RwLock,
    time::{SystemTime, Duration},
};

/// 文件缓存项
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileCacheItem {
    /// 文件内容
    pub content: String,
    /// 文件修改时间
    pub modified_time: SystemTime,
    /// 缓存创建时间
    pub created_time: SystemTime,
}

/// 渲染缓存项
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RenderCacheItem {
    /// 渲染结果
    pub result: String,
    /// 依赖文件的修改时间
    pub dependencies: HashMap<PathBuf, SystemTime>,
    /// 缓存创建时间
    pub created_time: SystemTime,
}

/// 缓存配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheConfig {
    /// 缓存目录
    pub cache_dir: PathBuf,
    /// 最大缓存大小（字节）
    pub max_size: u64,
    /// 缓存过期时间
    pub expiration: Duration,
    /// 是否启用磁盘持久化
    pub enable_persistence: bool,
}

/// 缓存管理器
#[derive(Debug)]
pub struct CacheManager {
    /// 文件内容缓存
    file_cache: RwLock<HashMap<PathBuf, FileCacheItem>>,
    /// 渲染结果缓存
    render_cache: RwLock<HashMap<String, RenderCacheItem>>,
    /// 缓存配置
    config: CacheConfig,
}

impl CacheManager {
    /// 创建新的缓存管理器
    pub fn new() -> Self {
        let config = CacheConfig {
            cache_dir: Path::new(".astro").join("cache"),
            max_size: 1024 * 1024 * 100, // 100MB
            expiration: Duration::from_secs(24 * 60 * 60), // 24 hours
            enable_persistence: true,
        };
        
        let mut manager = Self {
            file_cache: RwLock::new(HashMap::new()),
            render_cache: RwLock::new(HashMap::new()),
            config,
        };
        
        // 加载缓存
        manager.load_cache();
        
        manager
    }

    /// 创建带有自定义配置的缓存管理器
    pub fn with_config(config: CacheConfig) -> Self {
        let mut manager = Self {
            file_cache: RwLock::new(HashMap::new()),
            render_cache: RwLock::new(HashMap::new()),
            config,
        };
        
        // 加载缓存
        manager.load_cache();
        
        manager
    }

    /// 获取文件内容缓存
    pub fn get_file(&self, path: &Path) -> Option<FileCacheItem> {
        let file_cache = self.file_cache.read().unwrap();
        file_cache.get(path).cloned()
    }

    /// 设置文件内容缓存
    pub fn set_file(&self, path: &Path, content: String) {
        let modified_time = path.metadata().ok().and_then(|m| m.modified().ok()).unwrap_or(SystemTime::now());
        let created_time = SystemTime::now();
        let mut file_cache = self.file_cache.write().unwrap();
        file_cache.insert(path.to_path_buf(), FileCacheItem { content, modified_time, created_time });
        
        // 检查缓存大小
        self.check_cache_size();
        
        // 保存缓存
        self.save_cache();
    }

    /// 检查文件是否需要更新
    pub fn needs_update(&self, path: &Path) -> bool {
        if let Some(cache_item) = self.get_file(path) {
            // 检查缓存是否过期
            if self.is_expired(&cache_item.created_time) {
                return true;
            }
            
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
        let created_time = SystemTime::now();
        let mut render_cache = self.render_cache.write().unwrap();
        render_cache.insert(key.to_string(), RenderCacheItem { result, dependencies, created_time });
        
        // 检查缓存大小
        self.check_cache_size();
        
        // 保存缓存
        self.save_cache();
    }

    /// 检查渲染结果是否需要更新
    pub fn render_needs_update(&self, key: &str) -> bool {
        if let Some(cache_item) = self.get_render(key) {
            // 检查缓存是否过期
            if self.is_expired(&cache_item.created_time) {
                return true;
            }
            
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
        
        // 保存缓存
        self.save_cache();
    }

    /// 清除指定文件的缓存
    pub fn clear_file(&self, path: &Path) {
        let mut file_cache = self.file_cache.write().unwrap();
        file_cache.remove(path);
        
        // 保存缓存
        self.save_cache();
    }

    /// 清除指定渲染结果的缓存
    pub fn clear_render(&self, key: &str) {
        let mut render_cache = self.render_cache.write().unwrap();
        render_cache.remove(key);
        
        // 保存缓存
        self.save_cache();
    }

    /// 检查缓存是否过期
    fn is_expired(&self, created_time: &SystemTime) -> bool {
        if let Ok(elapsed) = created_time.elapsed() {
            elapsed > self.config.expiration
        } else {
            true
        }
    }

    /// 检查缓存大小
    fn check_cache_size(&self) {
        let file_cache = self.file_cache.read().unwrap();
        let render_cache = self.render_cache.read().unwrap();
        
        let mut total_size = 0;
        
        // 计算文件缓存大小
        for item in file_cache.values() {
            total_size += item.content.len() as u64;
        }
        
        // 计算渲染缓存大小
        for item in render_cache.values() {
            total_size += item.result.len() as u64;
        }
        
        // 如果超过最大缓存大小，清理过期缓存
        if total_size > self.config.max_size {
            self.cleanup_expired();
        }
    }

    /// 清理过期缓存
    fn cleanup_expired(&self) {
        let mut file_cache = self.file_cache.write().unwrap();
        let mut render_cache = self.render_cache.write().unwrap();
        
        // 清理过期的文件缓存
        file_cache.retain(|_, item| !self.is_expired(&item.created_time));
        
        // 清理过期的渲染缓存
        render_cache.retain(|_, item| !self.is_expired(&item.created_time));
        
        // 保存缓存
        self.save_cache();
    }

    /// 保存缓存到磁盘
    fn save_cache(&self) {
        if !self.config.enable_persistence {
            return;
        }
        
        // 创建缓存目录
        if let Err(err) = fs::create_dir_all(&self.config.cache_dir) {
            eprintln!("Error creating cache directory: {}", err);
            return;
        }
        
        // 保存文件缓存
        let file_cache_path = self.config.cache_dir.join("file_cache.json");
        let file_cache = self.file_cache.read().unwrap();
        if let Err(err) = serde_json::to_writer(
            fs::File::create(&file_cache_path).unwrap(),
            &*file_cache
        ) {
            eprintln!("Error saving file cache: {}", err);
        }
        
        // 保存渲染缓存
        let render_cache_path = self.config.cache_dir.join("render_cache.json");
        let render_cache = self.render_cache.read().unwrap();
        if let Err(err) = serde_json::to_writer(
            fs::File::create(&render_cache_path).unwrap(),
            &*render_cache
        ) {
            eprintln!("Error saving render cache: {}", err);
        }
    }

    /// 从磁盘加载缓存
    fn load_cache(&self) {
        if !self.config.enable_persistence {
            return;
        }
        
        // 加载文件缓存
        let file_cache_path = self.config.cache_dir.join("file_cache.json");
        if file_cache_path.exists() {
            if let Ok(file) = fs::File::open(&file_cache_path) {
                if let Ok(cache) = serde_json::from_reader(file) {
                    let mut file_cache = self.file_cache.write().unwrap();
                    *file_cache = cache;
                }
            }
        }
        
        // 加载渲染缓存
        let render_cache_path = self.config.cache_dir.join("render_cache.json");
        if render_cache_path.exists() {
            if let Ok(file) = fs::File::open(&render_cache_path) {
                if let Ok(cache) = serde_json::from_reader(file) {
                    let mut render_cache = self.render_cache.write().unwrap();
                    *render_cache = cache;
                }
            }
        }
    }
}

impl Default for CacheManager {
    fn default() -> Self {
        Self::new()
    }
}
