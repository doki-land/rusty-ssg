//! 部署器

use serde_json::Value;
use std::path::PathBuf;

use crate::types::Result;

/// 导入策略结构体
pub use crate::deploy::strategies::{GitStrategy, GithubPagesStrategy, HerokuStrategy, NetlifyStrategy};

/// 部署策略枚举
pub enum DeployStrategy {
    /// GitHub Pages 策略
    GithubPages(GithubPagesStrategy),
    /// Heroku 策略
    Heroku(HerokuStrategy),
    /// Netlify 策略
    Netlify(NetlifyStrategy),
    /// Git 策略
    Git(GitStrategy),
}

impl DeployStrategy {
    /// 部署
    pub async fn deploy(&self, public_dir: &PathBuf, config: &Value) -> Result<()> {
        match self {
            DeployStrategy::GithubPages(strategy) => strategy.deploy(public_dir, config).await,
            DeployStrategy::Heroku(strategy) => strategy.deploy(public_dir, config).await,
            DeployStrategy::Netlify(strategy) => strategy.deploy(public_dir, config).await,
            DeployStrategy::Git(strategy) => strategy.deploy(public_dir, config).await,
        }
    }

    /// 获取策略名称
    pub fn name(&self) -> &str {
        match self {
            DeployStrategy::GithubPages(_) => "github_pages",
            DeployStrategy::Heroku(_) => "heroku",
            DeployStrategy::Netlify(_) => "netlify",
            DeployStrategy::Git(_) => "git",
        }
    }
}

/// 部署器
pub struct Deployer {
    /// 部署策略
    strategies: Vec<DeployStrategy>,
}

impl Deployer {
    /// 创建部署器
    pub fn new() -> Self {
        Self { strategies: vec![] }
    }

    /// 添加部署策略
    pub fn add_strategy(&mut self, strategy: DeployStrategy) {
        self.strategies.push(strategy);
    }

    /// 执行部署
    pub async fn deploy(&self, public_dir: &PathBuf, config: &Value) -> Result<()> {
        for strategy in &self.strategies {
            strategy.deploy(public_dir, config).await?;
        }
        Ok(())
    }
}
