//! 命令行工具模块

use clap::Parser;
use crate::config::Config;
use crate::types::Cli;

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
        println!("  -v, --version    Show version information");
        println!("  -h, --help       Show help information");
        println!("  -c, --config     Path to config file (default: .eleventy.js)");
        println!("\nCommands:");
        println!("  build            Build the site");
        println!("  serve            Start a development server");
        println!("  watch            Watch for changes");
        println!("  help             Show help information");
        println!("  version          Show version information");
        return;
    }
    
    // 加载配置
    let config = match Config::from_file(&cli.config) {
        Ok(config) => config,
        Err(e) => {
            println!("Error loading config file: {:?}", e);
            println!("Using default configuration");
            Config::default()
        }
    };
    
    // 处理命令
    match cli.command {
        Some(crate::types::Command::Build { input, output, verbose }) => {
            println!("Building site...");
            println!("Input: {:?}", input.unwrap_or_else(|| config.input_dir.clone()));
            println!("Output: {:?}", output.unwrap_or_else(|| config.output_dir.clone()));
            println!("Verbose: {:?}", verbose);
            println!("Config: {:?}", cli.config);
            
            // 创建构建系统
            let mut build_system = crate::build::BuildSystem::new(config);
            
            // 注册简单模板引擎
            build_system.register_template_engine("simple", Box::new(crate::compiler::template_engine::SimpleTemplateEngine::new()));
            
            // 执行构建
            match build_system.build() {
                Ok(_) => println!("Build completed successfully"),
                Err(e) => println!("Build failed: {:?}", e),
            }
        }
        Some(crate::types::Command::Serve { port, input, output, verbose }) => {
            println!("Starting dev server on port {}", port);
            println!("Input: {:?}", input.unwrap_or_else(|| config.input_dir.clone()));
            println!("Output: {:?}", output.clone().unwrap_or_else(|| config.output_dir.clone()));
            println!("Verbose: {:?}", verbose);
            println!("Config: {:?}", cli.config);
            
            // 创建构建系统
            let build_system = crate::build::BuildSystem::new(config);
            
            // 注册简单模板引擎
            let mut build_system = build_system;
            build_system.register_template_engine("simple", Box::new(crate::compiler::template_engine::SimpleTemplateEngine::new()));
            
            // 创建服务器
            let output_dir = output.unwrap_or_else(|| build_system.config.output_dir.clone());
            let mut server = crate::server::Server::new(port, &output_dir, build_system);
            
            // 启动服务器
            tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap()
                .block_on(async {
                    match server.start().await {
                        Ok(_) => println!("Server stopped"),
                        Err(e) => println!("Server failed: {:?}", e),
                    }
                });
        }
        Some(crate::types::Command::Watch { input, output, verbose }) => {
            println!("Watching files...");
            println!("Input: {:?}", input.unwrap_or_else(|| config.input_dir.clone()));
            println!("Output: {:?}", output.unwrap_or_else(|| config.output_dir.clone()));
            println!("Verbose: {:?}", verbose);
            println!("Config: {:?}", cli.config);
            // 监视实现
        }
        Some(crate::types::Command::Help) => {
            println!("Eleventy static site generator");
            println!("Usage: eleventy [OPTIONS] [COMMAND]");
            println!("\nOptions:");
            println!("  -v, --version    Show version information");
            println!("  -h, --help       Show help information");
            println!("  -c, --config     Path to config file (default: .eleventy.js)");
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
            // 没有指定命令时，默认执行构建
            println!("Building site...");
            println!("Config: {:?}", cli.config);
            println!("Input directory: {}", config.input_dir);
            println!("Output directory: {}", config.output_dir);
            // 默认构建实现
        }
    }
}
