//! 路由系统模块

use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

/// 路由参数类型
pub type RouteParams = HashMap<String, String>;

/// 路由节点
#[derive(Debug, Clone)]
pub struct RouteNode {
    /// 路由路径段
    pub segment: String,
    /// 是否为参数段
    pub is_param: bool,
    /// 子路由
    pub children: Vec<RouteNode>,
    /// 对应文件路径
    pub file_path: Option<PathBuf>,
    /// 布局文件路径
    pub layout_path: Option<PathBuf>,
}

impl RouteNode {
    /// 创建新的路由节点
    pub fn new(segment: &str, is_param: bool) -> Self {
        Self { segment: segment.to_string(), is_param, children: Vec::new(), file_path: None, layout_path: None }
    }

    /// 添加子路由
    pub fn add_child(&mut self, child: RouteNode) {
        self.children.push(child);
    }

    /// 查找子路由
    pub fn find_child(&self, segment: &str) -> Option<&RouteNode> {
        self.children.iter().find(|child| child.segment == segment || child.is_param)
    }
}

/// 路由树
#[derive(Debug, Clone)]
pub struct RouteTree {
    /// 根节点
    pub root: RouteNode,
}

impl RouteTree {
    /// 创建新的路由树
    pub fn new() -> Self {
        Self { root: RouteNode::new("", false) }
    }

    /// 添加路由
    pub fn add_route(&mut self, path: &Path, file_path: PathBuf, layout_path: Option<PathBuf>) {
        let mut current = &mut self.root;

        for segment in path.iter() {
            let segment_str = segment.to_string_lossy();
            let is_param = segment_str.starts_with('[') && segment_str.ends_with(']');
            let clean_segment =
                if is_param { segment_str.trim_matches(|c| c == '[' || c == ']').to_string() } else { segment_str.to_string() };

            let child_index =
                current.children.iter().position(|child| child.segment == clean_segment && child.is_param == is_param);

            if let Some(index) = child_index {
                current = &mut current.children[index];
            }
            else {
                let new_child = RouteNode::new(&clean_segment, is_param);
                current.children.push(new_child);
                current = current.children.last_mut().unwrap();
            }
        }

        current.file_path = Some(file_path);
        current.layout_path = layout_path;
    }

    /// 匹配路由
    pub fn match_route(&self, path: &str) -> Option<(PathBuf, Option<PathBuf>, RouteParams)> {
        let segments: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();
        let mut current = &self.root;
        let mut params = RouteParams::new();

        for segment in segments {
            if let Some(child) = current.find_child(segment) {
                if child.is_param {
                    params.insert(child.segment.clone(), segment.to_string());
                }
                current = child;
            }
            else {
                return None;
            }
        }

        if let Some(file_path) = &current.file_path {
            Some((file_path.clone(), current.layout_path.clone(), params))
        }
        else {
            None
        }
    }
}

/// 路由解析器
#[derive(Debug, Clone)]
pub struct RouteResolver {
    /// 路由树
    pub route_tree: RouteTree,
    /// 页面目录
    pub pages_dir: PathBuf,
    /// 布局目录
    pub layouts_dir: PathBuf,
}

impl RouteResolver {
    /// 创建新的路由解析器
    pub fn new(pages_dir: &Path, layouts_dir: &Path) -> Self {
        Self { route_tree: RouteTree::new(), pages_dir: pages_dir.to_path_buf(), layouts_dir: layouts_dir.to_path_buf() }
    }

    /// 扫描目录生成路由
    pub fn scan_routes(&mut self) -> Result<(), String> {
        let pages_dir = self.pages_dir.clone();
        self.scan_directory(&pages_dir, PathBuf::new())
    }

    /// 扫描目录
    fn scan_directory(&mut self, dir: &Path, current_path: PathBuf) -> Result<(), String> {
        if !dir.exists() {
            return Ok(());
        }

        for entry in dir.read_dir().map_err(|e| e.to_string())? {
            let entry = entry.map_err(|e| e.to_string())?;
            let path = entry.path();

            if path.is_dir() {
                let mut new_path = current_path.clone();
                new_path.push(path.file_name().unwrap());
                self.scan_directory(&path, new_path)?;
            }
            else if path.extension().map_or(false, |ext| ext == "astro") {
                let mut route_path = current_path.clone();

                // 处理特殊文件：index.astro -> /
                if path.file_name().unwrap() == "index.astro" {
                    // 保持 current_path 不变
                }
                else {
                    let file_stem = path.file_stem().unwrap().to_string_lossy();
                    route_path.push(file_stem.as_ref());
                }

                // 查找对应的布局文件
                let layout_path = self.find_layout(&route_path);

                self.route_tree.add_route(&route_path, path, layout_path);
            }
        }

        Ok(())
    }

    /// 查找布局文件
    fn find_layout(&self, route_path: &Path) -> Option<PathBuf> {
        // 从当前路径向上查找 +layout.astro 文件
        let mut current = route_path;

        while current.parent().is_some() {
            let layout_path = current.join("+layout.astro");
            let full_layout_path = self.layouts_dir.join(layout_path);

            if full_layout_path.exists() {
                return Some(full_layout_path);
            }

            current = current.parent().unwrap();
        }

        // 检查根布局
        let root_layout = self.layouts_dir.join("+layout.astro");
        if root_layout.exists() { Some(root_layout) } else { None }
    }

    /// 解析路由
    pub fn resolve(&self, path: &str) -> Option<(PathBuf, Option<PathBuf>, RouteParams)> {
        self.route_tree.match_route(path)
    }
}
