//! Dev 命令实现

#[cfg(feature = "dev")]
use crate::DevArgs;
use console::style;
use std::{fs, path::PathBuf, sync::Arc, net::SocketAddr};
use crate::types::Result;
use wae_https::{Router, HttpsServerBuilder, static_files_router};
use http;
#[cfg(feature = "dev")]
use notify::{RecommendedWatcher, RecursiveMode, Watcher, Event, EventKind};
#[cfg(feature = "dev")]
use tokio::sync::mpsc;

/// Dev 命令
#[cfg(feature = "dev")]
pub struct DevCommand;

#[cfg(feature = "dev")]
impl DevCommand {
    /// 执行 dev 命令
    pub async fn execute(args: DevArgs) -> Result<()> {
        println!("{}", style("Starting Jekyll development server...").cyan());

        let source_dir = args.source.unwrap_or_else(|| PathBuf::from("."));
        let port = args.port;

        println!("  Source directory: {}", source_dir.display());
        println!("  Port: {}", port);

        // 构建输出目录
        let output_dir = source_dir.join("_site");
        if !output_dir.exists() {
            fs::create_dir_all(&output_dir)?;
            println!("  {} Created _site directory", style("✓").green());
        }

        // 初始构建
        println!("  {} Initial build...", style("→").blue());
        if let Err(e) = Self::build_site(&source_dir, &output_dir).await {
            println!("  {} Initial build failed: {}", style("✗").red(), e);
        } else {
            println!("  {} Initial build completed", style("✓").green());
        }

        // 设置文件监视
        let (tx, mut rx) = mpsc::channel(1);
        let watcher = Self::setup_watcher(&source_dir, tx)?;

        // 初始化路由
        let mut router = Router::new();
        router.add_route(http::Method::GET, "/", Self::index_handler);
        
        // 添加静态文件服务
        let static_router = static_files_router(&output_dir, "");
        let router = router.merge(static_router);

        // 启动服务器
        let addr: SocketAddr = format!("0.0.0.0:{}", port).parse()?;
        println!("  {} Development server starting at http://localhost:{}", style("→").blue(), port);
        println!("  {} Press Ctrl+C to stop the server", style("ℹ").blue());

        // 启动文件监视任务
        let source_dir_clone = source_dir.clone();
        let output_dir_clone = output_dir.clone();
        tokio::spawn(async move {
            while let Some(event) = rx.recv().await {
                Self::handle_file_change(event, &source_dir_clone, &output_dir_clone).await;
            }
        });

        let server = HttpsServerBuilder::new()
            .addr(addr)
            .router(router)
            .build();

        server.serve().await
            .map_err(|e| crate::types::Error::new(crate::types::ErrorKind::Other, e.to_string()))?;

        Ok(())
    }

    /// 构建站点
    async fn build_site(source_dir: &PathBuf, output_dir: &PathBuf) -> Result<()>
    {
        use crate::jekyll::{JekyllStructure, JekyllConfigLoader, PostManager};

        let structure = JekyllStructure::new(source_dir)?;
        let config = JekyllConfigLoader::load_from_dir(source_dir)?;
        let mut post_manager = PostManager::new(structure, config);
        post_manager.load_posts()?;

        // 这里可以添加更多的构建逻辑

        Ok(())
    }

    /// 设置文件监视
    fn setup_watcher(source_dir: &PathBuf, tx: mpsc::Sender<notify::Result<Event>>) -> Result<RecommendedWatcher>
    {
        let mut watcher = notify::recommended_watcher(move |res| {
            tx.blocking_send(res).unwrap();
        })?;

        // 监视源目录
        watcher.watch(source_dir, RecursiveMode::Recursive)?;

        Ok(watcher)
    }

    /// 处理文件变化
    async fn handle_file_change(event: notify::Result<Event>, source_dir: &PathBuf, output_dir: &PathBuf)
    {
        match event {
            Ok(event) => {
                if event.kind.is_modify() || event.kind.is_create() || event.kind.is_remove() {
                    println!("  {} File changed, rebuilding...", style("→").blue());
                    if let Err(e) = Self::build_site(source_dir, output_dir).await {
                        println!("  {} Rebuild failed: {}", style("✗").red(), e);
                    } else {
                        println!("  {} Rebuild completed", style("✓").green());
                    }
                }
            },
            Err(e) => {
                println!("  {} Watcher error: {}", style("⚠").yellow(), e);
            }
        }
    }

    /// 索引页面处理器
    fn index_handler() -> http::Response<wae_https::Body>
    {
        let content = r#"
        <!DOCTYPE html>
        <html>
        <head>
            <title>Jekyll Development Server</title>
            <style>
                body {
                    font-family: Arial, sans-serif;
                    line-height: 1.6;
                    margin: 0;
                    padding: 2rem;
                    background-color: #f4f4f4;
                }
                .container {
                    max-width: 800px;
                    margin: 0 auto;
                    background: white;
                    padding: 2rem;
                    border-radius: 8px;
                    box-shadow: 0 2px 4px rgba(0,0,0,0.1);
                }
                h1 {
                    color: #333;
                }
                p {
                    color: #666;
                }
                .info {
                    background: #e3f2fd;
                    padding: 1rem;
                    border-radius: 4px;
                    margin-top: 1rem;
                }
            </style>
        </head>
        <body>
            <div class="container">
                <h1>Jekyll Development Server</h1>
                <p>Welcome to the Rusty Jekyll development server!</p>
                <div class="info">
                    <p><strong>Status:</strong> Server is running</p>
                    <p><strong>Features:</strong></p>
                    <ul>
                        <li>Automatic rebuild on file changes</li>
                        <li>Live preview</li>
                        <li>Static file serving</li>
                    </ul>
                </div>
            </div>
        </body>
        </html>
        "#;
        http::Response::builder()
            .status(http::StatusCode::OK)
            .header(http::header::CONTENT_TYPE, "text/html; charset=utf-8")
            .body(wae_https::full_body(content))
            .unwrap()
    }
}
