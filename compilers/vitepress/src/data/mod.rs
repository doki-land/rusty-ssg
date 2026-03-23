//! 数据处理模块
//! 负责加载和处理模板所需的数据

use nargo_types::NargoValue;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

/// 数据加载器
/// 用于从各种格式的文件中加载数据
pub struct DataLoader {
    root_dirs: Vec<PathBuf>,
}

impl DataLoader {
    /// 创建新的数据加载器
    pub fn new() -> Self {
        Self { root_dirs: Vec::new() }
    }

    /// 添加数据根目录
    pub fn add_root_dir<P: AsRef<Path>>(&mut self, dir: P) {
        self.root_dirs.push(dir.as_ref().to_path_buf());
    }

    /// 加载 JSON 文件
    pub fn load_json<P: AsRef<Path>>(&self, path: P) -> Result<serde_json::Value, DataError> {
        let content = std::fs::read_to_string(path.as_ref())?;
        serde_json::from_str(&content).map_err(|e| DataError::ParseError { message: e.to_string() })
    }

    /// 加载 TOML 文件
    pub fn load_toml<P: AsRef<Path>, T: DeserializeOwned>(&self, path: P) -> Result<T, DataError> {
        let content = std::fs::read_to_string(path.as_ref())?;
        oak_toml::from_str(&content).map_err(|e| DataError::ParseError { message: e.to_string() })
    }

    /// 加载 YAML 文件
    pub fn load_yaml<P: AsRef<Path>, T: DeserializeOwned>(&self, path: P) -> Result<T, DataError> {
        let content = std::fs::read_to_string(path.as_ref())?;
        oak_yaml::from_str(&content).map_err(|e| DataError::ParseError { message: e.to_string() })
    }

    /// 从环境变量加载数据
    pub fn load_from_env(&self, prefix: &str) -> HashMap<String, NargoValue> {
        let mut data = HashMap::new();

        for (key, value) in env::vars() {
            if key.starts_with(prefix) {
                let normalized_key = key[prefix.len()..].to_lowercase().replace("_", ".");
                data.insert(normalized_key, NargoValue::String(value));
            }
        }

        data
    }

    /// 从数据目录加载所有数据文件
    pub fn load_data_dir(&self, dir: &Path) -> Result<HashMap<String, NargoValue>, DataError> {
        let mut data = HashMap::new();

        if !dir.exists() {
            return Ok(data);
        }

        for entry in walkdir::WalkDir::new(dir).max_depth(2) {
            let entry = entry.map_err(|e| DataError::IoError { message: e.to_string() })?;
            let path = entry.path();

            if path.is_file() {
                if let Some(ext) = path.extension() {
                    let stem = path.file_stem().and_then(|s| s.to_str()).unwrap_or("");
                    let key = stem.to_string();

                    match ext.to_str() {
                        Some("json") => {
                            if let Ok(value) = self.load_json(path) {
                                data.insert(key, NargoValue::from_json(value));
                            }
                        }
                        Some("toml") => {
                            if let Ok(value) = self.load_toml::<_, serde_json::Value>(path) {
                                data.insert(key, NargoValue::from_json(value));
                            }
                        }
                        Some("yaml") | Some("yml") => {
                            if let Ok(value) = self.load_yaml::<_, serde_json::Value>(path) {
                                data.insert(key, NargoValue::from_json(value));
                            }
                        }
                        _ => {}
                    }
                }
            }
        }

        Ok(data)
    }

    /// 加载指定路径的数据文件
    pub fn load_data_file<P: AsRef<Path>>(&self, path: P) -> Result<NargoValue, DataError> {
        let path = path.as_ref();

        if !path.exists() {
            return Err(DataError::NotFound { path: path.to_string_lossy().to_string() });
        }

        if let Some(ext) = path.extension() {
            match ext.to_str() {
                Some("json") => {
                    let value = self.load_json(path)?;
                    Ok(NargoValue::from_json(value))
                }
                Some("toml") => {
                    let value = self.load_toml::<_, serde_json::Value>(path)?;
                    Ok(NargoValue::from_json(value))
                }
                Some("yaml") | Some("yml") => {
                    let value = self.load_yaml::<_, serde_json::Value>(path)?;
                    Ok(NargoValue::from_json(value))
                }
                _ => Err(DataError::ParseError { message: "Unsupported file format".to_string() }),
            }
        } else {
            Err(DataError::ParseError { message: "No file extension".to_string() })
        }
    }
}

impl Default for DataLoader {
    fn default() -> Self {
        Self::new()
    }
}

/// 数据上下文
/// 提供给模板的数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataContext {
    /// 全局数据
    pub global: HashMap<String, NargoValue>,
    /// 页面数据
    pub page: HashMap<String, NargoValue>,
    /// 站点数据
    pub site: HashMap<String, NargoValue>,
}

impl DataContext {
    /// 创建新的数据上下文
    pub fn new() -> Self {
        Self { global: HashMap::new(), page: HashMap::new(), site: HashMap::new() }
    }

    /// 从页面数据创建
    pub fn from_page(page_data: HashMap<String, NargoValue>) -> Self {
        Self { global: HashMap::new(), page: page_data, site: HashMap::new() }
    }

    /// 插入全局数据
    pub fn insert_global(&mut self, key: String, value: NargoValue) {
        self.global.insert(key, value);
    }

    /// 插入页面数据
    pub fn insert_page(&mut self, key: String, value: NargoValue) {
        self.page.insert(key, value);
    }

    /// 插入站点数据
    pub fn insert_site(&mut self, key: String, value: NargoValue) {
        self.site.insert(key, value);
    }

    /// 合并数据上下文
    pub fn merge(&mut self, other: DataContext) {
        self.global.extend(other.global);
        self.page.extend(other.page);
        self.site.extend(other.site);
    }

    /// 转换为 serde_json::Value
    pub fn to_json(&self) -> serde_json::Value {
        let mut map = serde_json::Map::new();

        map.insert("global".to_string(), serde_json::to_value(&self.global).unwrap_or_default());
        map.insert("page".to_string(), serde_json::to_value(&self.page).unwrap_or_default());
        map.insert("site".to_string(), serde_json::to_value(&self.site).unwrap_or_default());

        serde_json::Value::Object(map)
    }
}

impl Default for DataContext {
    fn default() -> Self {
        Self::new()
    }
}

/// 数据错误
#[derive(Debug, Clone)]
pub enum DataError {
    /// 解析错误
    ParseError { message: String },
    /// IO 错误
    IoError { message: String },
    /// 找不到文件
    NotFound { path: String },
}

impl std::fmt::Display for DataError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DataError::ParseError { message } => write!(f, "Data parse error: {}", message),
            DataError::IoError { message } => write!(f, "Data IO error: {}", message),
            DataError::NotFound { path } => write!(f, "Data file not found: {}", path),
        }
    }
}

impl std::error::Error for DataError {}

/// 数据结果类型
pub type DataResult<T> = Result<T, DataError>;

/// 全局数据管理器
/// 管理站点全局数据
pub struct GlobalDataManager {
    data: HashMap<String, NargoValue>,
}

impl GlobalDataManager {
    /// 创建新的全局数据管理器
    pub fn new() -> Self {
        Self { data: HashMap::new() }
    }

    /// 设置全局数据
    pub fn set(&mut self, key: String, value: NargoValue) {
        self.data.insert(key, value);
    }

    /// 获取全局数据
    pub fn get(&self, key: &str) -> Option<&NargoValue> {
        self.data.get(key)
    }

    /// 获取所有全局数据
    pub fn all(&self) -> &HashMap<String, NargoValue> {
        &self.data
    }

    /// 从目录加载全局数据
    pub fn load_from_dir(&mut self, dir: &Path) -> DataResult<()> {
        let loader = DataLoader::new();
        let loaded_data = loader.load_data_dir(dir)?;

        for (key, value) in loaded_data {
            self.data.insert(key, value);
        }

        Ok(())
    }
}

impl Default for GlobalDataManager {
    fn default() -> Self {
        Self::new()
    }
}
