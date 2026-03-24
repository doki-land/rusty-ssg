//! Build 命令实现

use crate::{
    BuildArgs, MkDocsCompiler,
    tools::{ConfigLoader, StaticSiteGenerator},
    types::Result,
};
use console::style;
use std::{collections::HashMap, fs, path::PathBuf};
use walkdir::WalkDir;

/// Build 命令
pub struct BuildCommand;

impl BuildCommand {
    /// 执行 build 命令
    pub async fn execute(args: BuildArgs) -> Result<()> {
        println!("{}", style("Starting MkDocs build...").cyan());

        let source_dir = args.source.unwrap_or_else(|| PathBuf::from("."));
        let output_dir = args.destination;

        println!("  Source directory: {}", source_dir.display());
        println!("  Output directory: {}", output_dir.display());

        if args.clean || output_dir.exists() {
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

        let docs_dir = source_dir.join("docs");
        let search_dir = if docs_dir.exists() { docs_dir } else { source_dir.clone() };

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

        if file_count == 0 {
            println!("  {} No Markdown files found", style("⚠").yellow());
            return Ok(());
        }

        println!("  {} Compiling documents...", style("→").blue());

        let compiler = MkDocsCompiler::new(config.clone(), &source_dir, &output_dir);
        let compile_results = compiler.compile_all()?;

        if !compile_results.is_empty() {
            let total_time = compile_results.iter().sum::<u64>();
            println!("  {} Compiled {} documents in {}ms", style("✓").green(), compile_results.len(), total_time);

            println!("  {} Generating static site...", style("→").blue());

            let mut site_generator = StaticSiteGenerator::new(config)?;
            site_generator.generate(&documents, &output_dir)?;

            println!("  {} Static site generated successfully", style("✓").green());
            println!("  {} Output written to {}", style("✓").green(), output_dir.display());
        }
        else {
            println!("  {} Compilation failed", style("✗").red());
        }

        Ok(())
    }
}
