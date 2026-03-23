//! 数据模块测试

use astro::data::{DataLoader, DataManager};

#[test]
fn test_data_loader() {
    // 创建数据加载器
    let loader = DataLoader::new();
    
    // 加载 JSON 数据
    let json_data = r#"
    {
        "name": "Test Data",
        "items": [1, 2, 3]
    }
    "#;
    
    let data = loader.load_json(json_data).unwrap();
    
    // 验证数据加载
    assert_eq!(data["name"], "Test Data");
    assert_eq!(data["items"].as_array().unwrap().len(), 3);
}

#[test]
fn test_data_manager() {
    // 创建数据管理器
    let mut manager = DataManager::new();
    
    // 添加数据
    manager.add("test", serde_json::json!({
        "name": "Test Data",
        "value": 42
    }));
    
    // 检查数据是否存在
    assert!(manager.exists("test"));
    assert!(!manager.exists("non_existent"));
    
    // 获取数据
    let data = manager.get("test").unwrap();
    assert_eq!(data["name"], "Test Data");
    assert_eq!(data["value"], 42);
}

#[test]
fn test_data_manager_merge() {
    // 创建数据管理器
    let mut manager = DataManager::new();
    
    // 添加基础数据
    manager.add("base", serde_json::json!({
        "name": "Base Data",
        "value": 42
    }));
    
    // 添加覆盖数据
    manager.add("override", serde_json::json!({
        "name": "Override Data",
        "new_value": 84
    }));
    
    // 合并数据
    let merged = manager.merge();
    
    // 验证合并结果
    assert_eq!(merged["override"]["name"], "Override Data");
    assert_eq!(merged["override"]["new_value"], 84);
    assert_eq!(merged["base"]["name"], "Base Data");
    assert_eq!(merged["base"]["value"], 42);
}

#[test]
fn test_data_loader_yaml() {
    // 创建数据加载器
    let loader = DataLoader::new();
    
    // 加载 YAML 数据
    let yaml_data = r#"
name: Test Data
items:
  - 1
  - 2
  - 3
"#;
    
    let data = loader.load_yaml(yaml_data).unwrap();
    
    // 验证数据加载
    assert_eq!(data["name"], "Test Data");
    assert_eq!(data["items"].as_array().unwrap().len(), 3);
}
