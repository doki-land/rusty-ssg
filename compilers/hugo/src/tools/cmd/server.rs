//! Server 命令实现 - 开发服务器（支持热重载）

use crate::{
    ConfigLoader, VutexCompiler,
    types::{HugoConfig, Result},
};
use console::style;
use http;
#[cfg(feature = "dev")]
use notify::{Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use std::{collections::HashMap, fs, net::SocketAddr, path::PathBuf, sync::Arc};
use wae_https::{HttpsServerBuilder, Router, get};
use walkdir::WalkDir;

/// Server 命令
pub struct ServerCommand;

impl ServerCommand {
    /// 执行 server 命令
    #[cfg(feature = "dev")]
    pub async fn execute(args: crate::ServerArgs) -> Result<()> {
        println!("{}", style("Starting Hugo development server...").cyan());

        let source_dir = args.source.unwrap_or_else(|| PathBuf::from("."));
        let port = args.port;
        let bind = args.bind;

        println!("  Source directory: {}", source_dir.display());
        println!("  Server address: http://{}:{}", bind, port);

        // 初始构建
        let (documents, config) = Self::build_site(&source_dir).await?;
        let documents = Arc::new(documents);
        let config = Arc::new(config);

        // 创建路由
        let mut router = Router::new();
        router.add_route(http::Method::GET, "/", Self::handle_root);
        router.add_route(http::Method::GET, "/assets/*path", Self::handle_assets);
        router.add_route(http::Method::GET, "/*path", Self::handle_page);

        // 启动服务器
        let addr: SocketAddr = format!("{}:{}", bind, port).parse()?;
        let server = HttpsServerBuilder::new().addr(addr).router(router).build();

        println!("  {} Development server started", style("✓").green());
        println!("  {} Watching for changes...", style("→").blue());

        // 启动文件监听
        tokio::spawn(async move {
            Self::watch_files(&source_dir).await;
        });

        server.serve().await?;

        Ok(())
    }

    /// 构建站点
    async fn build_site(source_dir: &PathBuf) -> Result<(HashMap<String, crate::Document>, HugoConfig)> {
        println!("  {} Loading configuration...", style("→").blue());
        let config = ConfigLoader::load_from_dir(source_dir)?;
        println!("  {} Configuration loaded", style("✓").green());

        let mut content_map = HashMap::new();
        let mut file_count = 0;

        println!("  {} Scanning for Markdown files...", style("→").blue());

        for entry in WalkDir::new(source_dir) {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() {
                if let Some(ext) = path.extension() {
                    if ext == "md" {
                        let rel_path = path.strip_prefix(source_dir).unwrap_or(path).to_string_lossy().to_string();

                        let path_components: Vec<&str> = rel_path.split(std::path::MAIN_SEPARATOR).collect();
                        if path_components.iter().any(|&c| c == "node_modules" || c == ".git" || c == "dist" || c == ".vutex") {
                            continue;
                        }

                        file_count += 1;
                        let content = fs::read_to_string(path)?;
                        let normalized_path = rel_path.replace(std::path::MAIN_SEPARATOR, "/");

                        content_map.insert(normalized_path, content);
                    }
                }
            }
        }

        println!("  {} Found {} Markdown files", style("✓").green(), file_count);

        let mut documents = HashMap::new();

        if file_count > 0 {
            println!("  {} Compiling documents...", style("→").blue());

            let mut compiler = VutexCompiler::with_config(config.clone());
            let result = compiler.compile_batch(&content_map);

            documents = result.documents;
            let success = result.success;
            let compile_time_ms = result.compile_time_ms;
            let errors = result.errors;

            if success {
                println!("  {} Compiled {} documents in {}ms", style("✓").green(), documents.len(), compile_time_ms);
            }
            else {
                println!("  {} Compilation failed with {} errors", style("✗").red(), errors.len());

                for error in &errors {
                    println!("    {}", style(error).red());
                }
            }
        }
        else {
            println!("  {} No Markdown files found", style("⚠").yellow());
        }

        Ok((documents, config))
    }

    /// 处理根路径
    fn handle_root(_parts: wae_https::extract::RequestParts) -> http::Response<wae_https::Body> {
        http::Response::builder()
            .status(http::StatusCode::OK)
            .header(http::header::CONTENT_TYPE, "text/plain; charset=utf-8")
            .body(wae_https::full_body("Welcome to Hugo development server"))
            .unwrap()
    }

    /// 处理资源文件
    fn handle_assets(parts: wae_https::extract::RequestParts) -> http::Response<wae_https::Body> {
        let path = parts.path_params.iter().find(|(k, _)| k == "path").map(|(_, v)| v.as_str()).unwrap_or("");
        let response = format!("Asset: {}", path);
        http::Response::builder()
            .status(http::StatusCode::OK)
            .header(http::header::CONTENT_TYPE, "text/plain; charset=utf-8")
            .body(wae_https::full_body(response))
            .unwrap()
    }

    /// 处理页面
    fn handle_page(parts: wae_https::extract::RequestParts) -> http::Response<wae_https::Body> {
        let path = parts.path_params.iter().find(|(k, _)| k == "path").map(|(_, v)| v.as_str()).unwrap_or("");
        let response = format!("Page: {}", path);
        http::Response::builder()
            .status(http::StatusCode::OK)
            .header(http::header::CONTENT_TYPE, "text/plain; charset=utf-8")
            .body(wae_https::full_body(response))
            .unwrap()
    }

    /// 监听文件变化
    #[cfg(feature = "dev")]
    async fn watch_files(source_dir: &PathBuf) {
        let (tx, mut rx) = tokio::sync::mpsc::channel(1);

        let mut watcher: RecommendedWatcher = match notify::recommended_watcher(move |res| {
            tx.blocking_send(res).unwrap();
        }) {
            Ok(watcher) => watcher,
            Err(e) => {
                println!("  {} Failed to start file watcher: {}", style("✗").red(), e);
                return;
            }
        };

        if let Err(e) = watcher.watch(source_dir, RecursiveMode::Recursive) {
            println!("  {} Failed to watch directory: {}", style("✗").red(), e);
            return;
        }

        println!("  {} File watcher started", style("✓").green());

        while let Some(res) = rx.recv().await {
            match res {
                Ok(event) if matches!(event.kind, EventKind::Modify(notify::event::ModifyKind::Data(_))) => {
                    if let Some(path) = event.paths.first() {
                        if let Some(ext) = path.extension() {
                            if ext == "md" {
                                println!("  {} File changed: {}", style("→").blue(), path.display());
                                // 这里可以触发增量构建
                            }
                        }
                    }
                }
                _ => {}
            }
        }
    }

    /// 监听文件变化（空实现，当 dev 特性未启用时使用）
    #[cfg(not(feature = "dev"))]
    async fn watch_files(_source_dir: &PathBuf) {
        // 空实现
    }
}
