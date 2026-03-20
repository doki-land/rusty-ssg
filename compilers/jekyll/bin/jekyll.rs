#![warn(missing_docs)]
#![doc = "Jekyll 静态站点生成器 CLI 入口点"]

use clap::Parser;
use jekyll::{
    Commands, JekyllCli,
    tools::cmd::{BuildCommand, CheckCommand, InitCommand},
    types::Result,
};

#[cfg(feature = "dev")]
use jekyll::tools::cmd::DevCommand;

/// 主函数入口
#[tokio::main]
async fn main() -> Result<()> {
    let cli = JekyllCli::parse();

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
