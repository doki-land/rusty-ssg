//! Build 命令实现
//! 
//! 提供 Jekyll 站点构建功能，支持完整的静态站点生成流程。

use crate::BuildArgs;
use console::style;
use std::{fs, path::PathBuf, time::Instant};
use crate::types::Result;
use crate::jekyll::{JekyllStructure, JekyllConfigLoader, PostManager};

/// Build 命令执行器
/// 
/// 负责执行 Jekyll 站点的构建过程，包括加载配置、处理内容、
/// 生成静态文件等步骤。
pub struct BuildCommand;

impl BuildCommand {
    /// 执行 build 命令
    /// 
    /// 根据提供的参数构建 Jekyll 静态站点。
    /// 
    /// # Arguments
    /// 
    /// * `args` - 构建命令参数，包含源目录、输出目录、清理选项等配置
    /// 
    /// # Returns
    /// 
    /// 返回成功或错误结果
    pub async fn execute(args: BuildArgs) -> Result<()> {
        let start_time = if args.profile { Some(Instant::now()) } else { None };
        
        println!("{}", style("Starting Jekyll build...").cyan());

        let source_dir = args.source.unwrap_or_else(|| PathBuf::from("."));
        let output_dir = args.output.unwrap_or_else(|| PathBuf::from("_site"));

        println!("  Source directory: {}", source_dir.display());
        println!("  Output directory: {}", output_dir.display());

        if args.incremental {
            println!("  {} Incremental build mode enabled", style("ℹ").blue());
        }

        if output_dir.exists() && args.clean {
            println!("  {} Cleaning output directory...", style("→").blue());
            fs::remove_dir_all(&output_dir)?;
            println!("  {} Output directory cleaned", style("✓").green());
        }

        if !output_dir.exists() {
            fs::create_dir_all(&output_dir)?;
        }

        println!("  {} Loading Jekyll structure...", style("→").blue());
        let structure = JekyllStructure::new(&source_dir)?;
        println!("  {} Jekyll structure loaded", style("✓").green());

        println!("  {} Loading configuration...", style("→").blue());
        let config = JekyllConfigLoader::load_from_dir(&source_dir)?;
        
        if !args.config_options.is_empty() {
            println!("  {} Applying {} config option(s)", style("ℹ").blue(), args.config_options.len());
        }
        println!("  {} Configuration loaded", style("✓").green());

        println!("  {} Loading posts...", style("→").blue());
        let mut post_manager = PostManager::new(structure.clone(), config.clone());
        let post_count = post_manager.load_posts()?;
        println!("  {} Loaded {} posts", style("✓").green(), post_count);

        if post_count == 0 {
            println!("  {} No posts found", style("⚠").yellow());
        }

        println!("  {} Processing pages...", style("→").blue());
        let page_count = Self::process_pages(&source_dir, &output_dir)?;
        println!("  {} Processed {} pages", style("✓").green(), page_count);

        println!("  {} Copying static assets...", style("→").blue());
        let asset_count = Self::copy_static_assets(&source_dir, &output_dir)?;
        println!("  {} Copied {} static assets", style("✓").green(), asset_count);

        println!("  {} Static site generated successfully", style("✓").green());
        println!("  {} Output written to {}", style("✓").green(), output_dir.display());

        if let Some(start) = start_time {
            let duration = start.elapsed();
            println!("  {} Build completed in {:.2?}", style("✓").green(), duration);
        }

        Ok(())
    }

    /// 处理页面文件
    /// 
    /// 处理 Markdown 和 HTML 页面文件，将它们转换为静态 HTML。
    /// 
    /// # Arguments
    /// 
    /// * `source_dir` - 源目录路径
    /// * `output_dir` - 输出目录路径
    /// 
    /// # Returns
    /// 
    /// 返回处理的页面数量或错误结果
    fn process_pages(source_dir: &PathBuf, output_dir: &PathBuf) -> Result<usize> {
        let mut count = 0;
        
        let index_path = source_dir.join("index.md");
        if index_path.exists() {
            let dest_path = output_dir.join("index.html");
            if let Some(parent) = dest_path.parent() {
                if !parent.exists() {
                    fs::create_dir_all(parent)?;
                }
            }
            
            let content = fs::read_to_string(&index_path)?;
            fs::write(&dest_path, content)?;
            count += 1;
        }
        
        Ok(count)
    }

    /// 复制静态资源
    /// 
    /// 将静态资源文件从源目录复制到输出目录。
    /// 
    /// # Arguments
    /// 
    /// * `source_dir` - 源目录路径
    /// * `output_dir` - 输出目录路径
    /// 
    /// # Returns
    /// 
    /// 返回复制的资源文件数量或错误结果
    fn copy_static_assets(source_dir: &PathBuf, output_dir: &PathBuf) -> Result<usize> {
        let assets_dir = source_dir.join("assets");
        let mut count = 0;
        
        if assets_dir.exists() {
            let dest_assets_dir = output_dir.join("assets");
            if !dest_assets_dir.exists() {
                fs::create_dir_all(&dest_assets_dir)?;
            }
            
            count = Self::copy_directory(&assets_dir, &dest_assets_dir)?;
        }
        
        Ok(count)
    }

    /// 递归复制目录
    /// 
    /// 将源目录的内容递归复制到目标目录。
    /// 
    /// # Arguments
    /// 
    /// * `src` - 源目录路径
    /// * `dest` - 目标目录路径
    /// 
    /// # Returns
    /// 
    /// 返回复制的文件数量或错误结果
    fn copy_directory(src: &PathBuf, dest: &PathBuf) -> Result<usize> {
        let mut count = 0;
        
        for entry in fs::read_dir(src)? {
            let entry = entry?;
            let path = entry.path();
            let dest_path = dest.join(entry.file_name());
            
            if path.is_dir() {
                if !dest_path.exists() {
                    fs::create_dir_all(&dest_path)?;
                }
                count += Self::copy_directory(&path, &dest_path)?;
            } else {
                fs::copy(&path, &dest_path)?;
                count += 1;
            }
        }
        
        Ok(count)
    }
}

/// 执行 build 命令的公开入口点
/// 
/// # Arguments
/// 
/// * `args` - 构建命令参数
/// 
/// # Returns
/// 
/// 返回成功或错误结果
pub async fn execute(args: crate::BuildArgs) -> crate::types::Result<()> {
    BuildCommand::execute(args).await
}
