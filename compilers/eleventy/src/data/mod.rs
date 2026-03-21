//! 数据系统模块

use std::fs::File;
use std::io::Read;
use std::path::Path;

use serde::{Deserialize, Serialize};

/// 数据错误类型
#[derive(Debug)]
pub enum DataError {
    /// 文件读取错误
    FileReadError(std::io::Error),
    /// 数据解析错误
    ParseError(String),
    /// 数据加载错误
    LoadError(String),
}

/// 数据系统
pub struct DataSystem {
    /// 数据目录
    data_dir: String,
    
    /// 全局数据
    global_data: serde_json::Value,
    
    /// 模板数据
    template_data: std::collections::HashMap<String, serde_json::Value>,
    
    /// 前置数据
    frontmatter_data: std::collections::HashMap<String, serde_json::Value>,
}

impl DataSystem {
    /// 创建新的数据系统
    pub fn new(data_dir: &str) -> Self {
        Self {
            data_dir: data_dir.to_string(),
            global_data: serde_json::Value::Object(serde_json::Map::new()),
            template_data: std::collections::HashMap::new(),
            frontmatter_data: std::collections::HashMap::new(),
        }
    }
    
    /// 加载全局数据
    pub fn load_global_data(&mut self) -> Result<(), DataError> {
        let data_path = Path::new(&self.data_dir);
        
        if !data_path.exists() || !data_path.is_dir() {
            return Ok(());
        }
        
        // 加载数据目录中的所有文件
        for entry in std::fs::read_dir(data_path).map_err(|e| DataError::FileReadError(e))? {
            let entry = entry.map_err(|e| DataError::FileReadError(e))?;
            let path = entry.path();
            
            if path.is_file() {
                let data = self.load_data_file(&path)?;
                let filename = path.file_stem().unwrap_or_default().to_string_lossy().to_string();
                
                // 将数据添加到全局数据中
                self.global_data.as_object_mut().unwrap().insert(filename, data);
            }
        }
        
        Ok(())
    }
    
    /// 加载模板数据
    pub fn load_template_data(&mut self, template_path: &str) -> Result<serde_json::Value, DataError> {
        // 这里需要实现模板数据的加载逻辑
        Ok(serde_json::Value::Object(serde_json::Map::new()))
    }
    
    /// 加载前置数据
    pub fn load_frontmatter(&mut self, content: &str) -> Result<(serde_json::Value, String), DataError> {
        // 这里需要实现前置数据的解析逻辑
        // 暂时返回空前置数据和原始内容
        Ok((serde_json::Value::Object(serde_json::Map::new()), content.to_string()))
    }
    
    /// 加载数据文件
    fn load_data_file<P: AsRef<Path>>(&self, path: P) -> Result<serde_json::Value, DataError> {
        let path = path.as_ref();
        let extension = path.extension().unwrap_or_default().to_string_lossy().to_lowercase();
        
        let mut file = File::open(path).map_err(|e| DataError::FileReadError(e))?;
        let mut content = String::new();
        file.read_to_string(&mut content).map_err(|e| DataError::FileReadError(e))?;
        
        match extension.as_str() {
            "json" => {
                serde_json::from_str(&content).map_err(|e| DataError::ParseError(e.to_string()))
            }
            "yaml" | "yml" => {
                serde_yaml::from_str(&content).map_err(|e| DataError::ParseError(e.to_string()))
            }
            "toml" => {
                toml::from_str(&content).map_err(|e| DataError::ParseError(e.to_string()))
            }
            _ => {
                Err(DataError::LoadError(format!("Unsupported file type: {}", extension)))
            }
        }
    }
    
    /// 获取全局数据
    pub fn global_data(&self) -> &serde_json::Value {
        &self.global_data
    }
    
    /// 获取模板数据
    pub fn template_data(&self, template_path: &str) -> Option<&serde_json::Value> {
        self.template_data.get(template_path)
    }
    
    /// 获取前置数据
    pub fn frontmatter_data(&self, content_path: &str) -> Option<&serde_json::Value> {
        self.frontmatter_data.get(content_path)
    }
}