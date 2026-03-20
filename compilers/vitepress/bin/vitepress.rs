#![warn(missing_docs)]
#![doc = "VuTeX 文档系统 CLI 入口点"]

use clap::Parser;
use vitepress::{
    tools::{Commands, VutexCli},
    tools::cmd::{BuildCommand, CheckCommand, InitCommand},
    types::Result,
};

#[cfg(feature = "dev")]
use vitepress::tools::cmd::DevCommand;

/// 主函数入口
#[tokio::main]
async fn main() -> Result<()> {
    let cli = VutexCli::parse();

    match cli.command {
        Commands::Build(args) => {
            BuildCommand::execute(args).await?;
        }
        #[cfg(feature = "dev")]
        Commands::Dev(args) => {
            DevCommand::execute(args).await?;
        }
        Commands::Init(args) => {
            InitCommand::execute(args).await?;
        }
        Commands::Check(args) => {
            CheckCommand::execute(args).await?;
        }
    }

    Ok(())
}
