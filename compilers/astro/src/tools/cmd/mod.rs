//! 命令行工具模块

use std::path::Path;
use std::fs;
use std::fs::File;
use std::io::Write;
use walkdir::WalkDir;
use crate::compiler::component::ComponentParser;
use crate::compiler::renderer::html_renderer::HtmlRenderer;
use crate::compiler::renderer::html_renderer::Context;

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
    
    // 2. 处理文件
    println!("🔍 Finding and processing files...");
    let component_parser = process_files(project_path);
    
    // 3. 生成静态文件
    println!("✨ Generating static files...");
    generate_static_files(&component_parser, project_path, outdir);
    
    // 4. 输出到指定目录
    println!("✅ Build completed successfully!");
}

/// 处理项目文件
fn process_files(project_path: &Path) -> ComponentParser {
    let mut parser = ComponentParser::new();
    
    // 查找所有组件文件
    let src_dir = project_path.join("src");
    if src_dir.exists() {
        for entry in WalkDir::new(&src_dir).into_iter().filter_map(|e| e.ok()) {
            let path = entry.path();
            if path.is_file() {
                if let Some(ext) = path.extension() {
                    if ext == "astro" || ext == "jsx" || ext == "tsx" || ext == "vue" || ext == "svelte" {
                        // 解析并注册组件
                        if let Err(err) = parser.parse_and_register_from_path(path) {
                            eprintln!("Error processing file {}: {}", path.display(), err);
                        }
                    }
                }
            }
        }
    }
    
    parser
}

/// 生成静态文件
fn generate_static_files(parser: &ComponentParser, project_path: &Path, outdir: &str) {
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
    let context = Context::new();
    
    // 为每个注册的组件生成静态文件
    let registry = parser.registry();
    for (name, component) in registry.iter() {
        let rendered = component.render(&context);
        
        // 生成 HTML 文件
        let html_path = out_path.join(format!("{}.html", name));
        if let Ok(mut file) = File::create(&html_path) {
            if let Err(err) = file.write_all(rendered.as_bytes()) {
                eprintln!("Error writing file {}: {}", html_path.display(), err);
            } else {
                println!("Generated: {}", html_path.display());
            }
        }
    }
    
    // 复制静态资源
    let public_dir = project_path.join("public");
    if public_dir.exists() {
        let public_out_dir = out_path.join("public");
        if let Err(err) = fs::create_dir_all(&public_out_dir) {
            eprintln!("Error creating public directory: {}", err);
        } else {
            for entry in WalkDir::new(&public_dir).into_iter().filter_map(|e| e.ok()) {
                let path = entry.path();
                if path.is_file() {
                    let relative_path = path.strip_prefix(&public_dir).unwrap();
                    let dest_path = public_out_dir.join(relative_path);
                    
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
