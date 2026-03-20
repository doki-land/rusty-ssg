#![warn(missing_docs)]
#![doc = "Gatsby 兼容静态站点生成器 CLI 入口点"]

use clap::Parser;
use gatsby::{
    GatsbyCli, GatsbyCommands,
    tools::cmd::{BuildCommand, CheckCommand, InitCommand, NewCommand, VersionCommand},
    types::Result,
};

#[cfg(feature = "dev")]
use gatsby::tools::cmd::DevelopCommand;

/// 主函数入口
#[tokio::main]
async fn main() -> Result<()> {
    let cli = GatsbyCli::parse();

    match cli.command {
        GatsbyCommands::New(args) => {
            NewCommand::execute(args).await?;
        }
        GatsbyCommands::Build(args) => {
            BuildCommand::execute(args).await?;
        }
        #[cfg(feature = "dev")]
        GatsbyCommands::Develop(args) => {
            DevelopCommand::execute(args).await?;
        }
        GatsbyCommands::Version => {
            VersionCommand::execute().await;
        }
        GatsbyCommands::Init(args) => {
            InitCommand::execute(args).await?;
        }
        GatsbyCommands::Check(args) => {
            CheckCommand::execute(args).await?;
        }
    }

    Ok(())
}
