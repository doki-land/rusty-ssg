//! Check 命令实现
//! 
//! 提供 Jekyll 站点检查功能，验证配置文件、内容文件和目录结构的有效性。

use crate::CheckArgs;
use console::style;
use std::{fs, path::PathBuf};
use crate::types::Result;
use crate::jekyll::{JekyllStructure, JekyllConfigLoader, PostManager};

/// Check 命令执行器
/// 
/// 负责检查 Jekyll 站点的配置、内容和目录结构的有效性。
pub struct CheckCommand;

impl CheckCommand {
    /// 执行 check 命令
    /// 
    /// 根据提供的参数检查 Jekyll 站点的有效性。
    /// 
    /// # Arguments
    /// 
    /// * `args` - 检查命令参数，包含检查范围、详细程度等配置
    /// 
    /// # Returns
    /// 
    /// 返回成功或错误结果
    pub async fn execute(args: CheckArgs) -> Result<()> {
        println!("{}", style("Checking Jekyll site...").cyan());

        let source_dir = args.source.unwrap_or_else(|| PathBuf::from("."));

        println!("  Source directory: {}", source_dir.display());
        
        let mut total_errors = 0;
        let mut total_warnings = 0;

        if !args.content_only {
            println!("  {} Checking configuration...", style("→").blue());
            match Self::check_configuration(&source_dir, args.detailed) {
                Ok(warnings) => {
                    total_warnings += warnings;
                    println!("  {} Configuration is valid", style("✓").green());
                }
                Err(e) => {
                    total_errors += 1;
                    println!("    {} Configuration error: {}", style("✗").red(), e);
                }
            }
        }

        if !args.config_only {
            println!("  {} Checking Jekyll structure...", style("→").blue());
            let structure = JekyllStructure::new(&source_dir)?;
            println!("  {} Jekyll structure loaded", style("✓").green());

            println!("  {} Checking directory structure...", style("→").blue());
            let missing_dirs = Self::check_directory_structure(&source_dir, args.detailed);
            if !missing_dirs.is_empty() {
                total_warnings += missing_dirs.len();
            }

            println!("  {} Checking posts...", style("→").blue());
            let config = JekyllConfigLoader::load_from_dir(&source_dir)?;
            let mut post_manager = PostManager::new(structure.clone(), config);
            let post_result = post_manager.load_posts();
            match post_result {
                Ok(count) => {
                    println!("  {} Checked {} posts", style("✓").green(), count);
                }
                Err(e) => {
                    total_errors += 1;
                    println!("    {} Post error: {}", style("✗").red(), e);
                }
            }

            println!("  {} Checking content files...", style("→").blue());
            let (content_errors, content_warnings) = Self::check_content_files(&source_dir, &structure, args.drafts, args.detailed);
            total_errors += content_errors;
            total_warnings += content_warnings;
        }

        println!("{}", style("Site check complete!").green());

        if total_errors > 0 {
            println!("  {} Found {} error(s)", style("✗").red(), total_errors);
        }
        if total_warnings > 0 {
            println!("  {} Found {} warning(s)", style("⚠").yellow(), total_warnings);
        }
        if total_errors == 0 && total_warnings == 0 {
            println!("  {} No issues found", style("✓").green());
        }

        Ok(())
    }

    /// 检查配置文件
    /// 
    /// 验证 _config.yml 文件的存在性和有效性。
    /// 
    /// # Arguments
    /// 
    /// * `source_dir` - 源目录路径
    /// * `detailed` - 是否显示详细信息
    /// 
    /// # Returns
    /// 
    /// 返回警告数量或错误结果
    fn check_configuration(source_dir: &PathBuf, detailed: bool) -> Result<usize> {
        let config_path = source_dir.join("_config.yml");
        
        if !config_path.exists() {
            println!("    {} Configuration file _config.yml not found", style("✗").red());
            return Ok(1);
        }

        let mut warnings = 0;
        
        let content = fs::read_to_string(&config_path)?;
        
        if detailed {
            println!("    {} Configuration file exists", style("✓").green());
        }

        if content.trim().is_empty() {
            warnings += 1;
            if detailed {
                println!("    {} Configuration file is empty", style("⚠").yellow());
            }
        }

        Ok(warnings)
    }

    /// 检查目录结构
    /// 
    /// 验证 Jekyll 标准目录是否存在。
    /// 
    /// # Arguments
    /// 
    /// * `source_dir` - 源目录路径
    /// * `detailed` - 是否显示详细信息
    /// 
    /// # Returns
    /// 
    /// 返回缺失的目录列表
    fn check_directory_structure(source_dir: &PathBuf, detailed: bool) -> Vec<&'static str> {
        let required_dirs = ["_posts", "_layouts"];
        let optional_dirs = ["_includes", "_data", "_drafts", "_sass", "assets"];
        let mut missing_dirs = Vec::new();

        for dir in &required_dirs {
            let dir_path = source_dir.join(dir);
            if !dir_path.exists() {
                missing_dirs.push(*dir);
                if detailed {
                    println!("    {} Missing required directory: {}", style("✗").red(), dir);
                }
            } else if detailed {
                println!("    {} Required directory exists: {}", style("✓").green(), dir);
            }
        }

        for dir in &optional_dirs {
            let dir_path = source_dir.join(dir);
            if !dir_path.exists() && detailed {
                println!("    {} Optional directory missing: {}", style("⚠").yellow(), dir);
            } else if detailed {
                println!("    {} Optional directory exists: {}", style("✓").green(), dir);
            }
        }

        if missing_dirs.is_empty() {
            println!("  {} All required directories exist", style("✓").green());
        } else {
            println!("  {} Missing directories: {:?}", style("⚠").yellow(), missing_dirs);
        }

        missing_dirs
    }

    /// 检查内容文件
    /// 
    /// 验证 Markdown 和 HTML 文件的有效性。
    /// 
    /// # Arguments
    /// 
    /// * `source_dir` - 源目录路径
    /// * `structure` - Jekyll 结构对象
    /// * `include_drafts` - 是否包含草稿文件
    /// * `detailed` - 是否显示详细信息
    /// 
    /// # Returns
    /// 
    /// 返回 (错误数量, 警告数量)
    fn check_content_files(
        source_dir: &PathBuf, 
        structure: &JekyllStructure, 
        include_drafts: bool,
        detailed: bool
    ) -> (usize, usize) {
        let mut errors = 0;
        let mut warnings = 0;

        let markdown_files = match structure.collect_markdown_files() {
            Ok(files) => files,
            Err(e) => {
                if detailed {
                    println!("    {} Failed to collect Markdown files: {}", style("✗").red(), e);
                }
                errors += 1;
                return (errors, warnings);
            }
        };

        let mut checked_count = 0;

        for file in &markdown_files {
            let file_name = file.file_name().and_then(|n| n.to_str()).unwrap_or("");
            
            if !include_drafts {
                let drafts_dir = source_dir.join("_drafts");
                if file.starts_with(&drafts_dir) {
                    continue;
                }
            }

            checked_count += 1;

            match fs::read_to_string(file) {
                Ok(content) => {
                    if content.trim().is_empty() {
                        warnings += 1;
                        if detailed {
                            println!("    {} Empty file: {}", style("⚠").yellow(), file.display());
                        }
                    } else if detailed {
                        println!("    {} Valid file: {}", style("✓").green(), file.display());
                    }
                }
                Err(e) => {
                    errors += 1;
                    if detailed {
                        println!("    {} Error reading {}: {}", style("✗").red(), file.display(), e);
                    }
                }
            }
        }

        if errors == 0 && warnings == 0 {
            println!("  {} Checked {} content files, all valid", style("✓").green(), checked_count);
        } else {
            println!("  {} Checked {} content files", style("ℹ").blue(), checked_count);
        }

        (errors, warnings)
    }
}

/// 执行 check 命令的公开入口点
/// 
/// # Arguments
/// 
/// * `args` - 检查命令参数
/// 
/// # Returns
/// 
/// 返回成功或错误结果
pub async fn execute(args: crate::CheckArgs) -> crate::types::Result<()> {
    CheckCommand::execute(args).await
}
