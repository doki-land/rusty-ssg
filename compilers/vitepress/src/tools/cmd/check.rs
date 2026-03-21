//! Check 命令实现

use crate::{
    compiler::VutexCompiler,
    tools::{CheckArgs, ConfigLoader},
    types::Result,
};
use console::style;
use std::{collections::HashMap, fs, path::PathBuf};
use walkdir::WalkDir;

/// Check 命令
pub struct CheckCommand;

impl CheckCommand {
    /// 执行 check 命令
    pub async fn execute(args: CheckArgs) -> Result<()> {
        println!("{}", style("Checking VuTeX documents...").cyan());

        let source_dir = args.source.unwrap_or_else(|| PathBuf::from("."));
        println!("  Source directory: {}", source_dir.display());

        let mut errors = Vec::new();
        let mut warnings = Vec::new();

        Self::check_config(&source_dir, &mut errors, &mut warnings)?;
        Self::check_documents(&source_dir, &mut errors, &mut warnings)?;

        if !warnings.is_empty() {
            println!("\n  {} Warnings ({}):", style("⚠").yellow(), warnings.len());
            for warning in &warnings {
                println!("    {}", style(warning).yellow());
            }
        }

        if !errors.is_empty() {
            println!("\n  {} Errors ({}):", style("✗").red(), errors.len());
            for error in &errors {
                println!("    {}", style(error).red());
            }
            println!("\n  {} Check failed with {} errors", style("✗").red(), errors.len());
        }
        else {
            println!("\n  {} All checks passed!", style("✓").green());
        }

        Ok(())
    }

    /// 检查配置文件
    fn check_config(source_dir: &PathBuf, errors: &mut Vec<String>, warnings: &mut Vec<String>) -> Result<()> {
        println!("  {} Checking configuration...", style("→").blue());

        let config_paths = [
            source_dir.join(".vitepress").join("vitepress.config.toml"),
            source_dir.join(".vitepress").join("vitepress.config.json"),
            source_dir.join("vitepress.config.toml"),
            source_dir.join("vitepress.config.json"),
            source_dir.join("vutex.toml"),
            source_dir.join("vutex.json"),
            source_dir.join("config.toml"),
            source_dir.join("config.json"),
        ];

        let config_exists = config_paths.iter().any(|p| p.exists());
        if !config_exists {
            warnings.push("Configuration file not found, using defaults".to_string());
        }

        match ConfigLoader::load_from_dir(source_dir) {
            Ok(config) => {
                if config.title.is_none() {
                    warnings.push("Site title not configured".to_string());
                }
                println!("    {} Configuration is valid", style("✓").green());
            }
            Err(e) => {
                errors.push(format!("Invalid configuration: {}", e));
            }
        }

        Ok(())
    }

    /// 检查文档
    fn check_documents(source_dir: &PathBuf, errors: &mut Vec<String>, warnings: &mut Vec<String>) -> Result<()> {
        println!("  {} Checking documents...", style("→").blue());

        let mut documents = HashMap::new();
        let mut file_count = 0;

        for entry in WalkDir::new(source_dir) {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() {
                if let Some(ext) = path.extension() {
                    if ext == "md" {
                        let rel_path = path.strip_prefix(source_dir).unwrap_or(path).to_string_lossy().to_string();

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

        if file_count == 0 {
            warnings.push("No Markdown documents found".to_string());
            return Ok(());
        }

        println!("    {} Found {} document(s)", style("✓").green(), file_count);

        let config = ConfigLoader::load_from_dir(source_dir).unwrap_or_default();
        let mut compiler = VutexCompiler::with_config(config);
        let result = compiler.compile_batch(&documents);

        if !result.errors.is_empty() {
            for error in result.errors {
                errors.push(error);
            }
        }

        if result.success {
            println!("    {} All documents compiled successfully", style("✓").green());
        }

        Ok(())
    }
}
