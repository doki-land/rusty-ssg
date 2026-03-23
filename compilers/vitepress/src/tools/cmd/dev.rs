//! Dev 命令实现

#[cfg(feature = "dev")]
use crate::{
    compiler::{PluginHost, VitePressCompiler},
    tools::{ConfigLoader, DevArgs, StaticSiteGenerator},
    types::Result,
};
#[cfg(feature = "dev")]
use console::style;
#[cfg(feature = "dev")]
use fs_extra::dir::{CopyOptions, copy};
#[cfg(feature = "dev")]
use notify::{Config as NotifyConfig, Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
#[cfg(feature = "dev")]
use open;
#[cfg(feature = "dev")]
use std::{
    collections::HashMap,
    fs,
    net::SocketAddr,
    path::PathBuf,
    sync::{Arc, Mutex},
};
#[cfg(feature = "dev")]
use wae_https::{HttpsServerBuilder, static_files_router};
#[cfg(feature = "dev")]
use walkdir::WalkDir;

/// Dev 命令
#[cfg(feature = "dev")]
pub struct DevCommand;

/// Dev 服务器状态
#[cfg(feature = "dev")]
#[derive(Clone)]
pub struct DevServerState {
    /// 源目录路径
    pub source_dir: PathBuf,
    /// 输出目录路径
    pub output_dir: PathBuf,
    /// 最后构建是否成功
    pub last_build_successful: Arc<Mutex<bool>>,
}

#[cfg(feature = "dev")]
impl DevCommand {
    /// 执行 dev 命令
    pub async fn execute(args: DevArgs) -> Result<()> {
        println!("{}", style("Starting VitePress dev server...").cyan());

        let source_dir = args.source.unwrap_or_else(|| PathBuf::from("."));
        let output_dir = PathBuf::from(".vitepress").join("temp");

        println!("  Source directory: {}", source_dir.display());
        println!("  Output directory: {}", output_dir.display());
        println!("  Host: {}", args.host);
        println!("  Port: {}", args.port);
        println!("  Open: {}", args.open);

        if output_dir.exists() {
            fs::remove_dir_all(&output_dir)?;
        }
        fs::create_dir_all(&output_dir)?;

        let state = DevServerState {
            source_dir: source_dir.clone(),
            output_dir: output_dir.clone(),
            last_build_successful: Arc::new(Mutex::new(false)),
        };

        println!("  {} Initial build...", style("→").blue());
        Self::build_site(&source_dir, &output_dir, args.config.clone())?;
        *state.last_build_successful.lock().unwrap() = true;
        println!("  {} Initial build complete", style("✓").green());

        println!("  {} Starting file watcher...", style("→").blue());
        Self::start_file_watcher(source_dir.clone(), output_dir.clone(), state.clone(), args.config.clone())?;

        println!("  {} Starting HTTP server...", style("→").blue());
        Self::start_http_server(&args.host, args.port, state, args.open).await?;

        Ok(())
    }

    /// 构建站点
    pub fn build_site(source_dir: &PathBuf, output_dir: &PathBuf, config_path: Option<PathBuf>) -> Result<()> {
        if !output_dir.exists() {
            fs::create_dir_all(output_dir)?;
        }

        let config = if let Some(config_path) = config_path {
            ConfigLoader::load_from_file(&config_path)?
        } else {
            ConfigLoader::load_from_dir(source_dir)?
        };

        let mut documents = HashMap::new();

        for entry in WalkDir::new(source_dir) {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() {
                if let Some(ext) = path.extension() {
                    if ext == "md" {
                        let content = fs::read_to_string(path)?;
                        let rel_path = path.strip_prefix(source_dir).unwrap_or(path).to_string_lossy().to_string();
                        documents.insert(rel_path, content);
                    }
                }
            }
        }

        let project_root = std::env::current_dir()?;
        let ipc_server_path = project_root.join("runtimes").join("vitepress-ipc-server").join("dist").join("index.js");

        let result;

        match PluginHost::new("node", ipc_server_path.to_str().unwrap()) {
            Ok(mut plugin_host) => {
                let mut compiler = VitePressCompiler::with_config_and_plugin_host(config.clone(), plugin_host);
                result = compiler.compile_batch(&documents);

                if let Some(mut host) = compiler.plugin_host_mut().take() {
                    let _ = host.shutdown();
                }
            }
            Err(_) => {
                let mut compiler = VitePressCompiler::with_config(config.clone());
                result = compiler.compile_batch(&documents);
            }
        }

        if !result.success {
            println!("  {} Compilation failed with {} errors", style("✗").red(), result.errors.len());
            for error in &result.errors {
                println!("    {}", style(error).red());
            }
            return Ok(());
        }

        let mut site_generator = StaticSiteGenerator::new(config)?;
        site_generator.generate(&result.documents, output_dir)?;

        Self::copy_public_assets(source_dir, output_dir)?;

        Ok(())
    }

    /// 复制 public 目录的静态资源
    pub fn copy_public_assets(source_dir: &PathBuf, output_dir: &PathBuf) -> Result<()> {
        let public_dir = source_dir.join("public");

        if public_dir.exists() && public_dir.is_dir() {
            let output_public_dir = output_dir.join("public");
            let mut options = CopyOptions::new();
            options.overwrite = true;
            options.copy_inside = true;

            copy(&public_dir, &output_public_dir, &options).map_err(|e| {
                crate::types::VitePressError::from(std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))
            })?;
        }

        Ok(())
    }

    /// 启动文件监听器
    pub fn start_file_watcher(source_dir: PathBuf, output_dir: PathBuf, state: DevServerState, config_path: Option<PathBuf>) -> Result<()> {
        let (tx, rx) = std::sync::mpsc::channel();

        let mut watcher = RecommendedWatcher::new(
            move |res| {
                tx.send(res).unwrap();
            },
            NotifyConfig::default(),
        )?;

        watcher.watch(&source_dir, RecursiveMode::Recursive)?;

        std::thread::spawn(move || {
            for res in rx {
                match res {
                    Ok(event) => {
                        if Self::should_rebuild(&event) {
                            println!("\n  {} File change detected, rebuilding...", style("→").blue());
                            match Self::build_site(&source_dir, &output_dir, config_path.clone()) {
                                Ok(_) => {
                                    *state.last_build_successful.lock().unwrap() = true;
                                    println!("  {} Rebuild complete", style("✓").green());
                                }
                                Err(e) => {
                                    *state.last_build_successful.lock().unwrap() = false;
                                    println!("  {} Rebuild failed: {}", style("✗").red(), e);
                                }
                            }
                        }
                    }
                    Err(e) => {
                        println!("  {} Watch error: {}", style("⚠").yellow(), e);
                    }
                }
            }
        });

        Ok(())
    }

    /// 判断是否需要重新构建
    pub fn should_rebuild(event: &Event) -> bool {
        match event.kind {
            EventKind::Create(_) | EventKind::Modify(_) | EventKind::Remove(_) => {
                for path in &event.paths {
                    if let Some(ext) = path.extension() {
                        if ext == "md"
                            || ext == "html"
                            || ext == "css"
                            || ext == "js"
                            || ext == "ts"
                            || ext == "toml"
                            || ext == "json"
                            || ext == "yaml"
                            || ext == "yml"
                        {
                            return true;
                        }
                    }
                    if let Some(file_name) = path.file_name() {
                        if file_name == "vitepress.config.toml"
                            || file_name == "vitepress.config.json"
                            || file_name == "vitepress.config.yaml"
                            || file_name == "vitepress.config.yml"
                            || file_name == "vitepress.config.ts"
                        {
                            return true;
                        }
                    }
                }
                false
            }
            _ => false,
        }
    }

    /// 启动 HTTP 服务器
    pub async fn start_http_server(host: &str, port: u16, _state: DevServerState, open: bool) -> Result<()> {
        let output_dir = _state.output_dir;
        let router = static_files_router(&output_dir, "");

        let addr: SocketAddr = format!("{}:{}", host, port).parse()?;

        println!("\n{}", style("Dev server is running!").green().bold());
        println!("  Local:   http://{}/", addr);
        println!("\n  {} Press Ctrl+C to stop", style("ℹ").blue());

        // 自动打开浏览器
        if open {
            let url = format!("http://{}/", addr);
            println!("  {} Opening browser at {}", style("→").blue(), url);
            if let Err(e) = open::that(url) {
                println!("  {} Failed to open browser: {}", style("⚠").yellow(), e);
            }
        }

        let server = HttpsServerBuilder::new().addr(addr).router(router).build();

        server.serve().await.map_err(|e| crate::types::VitePressError::ConfigError { message: e.to_string(), path: None, suggestion: None })?;

        Ok(())
    }
}
