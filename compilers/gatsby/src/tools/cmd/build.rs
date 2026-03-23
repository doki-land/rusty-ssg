//! Build 命令实现

use crate::{BuildArgs, GatsbyCompiler, StaticSiteGenerator, types::Result};
use console::style;
use std::{fs, path::PathBuf};
use walkdir::WalkDir;

/// Build 命令
pub struct BuildCommand;

impl BuildCommand {
    /// 执行 build 命令
    pub async fn execute(args: BuildArgs) -> Result<()> {
        println!("{}", style("Starting Gatsby build...").cyan());

        let source_dir = args.source.unwrap_or_else(|| PathBuf::from("."));
        let output_dir = args.destination;

        println!("  Source directory: {}", source_dir.display());
        println!("  Output directory: {}", output_dir.display());

        if args.clean_destination_dir || output_dir.exists() {
            println!("  {} Cleaning output directory...", style("✓").green());
            if output_dir.exists() {
                fs::remove_dir_all(&output_dir)?;
            }
        }

        if !output_dir.exists() {
            fs::create_dir_all(&output_dir)?;
        }

        println!("  {} Loading configuration...", style("→").blue());
        let config = crate::ConfigLoader::load_from_dir(&source_dir)?;
        println!("  {} Configuration loaded", style("✓").green());

        println!("  {} Scanning for source files...", style("→").blue());

        let mut documents = std::collections::HashMap::new();
        let mut file_count = 0;

        // 扫描 src/pages 目录
        let src_pages_dir = source_dir.join("src").join("pages");
        if src_pages_dir.exists() {
            for entry in WalkDir::new(&src_pages_dir).into_iter().filter_map(|e| e.ok()) {
                let path = entry.path();
                if path.is_file() {
                    if let Some(ext) = path.extension() {
                        let ext_str = ext.to_str().unwrap_or("");
                        if matches!(ext_str, "md") {
                            file_count += 1;
                            let content = fs::read_to_string(path)?;
                            let relative_path = path.strip_prefix(&source_dir).unwrap().to_str().unwrap();
                            documents.insert(relative_path.to_string(), content);
                        }
                    }
                }
            }
        }

        // 扫描 content 目录
        let content_dir = source_dir.join("content");
        if content_dir.exists() {
            for entry in WalkDir::new(&content_dir).into_iter().filter_map(|e| e.ok()) {
                let path = entry.path();
                if path.is_file() {
                    if let Some(ext) = path.extension() {
                        let ext_str = ext.to_str().unwrap_or("");
                        if matches!(ext_str, "md") {
                            file_count += 1;
                            let content = fs::read_to_string(path)?;
                            let relative_path = path.strip_prefix(&source_dir).unwrap().to_str().unwrap();
                            documents.insert(relative_path.to_string(), content);
                        }
                    }
                }
            }
        }

        println!("  {} Found {} Markdown file(s)", style("✓").green(), file_count);

        if file_count == 0 {
            println!("  {} No Markdown files found", style("⚠").yellow());
            return Ok(());
        }

        println!("  {} Compiling and generating static site...", style("→").blue());

        // 编译文档
        let mut compiler = GatsbyCompiler::with_config(config.clone());
        let compile_result = compiler.compile_batch(&documents);

        if !compile_result.success {
            println!("  {} Compilation failed", style("✗").red());
            for error in compile_result.errors {
                println!("    - {}", error);
            }
            return Err(crate::types::GatsbyError::compile("Compilation failed".to_string()));
        }

        // 生成静态站点
        let mut generator = StaticSiteGenerator::new(config)?;
        generator.generate(&compile_result.documents, &output_dir)?;

        // 复制静态资源
        let static_dir = source_dir.join("static");
        if static_dir.exists() {
            let static_out_dir = output_dir.join("static");
            fs::create_dir_all(&static_out_dir)?;

            for entry in WalkDir::new(&static_dir).into_iter().filter_map(|e| e.ok()) {
                let path = entry.path();
                if path.is_file() {
                    let relative_path =
                        path.strip_prefix(&static_dir).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
                    let dest_path = static_out_dir.join(relative_path);

                    if let Some(parent) = dest_path.parent() {
                        fs::create_dir_all(parent)?;
                    }

                    fs::copy(path, dest_path)?;
                }
            }
        }

        println!("  {} Static site generated successfully", style("✓").green());
        println!("  {} Output written to {}", style("✓").green(), output_dir.display());
        println!("  {} Build completed in {}ms", style("✓").green(), compile_result.compile_time_ms);

        Ok(())
    }
}
