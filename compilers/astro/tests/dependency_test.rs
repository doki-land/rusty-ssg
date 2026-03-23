//! 依赖管理测试

use astro::compiler::dependency::{Dependency, DependencyAnalyzer, DependencyGraph, DependencyType};
use std::path::PathBuf;

#[test]
fn test_dependency_creation() {
    // 创建依赖
    let source = PathBuf::from("src/main.js");
    let target = PathBuf::from("src/components/Button.js");
    let dependency = Dependency {
        source,
        target,
        dep_type: DependencyType::Import
    };
    
    // 验证依赖属性
    assert_eq!(dependency.source, PathBuf::from("src/main.js"));
    assert_eq!(dependency.target, PathBuf::from("src/components/Button.js"));
    assert_eq!(dependency.dep_type, DependencyType::Import);
}

#[test]
fn test_dependency_graph() {
    // 创建依赖图
    let graph = DependencyGraph::new();
    
    // 添加依赖关系
    let source = PathBuf::from("src/main.js");
    let target = PathBuf::from("src/components/Button.js");
    graph.add_dependency(source.clone(), target.clone());
    
    // 检查依赖关系
    assert!(graph.has_dependency(&source, &target));
    
    // 获取依赖
    let dependencies = graph.get_dependencies(&source);
    assert!(dependencies.is_some());
    assert!(dependencies.unwrap().contains(&target));
    
    // 获取反向依赖
    let reverse_dependencies = graph.get_reverse_dependencies(&target);
    assert!(reverse_dependencies.is_some());
    assert!(reverse_dependencies.unwrap().contains(&source));
}

#[test]
fn test_dependency_analyzer() {
    // 创建依赖分析器
    let analyzer = DependencyAnalyzer::new();
    
    // 测试文件路径
    let file_path = PathBuf::from("src/main.js");
    
    // 测试文件内容
    let content = r#"
import Button from './components/Button.js';
import { useState } from 'react';

export { Button } from './components/Button.js';
"#;
    
    // 分析文件依赖
    let result = analyzer.analyze_file(&file_path, content);
    assert!(result.is_ok());
    
    // 获取依赖图
    let graph = analyzer.graph();
    
    // 验证依赖关系
    let target_path = PathBuf::from("src/components/Button.js");
    assert!(graph.has_dependency(&file_path, &target_path));
}

#[test]
fn test_dependency_analyzer_batch() {
    // 创建依赖分析器
    let analyzer = DependencyAnalyzer::new();
    
    // 测试文件
    let files = vec![
        (PathBuf::from("src/main.js"), r#"
import Button from './components/Button.js';
"#.to_string()),
        (PathBuf::from("src/components/Button.js"), r#"
import Icon from './Icon.js';
"#.to_string())
    ];
    
    // 批量分析文件依赖
    analyzer.analyze_files(&files);
    
    // 获取依赖图
    let graph = analyzer.graph();
    
    // 验证依赖关系
    let main_path = PathBuf::from("src/main.js");
    let button_path = PathBuf::from("src/components/Button.js");
    let icon_path = PathBuf::from("src/components/Icon.js");
    
    assert!(graph.has_dependency(&main_path, &button_path));
    assert!(graph.has_dependency(&button_path, &icon_path));
}

