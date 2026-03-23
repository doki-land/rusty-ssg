//! 路由解析模块

use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// 路由解析器，用于解析 Astro 项目的路由
pub struct RouteResolver {
    /// 页面目录路径
    pages_dir: PathBuf,
    /// 布局目录路径
    layouts_dir: PathBuf,
    /// 路由映射
    routes: Vec<Route>,
}

/// 路由定义
struct Route {
    /// 路由模式
    pattern: String,
    /// 对应的文件路径
    file_path: PathBuf,
    /// 布局文件路径
    layout_path: Option<PathBuf>,
    /// 参数名
    param_names: Vec<String>,
}

impl RouteResolver {
    /// 创建新的路由解析器
    pub fn new(pages_dir: &Path, layouts_dir: &Path) -> Self {
        Self {
            pages_dir: pages_dir.to_path_buf(),
            layouts_dir: layouts_dir.to_path_buf(),
            routes: Vec::new(),
        }
    }

    /// 扫描路由
    pub fn scan_routes(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.routes.clear();
        let pages_dir = self.pages_dir.clone();
        self.scan_directory(&pages_dir, "")?;
        Ok(())
    }

    /// 扫描目录
    fn scan_directory(&mut self, dir: &Path, prefix: &str) -> Result<(), Box<dyn std::error::Error>> {
        for entry in std::fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            let file_name_os = entry.file_name();
            let file_name = file_name_os.to_string_lossy();

            if path.is_dir() {
                let new_prefix = if prefix.is_empty() {
                    file_name.to_string()
                } else {
                    format!("{}/{}", prefix, file_name)
                };
                self.scan_directory(&path, &new_prefix)?;
            } else if path.extension().map(|ext| ext == "astro").unwrap_or(false) {
                let route = self.create_route(&path, prefix);
                self.routes.push(route);
            }
        }
        Ok(())
    }

    /// 创建路由
    fn create_route(&self, file_path: &Path, prefix: &str) -> Route {
        let file_name = file_path.file_stem().unwrap().to_string_lossy();
        let mut pattern = if prefix.is_empty() {
            file_name.to_string()
        } else {
            format!("{}/{}", prefix, file_name)
        };

        let mut param_names = Vec::new();

        // 处理参数路由，如 [id].astro
        if pattern.starts_with('[') && pattern.ends_with(']') {
            let param_name = pattern.trim_start_matches('[').trim_end_matches(']');
            param_names.push(param_name.to_string());
            pattern = "*".to_string(); // 简单实现，使用通配符匹配
        }

        // 处理 index.astro 作为根路由
        if pattern.ends_with("/index") {
            pattern = pattern.replace("/index", "");
        } else if pattern == "index" {
            pattern = "".to_string();
        }

        // 构建完整的路由路径
        let route_pattern = if pattern.is_empty() {
            "/".to_string()
        } else {
            format!("/{}", pattern)
        };

        // 查找布局文件
        let layout_path = self.find_layout_file(file_path);

        Route {
            pattern: route_pattern,
            file_path: file_path.to_path_buf(),
            layout_path,
            param_names,
        }
    }

    /// 查找布局文件
    fn find_layout_file(&self, file_path: &Path) -> Option<PathBuf> {
        // 简单实现，查找同目录下的 +layout.astro
        let dir = file_path.parent()?;
        let layout_file = dir.join("+layout.astro");
        if layout_file.exists() {
            Some(layout_file)
        } else {
            None
        }
    }

    /// 解析路由
    pub fn resolve(&self, path: &str) -> Option<(PathBuf, Option<PathBuf>, HashMap<String, String>)> {
        let path = path.trim_start_matches('/');
        let path_parts: Vec<&str> = path.split('/').collect();

        for route in &self.routes {
            let route_parts: Vec<&str> = route.pattern.trim_start_matches('/').split('/').collect();

            if route_parts.len() == path_parts.len() {
                let mut params = HashMap::new();
                let mut match_found = true;

                for (i, (route_part, path_part)) in route_parts.iter().zip(path_parts.iter()).enumerate() {
                    if *route_part == "*" && !route.param_names.is_empty() {
                        params.insert(route.param_names[i].clone(), path_part.to_string());
                    } else if *route_part != *path_part {
                        match_found = false;
                        break;
                    }
                }

                if match_found {
                    return Some((route.file_path.clone(), route.layout_path.clone(), params));
                }
            }
        }

        None
    }
}
