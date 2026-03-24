//! 开发服务器模块
//! 提供本地开发服务器和热重载功能

use crate::{GatsbyConfig, Parser, StaticSiteGenerator, types::Result};
use console::style;
use std::path::PathBuf;
use tokio::sync::mpsc;
use warp::Filter;

use crate::watcher::FileWatcher;

/// 开发服务器
pub struct DevServer {
    config: GatsbyConfig,
    source_dir: PathBuf,
    output_dir: PathBuf,
    bind: String,
    port: u16,
    open_browser: bool,
}

impl DevServer {
    /// 创建新的开发服务器
    pub fn new(
        config: GatsbyConfig,
        source_dir: PathBuf,
        output_dir: PathBuf,
        bind: String,
        port: u16,
        open_browser: bool,
    ) -> Self {
        Self { config, source_dir, output_dir, bind, port, open_browser }
    }

    /// 启动服务器
    pub async fn start(&self) -> Result<()> {
        println!("{} Starting Gatsby development server...", style("▶").cyan());

        // 首次构建
        self.build_site()?;

        // 创建文件监听器
        let mut watcher = FileWatcher::new()
            .map_err(|e| crate::types::GatsbyError::config(format!("Failed to create file watcher: {:?}", e)))?;

        // 监听源目录
        watcher
            .watch_directory(self.source_dir.to_str().unwrap())
            .map_err(|e| crate::types::GatsbyError::config(format!("Failed to watch directory: {:?}", e)))?;

        // 创建通道用于文件变更通知
        let (tx, mut rx) = mpsc::unbounded_channel();

        // 启动文件监听线程
        std::thread::spawn(move || {
            watcher.listen(move |event| {
                tx.send(event).unwrap();
            });
        });

        // 处理文件变更
        let output_dir = self.output_dir.clone();
        let config = self.config.clone();
        tokio::spawn(async move {
            while let Some(event) = rx.recv().await {
                println!("{} File changed: {:?}", style("♻").yellow(), event);
                // 重新构建站点
                let mut generator = StaticSiteGenerator::new(config.clone()).unwrap();
                let documents = load_documents(&PathBuf::from(".")).unwrap();
                generator.generate(&documents, &output_dir).unwrap();
                println!("{} Site rebuilt successfully", style("✓").green());
            }
        });

        // 静态文件服务
        let output_dir = self.output_dir.clone();
        let static_files = warp::fs::dir(output_dir);

        // GraphQL playground
        let graphql_playground = warp::path!("___graphql").map(|| {
            warp::reply::html(
                r#"<!DOCTYPE html>
<html>
<head>
    <title>GraphQL Playground</title>
    <link rel="stylesheet" href="https://unpkg.com/graphql-playground-react@1.7.28/styles.css" />
</head>
<body style="margin: 0; overflow: hidden;">
    <div id="root" style="height: 100vh; width: 100vw;"></div>
    <script src="https://unpkg.com/graphql-playground-react@1.7.28/build/static/js/middleware.js"></script>
    <script>
        window.addEventListener('load', function() {
            GraphQLPlayground.init(document.getElementById('root'), {
                endpoint: '/api/graphql'
            });
        });
    </script>
</body>
</html>"#,
            )
        });

        // GraphQL API
        let graphql_api = warp::path!("api" / "graphql").map(|| {
            warp::reply::json(&serde_json::json!({
                "data": {
                    "site": {
                        "siteMetadata": {
                            "title": "Gatsby Default Starter"
                        }
                    }
                }
            }))
        });

        // 组合所有路由
        let routes = static_files.or(graphql_playground).or(graphql_api);

        // 启动服务器
        let server_addr = format!("{}:{}", self.bind, self.port);
        println!("{} Development server started", style("✓").green());
        println!("  Server URL: http://{}", server_addr);
        println!("  Output directory: {}", self.output_dir.display());
        println!("  GraphQL Playground: http://{}/___graphql", server_addr);

        if self.open_browser {
            println!("  {} Opening browser...", style("→").blue());
            // 打开浏览器
            if let Err(e) = open::that(format!("http://{}", server_addr)) {
                eprintln!("Failed to open browser: {:?}", e);
            }
        }

        println!("\n  {} Press Ctrl+C to stop the server", style("ℹ").blue());

        // 运行服务器
        let addr: std::net::SocketAddr = server_addr
            .parse()
            .map_err(|e| crate::types::GatsbyError::config(format!("Failed to parse server address: {:?}", e)))?;
        warp::serve(routes).run(addr).await;

        Ok(())
    }

    /// 构建站点
    fn build_site(&self) -> Result<()> {
        println!("{} Building site...", style("▶").cyan());

        let mut generator = StaticSiteGenerator::new(self.config.clone())?;

        // 加载文档
        let documents = self.load_documents()?;

        // 生成站点
        generator.generate(&documents, &self.output_dir)?;

        println!("{} Site built successfully", style("✓").green());

        Ok(())
    }

    /// 加载文档
    fn load_documents(&self) -> Result<std::collections::HashMap<String, nargo_types::Document>> {
        load_documents(&self.source_dir)
    }
}

/// 加载文档的辅助函数
fn load_documents(source_dir: &PathBuf) -> Result<std::collections::HashMap<String, nargo_types::Document>> {
    use std::collections::HashMap;
    use walkdir::WalkDir;

    let mut documents = HashMap::new();

    // 遍历源目录，加载所有 Markdown 文件
    for entry in WalkDir::new(source_dir) {
        let entry = entry.map_err(|e| crate::types::GatsbyError::config(format!("Failed to walk directory: {:?}", e)))?;
        let path = entry.path();

        if path.is_file() && path.extension().map(|ext| ext == "md").unwrap_or(false) {
            let content = std::fs::read_to_string(path)?;
            let relative_path = path.strip_prefix(source_dir).unwrap().to_str().unwrap();

            // 解析文档
            let parser = crate::MarkdownParser::new();
            let doc = parser.parse(&content, relative_path).map_err(|e| crate::types::GatsbyError::config(e))?;
            documents.insert(relative_path.to_string(), doc);
        }
    }

    Ok(documents)
}
