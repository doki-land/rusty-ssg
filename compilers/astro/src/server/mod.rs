//! 服务器模块

use crate::tools::cmd::{generate_static_files, process_files};
use crate::compiler::{ComponentParser, DependencyAnalyzer};
use crate::config::{AstroConfig, ConfigManager};
use crate::watcher::FileWatcher;
use hyper::body::Incoming;
use hyper::Request;
use hyper::Response;
use hyper::StatusCode;
use hyper::Server;
use hyper::service::service_fn;
use std::path::Path;
use std::sync::Arc;
use std::sync::Mutex;
use std::fs::File;
use std::io::Read;

/// 开发服务器
pub struct DevServer {
    /// 服务器地址
    address: String,
    /// 项目路径
    project_path: String,
    /// 输出目录
    out_dir: String,
    /// 配置
    config: Arc<Mutex<AstroConfig>>,
    /// 组件解析器
    parser: Arc<Mutex<ComponentParser>>,
    /// 依赖分析器
    analyzer: Arc<Mutex<DependencyAnalyzer>>,
}

impl DevServer {
    /// 创建新的开发服务器
    pub fn new(address: &str, project_path: &str, out_dir: &str) -> Self {
        let mut config_manager = ConfigManager::new();
        let config = config_manager.load_from_project(Path::new(project_path)).unwrap_or_default();
        
        Self {
            address: address.to_string(),
            project_path: project_path.to_string(),
            out_dir: out_dir.to_string(),
            config: Arc::new(Mutex::new(config)),
            parser: Arc::new(Mutex::new(ComponentParser::new())),
            analyzer: Arc::new(Mutex::new(DependencyAnalyzer::new())),
        }
    }

    /// 启动服务器
    pub async fn start(&self) -> Result<(), hyper::Error> {
        // 初始构建
        self.build_project();

        // 启动文件监听器
        self.start_file_watcher();

        // 启动 HTTP 服务器
        let addr: std::net::SocketAddr = self.address.parse().expect("Invalid address");
        
        let project_path = self.project_path.clone();
        let out_dir = self.out_dir.clone();
        
        let make_svc = hyper::service::service_fn(move |req| {
            let project_path = project_path.clone();
            let out_dir = out_dir.clone();
            
            async move {
                DevServer::handle_request(req, &project_path, &out_dir).await
            }
        });

        let server = Builder::new(AddrIncoming::bind(&addr).unwrap(), make_svc);
        println!("Dev server running at http://{}", addr);
        server.await
    }

    /// 处理 HTTP 请求
    async fn handle_request(req: Request<Incoming>, project_path: &str, out_dir: &str) -> Result<Response<String>, hyper::Error> {
        let path = req.uri().path();
        
        // 处理根路径
        let file_path = if path == "/" {
            Path::new(out_dir).join("index.html")
        } else {
            // 尝试直接访问文件
            let mut file_path = Path::new(out_dir).join(&path[1..]);
            
            // 如果路径是目录，尝试访问 index.html
            if file_path.is_dir() {
                file_path = file_path.join("index.html");
            }
            
            file_path
        };

        // 读取文件
        match File::open(&file_path) {
            Ok(mut file) => {
                let mut content = Vec::new();
                file.read_to_end(&mut content).unwrap();
                
                // 确定内容类型
                let content_type = DevServer::get_content_type(&file_path);
                
                Ok(Response::builder()
                    .status(StatusCode::OK)
                    .header("Content-Type", content_type)
                    .body(String::from_utf8_lossy(&content).to_string())
                    .unwrap())
            }
            Err(_) => {
                // 如果文件不存在，返回 404
                Ok(Response::builder()
                    .status(StatusCode::NOT_FOUND)
                    .body(String::from("404 Not Found"))
                    .unwrap())
            }
        }
    }

    /// 获取文件的内容类型
    fn get_content_type(path: &Path) -> &'static str {
        match path.extension().and_then(|ext| ext.to_str()) {
            Some("html") => "text/html",
            Some("css") => "text/css",
            Some("js") => "application/javascript",
            Some("json") => "application/json",
            Some("png") => "image/png",
            Some("jpg") | Some("jpeg") => "image/jpeg",
            Some("gif") => "image/gif",
            Some("svg") => "image/svg+xml",
            _ => "application/octet-stream",
        }
    }

    /// 构建项目
    fn build_project(&self) {
        println!("Building project...");
        
        let project_path = Path::new(&self.project_path);
        let out_path = Path::new(&self.out_dir);
        
        // 读取配置
        let mut config_manager = ConfigManager::new();
        let config = config_manager.load_from_project(project_path).unwrap_or_default();
        
        // 处理文件
        let (parser, analyzer) = process_files(project_path, &crate::cache::CacheManager::new());
        
        // 生成静态文件
        generate_static_files(
            &parser,
            &analyzer,
            project_path,
            &self.out_dir,
            &config,
            &crate::plugin::PluginManager::new(),
            &crate::cache::CacheManager::new(),
        );
        
        println!("Build completed!");
    }

    /// 启动文件监听器
    fn start_file_watcher(&self) {
        let project_path = self.project_path.clone();
        let out_dir = self.out_dir.clone();
        
        // 创建文件监听器
        let watcher = FileWatcher::new(move |path| {
            println!("File changed: {}", path.display());
            
            // 重新构建项目
            let project_path = Path::new(&project_path);
            let mut config_manager = ConfigManager::new();
            let config = config_manager.load_from_project(project_path).unwrap_or_default();
            
            let (parser, analyzer) = process_files(project_path, &crate::cache::CacheManager::new());
            
            generate_static_files(
                &parser,
                &analyzer,
                project_path,
                &out_dir,
                &config,
                &crate::plugin::PluginManager::new(),
                &crate::cache::CacheManager::new(),
            );
            
            println!("Rebuild completed!");
        }).expect("Failed to create file watcher");
        
        // 监听项目目录
        let mut watcher = watcher;
        watcher.watch(Path::new(&self.project_path)).expect("Failed to start watching directory");
        
        println!("File watcher started");
    }
}

/// 预览服务器
pub struct PreviewServer {
    /// 服务器地址
    address: String,
    /// 构建输出目录
    build_path: String,
}

impl PreviewServer {
    /// 创建新的预览服务器
    pub fn new(address: &str, build_path: &str) -> Self {
        Self {
            address: address.to_string(),
            build_path: build_path.to_string(),
        }
    }

    /// 启动服务器
    pub async fn start(&self) -> Result<(), hyper::Error> {
        // 启动 HTTP 服务器
        let addr: std::net::SocketAddr = self.address.parse().expect("Invalid address");
        
        let build_path = self.build_path.clone();
        
        let make_svc = hyper::service::service_fn(move |req| {
            let build_path = build_path.clone();
            
            async move {
                PreviewServer::handle_request(req, &build_path).await
            }
        });

        let server = Builder::new(AddrIncoming::bind(&addr).unwrap(), make_svc);
        println!("Preview server running at http://{}", addr);
        server.await
    }

    /// 处理 HTTP 请求
    async fn handle_request(req: Request<Incoming>, build_path: &str) -> Result<Response<String>, hyper::Error> {
        let path = req.uri().path();
        
        // 处理根路径
        let file_path = if path == "/" {
            Path::new(build_path).join("index.html")
        } else {
            // 尝试直接访问文件
            let mut file_path = Path::new(build_path).join(&path[1..]);
            
            // 如果路径是目录，尝试访问 index.html
            if file_path.is_dir() {
                file_path = file_path.join("index.html");
            }
            
            file_path
        };

        // 读取文件
        match File::open(&file_path) {
            Ok(mut file) => {
                let mut content = Vec::new();
                file.read_to_end(&mut content).unwrap();
                
                // 确定内容类型
                let content_type = PreviewServer::get_content_type(&file_path);
                
                Ok(Response::builder()
                    .status(StatusCode::OK)
                    .header("Content-Type", content_type)
                    .body(String::from_utf8_lossy(&content).to_string())
                    .unwrap())
            }
            Err(_) => {
                // 如果文件不存在，返回 404
                Ok(Response::builder()
                    .status(StatusCode::NOT_FOUND)
                    .body(String::from("404 Not Found"))
                    .unwrap())
            }
        }
    }

    /// 获取文件的内容类型
    fn get_content_type(path: &Path) -> &'static str {
        match path.extension().and_then(|ext| ext.to_str()) {
            Some("html") => "text/html",
            Some("css") => "text/css",
            Some("js") => "application/javascript",
            Some("json") => "application/json",
            Some("png") => "image/png",
            Some("jpg") | Some("jpeg") => "image/jpeg",
            Some("gif") => "image/gif",
            Some("svg") => "image/svg+xml",
            _ => "application/octet-stream",
        }
    }
}
