#![warn(missing_docs)]
#![doc = "MkDocs 文档系统 CLI 入口点"]

use clap::Parser;
use mkdocs::{
    MkDocsCli,
    MkDocsCommands,
    types::Result,
};

/// 主函数入口
#[tokio::main]
async fn main() -> Result<()> {
    let cli = MkDocsCli::parse();

    match cli.command {
        MkDocsCommands::New(args) => {
            mkdocs::NewCommand::execute(args).await?;
        }
        MkDocsCommands::Build(args) => {
            mkdocs::BuildCommand::execute(args).await?;
        }
        MkDocsCommands::Serve(args) => {
            mkdocs::ServeCommand::execute(args).await?;
        }
        MkDocsCommands::Version => {
            mkdocs::VersionCommand::execute().await?;
        }
        MkDocsCommands::Init(args) => {
            mkdocs::InitCommand::execute(args).await?;
        }
        MkDocsCommands::Check(args) => {
            mkdocs::CheckCommand::execute(args).await?;
        }
    }

    Ok(())
}
