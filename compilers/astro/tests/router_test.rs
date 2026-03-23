use astro::router::RouteResolver;
use std::path::PathBuf;
use tempfile::tempdir;

#[test]
fn test_route_resolver() {
    // 创建临时目录结构用于测试
    let temp_dir = tempdir().unwrap();
    let pages_dir = temp_dir.path().join("pages");
    let layouts_dir = temp_dir.path().join("layouts");

    // 创建目录
    std::fs::create_dir_all(&pages_dir).unwrap();
    std::fs::create_dir_all(&layouts_dir).unwrap();
    std::fs::create_dir_all(&pages_dir.join("blog")).unwrap();
    std::fs::create_dir_all(&pages_dir.join("[id]")).unwrap();

    // 创建测试文件
    std::fs::write(&pages_dir.join("index.astro"), "<h1>Home</h1>").unwrap();
    std::fs::write(&pages_dir.join("about.astro"), "<h1>About</h1>").unwrap();
    std::fs::write(&pages_dir.join("blog").join("index.astro"), "<h1>Blog</h1>").unwrap();
    std::fs::write(&pages_dir.join("[id]").join("index.astro"), "<h1>Post</h1>").unwrap();
    std::fs::write(&layouts_dir.join("+layout.astro"), "<layout><slot/></layout>").unwrap();

    // 创建路由解析器
    let mut resolver = RouteResolver::new(&pages_dir, &layouts_dir);
    resolver.scan_routes().unwrap();

    // 测试路由解析
    let result = resolver.resolve("/");
    assert!(result.is_some());

    let result = resolver.resolve("/about");
    assert!(result.is_some());

    let result = resolver.resolve("/blog");
    assert!(result.is_some());

    let result = resolver.resolve("/123");
    assert!(result.is_some());

    // 测试路由参数
    if let Some((_, _, params)) = resolver.resolve("/123") {
        assert_eq!(params.get("id"), Some(&"123".to_string()));
    }

    // 测试参数路由匹配
    let result = resolver.resolve("/nonexistent");
    assert!(result.is_some());

    // 测试不存在的深层路由
    let result = resolver.resolve("/blog/nonexistent");
    assert!(result.is_none());
}
