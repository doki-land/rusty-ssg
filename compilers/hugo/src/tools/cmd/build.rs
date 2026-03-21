//! Build 命令实现

use crate::{ConfigLoader, StaticSiteGenerator, VutexCompiler, types::Result};
use console::style;
use std::{collections::HashMap, fs, path::PathBuf};
use walkdir::WalkDir;

/// Build 命令
pub struct BuildCommand;

impl BuildCommand {
    /// 执行 build 命令
    pub async fn execute(args: crate::BuildArgs) -> Result<()> {
        println!("{}", style("Starting Hugo build...").cyan());

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

        let mut compiler = VutexCompiler::with_config(config.clone());
        let result = compiler.compile_batch(&documents);

        if result.success {
            println!("  {} Compiled {} documents in {}ms", style("✓").green(), result.documents.len(), result.compile_time_ms);

            println!("  {} Generating static site...", style("→").blue());

            let mut site_generator = StaticSiteGenerator::new(config.clone())?;
            site_generator.generate(&result.documents, &output_dir)?;

            println!("  {} Copying static assets...", style("→").blue());
            Self::copy_static_assets(&source_dir, &output_dir)?;

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

    /// 复制静态资源
    fn copy_static_assets(source_dir: &PathBuf, output_dir: &PathBuf) -> Result<()> {
        // 复制主题静态资源
        let themes_dir = source_dir.join("themes");
        if themes_dir.exists() {
            // 复制 Ananke 主题的静态资源
            let ananke_dir = themes_dir.join("ananke");
            if ananke_dir.exists() {
                // 复制 images 文件
                let ananke_images_dir = ananke_dir.join("static").join("images");
                if ananke_images_dir.exists() {
                    let output_images_dir = output_dir.join("images");
                    fs::create_dir_all(&output_images_dir)?;

                    for entry in WalkDir::new(ananke_images_dir) {
                        let entry = entry?;
                        let path = entry.path();
                        if path.is_file() {
                            let file_name = path.file_name().unwrap();
                            let output_path = output_images_dir.join(file_name);
                            fs::copy(path, output_path)?;
                        }
                    }
                }

                // 复制 CSS 文件（从主题源代码）
                let ananke_css_dir = ananke_dir.join("assets").join("ananke").join("css");
                println!("  Checking CSS source directory: {}", ananke_css_dir.display());
                if ananke_css_dir.exists() {
                    let output_css_dir = output_dir.join("ananke").join("css");
                    fs::create_dir_all(&output_css_dir)?;
                    println!("  Created output CSS directory: {}", output_css_dir.display());

                    // 读取所有 CSS 文件内容
                    let mut css_content = String::new();
                    for entry in WalkDir::new(ananke_css_dir) {
                        let entry = entry?;
                        let path = entry.path();
                        if path.is_file() && path.extension().unwrap_or_default() == "css" {
                            println!("  Reading CSS file: {}", path.display());
                            let content = fs::read_to_string(path)?;
                            css_content.push_str(&content);
                            css_content.push_str("\n");
                        }
                    }

                    // 生成简单的 minified CSS（实际项目中应该使用专业的 CSS 压缩库）
                    let minified_css = css_content.replace("\n", "").replace("\t", "").replace("  ", "");

                    // 生成哈希值
                    use sha2::{Digest, Sha256};
                    let mut hasher = Sha256::new();
                    hasher.update(minified_css.as_bytes());
                    let hash = hasher.finalize();
                    let hash_hex = format!("{:x}", hash);

                    // 写入 CSS 文件
                    let css_file_name = format!("main.min.{}.css", hash_hex);
                    let output_path = output_css_dir.join(css_file_name);
                    println!("  Writing CSS file: {}", output_path.display());
                    fs::write(output_path, minified_css)?;

                    // 写入 map 文件（简化版）
                    let map_file_name = "main.css.map";
                    let map_content = format!(
                        "{{\"version\":3,\"sources\":[],\"names\":[],\"mappings\":\"\",\"file\":\"main.min.{}.css\"}}",
                        hash_hex
                    );
                    let map_output_path = output_css_dir.join(map_file_name);
                    println!("  Writing map file: {}", map_output_path.display());
                    fs::write(map_output_path, map_content)?;
                }
                else {
                    println!("  CSS source directory does not exist: {}", ananke_css_dir.display());
                    // 检查是否存在官方构建的 CSS 文件，如果存在则复制（作为后备）
                    let official_css_dir = source_dir.join("public-official").join("ananke").join("css");
                    println!("  Checking official CSS directory: {}", official_css_dir.display());
                    if official_css_dir.exists() {
                        println!("  Official CSS directory exists, copying files");
                        let output_css_dir = output_dir.join("ananke").join("css");
                        fs::create_dir_all(&output_css_dir)?;

                        for entry in WalkDir::new(official_css_dir) {
                            let entry = entry?;
                            let path = entry.path();
                            if path.is_file() {
                                let file_name = path.file_name().unwrap();
                                let output_path = output_css_dir.join(file_name);
                                println!("  Copying CSS file: {} -> {}", path.display(), output_path.display());
                                fs::copy(path, output_path)?;
                            }
                        }
                    }
                    else {
                        println!("  Official CSS directory does not exist: {}", official_css_dir.display());
                    }
                }
            }
        }

        Ok(())
    }
}
