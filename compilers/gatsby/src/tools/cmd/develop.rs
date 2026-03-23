//! Develop 命令实现 - 开发服务器

use crate::{DevelopArgs, GatsbyConfig, types::Result};
use console::style;
use std::path::PathBuf;

use super::super::server::DevServer;

/// Develop 命令
#[cfg(feature = "dev")]
pub struct DevelopCommand;

#[cfg(feature = "dev")]
impl DevelopCommand {
    /// 执行 develop 命令
    pub async fn execute(args: DevelopArgs) -> Result<()> {
        let source_dir = args.source.unwrap_or_else(|| PathBuf::from("."));
        let output_dir = PathBuf::from(args.output.unwrap_or_else(|| "public".to_string()));

        // 加载配置
        let config = GatsbyConfig::load_from_dir(&source_dir)?;

        // 创建开发服务器
        let server = DevServer::new(config, source_dir, output_dir, args.bind, args.port, !args.no_browser);

        // 启动服务器
        server.start().await
    }
}
