//! 服务器命令实现

use super::super::ServerArgs;
use crate::types::Result;
use http;
use std::{net::SocketAddr, path::PathBuf};
use wae_https::{HttpsServerBuilder, Router, static_files_router};

/// 服务器命令
pub struct ServerCommand;

impl ServerCommand {
    /// 执行服务器命令
    pub async fn execute(args: ServerArgs) -> Result<()> {
        // 确定静态文件目录
        let public_dir = PathBuf::from("public");

        // 创建路由
        let router = static_files_router(&public_dir, "");

        // 构建地址
        let addr: SocketAddr = format!("{}:{}", args.host, args.port)
            .parse()
            .map_err(|e: std::net::AddrParseError| crate::types::HexoError::CustomError { message: e.to_string() })?;

        println!("Server started at http://{}:{}", args.host, args.port);
        println!("Press Ctrl+C to stop the server");

        // 启动服务器
        let server = HttpsServerBuilder::new().addr(addr).router(router).build();

        server.serve().await.map_err(|e| crate::types::HexoError::CustomError { message: e.to_string() })?;

        Ok(())
    }
}
