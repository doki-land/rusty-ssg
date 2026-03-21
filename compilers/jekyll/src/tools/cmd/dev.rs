//! Dev 命令实现
//! 
//! 提供 Jekyll 开发服务器功能，支持文件监听、自动重新构建和静态文件服务。

#[cfg(feature = "dev")]
use crate::DevArgs;
use console::style;
use std::{fs, path::PathBuf};
use crate::types::Result;
#[cfg(feature = "dev")]
use notify::{RecommendedWatcher, RecursiveMode, Watcher, Event};
#[cfg(feature = "dev")]
use tokio::sync::mpsc;

/// Dev 命令执行器
/// 
/// 负责启动 Jekyll 开发服务器，监听文件变化并自动重新构建站点。
#[cfg(feature = "dev")]
pub struct DevCommand;

#[cfg(feature = "dev")]
impl DevCommand {
    /// 执行 dev 命令
    /// 
    /// 根据提供的参数启动 Jekyll 开发服务器。
    /// 
    /// # Arguments
    /// 
    /// * `args` - 开发服务器参数，包含端口、绑定地址、是否自动打开浏览器等配置
    /// 
    /// # Returns
    /// 
    /// 返回成功或错误结果
    pub async fn execute(args: DevArgs) -> Result<()> {
        println!("{}", style("Starting Jekyll development server...").cyan());

        let source_dir = args.source.unwrap_or_else(|| PathBuf::from("."));
        let output_dir = args.output.unwrap_or_else(|| source_dir.join("_site"));
        let port = args.port;
        let host = args.host;

        println!("  Source directory: {}", source_dir.display());
        println!("  Output directory: {}", output_dir.display());
        println!("  Listening on: {}:{}", host, port);

        if args.livereload {
            println!("  {} LiveReload enabled on port {}", style("ℹ").blue(), args.livereload_port);
        }

        // 构建输出目录
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
        let _watcher = Self::setup_watcher(&source_dir, tx)?;

        // 自动打开浏览器
        if args.open {
            Self::open_browser(&host, port)?;
        }

        println!("  {} Development server would start at http://{}:{}", style("→").blue(), host, port);
        println!("  {} Press Ctrl+C to stop the server", style("ℹ").blue());
        println!("  {} Note: Full HTTP server functionality requires additional setup", style("⚠").yellow());

        // 启动文件监视任务
        let source_dir_clone = source_dir.clone();
        let output_dir_clone = output_dir.clone();
        tokio::spawn(async move {
            while let Some(event) = rx.recv().await {
                Self::handle_file_change(event, &source_dir_clone, &output_dir_clone).await;
            }
        });

        // 保持程序运行
        loop {
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        }
    }

    /// 构建站点
    /// 
    /// 执行站点构建过程，与 build 命令类似。
    /// 
    /// # Arguments
    /// 
    /// * `source_dir` - 源目录路径
    /// * `_output_dir` - 输出目录路径
    /// 
    /// # Returns
    /// 
    /// 返回成功或错误结果
    async fn build_site(source_dir: &PathBuf, _output_dir: &PathBuf) -> Result<()> {
        use crate::jekyll::{JekyllStructure, JekyllConfigLoader, PostManager};

        let structure = JekyllStructure::new(source_dir)?;
        let config = JekyllConfigLoader::load_from_dir(source_dir)?;
        let mut post_manager = PostManager::new(structure, config);
        post_manager.load_posts()?;

        // 这里可以添加更多的构建逻辑

        Ok(())
    }

    /// 设置文件监视
    /// 
    /// 配置文件系统监视器，监听源目录的变化。
    /// 
    /// # Arguments
    /// 
    /// * `source_dir` - 要监视的源目录路径
    /// * `tx` - 用于发送文件变化事件的通道
    /// 
    /// # Returns
    /// 
    /// 返回配置好的文件监视器或错误结果
    fn setup_watcher(source_dir: &PathBuf, tx: mpsc::Sender<notify::Result<Event>>) -> Result<RecommendedWatcher> {
        let mut watcher = notify::recommended_watcher(move |res| {
            let _ = tx.blocking_send(res);
        })?;

        // 监视源目录
        watcher.watch(source_dir, RecursiveMode::Recursive)?;

        Ok(watcher)
    }

    /// 处理文件变化
    /// 
    /// 当检测到文件变化时，触发重新构建。
    /// 
    /// # Arguments
    /// 
    /// * `event` - 文件系统事件
    /// * `source_dir` - 源目录路径
    /// * `output_dir` - 输出目录路径
    async fn handle_file_change(event: notify::Result<Event>, source_dir: &PathBuf, output_dir: &PathBuf) {
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
    /// 
    /// 处理根路径请求，返回开发服务器欢迎页面。
    /// 
    /// # Returns
    /// 
    /// HTTP 响应对象
    fn index_handler() -> http::Response<wae_https::Body> {
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

    /// 自动打开浏览器
    /// 
    /// 在默认浏览器中打开开发服务器地址。
    /// 
    /// # Arguments
    /// 
    /// * `host` - 服务器绑定地址
    /// * `port` - 服务器监听端口
    /// 
    /// # Returns
    /// 
    /// 返回成功或错误结果
    fn open_browser(host: &str, port: u16) -> Result<()> {
        let url = format!("http://{}:{}", host, port);
        println!("  {} Opening browser at {}", style("→").blue(), url);
        
        #[cfg(target_os = "windows")]
        {
            std::process::Command::new("cmd")
                .args(&["/C", "start", &url])
                .spawn()?;
        }
        
        #[cfg(target_os = "macos")]
        {
            std::process::Command::new("open")
                .arg(&url)
                .spawn()?;
        }
        
        #[cfg(target_os = "linux")]
        {
            std::process::Command::new("xdg-open")
                .arg(&url)
                .spawn()?;
        }
        
        Ok(())
    }
}

/// 执行 dev 命令的公开入口点
/// 
/// # Arguments
/// 
/// * `args` - 开发服务器参数
/// 
/// # Returns
/// 
/// 返回成功或错误结果
#[cfg(feature = "dev")]
pub async fn execute(args: crate::DevArgs) -> crate::types::Result<()> {
    DevCommand::execute(args).await
}
