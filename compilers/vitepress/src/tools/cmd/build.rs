//! Build 命令实现

use crate::{BuildArgs, ConfigLoader, StaticSiteGenerator, VutexCompiler};
use console::style;
use std::{collections::HashMap, fs, path::PathBuf};
use crate::compiler::PluginHost;
use crate::types::Result;
use walkdir::WalkDir;

/// Build 命令
pub struct BuildCommand;

impl BuildCommand {
    /// 执行 build 命令
    pub async fn execute(args: BuildArgs) -> Result<()> {
        println!("{}", style("Starting VuTeX build...").cyan());

        let source_dir = args.source.unwrap_or_else(|| PathBuf::from("."));
        let output_dir = args.output.unwrap_or_else(|| PathBuf::from("dist"));

        println!("  Source directory: {}", source_dir.display());
        println!("  Output directory: {}", output_dir.display());

        if output_dir.exists() {
            println!("  {} Cleaning output directory...", style("✓").green());
            fs::remove_dir_all(&output_dir)?;
        }

        if !output_dir.exists() {
            fs::create_dir_all(&output_dir)?;
        }

        println!("  {} Loading configuration...", style("→").blue());
        let config = ConfigLoader::load_from_dir(&source_dir)?;
        println!("  {} Configuration loaded", style("✓").green());

        let mut documents = HashMap::new();
        let mut file_count = 0;

        println!("  {} Scanning for Markdown files...", style("→").blue());

        for entry in WalkDir::new(&source_dir) {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() {
                if let Some(ext) = path.extension() {
                    if ext == "md" {
                        let rel_path = path.strip_prefix(&source_dir).unwrap_or(path).to_string_lossy().to_string();

                        let path_components: Vec<&str> = rel_path.split(std::path::MAIN_SEPARATOR).collect();
                        if path_components.iter().any(|&c| c == "node_modules" || c == ".git" || c == "dist" || c == ".vutex") {
                            continue;
                        }

                        file_count += 1;
                        let content = fs::read_to_string(path)?;
                        let normalized_path = rel_path.replace(std::path::MAIN_SEPARATOR, "/");

                        documents.insert(normalized_path, content);
                    }
                }
            }
        }

        println!("  {} Found {} Markdown files", style("✓").green(), file_count);

        if file_count == 0 {
            println!("  {} No Markdown files found", style("⚠").yellow());
            return Ok(());
        }

        println!("  {} Compiling documents...", style("→").blue());

        let mut plugin_host_option: Option<PluginHost> = None;
        let project_root = std::env::current_dir()?;
        let ipc_server_path = project_root.join("runtimes").join("vutex-ipc-server").join("dist").join("index.js");

        match PluginHost::new("node", ipc_server_path.to_str().unwrap()) {
            Ok(host) => {
                println!("  {} Plugin host initialized (Node.js hybrid mode)", style("✓").green());
                plugin_host_option = Some(host);
            }
            Err(e) => {
                println!("  {} Failed to initialize plugin host: {}", style("⚠").yellow(), e);
                println!("    {} Please install Node.js to use all features", style("ℹ").blue());
                println!("    {} Continuing in Rust-only mode (limited functionality)", style("ℹ").blue());
            }
        }

        let result;
        let mut plugin_host_guard = plugin_host_option;

        if let Some(mut plugin_host) = plugin_host_guard.take() {
            let mut compiler = VutexCompiler::with_config_and_plugin_host(config.clone(), plugin_host);
            result = compiler.compile_batch(&documents);

            if let Some(mut host) = compiler.plugin_host_mut().take() {
                println!("  {} Shutting down plugin host...", style("→").blue());
                let _ = host.shutdown();
            }
        }
        else {
            let mut compiler = VutexCompiler::with_config(config.clone());
            result = compiler.compile_batch(&documents);
        }

        if result.success {
            println!("  {} Compiled {} documents in {}ms", style("✓").green(), result.documents.len(), result.compile_time_ms);

            println!("  {} Generating static site...", style("→").blue());

            let mut site_generator = StaticSiteGenerator::new(config)?;
            site_generator.generate(&result.documents, &output_dir)?;

            println!("  {} Static site generated successfully", style("✓").green());
            println!("  {} Output written to {}", style("✓").green(), output_dir.display());
        }
        else {
            println!("  {} Compilation failed with {} errors", style("✗").red(), result.errors.len());

            for error in &result.errors {
                println!("    {}", style(error).red());
            }
        }

        Ok(())
    }
}
