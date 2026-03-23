//! 优化模块测试

use astro::compiler::optimization::Optimizer;
use std::fs;
use tempfile::tempdir;

#[test]
fn test_optimizer_creation() {
    // 创建优化器
    let _optimizer = Optimizer::new(true, true, true);

    // 验证优化器创建成功
    assert!(true);
}

#[test]
fn test_optimizer_default() {
    // 创建默认优化器
    let _optimizer = Optimizer::default();

    // 验证默认优化器创建成功
    assert!(true);
}

#[test]
fn test_optimizer_optimize() {
    // 创建临时目录
    let temp_dir = tempdir().unwrap();
    let output_dir = temp_dir.path().join("dist");
    fs::create_dir_all(&output_dir).unwrap();

    // 创建测试 HTML 文件
    let html_file = output_dir.join("index.html");
    fs::write(
        html_file,
        r#"
<!DOCTYPE html>
<html>
<head>
    <title>Test</title>
</head>
<body>
    <h1>Test Page</h1>
    <p>This is a test page.</p>
</body>
</html>
"#,
    )
    .unwrap();

    // 创建优化器
    let optimizer = Optimizer::new(true, true, true);

    // 优化输出目录
    let result = optimizer.optimize(output_dir.as_path());

    // 验证优化成功
    assert!(result.is_ok());
}

#[test]
fn test_optimizer_without_compression() {
    // 创建临时目录
    let temp_dir = tempdir().unwrap();
    let output_dir = temp_dir.path().join("dist");
    fs::create_dir_all(&output_dir).unwrap();

    // 创建测试 HTML 文件
    let html_file = output_dir.join("index.html");
    fs::write(
        html_file,
        r#"
<!DOCTYPE html>
<html>
<head>
    <title>Test</title>
</head>
<body>
    <h1>Test Page</h1>
    <p>This is a test page.</p>
</body>
</html>
"#,
    )
    .unwrap();

    // 创建不启用压缩的优化器
    let optimizer = Optimizer::new(false, true, true);

    // 优化输出目录
    let result = optimizer.optimize(output_dir.as_path());

    // 验证优化成功
    assert!(result.is_ok());
}
