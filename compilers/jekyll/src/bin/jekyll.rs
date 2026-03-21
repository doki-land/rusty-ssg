/// Rusty Jekyll 命令行工具
///
/// 提供与官方 Jekyll 兼容的命令行接口，包括 build、dev、init、check 等命令
use jekyll::tools::{Commands, JekyllCli};

#[tokio::main]
async fn main() {
    let cli = JekyllCli::parse();

    let result = match cli.command {
        Commands::Build(args) => {
            jekyll::tools::cmd::build::execute(args).await
        }
        #[cfg(feature = "dev")]
        Commands::Dev(args) => {
            jekyll::tools::cmd::dev::execute(args).await
        }
        Commands::Init(args) => {
            jekyll::tools::cmd::init::execute(args).await
        }
        Commands::Check(args) => {
            jekyll::tools::cmd::check::execute(args).await
        }
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
