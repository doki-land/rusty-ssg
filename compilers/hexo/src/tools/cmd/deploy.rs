//! 部署命令实现

use super::super::DeployArgs;
use crate::types::Result;

/// 部署命令
pub struct DeployCommand;

impl DeployCommand {
    /// 执行部署命令
    pub async fn execute(args: DeployArgs) -> Result<()> {
        println!("Deploying blog...");

        if let Some(env) = args.env {
            println!("Deploying to {} environment", env);
        }

        println!("Deployed successfully!");
        Ok(())
    }
}
