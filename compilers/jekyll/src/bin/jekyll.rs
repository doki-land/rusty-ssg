/// Rusty Jekyll 命令行工具
///
/// 提供与官方 Jekyll 兼容的命令行接口，包括 build、serve、init 等命令
use clap::{Parser, Subcommand};
use jekyll::tools::{BuildArgs, CheckArgs, Commands, DevArgs, InitArgs, JekyllCli};
use std::path::PathBuf;

fn main() {
    let cli = JekyllCli::parse();

    match cli.command {
        Commands::Build(args) => {
            if let Err(e) = jekyll::tools::cmd::build::execute(args) {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        }
        Commands::Dev(args) => {
            if let Err(e) = jekyll::tools::cmd::dev::execute(args) {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        }
        Commands::Init(args) => {
            if let Err(e) = jekyll::tools::cmd::init::execute(args) {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        }
        Commands::Check(args) => {
            if let Err(e) = jekyll::tools::cmd::check::execute(args) {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        }
    }
}
