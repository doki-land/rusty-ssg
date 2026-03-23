//! 文件监听器模块
//! 提供文件系统监听和热重载功能

use crate::types::Result;
use notify::{RecommendedWatcher, RecursiveMode, Watcher, Event, EventKind};
use std::sync::mpsc;
use std::time::Duration;
use std::path::PathBuf;

/// 文件监听器
pub struct FileWatcher {
    /// 监听的目录
    watch_dirs: Vec<PathBuf>,
    /// 忽略的模式
    ignore_patterns: Vec<String>,
    /// 通知接收器
    rx: std::sync::Arc<std::sync::Mutex<mpsc::Receiver<notify::Result<Event>>>>,
    /// 监听器
    watcher: RecommendedWatcher,
}

impl FileWatcher {
    /// 创建新的文件监听器
    pub fn new() -> Result<Self> {
        let (tx, rx) = mpsc::channel();
        let watcher: RecommendedWatcher = notify::recommended_watcher(move |res| {
            tx.send(res).unwrap();
        })?;

        Ok(Self {
            watch_dirs: Vec::new(),
            ignore_patterns: Vec::new(),
            rx: std::sync::Arc::new(std::sync::Mutex::new(rx)),
            watcher,
        })
    }

    /// 添加监听目录
    pub fn add_watch_dir(&mut self, dir: impl AsRef<std::path::Path>) -> Result<()>
    {
        let dir = dir.as_ref();
        if dir.exists() && dir.is_dir() {
            self.watcher.watch(dir, RecursiveMode::Recursive)?;
            self.watch_dirs.push(dir.to_path_buf());
        }
        Ok(())
    }

    /// 添加忽略模式
    pub fn add_ignore_pattern(&mut self, pattern: &str) {
        self.ignore_patterns.push(pattern.to_string());
    }

    /// 检查是否应该忽略文件
    fn should_ignore(&self, path: &PathBuf) -> bool {
        let path_str = path.to_string_lossy();
        self.ignore_patterns.iter().any(|pattern| {
            path_str.contains(pattern)
        })
    }

    /// 开始监听
    pub async fn start<F>(&self, mut callback: F) -> Result<()>
    where
        F: FnMut(&PathBuf, EventKind) -> Result<()>,
    {
        let rx = self.rx.clone();
        loop {
            let mut rx_guard = rx.lock().unwrap();
            match rx_guard.recv_timeout(Duration::from_secs(1)) {
                Ok(Ok(event)) => {
                    for path in event.paths {
                        if !self.should_ignore(&path) {
                            callback(&path, event.kind)?;
                        }
                    }
                }
                Ok(Err(e)) => {
                    eprintln!("Watch error: {:?}", e);
                }
                Err(_) => {
                    // 超时，继续循环
                }
            }
        }
    }
}

/// 开发服务器
pub struct DevServer {
    /// 服务器地址
    addr: String,
    /// 端口
    port: u16,
    /// 源目录
    source_dir: PathBuf,
    /// 输出目录
    output_dir: PathBuf,
    /// 文件监听器
    watcher: FileWatcher,
}

impl DevServer {
    /// 创建新的开发服务器
    pub fn new(addr: &str, port: u16, source_dir: impl AsRef<std::path::Path>, output_dir: impl AsRef<std::path::Path>) -> Result<Self> {
        let watcher = FileWatcher::new()?;
        
        Ok(Self {
            addr: addr.to_string(),
            port,
            source_dir: source_dir.as_ref().to_path_buf(),
            output_dir: output_dir.as_ref().to_path_buf(),
            watcher,
        })
    }

    /// 启动服务器
    pub async fn start(&mut self) -> Result<()>
    {
        // 添加监听目录
        self.watcher.add_watch_dir(&self.source_dir)?;
        
        // 添加忽略模式
        self.watcher.add_ignore_pattern("node_modules");
        self.watcher.add_ignore_pattern(".git");
        self.watcher.add_ignore_pattern("site");
        
        println!("Starting development server at http://{}:{}", self.addr, self.port);
        println!("Watching for changes in: {}", self.source_dir.display());
        
        // 启动文件监听
        let server = self.clone();
        tokio::spawn(async move {
            server.watcher.start(|path, kind| {
                println!("File changed: {:?} ({:?})", path, kind);
                // 这里可以添加重新编译逻辑
                Ok(())
            }).await
        });
        
        // 启动 HTTP 服务器
        self.start_http_server().await
    }

    /// 启动 HTTP 服务器
    async fn start_http_server(&self) -> Result<()>
    {
        use hyper::body::Body;
        use hyper::{Request, Response, StatusCode, Server};
        use std::fs::File;
        use std::io::Read;
        
        let output_dir = self.output_dir.clone();
        let addr = format!("{}:{}", self.addr, self.port).parse()?;
        
        let service = hyper::service::service_fn(move |_req: Request<Body>| {
            let output_dir = output_dir.clone();
            async move {
                let path = _req.uri().path();
                let file_path = if path == "/" {
                    output_dir.join("index.html")
                } else {
                    output_dir.join(&path[1..])
                };
                
                if file_path.exists() && file_path.is_file() {
                    let mut file = File::open(file_path)?;
                    let mut content = Vec::new();
                    file.read_to_end(&mut content)?;
                    
                    let content_type = mime_guess::from_path(&file_path).first_or_octet_stream();
                    
                    Ok(Response::builder()
                        .status(StatusCode::OK)
                        .header(hyper::header::CONTENT_TYPE, content_type.to_string())
                        .body(Body::from(content))?)
                } else {
                    Ok(Response::builder()
                        .status(StatusCode::NOT_FOUND)
                        .body(Body::from("404 Not Found"))?)
                }
            }
        });
        
        let server = Server::bind(&addr).serve(service);
        server.await?;
        
        Ok(())
    }
}

impl Clone for DevServer {
    fn clone(&self) -> Self {
        Self {
            addr: self.addr.clone(),
            port: self.port,
            source_dir: self.source_dir.clone(),
            output_dir: self.output_dir.clone(),
            watcher: FileWatcher::new().unwrap(), // 注意：这里可能需要更好的错误处理
        }
    }
}