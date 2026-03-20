//! 部署命令实现

use super::super::DeployArgs;
use crate::{
    deploy::{Deployer, strategies},
    types::Result,
};
use std::path::PathBuf;

/// 部署命令
pub struct DeployCommand;

impl DeployCommand {
    /// 执行部署命令
    pub async fn execute(args: DeployArgs) -> Result<()> {
        println!("Deploying blog...");

        if let Some(env) = args.env {
            println!("Deploying to {} environment", env);
        }

        // 创建部署器
        let mut deployer = Deployer::new();

        // 添加部署策略
        // 这里可以根据配置文件添加不同的部署策略
        deployer.add_strategy(crate::deploy::DeployStrategy::GithubPages(strategies::GithubPagesStrategy::new(
            "https://github.com/username/blog.git",
            "gh-pages",
        )));

        deployer.add_strategy(crate::deploy::DeployStrategy::Netlify(strategies::NetlifyStrategy::new("my-blog")));

        // 执行部署
        let public_dir = PathBuf::from("public");
        deployer.deploy(&public_dir, &serde_json::Value::Null).await?;

        println!("Deployed successfully!");
        Ok(())
    }
}
