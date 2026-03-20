//! 命令行工具模块

use eleventy::types::Cli;

/// 运行命令行工具
pub fn run() {
    let cli = Cli::parse();
    
    match cli.command {
        eleventy::types::Command::Build { input, output } => {
            println!("Building site...");
            println!("Input: {:?}", input);
            println!("Output: {:?}", output);
            // 构建实现
        }
        eleventy::types::Command::Serve { port, input } => {
            println!("Starting dev server on port {}", port);
            println!("Input: {:?}", input);
            // 服务实现
        }
        eleventy::types::Command::Watch { input } => {
            println!("Watching files...");
            println!("Input: {:?}", input);
            // 监视实现
        }
    }
}
