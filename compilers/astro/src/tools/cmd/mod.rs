//! 命令行工具模块

use crate::compiler::{
    component::ComponentParser,
    DependencyAnalyzer,
    Optimizer,
    renderer::{HtmlRenderer, MarkdownRenderer},
    renderer::html_renderer::Context,
};
use crate::config::{ConfigManager, AstroConfig};
use crate::plugin::{PluginManager, PluginContext, PluginLifecycleEvent};
use std::{fs, fs::File, io::Write, path::Path};
use walkdir::WalkDir;

/// 构建命令
///
/// # 参数
/// * `path` - 项目目录路径
/// * `outdir` - 输出目录路径
pub fn build(path: &str, outdir: &str) {
    println!("Building project from {} to {}", path, outdir);
    // 检查项目目录是否存在
    let project_path = Path::new(path);
    if !project_path.exists() {
        eprintln!("Error: Project directory '{}' does not exist", path);
        return;
    }

    // 1. 读取项目配置
    println!("📦 Loading project configuration...");
    let mut config_manager = ConfigManager::new();
    let config = match config_manager.load_from_project(project_path) {
        Ok(config) => config,
        Err(err) => {
            eprintln!("Error loading configuration: {}", err);
            return;
        }
    };

    // 2. 处理文件
    println!("🔍 Finding and processing files...");
    let (component_parser, dependency_analyzer) = process_files(project_path);

    // 3. 生成静态文件
    println!("✨ Generating static files...");
    let actual_outdir = if !outdir.is_empty() {
        outdir
    } else {
        &config.out_dir
    };
    generate_static_files(&component_parser, &dependency_analyzer, project_path, actual_outdir, &config);

    // 4. 输出到指定目录
    println!("✅ Build completed successfully!");
}

/// 处理项目文件
fn process_files(project_path: &Path) -> (ComponentParser, DependencyAnalyzer) {
    let mut parser = ComponentParser::new();
    let mut analyzer = DependencyAnalyzer::new();

    // 查找所有组件文件和页面文件
    let src_dir = project_path.join("src");
    if src_dir.exists() {
        for entry in WalkDir::new(&src_dir).into_iter().filter_map(|e| e.ok()) {
            let path = entry.path();
            if path.is_file() {
                if let Some(ext) = path.extension() {
                    let ext_str = ext.to_string_lossy().to_lowercase();
                    // 支持的文件类型
                    if ["astro", "jsx", "tsx", "vue", "svelte", "md", "mdx", "js", "ts"].contains(&ext_str.as_str()) {
                        // 解析并注册组件
                        if let Err(err) = parser.parse_and_register_from_path(path) {
                            eprintln!("Error processing file {}: {}", path.display(), err);
                        }
                        
                        // 分析依赖关系
                        if let Ok(content) = std::fs::read_to_string(path) {
                            if let Err(err) = analyzer.analyze_file(path, &content) {
                                eprintln!("Error analyzing dependencies for file {}: {}", path.display(), err);
                            }
                        }
                    }
                }
            }
        }
    }

    (parser, analyzer)
}

/// 生成静态文件
fn generate_static_files(parser: &ComponentParser, analyzer: &DependencyAnalyzer, project_path: &Path, outdir: &str, config: &AstroConfig) {
    // 创建输出目录
    let out_path = Path::new(outdir);
    if !out_path.exists() {
        if let Err(err) = fs::create_dir_all(out_path) {
            eprintln!("Error creating output directory: {}", err);
            return;
        }
    }

    // 渲染组件
    let renderer = HtmlRenderer::new();
    let markdown_renderer = MarkdownRenderer::new();
    let context = Context::new();

    // 处理页面文件
    process_pages(project_path, out_path, &renderer, &markdown_renderer, &context);

    // 复制静态资源
    let public_dir = project_path.join("public");
    if public_dir.exists() {
        // 直接复制 public 目录下的所有文件到输出目录根目录
        for entry in WalkDir::new(&public_dir).into_iter().filter_map(|e| e.ok()) {
            let path = entry.path();
            if path.is_file() {
                let relative_path = path.strip_prefix(&public_dir).unwrap();
                let dest_path = out_path.join(relative_path);

                if let Err(err) = fs::create_dir_all(dest_path.parent().unwrap()) {
                    eprintln!("Error creating directory: {}", err);
                    continue;
                }

                if let Err(err) = fs::copy(path, dest_path) {
                    eprintln!("Error copying file: {}", err);
                }
            }
        }
    }

    // 复制其他静态资源
    copy_other_assets(project_path, out_path);

    // 生成必要的文件
    generate_essential_files(out_path, config);

    // 优化构建输出
    optimize_build(out_path, config);
}

/// 优化构建输出
fn optimize_build(output_dir: &Path, config: &AstroConfig) {
    println!("Optimizing build output...");
    
    let optimizer = Optimizer::new(
        config.compress_html,
        true, // 启用代码分割
        true  // 启用预加载
    );
    
    if let Err(err) = optimizer.optimize(output_dir) {
        eprintln!("Error optimizing build output: {}", err);
    }
}

/// 处理页面文件
fn process_pages(project_path: &Path, out_path: &Path, renderer: &HtmlRenderer, markdown_renderer: &MarkdownRenderer, context: &Context) {
    // 处理 src/pages 目录
    let pages_dir = project_path.join("src").join("pages");
    if pages_dir.exists() {
        for entry in WalkDir::new(&pages_dir).into_iter().filter_map(|e| e.ok()) {
            let path = entry.path();
            if path.is_file() {
                let ext = path.extension().unwrap_or_default().to_string_lossy().to_lowercase();
                
                // 生成相对路径
                let relative_path = path.strip_prefix(&pages_dir).unwrap();
                let html_path = out_path.join(relative_path).with_extension("html");
                
                // 创建输出目录
                if let Err(err) = fs::create_dir_all(html_path.parent().unwrap()) {
                    eprintln!("Error creating directory: {}", err);
                    continue;
                }
                
                match ext.as_str() {
                    "astro" => {
                        // 处理 Astro 页面
                        if let Ok(content) = fs::read_to_string(path) {
                            let rendered = renderer.render_astro(&content, context);
                            if let Ok(mut file) = File::create(&html_path) {
                                if let Err(err) = file.write_all(rendered.as_bytes()) {
                                    eprintln!("Error writing file {}: {}", html_path.display(), err);
                                } else {
                                    println!("Generated: {}", html_path.display());
                                }
                            }
                        }
                    }
                    "md" | "mdx" => {
                        // 处理 Markdown 页面
                        if let Ok(content) = fs::read_to_string(path) {
                            let rendered = markdown_renderer.render(&content);
                            if let Ok(mut file) = File::create(&html_path) {
                                if let Err(err) = file.write_all(rendered.as_bytes()) {
                                    eprintln!("Error writing file {}: {}", html_path.display(), err);
                                } else {
                                    println!("Generated: {}", html_path.display());
                                }
                            }
                        }
                    }
                    _ => {
                        // 其他文件类型，直接复制
                        if let Err(err) = fs::copy(path, &html_path) {
                            eprintln!("Error copying file {}: {}", html_path.display(), err);
                        } else {
                            println!("Copied: {}", html_path.display());
                        }
                    }
                }
            }
        }
    }
}

/// 生成必要的文件
fn generate_essential_files(out_path: &Path, config: &AstroConfig) {
    // 生成 404.html
    let not_found_path = out_path.join("404.html");
    let not_found_content = r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>404 Page Not Found</title>
</head>
<body>
    <h1>404 Page Not Found</h1>
    <p>The page you are looking for does not exist.</p>
</body>
</html>
"#;
    
    if let Ok(mut file) = File::create(&not_found_path) {
        if let Err(err) = file.write_all(not_found_content.as_bytes()) {
            eprintln!("Error writing 404.html: {}", err);
        } else {
            println!("Generated: {}", not_found_path.display());
        }
    }
    
    // 生成 sitemap.xml
    let sitemap_path = out_path.join("sitemap.xml");
    let sitemap_content = format!(r#"
<?xml version="1.0" encoding="UTF-8"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">
    <url>
        <loc>{}</loc>
        <lastmod>{}</lastmod>
        <changefreq>daily</changefreq>
        <priority>1.0</priority>
    </url>
</urlset>
"#, config.base.as_deref().unwrap_or("/"), chrono::Utc::now().format("%Y-%m-%d"));
    
    if let Ok(mut file) = File::create(&sitemap_path) {
        if let Err(err) = file.write_all(sitemap_content.as_bytes()) {
            eprintln!("Error writing sitemap.xml: {}", err);
        } else {
            println!("Generated: {}", sitemap_path.display());
        }
    }
}

/// 复制其他静态资源
fn copy_other_assets(project_path: &Path, out_path: &Path) {
    // 复制 src/assets 目录
    let assets_dir = project_path.join("src").join("assets");
    if assets_dir.exists() {
        let assets_out_dir = out_path.join("assets");
        if let Err(err) = fs::create_dir_all(&assets_out_dir) {
            eprintln!("Error creating assets directory: {}", err);
        }
        else {
            for entry in WalkDir::new(&assets_dir).into_iter().filter_map(|e| e.ok()) {
                let path = entry.path();
                if path.is_file() {
                    let relative_path = path.strip_prefix(&assets_dir).unwrap();
                    let dest_path = assets_out_dir.join(relative_path);

                    if let Err(err) = fs::create_dir_all(dest_path.parent().unwrap()) {
                        eprintln!("Error creating directory: {}", err);
                        continue;
                    }

                    if let Err(err) = fs::copy(path, dest_path) {
                        eprintln!("Error copying file: {}", err);
                    }
                }
            }
        }
    }
}

/// 处理 Markdown 文件
fn process_markdown_files(project_path: &Path, out_path: &Path, renderer: &MarkdownRenderer) {
    // 查找所有 Markdown 文件
    let src_dir = project_path.join("src");
    if src_dir.exists() {
        for entry in WalkDir::new(&src_dir).into_iter().filter_map(|e| e.ok()) {
            let path = entry.path();
            if path.is_file() {
                if let Some(ext) = path.extension() {
                    let ext_str = ext.to_string_lossy().to_lowercase();
                    if ext_str == "md" || ext_str == "mdx" {
                        // 读取 Markdown 文件内容
                        if let Ok(content) = fs::read_to_string(path) {
                            // 渲染 Markdown 为 HTML
                            let rendered = renderer.render(&content);
                            
                            // 生成 HTML 文件路径
                            let relative_path = path.strip_prefix(project_path).unwrap();
                            let html_path = out_path.join(relative_path).with_extension("html");
                            
                            // 创建输出目录
                            if let Err(err) = fs::create_dir_all(html_path.parent().unwrap()) {
                                eprintln!("Error creating directory: {}", err);
                                continue;
                            }
                            
                            // 写入 HTML 文件
                            if let Ok(mut file) = File::create(&html_path) {
                                if let Err(err) = file.write_all(rendered.as_bytes()) {
                                    eprintln!("Error writing file {}: {}", html_path.display(), err);
                                }
                                else {
                                    println!("Generated: {}", html_path.display());
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// 开发命令
///
/// # 参数
/// * `path` - 项目目录路径
/// * `port` - 开发服务器端口
pub fn dev(path: &str, port: u16) {
    println!("Starting dev server at http://localhost:{}", port);
    // 检查项目目录是否存在
    let project_path = Path::new(path);
    if !project_path.exists() {
        eprintln!("Error: Project directory '{}' does not exist", path);
        return;
    }

    // TODO: 实现实际的开发服务器逻辑
    // 1. 启动本地服务器
    // 2. 监听文件变化
    // 3. 自动重新构建
    // 4. 实时刷新浏览器
}

/// 预览命令
///
/// # 参数
/// * `path` - 构建输出目录路径
/// * `port` - 预览服务器端口
pub fn preview(path: &str, port: u16) {
    println!("Starting preview server at http://localhost:{}", port);
    // 检查构建输出目录是否存在
    let build_path = Path::new(path);
    if !build_path.exists() {
        eprintln!("Error: Build directory '{}' does not exist. Please run 'build' first.", path);
        return;
    }

    // TODO: 实现实际的预览服务器逻辑
    // 1. 启动本地服务器
    // 2. 提供静态文件服务
    // 3. 处理路由
}
