//! 数据模块
//! 
//! 负责处理模板所需的数据

use std::collections::HashMap;
use serde_json::Value;

/// 数据上下文
pub struct DataContext {
    /// 全局数据
    pub global: HashMap<String, Value>,
    /// 页面数据
    pub page: HashMap<String, Value>,
    /// 站点数据
    pub site: HashMap<String, Value>,
}

impl DataContext {
    /// 创建新的数据上下文
    pub fn new() -> Self {
        Self {
            global: HashMap::new(),
            page: HashMap::new(),
            site: HashMap::new(),
        }
    }

    /// 添加全局数据
    pub fn add_global(&mut self, key: &str, value: Value) {
        self.global.insert(key.to_string(), value);
    }

    /// 添加页面数据
    pub fn add_page(&mut self, key: &str, value: Value) {
        self.page.insert(key.to_string(), value);
    }

    /// 添加站点数据
    pub fn add_site(&mut self, key: &str, value: Value) {
        self.site.insert(key.to_string(), value);
    }

    /// 获取全局数据
    pub fn global(&self) -> &HashMap<String, Value> {
        &self.global
    }

    /// 获取页面数据
    pub fn page(&self) -> &HashMap<String, Value> {
        &self.page
    }

    /// 获取站点数据
    pub fn site(&self) -> &HashMap<String, Value> {
        &self.site
    }

    /// 合并数据
    pub fn merge(&mut self, other: DataContext) {
        self.global.extend(other.global);
        self.page.extend(other.page);
        self.site.extend(other.site);
    }
}

impl Default for DataContext {
    fn default() -> Self {
        Self::new()
    }
}

/// 数据加载器
pub trait DataLoader {
    /// 加载数据
    fn load(&self) -> crate::types::Result<HashMap<String, Value>>;
}

/// 文件数据加载器
pub struct FileDataLoader {
    /// 文件路径
    path: String,
}

impl FileDataLoader {
    /// 创建新的文件数据加载器
    pub fn new(path: &str) -> Self {
        Self { path: path.to_string() }
    }
}

impl DataLoader for FileDataLoader {
    fn load(&self) -> crate::types::Result<HashMap<String, Value>> {
        let content = std::fs::read_to_string(&self.path)?;
        let data: HashMap<String, Value> = serde_json::from_str(&content)?;
        Ok(data)
    }
}
