//! 数据处理模块

use serde_json::Value;

/// 数据加载器，用于从不同格式加载数据
pub struct DataLoader;

impl DataLoader {
    /// 创建新的数据加载器
    pub fn new() -> Self {
        Self
    }

    /// 从 JSON 字符串加载数据
    pub fn load_json(&self, json_str: &str) -> Result<Value, Box<dyn std::error::Error>> {
        let data: Value = serde_json::from_str(json_str)?;
        Ok(data)
    }

    /// 从 YAML 字符串加载数据
    pub fn load_yaml(&self, yaml_str: &str) -> Result<Value, Box<dyn std::error::Error>> {
        // 简单实现，使用 serde_json 模拟 YAML 解析
        // 实际项目中应该使用 yaml 库
        let data: Value = serde_json::from_str(&yaml_str.replace(':', ": "))?;
        Ok(data)
    }
}

/// 数据管理器，用于管理和合并数据
pub struct DataManager {
    /// 数据存储
    data: std::collections::HashMap<String, Value>,
}

impl DataManager {
    /// 创建新的数据管理器
    pub fn new() -> Self {
        Self {
            data: std::collections::HashMap::new(),
        }
    }

    /// 添加数据
    pub fn add(&mut self, key: &str, value: Value) {
        self.data.insert(key.to_string(), value);
    }

    /// 检查数据是否存在
    pub fn exists(&self, key: &str) -> bool {
        self.data.contains_key(key)
    }

    /// 获取数据
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.data.get(key)
    }

    /// 合并所有数据
    pub fn merge(&self) -> Value {
        let mut merged = Value::Object(serde_json::Map::new());

        for (key, value) in &self.data {
            if let Value::Object(map) = merged {
                let mut new_map = map;
                new_map.insert(key.to_string(), value.clone());
                merged = Value::Object(new_map);
            } else {
                let mut map = serde_json::Map::new();
                map.insert(key.to_string(), value.clone());
                merged = Value::Object(map);
            }
        }

        merged
    }
}
