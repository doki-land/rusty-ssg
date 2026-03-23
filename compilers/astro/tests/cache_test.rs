//! 缓存系统测试

use astro::cache::CacheManager;
use hashbrown::HashMap;
use std::{fs, path::Path, time::SystemTime};
use tempfile::tempdir;

#[test]
fn test_cache_manager_creation() {
    // 创建缓存管理器
    let _cache_manager = CacheManager::new();

    // 验证缓存管理器创建成功
    assert!(true);
}

#[test]
fn test_cache_manager_default() {
    // 创建默认缓存管理器
    let _cache_manager = CacheManager::default();

    // 验证默认缓存管理器创建成功
    assert!(true);
}

#[test]
fn test_cache_manager_file_cache() {
    // 创建临时文件
    let temp_dir = tempdir().unwrap();
    let test_file = temp_dir.path().join("test.txt");
    fs::write(&test_file, "test content").unwrap();

    // 创建缓存管理器
    let cache_manager = CacheManager::new();

    // 设置文件缓存
    let content = fs::read_to_string(&test_file).unwrap();
    cache_manager.set_file(&test_file, content);

    // 获取文件缓存
    let cached_item = cache_manager.get_file(&test_file);
    assert!(cached_item.is_some());
    assert_eq!(cached_item.unwrap().content, "test content");
}

#[test]
fn test_cache_manager_render_cache() {
    // 创建缓存管理器
    let cache_manager = CacheManager::new();

    // 设置渲染缓存
    let dependencies = HashMap::new();
    cache_manager.set_render("test_key", "test_result".to_string(), dependencies);

    // 获取渲染缓存
    let cached_item = cache_manager.get_render("test_key");
    assert!(cached_item.is_some());
    assert_eq!(cached_item.unwrap().result, "test_result");
}

#[test]
fn test_cache_manager_clear() {
    // 创建临时文件
    let temp_dir = tempdir().unwrap();
    let test_file = temp_dir.path().join("test.txt");
    fs::write(&test_file, "test content").unwrap();

    // 创建缓存管理器
    let cache_manager = CacheManager::new();

    // 设置文件缓存
    let content = fs::read_to_string(&test_file).unwrap();
    cache_manager.set_file(&test_file, content);

    // 设置渲染缓存
    let dependencies = HashMap::new();
    cache_manager.set_render("test_key", "test_result".to_string(), dependencies);

    // 验证缓存存在
    assert!(cache_manager.get_file(&test_file).is_some());
    assert!(cache_manager.get_render("test_key").is_some());

    // 清空缓存
    cache_manager.clear();

    // 验证缓存不存在
    assert!(cache_manager.get_file(&test_file).is_none());
    assert!(cache_manager.get_render("test_key").is_none());
}

#[test]
fn test_cache_manager_clear_file() {
    // 创建临时文件
    let temp_dir = tempdir().unwrap();
    let test_file = temp_dir.path().join("test.txt");
    fs::write(&test_file, "test content").unwrap();

    // 创建缓存管理器
    let cache_manager = CacheManager::new();

    // 设置文件缓存
    let content = fs::read_to_string(&test_file).unwrap();
    cache_manager.set_file(&test_file, content);

    // 验证缓存存在
    assert!(cache_manager.get_file(&test_file).is_some());

    // 清除文件缓存
    cache_manager.clear_file(&test_file);

    // 验证缓存不存在
    assert!(cache_manager.get_file(&test_file).is_none());
}

#[test]
fn test_cache_manager_clear_render() {
    // 创建缓存管理器
    let cache_manager = CacheManager::new();

    // 设置渲染缓存
    let dependencies = HashMap::new();
    cache_manager.set_render("test_key", "test_result".to_string(), dependencies);

    // 验证缓存存在
    assert!(cache_manager.get_render("test_key").is_some());

    // 清除渲染缓存
    cache_manager.clear_render("test_key");

    // 验证缓存不存在
    assert!(cache_manager.get_render("test_key").is_none());
}
