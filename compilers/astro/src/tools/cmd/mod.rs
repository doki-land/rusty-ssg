//! 命令行工具模块

use crate::{
    cache::CacheManager,
    compiler::{
        DependencyAnalyzer, Optimizer,
        component::ComponentParser,
        renderer::{HtmlRenderer, MarkdownRenderer, html_renderer::Context},
    },
    config::{AstroConfig, ConfigManager},
    plugin::{PluginContext, PluginLifecycleEvent, PluginManager},
};
use hashbrown::HashMap;
use rayon::prelude::*;
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

    // 2. 初始化插件系统
    println!("🔌 Initializing plugin system...");
    let mut plugin_manager = PluginManager::new();

    // 创建插件上下文
    let plugin_context = PluginContext {
        config: serde_json::to_value(&config).unwrap_or_default(),
        build_info: serde_json::json!({
            "timestamp": chrono::Utc::now().to_rfc3339(),
            "mode": "production",
            "outdir": outdir
        }),
        shared_data: serde_json::Value::Object(serde_json::Map::new()),
    };
    plugin_manager.set_context(plugin_context);

    // 从配置加载插件
    if let Err(err) = plugin_manager.load_from_config(&config, project_path) {
        eprintln!("Warning: Failed to load plugins: {}", err);
    }

    // 初始化插件
    if let Err(err) = plugin_manager.init_all() {
        eprintln!("Warning: Failed to initialize plugins: {}", err);
    }

    // 触发构建开始事件
    if let Err(err) = plugin_manager.trigger_event(&PluginLifecycleEvent::BuildStart) {
        eprintln!("Warning: Failed to trigger BuildStart event: {}", err);
    }

    // 3. 初始化缓存管理器
    let cache_manager = CacheManager::new();

    // 4. 处理文件
    println!("🔍 Finding and processing files...");
    let (component_parser, dependency_analyzer) = process_files(project_path, &cache_manager);

    // 5. 生成静态文件
    println!("✨ Generating static files...");
    let actual_outdir = if !outdir.is_empty() { outdir } else { &config.out_dir };
    generate_static_files(
        &component_parser,
        &dependency_analyzer,
        project_path,
        actual_outdir,
        &config,
        &plugin_manager,
        &cache_manager,
    );

    // 触发构建结束事件
    if let Err(err) = plugin_manager.trigger_event(&PluginLifecycleEvent::BuildEnd) {
        eprintln!("Warning: Failed to trigger BuildEnd event: {}", err);
    }

    // 清理插件
    plugin_manager.cleanup();

    // 5. 输出到指定目录
    println!("✅ Build completed successfully!");
}

/// 处理项目文件
fn process_files(project_path: &Path, cache_manager: &CacheManager) -> (ComponentParser, DependencyAnalyzer) {
    let parser = ComponentParser::new();
    let analyzer = DependencyAnalyzer::new();

    // 查找所有组件文件和页面文件
    let src_dir = project_path.join("src");
    if src_dir.exists() {
        // 收集所有文件路径
        let files: Vec<_> = WalkDir::new(&src_dir)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.path().is_file())
            .filter(|e| {
                if let Some(ext) = e.path().extension() {
                    let ext_str = ext.to_string_lossy().to_lowercase();
                    ["astro", "jsx", "tsx", "vue", "svelte", "md", "mdx", "js", "ts"].contains(&ext_str.as_str())
                }
                else {
                    false
                }
            })
            .map(|e| e.path().to_path_buf())
            .collect();

        // 首先分析所有文件的依赖关系，构建完整的依赖图
        let file_contents: Vec<_> = files
            .par_iter()
            .filter_map(|path| std::fs::read_to_string(path).ok().map(|content| (path.clone(), content)))
            .collect();

        analyzer.analyze_files(&file_contents);

        // 找出需要更新的文件
        let mut updated_files = Vec::new();
        for path in &files {
            if cache_manager.needs_update(path) {
                updated_files.push(path.clone());
            }
        }

        // 找出依赖于更新文件的其他文件
        let mut files_to_process = updated_files.clone();
        for path in &updated_files {
            if let Some(reverse_deps) = analyzer.graph().get_reverse_dependencies(path) {
                for dep in reverse_deps {
                    if !files_to_process.contains(&dep) {
                        files_to_process.push(dep);
                    }
                }
            }
        }

        // 并行处理需要更新的文件
        files_to_process.par_iter().for_each(|path| {
            // 解析并注册组件
            if let Err(err) = parser.parse_and_register_from_path(path) {
                eprintln!("Error processing file {}: {}", path.display(), err);
            }

            // 重新缓存文件内容
            if let Ok(content) = std::fs::read_to_string(path) {
                cache_manager.set_file(path, content.clone());
            }
        });
    }

    (parser, analyzer)
}

/// 生成静态文件
fn generate_static_files(
    _parser: &ComponentParser,
    _analyzer: &DependencyAnalyzer,
    project_path: &Path,
    outdir: &str,
    config: &AstroConfig,
    plugin_manager: &PluginManager,
    cache_manager: &CacheManager,
) {
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
    process_pages(project_path, out_path, &renderer, &markdown_renderer, &context, plugin_manager, cache_manager);

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
        true, // 启用预加载
    );

    if let Err(err) = optimizer.optimize(output_dir) {
        eprintln!("Error optimizing build output: {}", err);
    }
}

/// 处理页面文件
fn process_pages(
    project_path: &Path,
    out_path: &Path,
    renderer: &HtmlRenderer,
    markdown_renderer: &MarkdownRenderer,
    context: &Context,
    plugin_manager: &PluginManager,
    cache_manager: &CacheManager,
) {
    // 处理 src/pages 目录
    let pages_dir = project_path.join("src").join("pages");
    if pages_dir.exists() {
        // 收集所有页面文件
        let page_files: Vec<_> = WalkDir::new(&pages_dir)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.path().is_file())
            .map(|e| e.path().to_path_buf())
            .collect();

        // 并行处理页面文件
        page_files.par_iter().for_each(|path| {
            let ext = path.extension().unwrap_or_default().to_string_lossy().to_lowercase();

            // 生成相对路径
            let relative_path = path.strip_prefix(&pages_dir).unwrap();
            let html_path = out_path.join(relative_path).with_extension("html");

            // 创建输出目录
            if let Err(err) = fs::create_dir_all(html_path.parent().unwrap()) {
                eprintln!("Error creating directory: {}", err);
                return;
            }

            // 生成缓存键
            let cache_key = format!("page:{}", path.display());

            match ext.as_str() {
                "astro" => {
                    // 检查缓存是否有效
                    if !cache_manager.render_needs_update(&cache_key) {
                        // 使用缓存的渲染结果
                        if let Some(cache_item) = cache_manager.get_render(&cache_key) {
                            if let Ok(mut file) = File::create(&html_path) {
                                if let Err(err) = file.write_all(cache_item.result.as_bytes()) {
                                    eprintln!("Error writing file {}: {}", html_path.display(), err);
                                }
                                else {
                                    println!("Generated from cache: {}", html_path.display());
                                }
                            }
                            return;
                        }
                    }

                    // 处理 Astro 页面
                    if let Ok(content) = fs::read_to_string(path) {
                        // 执行插件处理内容
                        let processed_content = match plugin_manager.execute_all(&content) {
                            Ok(processed) => processed,
                            Err(err) => {
                                eprintln!("Warning: Failed to execute plugins on {}: {}", path.display(), err);
                                content
                            }
                        };

                        let rendered = renderer.render_astro(&processed_content, context);

                        // 对渲染结果再次执行插件
                        let final_content = match plugin_manager.execute_all(&rendered) {
                            Ok(processed) => processed,
                            Err(err) => {
                                eprintln!("Warning: Failed to execute plugins on rendered content: {}", err);
                                rendered
                            }
                        };

                        // 缓存渲染结果
                        let mut dependencies = HashMap::new();
                        if let Ok(metadata) = path.metadata() {
                            if let Ok(modified_time) = metadata.modified() {
                                dependencies.insert(path.to_path_buf(), modified_time);
                            }
                        }
                        cache_manager.set_render(&cache_key, final_content.clone(), dependencies);

                        if let Ok(mut file) = File::create(&html_path) {
                            if let Err(err) = file.write_all(final_content.as_bytes()) {
                                eprintln!("Error writing file {}: {}", html_path.display(), err);
                            }
                            else {
                                println!("Generated: {}", html_path.display());
                            }
                        }
                    }
                }
                "md" | "mdx" => {
                    // 检查缓存是否有效
                    if !cache_manager.render_needs_update(&cache_key) {
                        // 使用缓存的渲染结果
                        if let Some(cache_item) = cache_manager.get_render(&cache_key) {
                            if let Ok(mut file) = File::create(&html_path) {
                                if let Err(err) = file.write_all(cache_item.result.as_bytes()) {
                                    eprintln!("Error writing file {}: {}", html_path.display(), err);
                                }
                                else {
                                    println!("Generated from cache: {}", html_path.display());
                                }
                            }
                            return;
                        }
                    }

                    // 处理 Markdown 页面
                    if let Ok(content) = fs::read_to_string(path) {
                        // 执行插件处理内容
                        let processed_content = match plugin_manager.execute_all(&content) {
                            Ok(processed) => processed,
                            Err(err) => {
                                eprintln!("Warning: Failed to execute plugins on {}: {}", path.display(), err);
                                content
                            }
                        };

                        let rendered = markdown_renderer.render(&processed_content);

                        // 对渲染结果再次执行插件
                        let final_content = match plugin_manager.execute_all(&rendered) {
                            Ok(processed) => processed,
                            Err(err) => {
                                eprintln!("Warning: Failed to execute plugins on rendered content: {}", err);
                                rendered
                            }
                        };

                        // 缓存渲染结果
                        let mut dependencies = HashMap::new();
                        if let Ok(metadata) = path.metadata() {
                            if let Ok(modified_time) = metadata.modified() {
                                dependencies.insert(path.to_path_buf(), modified_time);
                            }
                        }
                        cache_manager.set_render(&cache_key, final_content.clone(), dependencies);

                        if let Ok(mut file) = File::create(&html_path) {
                            if let Err(err) = file.write_all(final_content.as_bytes()) {
                                eprintln!("Error writing file {}: {}", html_path.display(), err);
                            }
                            else {
                                println!("Generated: {}", html_path.display());
                            }
                        }
                    }
                }
                _ => {
                    // 其他文件类型，直接复制
                    if let Err(err) = fs::copy(path, &html_path) {
                        eprintln!("Error copying file {}: {}", html_path.display(), err);
                    }
                    else {
                        println!("Copied: {}", html_path.display());
                    }
                }
            }
        });
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
        }
        else {
            println!("Generated: {}", not_found_path.display());
        }
    }

    // 生成 sitemap.xml
    let sitemap_path = out_path.join("sitemap.xml");
    let sitemap_content = format!(
        r#"
<?xml version="1.0" encoding="UTF-8"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">
    <url>
        <loc>{}</loc>
        <lastmod>{}</lastmod>
        <changefreq>daily</changefreq>
        <priority>1.0</priority>
    </url>
</urlset>
"#,
        config.base.as_deref().unwrap_or("/"),
        chrono::Utc::now().format("%Y-%m-%d")
    );

    if let Ok(mut file) = File::create(&sitemap_path) {
        if let Err(err) = file.write_all(sitemap_content.as_bytes()) {
            eprintln!("Error writing sitemap.xml: {}", err);
        }
        else {
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

    // 1. 读取项目配置
    let mut config_manager = ConfigManager::new();
    let config = match config_manager.load_from_project(project_path) {
        Ok(config) => config,
        Err(err) => {
            eprintln!("Error loading configuration: {}", err);
            return;
        }
    };

    // 2. 初始化插件系统
    println!("🔌 Initializing plugin system...");
    let mut plugin_manager = PluginManager::new();

    // 创建插件上下文
    let plugin_context = PluginContext {
        config: serde_json::to_value(&config).unwrap_or_default(),
        build_info: serde_json::json!({
            "timestamp": chrono::Utc::now().to_rfc3339(),
            "mode": "development",
            "port": port
        }),
        shared_data: serde_json::Value::Object(serde_json::Map::new()),
    };
    plugin_manager.set_context(plugin_context);

    // 从配置加载插件
    if let Err(err) = plugin_manager.load_from_config(&config, project_path) {
        eprintln!("Warning: Failed to load plugins: {}", err);
    }

    // 初始化插件
    if let Err(err) = plugin_manager.init_all() {
        eprintln!("Warning: Failed to initialize plugins: {}", err);
    }

    // 触发服务器启动事件
    if let Err(err) = plugin_manager.trigger_event(&PluginLifecycleEvent::ServerStart) {
        eprintln!("Warning: Failed to trigger ServerStart event: {}", err);
    }

    // TODO: 实现实际的开发服务器逻辑
    // 1. 启动本地服务器
    // 2. 监听文件变化
    // 3. 自动重新构建
    // 4. 实时刷新浏览器

    // 触发服务器停止事件
    if let Err(err) = plugin_manager.trigger_event(&PluginLifecycleEvent::ServerStop) {
        eprintln!("Warning: Failed to trigger ServerStop event: {}", err);
    }

    // 清理插件
    plugin_manager.cleanup();
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
