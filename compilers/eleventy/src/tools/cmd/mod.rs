//! 命令行工具模块

use crate::{
    compiler::template_engine::{SimpleTemplateEngine, TemplateEngineFactory},
    config::Config,
    types::Cli,
};
use clap::Parser;
use std::{fs, fs::File, io::Write, path::Path};
use walkdir::WalkDir;

/// 运行命令行工具
pub fn run() {
    let cli = Cli::parse();

    // 处理版本信息
    if cli.version {
        println!("Eleventy v0.0.0");
        return;
    }

    // 处理帮助信息
    if cli.help {
        println!("Eleventy static site generator");
        println!("Usage: eleventy [OPTIONS] [COMMAND]");
        println!("\nOptions:");
        println!("  -v, --version           Show version information");
        println!("  -h, --help              Show help information");
        println!("  -c, --config            Path to config file (default: .eleventy.js)");
        println!("      --input             Input directory (default: .)");
        println!("      --output            Output directory (default: _site)");
        println!("      --formats           Template formats to process");
        println!("      --serve             Start a development server");
        println!("      --port              Server port (default: 8080)");
        println!("      --watch             Watch for changes");
        println!("      --quiet             Reduce console output");
        println!("      --dryrun            Run without writing to file system");
        println!("      --to                Output format (fs, json, ndjson) (default: fs)");
        println!("      --incremental       Enable incremental builds");
        println!("      --ignore-initial    Skip initial build");
        println!("\nCommands:");
        println!("  build            Build the site");
        println!("  serve            Start a development server");
        println!("  watch            Watch for changes");
        println!("  help             Show help information");
        println!("  version          Show version information");
        return;
    }

    // 加载配置
    let mut config = match Config::from_file(&cli.config) {
        Ok(config) => config,
        Err(e) => {
            println!("Error loading config file: {:?}", e);
            println!("Using default configuration");
            Config::default()
        }
    };

    // 只有当命令行显式指定了输入或输出目录时，才覆盖配置文件中的值
    if let Some(input) = cli.input {
        config.input_dir = input;
    }
    if let Some(output) = cli.output {
        config.output_dir = output;
    }

    // 处理命令
    match cli.command {
        Some(crate::types::Command::Build { input, output, verbose }) => {
            // 处理命令级别的输入输出目录覆盖
            let mut build_config = config;
            if let Some(input) = input {
                build_config.input_dir = input;
            }
            if let Some(output) = output {
                build_config.output_dir = output;
            }

            println!("Building site...");
            println!("Input: {}", build_config.input_dir);
            println!("Output: {}", build_config.output_dir);
            println!("Verbose: {:?}", verbose);
            println!("Config: {:?}", cli.config);
            println!("Formats: {:?}", cli.formats);
            println!("Quiet: {:?}", cli.quiet);
            println!("Dryrun: {:?}", cli.dryrun);
            println!("To: {:?}", cli.to);
            println!("Incremental: {:?}", cli.incremental);

            // 执行构建
            if let Err(err) = execute_build(&build_config, verbose) {
                eprintln!("Build failed: {:?}", err);
            }
        }
        Some(crate::types::Command::Serve { port, input, output, verbose }) => {
            // 处理命令级别的输入输出目录覆盖
            let mut serve_config = config;
            if let Some(input) = input {
                serve_config.input_dir = input;
            }
            if let Some(output) = output {
                serve_config.output_dir = output;
            }

            let server_port = port.unwrap_or(cli.port);
            println!("Starting dev server on port {}", server_port);
            println!("Input: {}", serve_config.input_dir);
            println!("Output: {}", serve_config.output_dir);
            println!("Verbose: {:?}", verbose);
            println!("Config: {:?}", cli.config);
            println!("Formats: {:?}", cli.formats);
            println!("Quiet: {:?}", cli.quiet);
            println!("Incremental: {:?}", cli.incremental);
            println!("Ignore initial: {:?}", cli.ignore_initial);

            // 简单的服务器实现
            println!("Server system not implemented yet");
        }
        Some(crate::types::Command::Watch { input, output, verbose }) => {
            // 处理命令级别的输入输出目录覆盖
            let mut watch_config = config;
            if let Some(input) = input {
                watch_config.input_dir = input;
            }
            if let Some(output) = output {
                watch_config.output_dir = output;
            }

            println!("Watching files...");
            println!("Input: {}", watch_config.input_dir);
            println!("Output: {}", watch_config.output_dir);
            println!("Verbose: {:?}", verbose);
            println!("Config: {:?}", cli.config);
            println!("Formats: {:?}", cli.formats);
            println!("Quiet: {:?}", cli.quiet);
            println!("Incremental: {:?}", cli.incremental);
            println!("Ignore initial: {:?}", cli.ignore_initial);
            // 监视实现
        }
        Some(crate::types::Command::Help) => {
            println!("Eleventy static site generator");
            println!("Usage: eleventy [OPTIONS] [COMMAND]");
            println!("\nOptions:");
            println!("  -v, --version           Show version information");
            println!("  -h, --help              Show help information");
            println!("  -c, --config            Path to config file (default: .eleventy.js)");
            println!("      --input             Input directory (default: .)");
            println!("      --output            Output directory (default: _site)");
            println!("      --formats           Template formats to process");
            println!("      --serve             Start a development server");
            println!("      --port              Server port (default: 8080)");
            println!("      --watch             Watch for changes");
            println!("      --quiet             Reduce console output");
            println!("      --dryrun            Run without writing to file system");
            println!("      --to                Output format (fs, json, ndjson) (default: fs)");
            println!("      --incremental       Enable incremental builds");
            println!("      --ignore-initial    Skip initial build");
            println!("\nCommands:");
            println!("  build            Build the site");
            println!("  serve            Start a development server");
            println!("  watch            Watch for changes");
            println!("  help             Show help information");
            println!("  version          Show version information");
        }
        Some(crate::types::Command::Version) => {
            println!("Eleventy v0.0.0");
        }
        None => {
            // 没有指定命令时，根据选项执行相应操作
            if cli.serve {
                println!("Starting dev server on port {}", cli.port);
                println!("Input: {}", config.input_dir);
                println!("Output: {}", config.output_dir);
                println!("Config: {:?}", cli.config);
                println!("Formats: {:?}", cli.formats);
                println!("Quiet: {:?}", cli.quiet);
                println!("Incremental: {:?}", cli.incremental);
                println!("Ignore initial: {:?}", cli.ignore_initial);

                // 简单的服务器实现
                println!("Server system not implemented yet");
            }
            else if cli.watch {
                println!("Watching files...");
                println!("Input: {}", config.input_dir);
                println!("Output: {}", config.output_dir);
                println!("Config: {:?}", cli.config);
                println!("Formats: {:?}", cli.formats);
                println!("Quiet: {:?}", cli.quiet);
                println!("Incremental: {:?}", cli.incremental);
                println!("Ignore initial: {:?}", cli.ignore_initial);
                // 监视实现
            }
            else {
                // 默认执行构建
                println!("Building site...");
                println!("Input: {}", config.input_dir);
                println!("Output: {}", config.output_dir);
                println!("Config: {:?}", cli.config);
                println!("Formats: {:?}", cli.formats);
                println!("Quiet: {:?}", cli.quiet);
                println!("Dryrun: {:?}", cli.dryrun);
                println!("To: {:?}", cli.to);
                println!("Incremental: {:?}", cli.incremental);

                // 执行构建
                if let Err(err) = execute_build(&config, false) {
                    eprintln!("Build failed: {:?}", err);
                }
            }
        }
    }
}

/// 执行构建流程
fn execute_build(config: &Config, verbose: bool) -> Result<(), Box<dyn std::error::Error>> {
    // 1. 初始化模板引擎
    let mut engine_factory = TemplateEngineFactory::new();
    engine_factory.register_engine("liquid", Box::new(SimpleTemplateEngine::new()));
    engine_factory.register_engine("njk", Box::new(SimpleTemplateEngine::new()));
    engine_factory.register_engine("md", Box::new(SimpleTemplateEngine::new()));

    // 2. 处理文件
    println!("🔍 Finding and processing files...");
    let input_path = Path::new(&config.input_dir);
    if !input_path.exists() {
        return Err(Box::new(std::io::Error::new(std::io::ErrorKind::NotFound, "Input directory not found")));
    }

    // 3. 创建输出目录
    let output_path = Path::new(&config.output_dir);
    if !output_path.exists() {
        fs::create_dir_all(output_path)?;
    }

    // 4. 处理所有模板文件
    for entry in WalkDir::new(input_path).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.is_file() {
            if let Some(ext) = path.extension() {
                let ext_str = ext.to_str().unwrap_or("");
                if matches!(ext_str, "liquid" | "njk" | "md" | "html") {
                    // 读取文件内容
                    let content = fs::read_to_string(path)?;

                    // 准备渲染数据
                    let data = serde_json::json!({
                        "site": {
                            "title": "Eleventy Site",
                            "description": "A static site generated with Eleventy"
                        },
                        "page": {
                            "title": "Page Title",
                            "url": "/"
                        }
                    });

                    // 渲染模板
                    let engine_name = match ext_str {
                        "liquid" => "liquid",
                        "njk" => "njk",
                        "md" => "md",
                        _ => "liquid",
                    };

                    let rendered = engine_factory.render(engine_name, &content, &data)?;

                    // 生成输出文件路径
                    let relative_path = path.strip_prefix(input_path)?;
                    let output_file_path = output_path.join(relative_path).with_extension("html");

                    // 创建输出目录
                    if let Some(parent) = output_file_path.parent() {
                        fs::create_dir_all(parent)?;
                    }

                    // 写入输出文件
                    let mut file = File::create(output_file_path)?;
                    file.write_all(rendered.as_bytes())?;

                    if verbose {
                        println!("Processed: {}", path.display());
                    }
                }
            }
        }
    }

    // 5. 复制静态资源
    let static_dir = input_path.join("_static");
    if static_dir.exists() {
        let static_out_dir = output_path.join("static");
        fs::create_dir_all(&static_out_dir)?;

        for entry in WalkDir::new(&static_dir).into_iter().filter_map(|e| e.ok()) {
            let path = entry.path();
            if path.is_file() {
                let relative_path = path.strip_prefix(&static_dir)?;
                let dest_path = static_out_dir.join(relative_path);

                if let Some(parent) = dest_path.parent() {
                    fs::create_dir_all(parent)?;
                }

                fs::copy(path, dest_path)?;
            }
        }
    }

    println!("✅ Build completed successfully!");
    Ok(())
}
