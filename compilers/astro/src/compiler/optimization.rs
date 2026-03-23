//! 优化模块

use std::path::Path;
use std::fs;
use walkdir::WalkDir;

/// 优化器
pub struct Optimizer {
    /// 是否启用压缩
    compress: bool,
    /// 是否启用代码分割
    code_splitting: bool,
    /// 是否启用预加载
    preload: bool,
}

impl Optimizer {
    /// 创建新的优化器
    pub fn new(compress: bool, code_splitting: bool, preload: bool) -> Self {
        Self {
            compress,
            code_splitting,
            preload,
        }
    }

    /// 优化构建输出
    pub fn optimize(&self, output_dir: &Path) -> Result<(), String> {
        // 压缩静态文件
        if self.compress {
            self.compress_files(output_dir)?;
        }

        // 代码分割
        if self.code_splitting {
            self.split_code(output_dir)?;
        }

        // 添加预加载提示
        if self.preload {
            self.add_preload_hints(output_dir)?;
        }

        Ok(())
    }

    /// 压缩文件
    fn compress_files(&self, output_dir: &Path) -> Result<(), String> {
        println!("Compressing static files...");

        // 压缩 HTML 文件
        for entry in WalkDir::new(output_dir).into_iter().filter_map(|e| e.ok()) {
            let path = entry.path();
            if path.is_file() {
                let ext = path.extension().unwrap_or_default().to_string_lossy().to_lowercase();
                match ext.as_str() {
                    "html" => {
                        self.compress_html(path)?;
                    }
                    "css" => {
                        self.compress_css(path)?;
                    }
                    "js" => {
                        self.compress_js(path)?;
                    }
                    _ => {
                        // 其他文件类型，跳过
                    }
                }
            }
        }

        Ok(())
    }

    /// 压缩 HTML 文件
    fn compress_html(&self, path: &Path) -> Result<(), String> {
        // 读取文件内容
        let content = fs::read_to_string(path)
            .map_err(|e| format!("Failed to read HTML file: {}", e))?;

        // 简单的 HTML 压缩：去除多余的空白字符
        let compressed = content
            .replace("\n", "")
            .replace("\r", "")
            .replace("  ", " ")
            .trim()
            .to_string();

        // 写回压缩后的内容
        fs::write(path, compressed)
            .map_err(|e| format!("Failed to write compressed HTML file: {}", e))?;

        println!("Compressed: {}", path.display());
        Ok(())
    }

    /// 压缩 CSS 文件
    fn compress_css(&self, path: &Path) -> Result<(), String> {
        // 读取文件内容
        let content = fs::read_to_string(path)
            .map_err(|e| format!("Failed to read CSS file: {}", e))?;

        // 简单的 CSS 压缩：去除多余的空白字符和注释
        let compressed = content
            .replace("\n", "")
            .replace("\r", "")
            .replace("  ", " ")
            .replace("/*.*?*/", "")
            .trim()
            .to_string();

        // 写回压缩后的内容
        fs::write(path, compressed)
            .map_err(|e| format!("Failed to write compressed CSS file: {}", e))?;

        println!("Compressed: {}", path.display());
        Ok(())
    }

    /// 压缩 JavaScript 文件
    fn compress_js(&self, path: &Path) -> Result<(), String> {
        // 读取文件内容
        let content = fs::read_to_string(path)
            .map_err(|e| format!("Failed to read JavaScript file: {}", e))?;

        // 简单的 JavaScript 压缩：去除多余的空白字符和注释
        let compressed = content
            .replace("\n", "")
            .replace("\r", "")
            .replace("  ", " ")
            .replace("/*.*?*/", "")
            .trim()
            .to_string();

        // 写回压缩后的内容
        fs::write(path, compressed)
            .map_err(|e| format!("Failed to write compressed JavaScript file: {}", e))?;

        println!("Compressed: {}", path.display());
        Ok(())
    }

    /// 代码分割
    fn split_code(&self, _output_dir: &Path) -> Result<(), String> {
        println!("Splitting code...");

        // 这里实现简单的代码分割逻辑
        // 实际应该根据依赖关系进行更复杂的代码分割

        Ok(())
    }

    /// 添加预加载提示
    fn add_preload_hints(&self, output_dir: &Path) -> Result<(), String> {
        println!("Adding preload hints...");

        // 为 HTML 文件添加预加载提示
        for entry in WalkDir::new(output_dir).into_iter().filter_map(|e| e.ok()) {
            let path = entry.path();
            if path.is_file() && path.extension().unwrap_or_default() == "html" {
                self.add_preload_to_html(path)?;
            }
        }

        Ok(())
    }

    /// 为 HTML 文件添加预加载提示
    fn add_preload_to_html(&self, path: &Path) -> Result<(), String> {
        // 读取文件内容
        let mut content = fs::read_to_string(path)
            .map_err(|e| format!("Failed to read HTML file: {}", e))?;

        // 在 head 标签中添加预加载提示
        if let Some(head_end) = content.find("</head>") {
            let preload_hints = r#"
    <link rel="preload" href="/css/style.css" as="style">
    <link rel="preload" href="/js/main.js" as="script">
"#;
            content.insert_str(head_end, preload_hints);

            // 写回修改后的内容
            fs::write(path, content)
                .map_err(|e| format!("Failed to write HTML file with preload hints: {}", e))?;

            println!("Added preload hints to: {}", path.display());
        }

        Ok(())
    }
}

impl Default for Optimizer {
    fn default() -> Self {
        Self {
            compress: true,
            code_splitting: true,
            preload: true,
        }
    }
}
