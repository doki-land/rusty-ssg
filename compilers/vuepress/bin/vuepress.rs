#![warn(missing_docs)]
#![doc = "VuTeX 文档系统 CLI 入口点"]

use clap::Parser;
use vuepress::types::Result;

// 直接从工具模块导入必要的类型
use vuepress::tools::VutexCli;
use vuepress::tools::Commands;
use vuepress::tools::cmd::BuildCommand;
use vuepress::tools::cmd::CheckCommand;
use vuepress::tools::cmd::InitCommand;

#[cfg(feature = "dev")]
use vuepress::tools::cmd::DevCommand;

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
