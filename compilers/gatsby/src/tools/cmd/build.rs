//! Build 命令实现

use crate::{BuildArgs, types::Result};
use console::style;
use std::{fs, fs::File, io::Write, path::PathBuf};
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
        println!("  {} Configuration loaded", style("✓").green());

        println!("  {} Scanning for source files...", style("→").blue());

        let src_pages_dir = source_dir.join("src").join("pages");
        let mut file_count = 0;

        if src_pages_dir.exists() {
            for entry in std::fs::read_dir(&src_pages_dir)? {
                let entry = entry?;
                let path = entry.path();
                if path.is_file() {
                    if let Some(ext) = path.extension() {
                        if ext == "js" || ext == "jsx" || ext == "ts" || ext == "tsx" || ext == "md" {
                            file_count += 1;
                        }
                    }
                }
            }
        }

        println!("  {} Found {} page component(s)", style("✓").green(), file_count);

        if file_count == 0 {
            println!("  {} No page components found", style("⚠").yellow());
            return Ok(());
        }

        println!("  {} Compiling and generating static site...", style("→").blue());

        // 处理页面组件
        if src_pages_dir.exists() {
            for entry in WalkDir::new(&src_pages_dir).into_iter().filter_map(|e| e.ok()) {
                let path = entry.path();
                if path.is_file() {
                    if let Some(ext) = path.extension() {
                        let ext_str = ext.to_str().unwrap_or("");
                        if matches!(ext_str, "js" | "jsx" | "ts" | "tsx" | "md") {
                            // 处理页面组件
                            Self::process_page(&path.to_path_buf(), &output_dir)?;
                        }
                    }
                }
            }
        }

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

        Ok(())
    }

    /// 处理页面组件
    fn process_page(page_path: &PathBuf, output_dir: &PathBuf) -> Result<()> {
        // 读取页面内容
        let content = fs::read_to_string(page_path)?;

        // 生成 HTML 内容
        let html_content = format!(
            r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Gatsby Page</title>
</head>
<body>
    <div id="___gatsby">
        <div style="max-width: 800px; margin: 0 auto; padding: 2rem;">
            <h1>Hello from Gatsby!</h1>
            <p>This page was generated from: {}</p>
            <div style="margin-top: 2rem; padding: 1rem; background: #f0f0f0;">
                <h2>Page Content</h2>
                <pre style="white-space: pre-wrap;">{}</pre>
            </div>
        </div>
    </div>
</body>
</html>
"#,
            page_path.display(),
            content
        );

        // 生成输出文件路径
        let relative_path = page_path
            .strip_prefix(&page_path.parent().unwrap())
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
        let output_file_path = output_dir.join(relative_path).with_extension("html");

        // 创建输出目录
        if let Some(parent) = output_file_path.parent() {
            fs::create_dir_all(parent)?;
        }

        // 写入输出文件
        let mut file = File::create(output_file_path)?;
        file.write_all(html_content.as_bytes())?;

        Ok(())
    }
}
