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
        // 检查是否已经加载过该模板的数据
        if let Some(data) = self.template_data.get(template_path) {
            return Ok(data.clone());
        }
        
        // 尝试从与模板文件同名的数据文件中加载数据
        let template_path = Path::new(template_path);
        let template_stem = template_path.file_stem().unwrap_or_default().to_string_lossy();
        let template_dir = template_path.parent().unwrap_or_else(|| Path::new("."));
        
        // 尝试不同的数据文件格式
        let extensions = ["json", "yaml", "yml", "toml"];
        for ext in &extensions {
            let data_path = template_dir.join(format!("{}.{}", template_stem, ext));
            if data_path.exists() && data_path.is_file() {
                let data = self.load_data_file(&data_path)?;
                self.template_data.insert(template_path.to_string_lossy().to_string(), data.clone());
                return Ok(data);
            }
        }
        
        // 尝试从 _data 目录中加载与模板同名的子目录数据
        let data_dir = Path::new(&self.data_dir);
        let template_data_dir = data_dir.join(template_stem.to_string());
        if template_data_dir.exists() && template_data_dir.is_dir() {
            let mut template_data = serde_json::Value::Object(serde_json::Map::new());
            
            // 加载子目录中的所有数据文件
            for entry in std::fs::read_dir(template_data_dir).map_err(|e| DataError::FileReadError(e))? {
                let entry = entry.map_err(|e| DataError::FileReadError(e))?;
                let path = entry.path();
                
                if path.is_file() {
                    let data = self.load_data_file(&path)?;
                    let filename = path.file_stem().unwrap_or_default().to_string_lossy().to_string();
                    template_data.as_object_mut().unwrap().insert(filename, data);
                }
            }
            
            self.template_data.insert(template_path.to_string_lossy().to_string(), template_data.clone());
            return Ok(template_data);
        }
        
        // 没有找到模板数据，返回空对象
        let empty_data = serde_json::Value::Object(serde_json::Map::new());
        self.template_data.insert(template_path.to_string_lossy().to_string(), empty_data.clone());
        Ok(empty_data)
    }
    
    /// 加载前置数据
    pub fn load_frontmatter(&mut self, content: &str) -> Result<(serde_json::Value, String), DataError> {
        // 解析 frontmatter
        if content.starts_with("---") {
            let mut lines = content.lines();
            lines.next(); // 跳过第一个 --- 行
            
            let mut frontmatter_lines = Vec::new();
            let mut content_lines = Vec::new();
            let mut in_frontmatter = true;
            
            for line in lines {
                if in_frontmatter && line == "---" {
                    in_frontmatter = false;
                    continue;
                }
                
                if in_frontmatter {
                    frontmatter_lines.push(line);
                } else {
                    content_lines.push(line);
                }
            }
            
            let frontmatter_content = frontmatter_lines.join("\n");
            let content_content = content_lines.join("\n");
            
            // 解析 frontmatter 内容
            let frontmatter = self.parse_frontmatter(&frontmatter_content)?;
            
            Ok((frontmatter, content_content))
        } else {
            // 没有 frontmatter，返回空前置数据和原始内容
            Ok((serde_json::Value::Object(serde_json::Map::new()), content.to_string()))
        }
    }
    
    /// 解析 frontmatter 内容
    fn parse_frontmatter(&self, content: &str) -> Result<serde_json::Value, DataError> {
        // 尝试解析为 YAML
        if let Ok(data) = serde_yaml::from_str(content) {
            return Ok(data);
        }
        
        // 尝试解析为 JSON
        if let Ok(data) = serde_json::from_str(content) {
            return Ok(data);
        }
        
        // 尝试解析为 TOML
        if let Ok(data) = toml::from_str(content) {
            return Ok(data);
        }
        
        Err(DataError::ParseError("Failed to parse frontmatter".to_string()))
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