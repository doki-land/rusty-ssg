//! Init 命令实现 - 初始化项目

use crate::{InitArgs, types::Result};
use console::style;
use std::{fs, path::PathBuf};

/// Init 命令
pub struct InitCommand;

impl InitCommand {
    /// 执行 init 命令
    pub async fn execute(args: InitArgs) -> Result<()> {
        println!("{}", style("Initializing MkDocs project...").cyan());

        let target_dir = args.directory.unwrap_or_else(|| PathBuf::from("."));
        let project_name = args
            .name
            .unwrap_or_else(|| target_dir.file_name().and_then(|s| s.to_str()).unwrap_or("mkdocs-project").to_string());

        println!("  Project name: {}", project_name);
        println!("  Target directory: {}", target_dir.display());

        if !target_dir.exists() {
            fs::create_dir_all(&target_dir)?;
            println!("  {} Created directory: {}", style("✓").green(), target_dir.display());
        }

        let mkdocs_yml_path = target_dir.join("mkdocs.yml");
        if !mkdocs_yml_path.exists() {
            let mkdocs_yml_content = format!(
                r#"site_name: {}
site_url: https://example.com/
nav:
  - Home: index.md
theme:
  name: material
"#,
                project_name
            );
            fs::write(&mkdocs_yml_path, mkdocs_yml_content)?;
            println!("  {} Created mkdocs.yml", style("✓").green());
        }
        else {
            println!("  {} mkdocs.yml already exists", style("⚠").yellow());
        }

        let docs_dir = target_dir.join("docs");
        if !docs_dir.exists() {
            fs::create_dir_all(&docs_dir)?;
            println!("  {} Created docs directory", style("✓").green());
        }

        let index_md_path = docs_dir.join("index.md");
        if !index_md_path.exists() {
            let index_md_content = format!(
                r#"# Welcome to {}

This is your new MkDocs project.
"#,
                project_name
            );
            fs::write(&index_md_path, index_md_content)?;
            println!("  {} Created docs/index.md", style("✓").green());
        }
        else {
            println!("  {} docs/index.md already exists", style("⚠").yellow());
        }

        println!("\n{}", style("Project initialized successfully!").green());
        println!("  {}", style("Next steps:").blue());
        println!("    1. cd into your project directory");
        println!("    2. Run 'mkdocs serve' to start a development server");
        println!("    3. Edit docs/index.md to customize your homepage");

        Ok(())
    }
}
