//! 命令行工具模块

use crate::{config::Config, types::Cli};
use clap::Parser;

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

            // 创建构建系统
            let mut build_system = crate::build::BuildSystem::new(build_config);

            // 注册简单模板引擎
            build_system
                .register_template_engine("simple", Box::new(crate::compiler::template_engine::SimpleTemplateEngine::new()));

            // 执行构建
            match build_system.build() {
                Ok(_) => println!("Build completed successfully"),
                Err(e) => println!("Build failed: {:?}", e),
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

            // 创建构建系统
            let build_system = crate::build::BuildSystem::new(serve_config);

            // 注册简单模板引擎
            let mut build_system = build_system;
            build_system
                .register_template_engine("simple", Box::new(crate::compiler::template_engine::SimpleTemplateEngine::new()));

            // 创建服务器
            let output_dir = build_system.config.output_dir.clone();
            let mut server = crate::server::Server::new(server_port, &output_dir, build_system);

            // 启动服务器
            tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap().block_on(async {
                match server.start().await {
                    Ok(_) => println!("Server stopped"),
                    Err(e) => println!("Server failed: {:?}", e),
                }
            });
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
        Some(crate::types::Command::Test) => {
            println!("Testing data system...");

            // 创建数据系统
            let mut data_system = crate::data::DataSystem::new("./_data");

            // 测试 frontmatter 解析
            println!("\nTesting frontmatter parsing...");
            let content = include_str!("../../../test-data.md");
            match data_system.load_frontmatter(content) {
                Ok((frontmatter, content)) => {
                    println!("Successfully parsed frontmatter");
                    println!("Frontmatter: {:?}", frontmatter);
                    println!("Content: {}", content);
                }
                Err(e) => {
                    println!("Error parsing frontmatter: {:?}", e);
                }
            }

            // 测试模板数据加载
            println!("\nTesting template data loading...");
            match data_system.load_template_data("test-data.md") {
                Ok(data) => {
                    println!("Successfully loaded template data");
                    println!("Template data: {:?}", data);
                }
                Err(e) => {
                    println!("Error loading template data: {:?}", e);
                }
            }

            // 测试全局数据加载
            println!("\nTesting global data loading...");
            match data_system.load_global_data() {
                Ok(_) => {
                    println!("Successfully loaded global data");
                    println!("Global data: {:?}", data_system.global_data());
                }
                Err(e) => {
                    println!("Error loading global data: {:?}", e);
                }
            }
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

                // 创建构建系统
                let build_system = crate::build::BuildSystem::new(config);

                // 注册简单模板引擎
                let mut build_system = build_system;
                build_system.register_template_engine(
                    "simple",
                    Box::new(crate::compiler::template_engine::SimpleTemplateEngine::new()),
                );

                // 创建服务器
                let output_dir = build_system.config.output_dir.clone();
                let mut server = crate::server::Server::new(cli.port, &output_dir, build_system);

                // 启动服务器
                tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap().block_on(async {
                    match server.start().await {
                        Ok(_) => println!("Server stopped"),
                        Err(e) => println!("Server failed: {:?}", e),
                    }
                });
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

                // 创建构建系统
                let mut build_system = crate::build::BuildSystem::new(config);

                // 注册简单模板引擎
                build_system.register_template_engine(
                    "simple",
                    Box::new(crate::compiler::template_engine::SimpleTemplateEngine::new()),
                );

                // 执行构建
                match build_system.build() {
                    Ok(_) => println!("Build completed successfully"),
                    Err(e) => println!("Build failed: {:?}", e),
                }
            }
        }
    }
}
