#![warn(missing_docs)]
#![doc = "Rusty Hexo 博客框架 CLI 入口点"]

use clap::Parser;
use hexo::{Commands, HexoCli, tools::cmd::*, types::Result};

/// 主函数入口
#[tokio::main]
async fn main() -> Result<()> {
    let cli = HexoCli::parse();

    match cli.command {
        Commands::Init(args) => {
            InitCommand::execute(args).await?;
        }
        Commands::New(args) => {
            NewCommand::execute(args).await?;
        }
        Commands::Generate(args) => {
            GenerateCommand::execute(args).await?;
        }
        Commands::Server(args) => {
            ServerCommand::execute(args).await?;
        }
        Commands::Deploy(args) => {
            DeployCommand::execute(args).await?;
        }
        Commands::Clean(args) => {
            CleanCommand::execute(args).await?;
        }
        Commands::Plugin(args) => {
            PluginCommand::execute(args).await?;
        }
    }

    Ok(())
}
