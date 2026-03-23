//! 编译器测试
//! 
//! 测试 MkDocs 编译器的核心功能

use mkdocs::{MkDocsCompiler, MkDocsConfig};
use std::fs;
use std::path::PathBuf;
use tempfile::tempdir;

#[tokio::test]
async fn test_compile_file() {
    // 创建临时目录
    let temp_dir = tempdir().unwrap();
    let source_dir = temp_dir.path().join("docs");
    let output_dir = temp_dir.path().join("site");
    fs::create_dir_all(&source_dir).unwrap();

    // 创建测试 Markdown 文件
    let test_file = source_dir.join("test.md");
    fs::write(&test_file, "# Test Page\n\nThis is a test page.")
        .unwrap();

    // 创建编译器
    let config = MkDocsConfig::new();
    let compiler = MkDocsCompiler::new(config, &source_dir, &output_dir);

    // 编译文件
    let result = compiler.compile_file(&test_file);
    assert!(result.is_ok());
    let html = result.unwrap();
    assert!(html.contains("<h1>Test Page</h1>"));
    assert!(html.contains("<p>This is a test page.</p>"));
}

#[tokio::test]
async fn test_compile_all() {
    // 创建临时目录
    let temp_dir = tempdir().unwrap();
    let source_dir = temp_dir.path().join("docs");
    let output_dir = temp_dir.path().join("site");
    fs::create_dir_all(&source_dir).unwrap();

    // 创建测试 Markdown 文件
    let test_file1 = source_dir.join("test1.md");
    let test_file2 = source_dir.join("test2.md");
    fs::write(&test_file1, "# Test Page 1\n\nThis is test page 1.")
        .unwrap();
    fs::write(&test_file2, "# Test Page 2\n\nThis is test page 2.")
        .unwrap();

    // 创建编译器
    let config = MkDocsConfig::new();
    let compiler = MkDocsCompiler::new(config, &source_dir, &output_dir);

    // 编译所有文件
    let result = compiler.compile_all();
    assert!(result.is_ok());
    let compile_times = result.unwrap();
    assert_eq!(compile_times.len(), 2);

    // 检查输出文件
    let output_file1 = output_dir.join("test1.html");
    let output_file2 = output_dir.join("test2.html");
    assert!(output_file1.exists());
    assert!(output_file2.exists());

    let content1 = fs::read_to_string(&output_file1).unwrap();
    let content2 = fs::read_to_string(&output_file2).unwrap();
    assert!(content1.contains("<h1>Test Page 1</h1>"));
    assert!(content2.contains("<h1>Test Page 2</h1>"));
}

#[tokio::test]
async fn test_copy_static_files() {
    // 创建临时目录
    let temp_dir = tempdir().unwrap();
    let source_dir = temp_dir.path().join("docs");
    let output_dir = temp_dir.path().join("site");
    fs::create_dir_all(&source_dir).unwrap();

    // 创建测试静态文件
    let static_file = source_dir.join("style.css");
    fs::write(&static_file, "body { background: #f0f0f0; }")
        .unwrap();

    // 创建编译器
    let config = MkDocsConfig::new();
    let compiler = MkDocsCompiler::new(config, &source_dir, &output_dir);

    // 复制静态文件
    let result = compiler.copy_static_files();
    assert!(result.is_ok());

    // 检查输出文件
    let output_file = output_dir.join("style.css");
    assert!(output_file.exists());
    let content = fs::read_to_string(&output_file).unwrap();
    assert_eq!(content, "body { background: #f0f0f0; }");
}

#[tokio::test]
async fn test_build() {
    // 创建临时目录
    let temp_dir = tempdir().unwrap();
    let source_dir = temp_dir.path().join("docs");
    let output_dir = temp_dir.path().join("site");
    fs::create_dir_all(&source_dir).unwrap();

    // 创建测试文件
    let test_file = source_dir.join("test.md");
    let static_file = source_dir.join("style.css");
    fs::write(&test_file, "# Test Page\n\nThis is a test page.")
        .unwrap();
    fs::write(&static_file, "body { background: #f0f0f0; }")
        .unwrap();

    // 创建编译器
    let config = MkDocsConfig::new();
    let compiler = MkDocsCompiler::new(config, &source_dir, &output_dir);

    // 构建项目
    let result = compiler.build();
    assert!(result.is_ok());

    // 检查输出文件
    let output_html = output_dir.join("test.html");
    let output_css = output_dir.join("style.css");
    assert!(output_html.exists());
    assert!(output_css.exists());
}
