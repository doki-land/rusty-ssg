//! 开发服务器模块
//! 提供本地开发服务器和热重载功能

use crate::{GatsbyConfig, StaticSiteGenerator, types::Result};
use console::style;
use std::path::PathBuf;
use std::net::SocketAddr;
use tokio::sync::mpsc;
use wae_https::{HttpsServerBuilder, static_files_router};
use http::Method;

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
        Self {
            config,
            source_dir,
            output_dir,
            bind,
            port,
            open_browser,
        }
    }
    
    /// 启动服务器
    pub async fn start(&self) -> Result<()> {
        println!("{} Starting Gatsby development server...", style("▶").cyan());
        
        // 首次构建
        self.build_site()?;
        
        // 创建文件监听器
        let mut watcher = FileWatcher::new().map_err(|e| crate::types::GatsbyError::config(format!("Failed to create file watcher: {:?}", e)))?;
        
        // 监听源目录
        watcher.watch_directory(self.source_dir.to_str().unwrap()).map_err(|e| {
            crate::types::GatsbyError::config(format!("Failed to watch directory: {:?}", e))
        })?;
        
        // 创建通道用于文件变更通知
        let (tx, mut rx) = mpsc::unbounded_channel();
        
        // 启动文件监听线程
        std::thread::spawn(move || {
            watcher.listen(move |event| {
                tx.send(event).unwrap();
            });
        });
        
        // 创建静态文件服务器
        let static_handler = static_files_router(self.output_dir.as_path(), "");
        
        // 解析地址
        let addr_str = format!("{}:{}", self.bind, self.port);
        let addr: SocketAddr = addr_str.parse().map_err(|e| {
            crate::types::GatsbyError::config(format!("Invalid address: {:?}", e))
        })?;
        
        // 启动 HTTP 服务器
        let server = HttpsServerBuilder::new()
            .addr(addr)
            .handler(static_handler)
            .build();
        
        println!("{} Development server started", style("✓").green());
        println!("  Server URL: http://{}:{}", self.bind, self.port);
        
        if self.open_browser {
            println!("  {} Opening browser...", style("→").blue());
            // 打开浏览器
            if let Err(e) = open::that(format!("http://{}:{}", self.bind, self.port)) {
                eprintln!("Failed to open browser: {:?}", e);
            }
        }
        
        println!("\n  {} Press Ctrl+C to stop the server", style("ℹ").blue());
        
        // 处理文件变更
        tokio::spawn(async move {
            while let Some(event) = rx.recv().await {
                println!("{} File changed: {:?}", style("♻").yellow(), event);
                // 这里可以添加热重载逻辑
            }
        });
        
        // 启动服务器
        match server {
            Ok(_) => Ok(()),
            Err(e) => Err(crate::types::GatsbyError::config(format!("Failed to start server: {:?}", e))),
        }?
        
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
        use std::collections::HashMap;
        use walkdir::WalkDir;
        
        let mut documents = HashMap::new();
        
        // 遍历源目录，加载所有 Markdown 文件
        for entry in WalkDir::new(&self.source_dir) {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_file() && path.extension().map(|ext| ext == "md").unwrap_or(false) {
                let content = std::fs::read_to_string(path)?;
                let relative_path = path.strip_prefix(&self.source_dir).unwrap().to_str().unwrap();
                
                // 解析文档
                let doc = nargo_parser::parse_document(&content, relative_path)?;
                documents.insert(relative_path.to_string(), doc);
            }
        }
        
        Ok(documents)
    }
}
