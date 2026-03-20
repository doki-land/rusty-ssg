//! 生成命令实现

use super::super::GenerateArgs;
use crate::{markdown::renderer::render_markdown_file, types::Result};
use std::{
    fs,
    path::{Path, PathBuf},
    time::Instant,
};

/// 生成命令
pub struct GenerateCommand;

impl GenerateCommand {
    /// 执行生成命令
    pub async fn execute(args: GenerateArgs) -> Result<()> {
        // 确定源目录和输出目录
        let source_dir = args.source.unwrap_or_else(|| PathBuf::from("source"));
        let output_dir = args.output.unwrap_or_else(|| PathBuf::from("public"));

        // 清理输出目录
        if args.clean {
            if output_dir.exists() {
                fs::remove_dir_all(&output_dir)?;
            }
        }

        // 创建输出目录
        fs::create_dir_all(&output_dir)?;

        // 开始计时
        let start_time = Instant::now();

        println!("Generating static files...");

        // 处理源目录中的文件
        if source_dir.exists() {
            Self::process_dir(&source_dir, &output_dir)?;
        }

        // 结束计时
        let elapsed = start_time.elapsed();
        let compile_time_ms = elapsed.as_millis() as u64;

        println!("Generated successfully in {} ms", compile_time_ms);
        Ok(())
    }

    /// 处理目录
    fn process_dir(src: &PathBuf, dst: &PathBuf) -> Result<()> {
        for entry in fs::read_dir(src)? {
            let entry = entry?;
            let path = entry.path();
            let file_name = path.file_name().unwrap();
            let dest_path = dst.join(file_name);

            if path.is_dir() {
                fs::create_dir_all(&dest_path)?;
                Self::process_dir(&path, &dest_path)?;
            }
            else {
                Self::process_file(&path, &dest_path)?;
            }
        }
        Ok(())
    }

    /// 处理文件
    fn process_file(src: &Path, dst: &Path) -> Result<()> {
        if let Some(ext) = src.extension() {
            if ext == "md" || ext == "markdown" {
                // 处理 Markdown 文件
                Self::process_markdown_file(src, dst)?;
            }
            else {
                // 复制其他文件
                fs::copy(src, dst)?;
            }
        }
        else {
            // 复制无扩展名文件
            fs::copy(src, dst)?;
        }
        Ok(())
    }

    /// 处理 Markdown 文件
    fn process_markdown_file(src: &Path, dst: &Path) -> Result<()> {
        // 渲染 Markdown 文件
        let (front_matter, html) = render_markdown_file(src)?;

        // 生成对应的 HTML 文件路径
        let html_path = dst.with_extension("html");

        // 生成 HTML 内容
        let html_content = Self::generate_html_page(&front_matter, &html);

        // 写入 HTML 文件
        fs::write(html_path, html_content)?;

        Ok(())
    }

    /// 生成 HTML 页面
    fn generate_html_page(front_matter: &Option<crate::markdown::parser::FrontMatter>, content: &str) -> String {
        let default_title = String::from("Untitled");
        let title = front_matter.as_ref().and_then(|fm| fm.title.as_ref()).unwrap_or(&default_title);

        format!(
            r#"
<!DOCTYPE html>
<html lang="zh-CN">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{}</title>
    <style>
        body {{
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, Cantarell, sans-serif;
            line-height: 1.6;
            color: #333;
            max-width: 800px;
            margin: 0 auto;
            padding: 20px;
        }}
        h1, h2, h3, h4, h5, h6 {{
            color: #2c3e50;
        }}
        code {{
            background: #f4f4f4;
            padding: 2px 4px;
            border-radius: 3px;
        }}
        pre {{
            background: #f4f4f4;
            padding: 10px;
            border-radius: 5px;
            overflow-x: auto;
        }}
        pre code {{
            background: none;
            padding: 0;
        }}
        blockquote {{
            border-left: 4px solid #3498db;
            padding-left: 16px;
            margin: 16px 0;
            color: #666;
        }}
    </style>
</head>
<body>
    <header>
        <h1>{}</h1>
    </header>
    <main>
        {}
    </main>
</body>
</html>
"#,
            title, title, content
        )
    }
}
