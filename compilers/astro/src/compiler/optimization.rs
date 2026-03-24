//! 优化模块

use std::{fs, path::Path};
use walkdir::WalkDir;

/// 资源信息
struct Resource {
    path: String,
    as_type: String,
}

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
        Self { compress, code_splitting, preload }
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
        let content = fs::read_to_string(path).map_err(|e| format!("Failed to read HTML file: {}", e))?;

        // 简单的 HTML 压缩：去除多余的空白字符
        let compressed = content.replace("\n", "").replace("\r", "").replace("  ", " ").trim().to_string();

        // 写回压缩后的内容
        fs::write(path, compressed).map_err(|e| format!("Failed to write compressed HTML file: {}", e))?;

        println!("Compressed: {}", path.display());
        Ok(())
    }

    /// 压缩 CSS 文件
    fn compress_css(&self, path: &Path) -> Result<(), String> {
        // 读取文件内容
        let content = fs::read_to_string(path).map_err(|e| format!("Failed to read CSS file: {}", e))?;

        // 简单的 CSS 压缩：去除多余的空白字符和注释
        let compressed =
            content.replace("\n", "").replace("\r", "").replace("  ", " ").replace("/*.*?*/", "").trim().to_string();

        // 写回压缩后的内容
        fs::write(path, compressed).map_err(|e| format!("Failed to write compressed CSS file: {}", e))?;

        println!("Compressed: {}", path.display());
        Ok(())
    }

    /// 压缩 JavaScript 文件
    fn compress_js(&self, path: &Path) -> Result<(), String> {
        // 读取文件内容
        let content = fs::read_to_string(path).map_err(|e| format!("Failed to read JavaScript file: {}", e))?;

        // 简单的 JavaScript 压缩：去除多余的空白字符和注释
        let compressed =
            content.replace("\n", "").replace("\r", "").replace("  ", " ").replace("/*.*?*/", "").trim().to_string();

        // 写回压缩后的内容
        fs::write(path, compressed).map_err(|e| format!("Failed to write compressed JavaScript file: {}", e))?;

        println!("Compressed: {}", path.display());
        Ok(())
    }

    /// 代码分割
    fn split_code(&self, output_dir: &Path) -> Result<(), String> {
        println!("Splitting code...");

        // 为 JavaScript 文件创建共享模块
        self.create_shared_modules(output_dir)?;

        // 分析并分割页面代码
        self.split_page_code(output_dir)?;

        Ok(())
    }

    /// 创建共享模块
    fn create_shared_modules(&self, output_dir: &Path) -> Result<(), String> {
        // 查找所有 JavaScript 文件
        let js_files: Vec<_> = WalkDir::new(output_dir)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.path().is_file())
            .filter(|e| e.path().extension().unwrap_or_default() == "js")
            .map(|e| e.path().to_path_buf())
            .collect();

        if js_files.is_empty() {
            return Ok(());
        }

        // 分析共享代码
        let mut shared_code = String::new();
        // 这里可以实现更复杂的共享代码分析逻辑
        // 例如，提取重复的函数和变量

        // 创建共享模块目录
        let shared_dir = output_dir.join("js").join("shared");
        if let Err(err) = fs::create_dir_all(&shared_dir) {
            return Err(format!("Failed to create shared directory: {}", err));
        }

        // 写入共享模块
        let shared_file = shared_dir.join("shared.js");
        if !shared_code.is_empty() {
            if let Err(err) = fs::write(&shared_file, shared_code) {
                return Err(format!("Failed to write shared module: {}", err));
            }
            println!("Created shared module: {}", shared_file.display());
        }

        Ok(())
    }

    /// 分割页面代码
    fn split_page_code(&self, output_dir: &Path) -> Result<(), String> {
        // 查找所有 HTML 文件
        let html_files: Vec<_> = WalkDir::new(output_dir)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.path().is_file())
            .filter(|e| e.path().extension().unwrap_or_default() == "html")
            .map(|e| e.path().to_path_buf())
            .collect();

        for html_path in html_files {
            // 读取 HTML 文件内容
            let mut content = fs::read_to_string(&html_path).map_err(|e| format!("Failed to read HTML file: {}", e))?;

            // 提取内联脚本并分割到单独的文件
            self.extract_inline_scripts(&html_path, &mut content)?;

            // 写回修改后的内容
            if let Err(err) = fs::write(&html_path, content) {
                return Err(format!("Failed to write HTML file: {}", err));
            }
        }

        Ok(())
    }

    /// 提取内联脚本
    fn extract_inline_scripts(&self, html_path: &Path, content: &mut String) -> Result<(), String> {
        // 简单的内联脚本提取逻辑
        // 实际应该使用更复杂的 HTML 解析器
        let mut scripts = Vec::new();
        let mut start = 0;

        while let Some(script_start) = content[start..].find("<script>") {
            let script_start = start + script_start + 8; // 8 是 "<script>" 的长度
            if let Some(script_end) = content[script_start..].find("</script>") {
                let script_end = script_start + script_end;
                let script_content = content[script_start..script_end].trim().to_string();
                scripts.push(script_content);
                start = script_end + 9; // 9 是 "</script>" 的长度
            } else {
                break;
            }
        }

        if scripts.is_empty() {
            return Ok(());
        }

        // 创建脚本目录
        let js_dir = html_path.parent().unwrap().join("js");
        if let Err(err) = fs::create_dir_all(&js_dir) {
            return Err(format!("Failed to create js directory: {}", err));
        }

        // 写入脚本文件并更新 HTML
        for (i, script_content) in scripts.iter().enumerate() {
            let script_file = js_dir.join(format!("script_{}.js", i));
            if let Err(err) = fs::write(&script_file, script_content) {
                return Err(format!("Failed to write script file: {}", err));
            }

            // 生成相对路径
            let relative_path = script_file.strip_prefix(html_path.parent().unwrap()).unwrap();
            let script_tag = format!(r#"<script src="{}"></script>"#, relative_path.display());

            // 替换内联脚本
            if let Some(script_start) = content.find("<script>") {
                let script_end = content[script_start..].find("</script>").unwrap() + script_start + 9;
                content.replace_range(script_start..script_end, &script_tag);
            }

            println!("Extracted script to: {}", script_file.display());
        }

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
        let mut content = fs::read_to_string(path).map_err(|e| format!("Failed to read HTML file: {}", e))?;

        // 提取需要预加载的资源
        let resources = self.extract_resources(&content, path);

        // 在 head 标签中添加预加载提示
        if let Some(head_end) = content.find("</head>") {
            let mut preload_hints = String::new();

            for resource in resources {
                preload_hints.push_str(&format!("    <link rel=\"preload\" href=\"{}\" as=\"{}\">
", resource.path, resource.as_type));
            }

            if !preload_hints.is_empty() {
                content.insert_str(head_end, &preload_hints);

                // 写回修改后的内容
                fs::write(path, content).map_err(|e| format!("Failed to write HTML file with preload hints: {}", e))?;

                println!("Added preload hints to: {}", path.display());
            }
        }

        Ok(())
    }



    /// 提取需要预加载的资源
    fn extract_resources(&self, content: &str, html_path: &Path) -> Vec<Resource> {
        let mut resources = Vec::new();

        // 提取 CSS 文件
        let css_pattern = regex::Regex::new(r#"<link[^>]+href="([^"]+)"[^>]+rel="stylesheet""#).unwrap();
        for cap in css_pattern.captures_iter(content) {
            if let Some(path) = cap.get(1) {
                let path_str = path.as_str();
                // 跳过外部资源
                if !path_str.starts_with("http://") && !path_str.starts_with("https://") {
                    resources.push(Resource {
                        path: path_str.to_string(),
                        as_type: "style".to_string(),
                    });
                }
            }
        }

        // 提取 JavaScript 文件
        let js_pattern = regex::Regex::new(r#"<script[^>]+src="([^"]+)""#).unwrap();
        for cap in js_pattern.captures_iter(content) {
            if let Some(path) = cap.get(1) {
                let path_str = path.as_str();
                // 跳过外部资源
                if !path_str.starts_with("http://") && !path_str.starts_with("https://") {
                    resources.push(Resource {
                        path: path_str.to_string(),
                        as_type: "script".to_string(),
                    });
                }
            }
        }

        // 提取图片资源
        let img_pattern = regex::Regex::new(r#"<img[^>]+src="([^"]+)""#).unwrap();
        for cap in img_pattern.captures_iter(content) {
            if let Some(path) = cap.get(1) {
                let path_str = path.as_str();
                // 跳过外部资源
                if !path_str.starts_with("http://") && !path_str.starts_with("https://") {
                    resources.push(Resource {
                        path: path_str.to_string(),
                        as_type: "image".to_string(),
                    });
                }
            }
        }

        resources
    }
}

impl Default for Optimizer {
    fn default() -> Self {
        Self { compress: true, code_splitting: true, preload: true }
    }
}
