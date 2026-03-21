//! 服务器模块

use std::{fs, path::Path, sync::Arc, time::Duration};

use axum::{
    Router,
    response::{Html, IntoResponse},
    routing::get,
};
use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use tower_http::services::ServeDir;

/// 服务器错误类型
#[derive(Debug)]
pub enum ServerError {
    /// 服务器启动错误
    StartError(String),
    /// 文件监听错误
    WatchError(String),
    /// 构建错误
    BuildError(crate::build::BuildError),
}

/// 服务器
pub struct Server {
    /// 端口
    port: u16,

    /// 输出目录
    output_dir: String,

    /// 构建系统
    build_system: Arc<tokio::sync::Mutex<crate::build::BuildSystem>>,
}

impl Server {
    /// 创建新的服务器
    pub fn new(port: u16, output_dir: &str, build_system: crate::build::BuildSystem) -> Self {
        Self { port, output_dir: output_dir.to_string(), build_system: Arc::new(tokio::sync::Mutex::new(build_system)) }
    }

    /// 启动服务器
    pub async fn start(&mut self) -> Result<(), ServerError> {
        // 确保输出目录存在
        if !Path::new(&self.output_dir).exists() {
            fs::create_dir_all(&self.output_dir).map_err(|e| ServerError::StartError(e.to_string()))?;
        }

        // 初始构建
        self.build().await?;

        // 启动文件监听
        self.start_watcher().await?;

        // 创建路由
        let app = self.create_router();

        // 启动服务器
        let address = format!("127.0.0.1:{}", self.port);
        println!("Server started at http://{}", address);

        let listener = tokio::net::TcpListener::bind(&address).await.map_err(|e| ServerError::StartError(e.to_string()))?;
        axum::serve(listener, app.into_make_service()).await.map_err(|e| ServerError::StartError(e.to_string()))?;

        Ok(())
    }

    /// 创建路由
    fn create_router(&self) -> Router {
        let output_dir = self.output_dir.clone();

        Router::new().route("/", get(root_handler)).nest_service("/", ServeDir::new(output_dir))
    }

    /// 构建站点
    async fn build(&self) -> Result<(), ServerError> {
        let mut build_system = self.build_system.lock().await;
        build_system.build().map_err(ServerError::BuildError)?;
        Ok(())
    }

    /// 启动文件监听
    async fn start_watcher(&self) -> Result<(), ServerError> {
        let (tx, rx) = std::sync::mpsc::channel();
        let config = notify::Config::default();
        let mut watcher: RecommendedWatcher =
            RecommendedWatcher::new(tx, config).map_err(|e| ServerError::WatchError(e.to_string()))?;

        // 监听输入目录
        let build_system = self.build_system.clone();
        let output_dir = self.output_dir.clone();

        tokio::spawn(async move {
            // 监听当前目录
            watcher.watch(Path::new("."), RecursiveMode::Recursive).unwrap();

            loop {
                match rx.recv() {
                    Ok(_) => {
                        println!("File changed, rebuilding...");

                        let mut build_system = build_system.lock().await;
                        match build_system.build() {
                            Ok(_) => println!("Rebuild completed successfully"),
                            Err(e) => println!("Rebuild failed: {:?}", e),
                        }
                    }
                    Err(_) => break,
                }
            }
        });

        Ok(())
    }
}

/// 根路径处理函数
async fn root_handler() -> impl IntoResponse {
    let content = r#"
    <!DOCTYPE html>
    <html>
    <head>
        <title>Eleventy Server</title>
    </head>
    <body>
        <h1>Eleventy Server</h1>
        <p>Server is running. Visit <a href="/index.html">index.html</a> to see your site.</p>
    </body>
    </html>
    "#;

    Html(content)
}
