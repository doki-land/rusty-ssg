//! Check 命令实现 - 检查文档

use crate::{
    CheckArgs,
    tools::{ConfigLoader, link_validator::LinkValidator},
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
        println!("{}", style("Checking MkDocs project...").cyan());

        let source_dir = args.source.unwrap_or_else(|| PathBuf::from("."));

        println!("  Source directory: {}", source_dir.display());

        // 加载配置
        let config = ConfigLoader::load_from_dir(&source_dir)?;
        let docs_dir = source_dir.join(config.docs_dir());
        let search_dir = if docs_dir.exists() { docs_dir } else { source_dir.clone() };

        // 收集所有 Markdown 文件
        let mut documents = HashMap::new();
        let mut file_count = 0;

        println!("  {} Scanning for Markdown files...", style("→").blue());

        for entry in WalkDir::new(&search_dir) {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() {
                if let Some(ext) = path.extension() {
                    if ext == "md" {
                        let rel_path = path.strip_prefix(&search_dir).unwrap_or(path).to_string_lossy().to_string();
                        let path_components: Vec<&str> = rel_path.split(std::path::MAIN_SEPARATOR).collect();
                        if path_components.iter().any(|&c| c == "node_modules" || c == ".git" || c == "site") {
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

        // 运行链接验证
        println!("  {} Validating links...", style("→").blue());

        let mut validator = LinkValidator::new(config);
        for (path, _) in &documents {
            validator.add_file(path);
        }

        validator.validate_all(&documents);
        validator.result().print();

        println!("\n{} Check completed!", style("✓").green());

        Ok(())
    }
}
