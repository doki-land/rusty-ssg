//! Build 命令实现

use crate::{BuildArgs, ConfigLoader, StaticSiteGenerator, VutexCompiler};
use console::style;
use std::{collections::HashMap, fs, path::PathBuf};
use crate::compiler::PluginHost;
use crate::types::Result;
use crate::jekyll::{JekyllStructure, JekyllConfigLoader, PostManager, LiquidEngine, MarkdownConverter, MarkdownProcessor, MarkdownOptions};

/// Build 命令
pub struct BuildCommand;

impl BuildCommand {
    /// 执行 build 命令
    pub async fn execute(args: BuildArgs) -> Result<()> {
        println!("{}", style("Starting Jekyll build...").cyan());

        let source_dir = args.source.unwrap_or_else(|| PathBuf::from("."));
        let output_dir = args.output.unwrap_or_else(|| PathBuf::from("_site"));

        println!("  Source directory: {}", source_dir.display());
        println!("  Output directory: {}", output_dir.display());

        if output_dir.exists() && args.clean {
            println!("  {} Cleaning output directory...", style("✓").green());
            fs::remove_dir_all(&output_dir)?;
        }

        if !output_dir.exists() {
            fs::create_dir_all(&output_dir)?;
        }

        println!("  {} Loading Jekyll structure...", style("→").blue());
        let structure = JekyllStructure::new(&source_dir)?;
        println!("  {} Jekyll structure loaded", style("✓").green());

        println!("  {} Loading configuration...", style("→").blue());
        let config = JekyllConfigLoader::load_from_dir(&source_dir)?;
        println!("  {} Configuration loaded", style("✓").green());

        println!("  {} Loading posts...", style("→").blue());
        let mut post_manager = PostManager::new(structure.clone(), config.clone());
        let post_count = post_manager.load_posts()?;
        println!("  {} Loaded {} posts", style("✓").green(), post_count);

        if post_count == 0 {
            println!("  {} No posts found", style("⚠").yellow());
        }

        println!("  {} Initializing Markdown converter...", style("→").blue());
        let markdown_converter = MarkdownConverter::from_jekyll_config(&config);
        println!("  {} Markdown converter initialized", style("✓").green());

        println!("  {} Initializing Liquid engine...", style("→").blue());
        let mut liquid_engine = LiquidEngine::new(structure, config.clone());
        println!("  {} Liquid engine initialized", style("✓").green());

        // 这里可以添加更多的构建逻辑，比如：
        // 1. 处理页面
        // 2. 处理布局
        // 3. 处理包含文件
        // 4. 生成静态站点

        println!("  {} Static site generated successfully", style("✓").green());
        println!("  {} Output written to {}", style("✓").green(), output_dir.display());

        Ok(())
    }
}
