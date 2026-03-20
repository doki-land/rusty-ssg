#![warn(missing_docs)]
#![doc = "Hugo 兼容静态站点生成器 CLI 入口点"]

use clap::Parser;
use hugo::{
    HugoCli, HugoCommands,
    tools::cmd::{BuildCommand, CheckCommand, InitCommand, NewCommand, VersionCommand},
    types::Result,
};

#[cfg(feature = "dev")]
use hugo::tools::cmd::ServerCommand;

/// 主函数入口
#[tokio::main]
async fn main() -> Result<()> {
    let cli = HugoCli::parse();

    match cli.command {
        HugoCommands::New(args) => {
            NewCommand::execute(args).await?;
        }
        HugoCommands::Build(args) => {
            BuildCommand::execute(args).await?;
        }
        #[cfg(feature = "dev")]
        HugoCommands::Server(args) => {
            ServerCommand::execute(args).await?;
        }
        HugoCommands::Version => {
            VersionCommand::execute().await;
        }
        HugoCommands::Init(args) => {
            InitCommand::execute(args).await?;
        }
        HugoCommands::Check(args) => {
            CheckCommand::execute(args).await?;
        }
    }

    Ok(())
}
