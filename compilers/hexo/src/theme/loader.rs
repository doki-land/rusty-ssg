//! 主题加载器

use std::{
    fs::File,
    io::Read,
    path::{Path, PathBuf},
};

use crate::types::{HexoError, Result};
use oak_yaml;

/// 主题结构
#[derive(Debug)]
pub struct Theme {
    /// 主题名称
    pub name: String,
    /// 主题路径
    pub path: PathBuf,
    /// 主题配置
    pub config: serde_json::Value,
}

impl Theme {
    /// 从路径加载主题
    pub fn load(path: &Path) -> Result<Self> {
        let name = path
            .file_name()
            .ok_or_else(|| HexoError::custom_error("Theme path must have a file name".to_string()))?
            .to_string_lossy()
            .to_string();

        // 加载主题配置
        let config_path = path.join("_config.yml");
        let config = if config_path.exists() {
            let mut file = File::open(&config_path)
                .map_err(|e| HexoError::io_error(Some(config_path.to_string_lossy().to_string()), e.to_string()))?;
            let mut content = String::new();
            file.read_to_string(&mut content)
                .map_err(|e| HexoError::io_error(Some(config_path.to_string_lossy().to_string()), e.to_string()))?;
            oak_yaml::language::from_str(&content)
                .map_err(|e| HexoError::yaml_error(Some(config_path.to_string_lossy().to_string()), e.to_string()))?
        }
        else {
            serde_json::Value::Object(serde_json::Map::new())
        };

        Ok(Self { name, path: path.to_path_buf(), config })
    }

    /// 获取模板文件路径
    pub fn get_template_path(&self, template_name: &str) -> PathBuf {
        self.path.join("layout").join(format!("{}.ejs", template_name))
    }

    /// 获取静态文件路径
    pub fn get_static_path(&self) -> PathBuf {
        self.path.join("source")
    }
}

/// 主题管理器
#[derive(Debug)]
pub struct ThemeManager {
    /// 当前主题
    pub current_theme: Option<Theme>,
    /// 主题搜索路径
    pub search_paths: Vec<PathBuf>,
}

impl ThemeManager {
    /// 创建主题管理器
    pub fn new() -> Self {
        Self { current_theme: None, search_paths: vec![] }
    }

    /// 添加主题搜索路径
    pub fn add_search_path(&mut self, path: &Path) {
        self.search_paths.push(path.to_path_buf());
    }

    /// 加载主题
    pub fn load_theme(&mut self, theme_name: &str) -> Result<()> {
        // 搜索主题
        for search_path in &self.search_paths {
            let theme_path = search_path.join(theme_name);
            if theme_path.exists() && theme_path.is_dir() {
                let theme = Theme::load(&theme_path)?;
                self.current_theme = Some(theme);
                return Ok(());
            }
        }

        Err(HexoError::custom_error(format!("Theme '{}' not found", theme_name)))
    }

    /// 获取当前主题
    pub fn current(&self) -> Option<&Theme> {
        self.current_theme.as_ref()
    }
}
