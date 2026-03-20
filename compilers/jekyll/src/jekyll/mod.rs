/// Jekyll 目录结构识别和处理模块
///
/// 该模块提供 Jekyll 标准目录结构的识别、文件收集和遍历功能，
/// 并集成 oak-vfs 作为虚拟文件系统。同时提供 YAML Front Matter 解析功能和
/// _config.yml 配置文件处理功能。此外，还提供完整的帖子（Posts）处理功能。
use oak_vfs::DiskVfs;
use oak_yaml;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

/// 导入错误类型
pub use super::errors::{CollectionError, DataError, JekyllError, LiquidError, MarkdownError, PostError, StaticFileError};

/// Jekyll 标准目录类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum JekyllDirectory {
    /// _posts 目录：包含博客文章
    Posts,
    /// _layouts 目录：包含页面布局模板
    Layouts,
    /// _includes 目录：包含可重用的代码片段
    Includes,
    /// _data 目录：包含数据文件（YAML、JSON、CSV）
    Data,
    /// _drafts 目录：包含草稿文章
    Drafts,
    /// _sass 目录：包含 Sass/SCSS 源文件
    Sass,
    /// _site 目录：编译输出目录
    Site,
    /// assets 目录：静态资源文件
    Assets,
}

impl JekyllDirectory {
    /// 获取目录的名称
    pub fn name(&self) -> &'static str {
        match self {
            JekyllDirectory::Posts => "_posts",
            JekyllDirectory::Layouts => "_layouts",
            JekyllDirectory::Includes => "_includes",
            JekyllDirectory::Data => "_data",
            JekyllDirectory::Drafts => "_drafts",
            JekyllDirectory::Sass => "_sass",
            JekyllDirectory::Site => "_site",
            JekyllDirectory::Assets => "assets",
        }
    }

    /// 从目录名称创建 JekyllDirectory
    pub fn from_name(name: &str) -> Option<Self> {
        match name {
            "_posts" => Some(JekyllDirectory::Posts),
            "_layouts" => Some(JekyllDirectory::Layouts),
            "_includes" => Some(JekyllDirectory::Includes),
            "_data" => Some(JekyllDirectory::Data),
            "_drafts" => Some(JekyllDirectory::Drafts),
            "_sass" => Some(JekyllDirectory::Sass),
            "_site" => Some(JekyllDirectory::Site),
            "assets" => Some(JekyllDirectory::Assets),
            _ => None,
        }
    }

    /// 获取所有 Jekyll 标准目录
    pub fn all() -> &'static [JekyllDirectory] {
        &[
            JekyllDirectory::Posts,
            JekyllDirectory::Layouts,
            JekyllDirectory::Includes,
            JekyllDirectory::Data,
            JekyllDirectory::Drafts,
            JekyllDirectory::Sass,
            JekyllDirectory::Site,
            JekyllDirectory::Assets,
        ]
    }
}

/// Jekyll 目录结构
pub struct JekyllStructure {
    /// 项目根目录
    root: PathBuf,
    /// 存在的 Jekyll 目录
    directories: HashMap<JekyllDirectory, PathBuf>,
    /// 虚拟文件系统
    vfs: DiskVfs,
}

impl std::fmt::Debug for JekyllStructure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JekyllStructure").field("root", &self.root).field("directories", &self.directories).finish()
    }
}

impl Clone for JekyllStructure {
    fn clone(&self) -> Self {
        Self { root: self.root.clone(), directories: self.directories.clone(), vfs: DiskVfs::new(self.root.clone()) }
    }
}

impl JekyllStructure {
    /// 创建新的 Jekyll 目录结构
    ///
    /// # Arguments
    ///
    /// * `root` - 项目根目录路径
    ///
    /// # Errors
    ///
    /// 返回 `JekyllError` 如果目录不存在或无法访问
    pub fn new<P: AsRef<Path>>(root: P) -> Result<Self, JekyllError> {
        let root = root.as_ref().canonicalize()?;
        if !root.is_dir() {
            return Err(JekyllError::DirectoryNotFound(root.to_string_lossy().to_string()));
        }

        let vfs = DiskVfs::new(root.clone());
        let mut structure = JekyllStructure { root, directories: HashMap::new(), vfs };

        structure.discover_directories()?;
        Ok(structure)
    }

    /// 发现项目中的 Jekyll 标准目录
    fn discover_directories(&mut self) -> Result<(), JekyllError> {
        for &dir_type in JekyllDirectory::all() {
            let dir_path = self.root.join(dir_type.name());
            if dir_path.is_dir() {
                self.directories.insert(dir_type, dir_path);
            }
        }
        Ok(())
    }

    /// 获取项目根目录
    pub fn root(&self) -> &Path {
        &self.root
    }

    /// 获取虚拟文件系统
    pub fn vfs(&self) -> &DiskVfs {
        &self.vfs
    }

    /// 检查特定的 Jekyll 目录是否存在
    pub fn has_directory(&self, dir_type: JekyllDirectory) -> bool {
        self.directories.contains_key(&dir_type)
    }

    /// 获取特定 Jekyll 目录的路径
    pub fn directory_path(&self, dir_type: JekyllDirectory) -> Option<&PathBuf> {
        self.directories.get(&dir_type)
    }

    /// 获取所有存在的 Jekyll 目录
    pub fn directories(&self) -> &HashMap<JekyllDirectory, PathBuf> {
        &self.directories
    }

    /// 收集特定目录中的所有文件
    ///
    /// # Arguments
    ///
    /// * `dir_type` - 要收集文件的 Jekyll 目录类型
    ///
    /// # Returns
    ///
    /// 返回该目录下所有文件的路径列表
    pub fn collect_files(&self, dir_type: JekyllDirectory) -> Result<Vec<PathBuf>, JekyllError> {
        let dir_path = match self.directory_path(dir_type) {
            Some(path) => path,
            None => return Ok(Vec::new()),
        };

        Self::walk_directory(dir_path)
    }

    /// 收集所有 Jekyll 目录中的文件
    ///
    /// # Returns
    ///
    /// 返回一个 HashMap，键为 Jekyll 目录类型，值为该目录下的文件路径列表
    pub fn collect_all_files(&self) -> Result<HashMap<JekyllDirectory, Vec<PathBuf>>, JekyllError> {
        let mut result = HashMap::new();

        for (&dir_type, _) in &self.directories {
            let files = self.collect_files(dir_type)?;
            result.insert(dir_type, files);
        }

        Ok(result)
    }

    /// 递归遍历目录并收集所有文件
    fn walk_directory(dir: &Path) -> Result<Vec<PathBuf>, JekyllError> {
        let mut files = Vec::new();

        if let Ok(entries) = std::fs::read_dir(dir) {
            for entry in entries {
                let entry = entry?;
                let path = entry.path();

                if path.is_file() {
                    files.push(path);
                }
                else if path.is_dir() {
                    files.extend(Self::walk_directory(&path)?);
                }
            }
        }

        Ok(files)
    }

    /// 获取项目中的所有 Markdown 文件
    ///
    /// 包括 _posts、_drafts 目录和根目录下的 Markdown 文件
    pub fn collect_markdown_files(&self) -> Result<Vec<PathBuf>, JekyllError> {
        let mut files = Vec::new();

        // 收集 _posts 中的 Markdown 文件
        if self.has_directory(JekyllDirectory::Posts) {
            files.extend(self.collect_files(JekyllDirectory::Posts)?);
        }

        // 收集 _drafts 中的 Markdown 文件
        if self.has_directory(JekyllDirectory::Drafts) {
            files.extend(self.collect_files(JekyllDirectory::Drafts)?);
        }

        // 收集根目录下的 Markdown 文件
        if let Ok(entries) = std::fs::read_dir(&self.root) {
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                if path.is_file() {
                    if let Some(ext) = path.extension() {
                        if ext == "md" || ext == "markdown" {
                            files.push(path);
                        }
                    }
                }
            }
        }

        // 过滤只保留 Markdown 文件
        Ok(files
            .into_iter()
            .filter(|path| path.extension().map(|ext| ext == "md" || ext == "markdown").unwrap_or(false))
            .collect())
    }

    /// 获取所有布局文件
    pub fn collect_layout_files(&self) -> Result<Vec<PathBuf>, JekyllError> {
        self.collect_files(JekyllDirectory::Layouts)
    }

    /// 获取所有包含文件
    pub fn collect_include_files(&self) -> Result<Vec<PathBuf>, JekyllError> {
        self.collect_files(JekyllDirectory::Includes)
    }

    /// 获取所有数据文件
    pub fn collect_data_files(&self) -> Result<Vec<PathBuf>, JekyllError> {
        self.collect_files(JekyllDirectory::Data)
    }

    /// 获取所有 Sass 文件
    pub fn collect_sass_files(&self) -> Result<Vec<PathBuf>, JekyllError> {
        self.collect_files(JekyllDirectory::Sass)
    }
}

/// YAML Front Matter 解析结果
///
/// 包含从 Markdown 文件头部解析出的 YAML Front Matter 信息
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FrontMatter {
    /// 原始的 YAML 内容（不包括开头和结尾的 ---）
    pub raw_yaml: String,
    /// 解析后的变量（JSON 格式存储，支持任意 YAML 结构）
    pub variables: HashMap<String, Value>,
    /// 剩余的内容（去除 Front Matter 后的 Markdown 内容）
    pub content: String,
}

impl FrontMatter {
    /// 创建一个新的 FrontMatter 实例
    ///
    /// # Arguments
    ///
    /// * `raw_yaml` - 原始 YAML 内容
    /// * `variables` - 解析后的变量
    /// * `content` - 剩余的 Markdown 内容
    pub fn new(raw_yaml: String, variables: HashMap<String, Value>, content: String) -> Self {
        Self { raw_yaml, variables, content }
    }

    /// 获取指定的变量值
    ///
    /// # Arguments
    ///
    /// * `key` - 变量键名
    ///
    /// # Returns
    ///
    /// 返回变量值的引用（如果存在）
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.variables.get(key)
    }

    /// 获取字符串类型的变量值
    ///
    /// # Arguments
    ///
    /// * `key` - 变量键名
    ///
    /// # Returns
    ///
    /// 返回字符串值的引用（如果变量存在且为字符串类型）
    pub fn get_str(&self, key: &str) -> Option<&str> {
        self.get(key).and_then(|v| v.as_str())
    }

    /// 获取布尔类型的变量值
    ///
    /// # Arguments
    ///
    /// * `key` - 变量键名
    ///
    /// # Returns
    ///
    /// 返回布尔值（如果变量存在且为布尔类型）
    pub fn get_bool(&self, key: &str) -> Option<bool> {
        self.get(key).and_then(|v| v.as_bool())
    }

    /// 获取整数类型的变量值
    ///
    /// # Arguments
    ///
    /// * `key` - 变量键名
    ///
    /// # Returns
    ///
    /// 返回整数值（如果变量存在且为整数类型）
    pub fn get_i64(&self, key: &str) -> Option<i64> {
        self.get(key).and_then(|v| v.as_i64())
    }

    /// 获取浮点数类型的变量值
    ///
    /// # Arguments
    ///
    /// * `key` - 变量键名
    ///
    /// # Returns
    ///
    /// 返回浮点数值（如果变量存在且为浮点数类型）
    pub fn get_f64(&self, key: &str) -> Option<f64> {
        self.get(key).and_then(|v| v.as_f64())
    }

    /// 获取数组类型的变量值
    ///
    /// # Arguments
    ///
    /// * `key` - 变量键名
    ///
    /// # Returns
    ///
    /// 返回数组值的引用（如果变量存在且为数组类型）
    pub fn get_array(&self, key: &str) -> Option<&Vec<Value>> {
        self.get(key).and_then(|v| v.as_array())
    }

    /// 获取对象类型的变量值
    ///
    /// # Arguments
    ///
    /// * `key` - 变量键名
    ///
    /// # Returns
    ///
    /// 返回对象值的引用（如果变量存在且为对象类型）
    pub fn get_object(&self, key: &str) -> Option<&serde_json::Map<String, Value>> {
        self.get(key).and_then(|v| v.as_object())
    }

    /// 检查是否包含指定的变量
    ///
    /// # Arguments
    ///
    /// * `key` - 变量键名
    ///
    /// # Returns
    ///
    /// 如果变量存在返回 true，否则返回 false
    pub fn has(&self, key: &str) -> bool {
        self.variables.contains_key(key)
    }

    /// 获取所有变量的引用
    ///
    /// # Returns
    ///
    /// 返回所有变量的不可变引用
    pub fn variables(&self) -> &HashMap<String, Value> {
        &self.variables
    }

    /// 获取变量的可变引用
    ///
    /// # Returns
    ///
    /// 返回所有变量的可变引用
    pub fn variables_mut(&mut self) -> &mut HashMap<String, Value> {
        &mut self.variables
    }

    /// 插入或更新变量
    ///
    /// # Arguments
    ///
    /// * `key` - 变量键名
    /// * `value` - 变量值
    pub fn insert(&mut self, key: String, value: Value) {
        self.variables.insert(key, value);
    }

    /// 删除指定的变量
    ///
    /// # Arguments
    ///
    /// * `key` - 变量键名
    ///
    /// # Returns
    ///
    /// 返回被删除的变量值（如果存在）
    pub fn remove(&mut self, key: &str) -> Option<Value> {
        self.variables.remove(key)
    }

    /// 获取去除 Front Matter 后的内容
    ///
    /// # Returns
    ///
    /// 返回 Markdown 内容的不可变引用
    pub fn content(&self) -> &str {
        &self.content
    }

    /// 获取内容的可变引用
    ///
    /// # Returns
    ///
    /// 返回 Markdown 内容的可变引用
    pub fn content_mut(&mut self) -> &mut String {
        &mut self.content
    }

    /// 获取原始 YAML 内容
    ///
    /// # Returns
    ///
    /// 返回原始 YAML 内容的不可变引用
    pub fn raw_yaml(&self) -> &str {
        &self.raw_yaml
    }

    /// 检查 Front Matter 是否为空（不包含任何变量）
    ///
    /// # Returns
    ///
    /// 如果没有变量返回 true，否则返回 false
    pub fn is_empty(&self) -> bool {
        self.variables.is_empty()
    }

    /// 获取变量数量
    ///
    /// # Returns
    ///
    /// 返回变量的数量
    pub fn len(&self) -> usize {
        self.variables.len()
    }
}

/// Front Matter 解析器
///
/// 用于从 Markdown 文件中解析 YAML Front Matter
pub struct FrontMatterParser;

impl FrontMatterParser {
    /// 从 Markdown 内容中解析 Front Matter
    ///
    /// Front Matter 必须位于文件开头，并用 `---` 包围。
    /// 如果没有 Front Matter，返回一个包含空变量和完整内容的 FrontMatter。
    ///
    /// # Arguments
    ///
    /// * `content` - Markdown 内容
    ///
    /// # Errors
    ///
    /// 返回 `JekyllError::InvalidFrontMatterFormat` 如果 Front Matter 格式无效
    /// 返回 `JekyllError::YamlParseError` 如果 YAML 解析失败
    pub fn parse(content: &str) -> Result<FrontMatter, JekyllError> {
        let trimmed = content.trim_start();

        if !trimmed.starts_with("---") {
            return Ok(FrontMatter::new(String::new(), HashMap::new(), content.to_string()));
        }

        let lines: Vec<&str> = content.lines().collect();
        let mut yaml_lines = Vec::new();
        let mut content_start = 0;
        let mut in_front_matter = false;
        let mut found_start = false;

        for (i, line) in lines.iter().enumerate() {
            let trimmed_line = line.trim();

            if trimmed_line == "---" {
                if !found_start {
                    found_start = true;
                    in_front_matter = true;
                    continue;
                }
                else if in_front_matter {
                    in_front_matter = false;
                    content_start = i + 1;
                    break;
                }
            }

            if in_front_matter {
                yaml_lines.push(*line);
            }
        }

        if in_front_matter {
            return Err(JekyllError::InvalidFrontMatterFormat);
        }

        let raw_yaml: String = yaml_lines.join("\n");
        let content = if content_start < lines.len() { 
            lines[content_start..].join("\n") 
        } else { 
            String::new() 
        };

        let variables = if raw_yaml.trim().is_empty() { 
            HashMap::new() 
        } else { 
            Self::parse_yaml(&raw_yaml)? 
        };

        Ok(FrontMatter::new(raw_yaml, variables, content))
    }

    /// 使用 oak_yaml 解析 YAML 内容
    ///
    /// YAML 必须以对象（键值对）作为根元素。
    /// 支持所有 YAML 数据类型，包括字符串、数字、布尔值、日期、数组、嵌套对象等。
    ///
    /// # Arguments
    ///
    /// * `yaml_content` - YAML 格式的字符串
    ///
    /// # Errors
    ///
    /// 返回 `JekyllError::YamlParseError` 如果 YAML 解析失败或根元素不是对象
    fn parse_yaml(yaml_content: &str) -> Result<HashMap<String, Value>, JekyllError> {
        let value: serde_json::Value =
            oak_yaml::from_str(yaml_content).map_err(|e| {
                JekyllError::YamlParseError(format!("Failed to parse YAML: {}", e))
            })?;

        match value {
            Value::Object(map) => Ok(map.into_iter().collect()),
            _ => Err(JekyllError::YamlParseError(
                "YAML root must be an object (key-value pairs)".to_string()
            )),
        }
    }

    /// 从文件路径中读取并解析 Front Matter
    ///
    /// # Arguments
    ///
    /// * `path` - 文件路径
    ///
    /// # Errors
    ///
    /// 返回 `JekyllError::FileSystemError` 如果文件读取失败
    /// 返回 `JekyllError::InvalidFrontMatterFormat` 如果 Front Matter 格式无效
    /// 返回 `JekyllError::YamlParseError` 如果 YAML 解析失败
    pub fn parse_file<P: AsRef<Path>>(path: P) -> Result<FrontMatter, JekyllError> {
        let content = std::fs::read_to_string(path.as_ref()).map_err(|e| {
            JekyllError::FileSystemError(e)
        })?;
        Self::parse(&content)
    }

    /// 检查内容是否包含 Front Matter
    ///
    /// # Arguments
    ///
    /// * `content` - 要检查的内容
    ///
    /// # Returns
    ///
    /// 如果内容包含有效的 Front Matter 标记返回 true
    pub fn has_front_matter(content: &str) -> bool {
        let trimmed = content.trim_start();
        if !trimmed.starts_with("---") {
            return false;
        }

        let lines: Vec<&str> = content.lines().collect();
        let mut found_start = false;
        let mut found_end = false;

        for line in lines.iter() {
            let trimmed_line = line.trim();
            if trimmed_line == "---" {
                if !found_start {
                    found_start = true;
                } else if found_start {
                    found_end = true;
                    break;
                }
            }
        }

        found_start && found_end
    }
}

/// Jekyll 配置结构体，支持所有 Jekyll 4.x 标准配置选项
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct JekyllConfig {
    /// 站点标题
    pub title: Option<String>,

    /// 站点描述
    pub description: Option<String>,

    /// 站点作者
    pub author: Option<String>,

    /// 站点 URL
    pub url: Option<String>,

    /// 站点基础路径
    pub baseurl: Option<String>,

    /// 永久链接格式
    pub permalink: Option<String>,

    /// Markdown 处理器
    pub markdown: Option<String>,

    /// 构建输出目录
    pub destination: Option<String>,

    /// 源文件目录
    pub source: Option<String>,

    /// 插件目录
    pub plugins_dir: Option<String>,

    /// 布局目录
    pub layouts_dir: Option<String>,

    /// 包含目录
    pub includes_dir: Option<String>,

    /// 数据目录
    pub data_dir: Option<String>,

    /// 文章目录
    pub posts_dir: Option<String>,

    /// 草稿目录
    pub drafts_dir: Option<String>,

    /// 集合目录
    pub collections_dir: Option<String>,

    /// 资源目录
    pub assets_dir: Option<String>,

    /// Sass 目录
    pub sass_dir: Option<String>,

    /// 排除的文件或目录
    pub exclude: Option<Vec<String>>,

    /// 包含的文件或目录
    pub include: Option<Vec<String>>,

    /// 保留的文件或目录
    pub keep_files: Option<Vec<String>>,

    /// 插件列表
    pub plugins: Option<Vec<String>>,

    /// 集合配置
    pub collections: Option<HashMap<String, Value>>,

    /// 默认值配置
    pub defaults: Option<Vec<HashMap<String, Value>>>,

    /// 安全模式：禁用非白名单插件、磁盘缓存和符号链接
    pub safe: Option<bool>,

    /// 禁用磁盘缓存
    pub disable_disk_cache: Option<bool>,

    /// 忽略主题配置
    pub ignore_theme_config: Option<bool>,

    /// 时区
    pub timezone: Option<String>,

    /// 文件编码
    pub encoding: Option<String>,

    /// 显示草稿文章
    pub show_drafts: Option<bool>,

    /// 发布未来日期的文章
    pub future: Option<bool>,

    /// 渲染未发布的文章
    pub unpublished: Option<bool>,

    /// 生成相关文章索引
    pub lsi: Option<bool>,

    /// 限制解析和发布的文章数量
    pub limit_posts: Option<u32>,

    /// 强制文件监控使用轮询
    pub force_polling: Option<bool>,

    /// 详细输出
    pub verbose: Option<bool>,

    /// 安静模式
    pub quiet: Option<bool>,

    /// 增量构建
    pub incremental: Option<bool>,

    /// 生成 Liquid 渲染分析
    pub profile: Option<bool>,

    /// 严格 Front Matter 模式：遇到 YAML 语法错误时构建失败
    pub strict_front_matter: Option<bool>,

    /// 服务器端口
    pub port: Option<u16>,

    /// 服务器主机名
    pub host: Option<String>,

    /// 实时重载
    pub livereload: Option<bool>,

    /// 实时重载忽略的文件模式
    pub livereload_ignore: Option<Vec<String>>,

    /// 实时重载最小延迟（秒）
    pub livereload_min_delay: Option<u32>,

    /// 实时重载最大延迟（秒）
    pub livereload_max_delay: Option<u32>,

    /// 实时重载端口
    pub livereload_port: Option<u16>,

    /// 打开浏览器访问网站
    pub open_url: Option<bool>,

    /// 服务器后台运行
    pub detach: Option<bool>,

    /// 跳过初始构建
    pub skip_initial_build: Option<bool>,

    /// 显示目录列表
    pub show_dir_listing: Option<bool>,

    /// 语法高亮器
    pub highlighter: Option<String>,

    /// 摘要分隔符
    pub excerpt_separator: Option<String>,

    /// 每页文章数
    pub paginate: Option<u32>,

    /// 分页路径
    pub paginate_path: Option<String>,

    /// Kramdown 配置
    pub kramdown: Option<HashMap<String, Value>>,

    /// Sass 配置
    pub sass: Option<HashMap<String, Value>>,

    /// 站点自定义配置（任意键值对）
    #[serde(flatten)]
    pub custom: HashMap<String, Value>,
}

impl JekyllConfig {
    /// 创建一个新的 Jekyll 配置
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置站点标题
    pub fn with_title(mut self, title: String) -> Self {
        self.title = Some(title);
        self
    }

    /// 设置站点描述
    pub fn with_description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    /// 设置站点作者
    pub fn with_author(mut self, author: String) -> Self {
        self.author = Some(author);
        self
    }

    /// 设置站点 URL
    pub fn with_url(mut self, url: String) -> Self {
        self.url = Some(url);
        self
    }

    /// 设置站点基础路径
    pub fn with_baseurl(mut self, baseurl: String) -> Self {
        self.baseurl = Some(baseurl);
        self
    }

    /// 设置永久链接格式
    pub fn with_permalink(mut self, permalink: String) -> Self {
        self.permalink = Some(permalink);
        self
    }

    /// 设置 Markdown 处理器
    pub fn with_markdown(mut self, markdown: String) -> Self {
        self.markdown = Some(markdown);
        self
    }

    /// 设置构建输出目录
    pub fn with_destination(mut self, destination: String) -> Self {
        self.destination = Some(destination);
        self
    }

    /// 设置源文件目录
    pub fn with_source(mut self, source: String) -> Self {
        self.source = Some(source);
        self
    }

    /// 设置插件目录
    pub fn with_plugins_dir(mut self, plugins_dir: String) -> Self {
        self.plugins_dir = Some(plugins_dir);
        self
    }

    /// 设置布局目录
    pub fn with_layouts_dir(mut self, layouts_dir: String) -> Self {
        self.layouts_dir = Some(layouts_dir);
        self
    }

    /// 设置包含目录
    pub fn with_includes_dir(mut self, includes_dir: String) -> Self {
        self.includes_dir = Some(includes_dir);
        self
    }

    /// 设置数据目录
    pub fn with_data_dir(mut self, data_dir: String) -> Self {
        self.data_dir = Some(data_dir);
        self
    }

    /// 设置文章目录
    pub fn with_posts_dir(mut self, posts_dir: String) -> Self {
        self.posts_dir = Some(posts_dir);
        self
    }

    /// 设置草稿目录
    pub fn with_drafts_dir(mut self, drafts_dir: String) -> Self {
        self.drafts_dir = Some(drafts_dir);
        self
    }

    /// 设置集合目录
    pub fn with_collections_dir(mut self, collections_dir: String) -> Self {
        self.collections_dir = Some(collections_dir);
        self
    }

    /// 设置资源目录
    pub fn with_assets_dir(mut self, assets_dir: String) -> Self {
        self.assets_dir = Some(assets_dir);
        self
    }

    /// 设置 Sass 目录
    pub fn with_sass_dir(mut self, sass_dir: String) -> Self {
        self.sass_dir = Some(sass_dir);
        self
    }

    /// 设置排除的文件或目录
    pub fn with_exclude(mut self, exclude: Vec<String>) -> Self {
        self.exclude = Some(exclude);
        self
    }

    /// 设置包含的文件或目录
    pub fn with_include(mut self, include: Vec<String>) -> Self {
        self.include = Some(include);
        self
    }

    /// 设置保留的文件或目录
    pub fn with_keep_files(mut self, keep_files: Vec<String>) -> Self {
        self.keep_files = Some(keep_files);
        self
    }

    /// 设置插件列表
    pub fn with_plugins(mut self, plugins: Vec<String>) -> Self {
        self.plugins = Some(plugins);
        self
    }

    /// 设置集合配置
    pub fn with_collections(mut self, collections: HashMap<String, Value>) -> Self {
        self.collections = Some(collections);
        self
    }

    /// 设置默认值配置
    pub fn with_defaults(mut self, defaults: Vec<HashMap<String, Value>>) -> Self {
        self.defaults = Some(defaults);
        self
    }

    /// 设置安全模式
    pub fn with_safe(mut self, safe: bool) -> Self {
        self.safe = Some(safe);
        self
    }

    /// 设置禁用磁盘缓存
    pub fn with_disable_disk_cache(mut self, disable_disk_cache: bool) -> Self {
        self.disable_disk_cache = Some(disable_disk_cache);
        self
    }

    /// 设置忽略主题配置
    pub fn with_ignore_theme_config(mut self, ignore_theme_config: bool) -> Self {
        self.ignore_theme_config = Some(ignore_theme_config);
        self
    }

    /// 设置时区
    pub fn with_timezone(mut self, timezone: String) -> Self {
        self.timezone = Some(timezone);
        self
    }

    /// 设置文件编码
    pub fn with_encoding(mut self, encoding: String) -> Self {
        self.encoding = Some(encoding);
        self
    }

    /// 设置显示草稿文章
    pub fn with_show_drafts(mut self, show_drafts: bool) -> Self {
        self.show_drafts = Some(show_drafts);
        self
    }

    /// 设置发布未来日期的文章
    pub fn with_future(mut self, future: bool) -> Self {
        self.future = Some(future);
        self
    }

    /// 设置渲染未发布的文章
    pub fn with_unpublished(mut self, unpublished: bool) -> Self {
        self.unpublished = Some(unpublished);
        self
    }

    /// 设置生成相关文章索引
    pub fn with_lsi(mut self, lsi: bool) -> Self {
        self.lsi = Some(lsi);
        self
    }

    /// 设置限制解析和发布的文章数量
    pub fn with_limit_posts(mut self, limit_posts: u32) -> Self {
        self.limit_posts = Some(limit_posts);
        self
    }

    /// 设置强制文件监控使用轮询
    pub fn with_force_polling(mut self, force_polling: bool) -> Self {
        self.force_polling = Some(force_polling);
        self
    }

    /// 设置详细输出
    pub fn with_verbose(mut self, verbose: bool) -> Self {
        self.verbose = Some(verbose);
        self
    }

    /// 设置安静模式
    pub fn with_quiet(mut self, quiet: bool) -> Self {
        self.quiet = Some(quiet);
        self
    }

    /// 设置增量构建
    pub fn with_incremental(mut self, incremental: bool) -> Self {
        self.incremental = Some(incremental);
        self
    }

    /// 设置生成 Liquid 渲染分析
    pub fn with_profile(mut self, profile: bool) -> Self {
        self.profile = Some(profile);
        self
    }

    /// 设置严格 Front Matter 模式
    pub fn with_strict_front_matter(mut self, strict_front_matter: bool) -> Self {
        self.strict_front_matter = Some(strict_front_matter);
        self
    }

    /// 设置服务器端口
    pub fn with_port(mut self, port: u16) -> Self {
        self.port = Some(port);
        self
    }

    /// 设置服务器主机名
    pub fn with_host(mut self, host: String) -> Self {
        self.host = Some(host);
        self
    }

    /// 设置实时重载
    pub fn with_livereload(mut self, livereload: bool) -> Self {
        self.livereload = Some(livereload);
        self
    }

    /// 设置实时重载忽略的文件模式
    pub fn with_livereload_ignore(mut self, livereload_ignore: Vec<String>) -> Self {
        self.livereload_ignore = Some(livereload_ignore);
        self
    }

    /// 设置实时重载最小延迟
    pub fn with_livereload_min_delay(mut self, livereload_min_delay: u32) -> Self {
        self.livereload_min_delay = Some(livereload_min_delay);
        self
    }

    /// 设置实时重载最大延迟
    pub fn with_livereload_max_delay(mut self, livereload_max_delay: u32) -> Self {
        self.livereload_max_delay = Some(livereload_max_delay);
        self
    }

    /// 设置实时重载端口
    pub fn with_livereload_port(mut self, livereload_port: u16) -> Self {
        self.livereload_port = Some(livereload_port);
        self
    }

    /// 设置打开浏览器访问网站
    pub fn with_open_url(mut self, open_url: bool) -> Self {
        self.open_url = Some(open_url);
        self
    }

    /// 设置服务器后台运行
    pub fn with_detach(mut self, detach: bool) -> Self {
        self.detach = Some(detach);
        self
    }

    /// 设置跳过初始构建
    pub fn with_skip_initial_build(mut self, skip_initial_build: bool) -> Self {
        self.skip_initial_build = Some(skip_initial_build);
        self
    }

    /// 设置显示目录列表
    pub fn with_show_dir_listing(mut self, show_dir_listing: bool) -> Self {
        self.show_dir_listing = Some(show_dir_listing);
        self
    }

    /// 设置语法高亮器
    pub fn with_highlighter(mut self, highlighter: String) -> Self {
        self.highlighter = Some(highlighter);
        self
    }

    /// 设置摘要分隔符
    pub fn with_excerpt_separator(mut self, excerpt_separator: String) -> Self {
        self.excerpt_separator = Some(excerpt_separator);
        self
    }

    /// 设置每页文章数
    pub fn with_paginate(mut self, paginate: u32) -> Self {
        self.paginate = Some(paginate);
        self
    }

    /// 设置分页路径
    pub fn with_paginate_path(mut self, paginate_path: String) -> Self {
        self.paginate_path = Some(paginate_path);
        self
    }

    /// 设置 Kramdown 配置
    pub fn with_kramdown(mut self, kramdown: HashMap<String, Value>) -> Self {
        self.kramdown = Some(kramdown);
        self
    }

    /// 设置 Sass 配置
    pub fn with_sass(mut self, sass: HashMap<String, Value>) -> Self {
        self.sass = Some(sass);
        self
    }

    /// 从 YAML 字符串解析配置
    ///
    /// # Arguments
    ///
    /// * `yaml_str` - YAML 格式的配置字符串
    ///
    /// # Errors
    ///
    /// 返回 `JekyllError::YamlParseError` 如果 YAML 解析失败
    pub fn from_yaml_str(yaml_str: &str) -> Result<Self, JekyllError> {
        oak_yaml::from_str(yaml_str).map_err(|e| JekyllError::YamlParseError(e.to_string()))
    }

    /// 从文件加载配置
    ///
    /// # Arguments
    ///
    /// * `path` - 配置文件路径
    ///
    /// # Errors
    ///
    /// 返回 `JekyllError` 如果文件读取或解析失败
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, JekyllError> {
        let content = std::fs::read_to_string(path.as_ref()).map_err(|e| JekyllError::ConfigReadError(e.to_string()))?;

        Self::from_yaml_str(&content)
    }

    /// 合并另一个配置到当前配置
    ///
    /// 另一个配置中的值会覆盖当前配置中的值
    ///
    /// # Arguments
    ///
    /// * `other` - 要合并的配置
    ///
    /// # Returns
    ///
    /// 返回合并后的新配置
    pub fn merge(&self, other: &Self) -> Self {
        let mut merged = self.clone();

        if let Some(title) = &other.title {
            merged.title = Some(title.clone());
        }

        if let Some(description) = &other.description {
            merged.description = Some(description.clone());
        }

        if let Some(author) = &other.author {
            merged.author = Some(author.clone());
        }

        if let Some(url) = &other.url {
            merged.url = Some(url.clone());
        }

        if let Some(baseurl) = &other.baseurl {
            merged.baseurl = Some(baseurl.clone());
        }

        if let Some(permalink) = &other.permalink {
            merged.permalink = Some(permalink.clone());
        }

        if let Some(markdown) = &other.markdown {
            merged.markdown = Some(markdown.clone());
        }

        if let Some(destination) = &other.destination {
            merged.destination = Some(destination.clone());
        }

        if let Some(source) = &other.source {
            merged.source = Some(source.clone());
        }

        if let Some(plugins_dir) = &other.plugins_dir {
            merged.plugins_dir = Some(plugins_dir.clone());
        }

        if let Some(layouts_dir) = &other.layouts_dir {
            merged.layouts_dir = Some(layouts_dir.clone());
        }

        if let Some(includes_dir) = &other.includes_dir {
            merged.includes_dir = Some(includes_dir.clone());
        }

        if let Some(data_dir) = &other.data_dir {
            merged.data_dir = Some(data_dir.clone());
        }

        if let Some(posts_dir) = &other.posts_dir {
            merged.posts_dir = Some(posts_dir.clone());
        }

        if let Some(drafts_dir) = &other.drafts_dir {
            merged.drafts_dir = Some(drafts_dir.clone());
        }

        if let Some(collections_dir) = &other.collections_dir {
            merged.collections_dir = Some(collections_dir.clone());
        }

        if let Some(assets_dir) = &other.assets_dir {
            merged.assets_dir = Some(assets_dir.clone());
        }

        if let Some(sass_dir) = &other.sass_dir {
            merged.sass_dir = Some(sass_dir.clone());
        }

        if let Some(exclude) = &other.exclude {
            let mut new_exclude = merged.exclude.unwrap_or_default();
            new_exclude.extend(exclude.clone());
            merged.exclude = Some(new_exclude);
        }

        if let Some(include) = &other.include {
            let mut new_include = merged.include.unwrap_or_default();
            new_include.extend(include.clone());
            merged.include = Some(new_include);
        }

        if let Some(keep_files) = &other.keep_files {
            let mut new_keep_files = merged.keep_files.unwrap_or_default();
            new_keep_files.extend(keep_files.clone());
            merged.keep_files = Some(new_keep_files);
        }

        if let Some(plugins) = &other.plugins {
            let mut new_plugins = merged.plugins.unwrap_or_default();
            new_plugins.extend(plugins.clone());
            merged.plugins = Some(new_plugins);
        }

        if let Some(collections) = &other.collections {
            let mut new_collections = merged.collections.unwrap_or_default();
            new_collections.extend(collections.clone());
            merged.collections = Some(new_collections);
        }

        if let Some(defaults) = &other.defaults {
            let mut new_defaults = merged.defaults.unwrap_or_default();
            new_defaults.extend(defaults.clone());
            merged.defaults = Some(new_defaults);
        }

        if let Some(safe) = &other.safe {
            merged.safe = Some(*safe);
        }

        if let Some(disable_disk_cache) = &other.disable_disk_cache {
            merged.disable_disk_cache = Some(*disable_disk_cache);
        }

        if let Some(ignore_theme_config) = &other.ignore_theme_config {
            merged.ignore_theme_config = Some(*ignore_theme_config);
        }

        if let Some(timezone) = &other.timezone {
            merged.timezone = Some(timezone.clone());
        }

        if let Some(encoding) = &other.encoding {
            merged.encoding = Some(encoding.clone());
        }

        if let Some(show_drafts) = &other.show_drafts {
            merged.show_drafts = Some(*show_drafts);
        }

        if let Some(future) = &other.future {
            merged.future = Some(*future);
        }

        if let Some(unpublished) = &other.unpublished {
            merged.unpublished = Some(*unpublished);
        }

        if let Some(lsi) = &other.lsi {
            merged.lsi = Some(*lsi);
        }

        if let Some(limit_posts) = &other.limit_posts {
            merged.limit_posts = Some(*limit_posts);
        }

        if let Some(force_polling) = &other.force_polling {
            merged.force_polling = Some(*force_polling);
        }

        if let Some(verbose) = &other.verbose {
            merged.verbose = Some(*verbose);
        }

        if let Some(quiet) = &other.quiet {
            merged.quiet = Some(*quiet);
        }

        if let Some(incremental) = &other.incremental {
            merged.incremental = Some(*incremental);
        }

        if let Some(profile) = &other.profile {
            merged.profile = Some(*profile);
        }

        if let Some(strict_front_matter) = &other.strict_front_matter {
            merged.strict_front_matter = Some(*strict_front_matter);
        }

        if let Some(port) = &other.port {
            merged.port = Some(*port);
        }

        if let Some(host) = &other.host {
            merged.host = Some(host.clone());
        }

        if let Some(livereload) = &other.livereload {
            merged.livereload = Some(*livereload);
        }

        if let Some(livereload_ignore) = &other.livereload_ignore {
            let mut new_livereload_ignore = merged.livereload_ignore.unwrap_or_default();
            new_livereload_ignore.extend(livereload_ignore.clone());
            merged.livereload_ignore = Some(new_livereload_ignore);
        }

        if let Some(livereload_min_delay) = &other.livereload_min_delay {
            merged.livereload_min_delay = Some(*livereload_min_delay);
        }

        if let Some(livereload_max_delay) = &other.livereload_max_delay {
            merged.livereload_max_delay = Some(*livereload_max_delay);
        }

        if let Some(livereload_port) = &other.livereload_port {
            merged.livereload_port = Some(*livereload_port);
        }

        if let Some(open_url) = &other.open_url {
            merged.open_url = Some(*open_url);
        }

        if let Some(detach) = &other.detach {
            merged.detach = Some(*detach);
        }

        if let Some(skip_initial_build) = &other.skip_initial_build {
            merged.skip_initial_build = Some(*skip_initial_build);
        }

        if let Some(show_dir_listing) = &other.show_dir_listing {
            merged.show_dir_listing = Some(*show_dir_listing);
        }

        if let Some(highlighter) = &other.highlighter {
            merged.highlighter = Some(highlighter.clone());
        }

        if let Some(excerpt_separator) = &other.excerpt_separator {
            merged.excerpt_separator = Some(excerpt_separator.clone());
        }

        if let Some(paginate) = &other.paginate {
            merged.paginate = Some(*paginate);
        }

        if let Some(paginate_path) = &other.paginate_path {
            merged.paginate_path = Some(paginate_path.clone());
        }

        if let Some(kramdown) = &other.kramdown {
            let mut new_kramdown = merged.kramdown.unwrap_or_default();
            new_kramdown.extend(kramdown.clone());
            merged.kramdown = Some(new_kramdown);
        }

        if let Some(sass) = &other.sass {
            let mut new_sass = merged.sass.unwrap_or_default();
            new_sass.extend(sass.clone());
            merged.sass = Some(new_sass);
        }

        merged.custom.extend(other.custom.clone());

        merged
    }

    /// 获取自定义配置值
    ///
    /// # Arguments
    ///
    /// * `key` - 配置键
    ///
    /// # Returns
    ///
    /// 返回配置值（如果存在）
    pub fn get_custom(&self, key: &str) -> Option<&Value> {
        self.custom.get(key)
    }

    /// 设置自定义配置值
    ///
    /// # Arguments
    ///
    /// * `key` - 配置键
    /// * `value` - 配置值
    pub fn set_custom(&mut self, key: String, value: Value) {
        self.custom.insert(key, value);
    }
}

/// Jekyll 配置加载器
pub struct JekyllConfigLoader;

impl JekyllConfigLoader {
    /// 从项目根目录加载配置
    ///
    /// 按以下顺序加载配置：
    /// 1. _config.yml（主配置）
    /// 2. _config.local.yml（本地覆盖配置，可选）
    ///
    /// # Arguments
    ///
    /// * `root_dir` - 项目根目录
    ///
    /// # Errors
    ///
    /// 返回 `JekyllError` 如果配置文件读取或解析失败
    pub fn load_from_dir<P: AsRef<Path>>(root_dir: P) -> Result<JekyllConfig, JekyllError> {
        let root_dir = root_dir.as_ref();

        let mut config = JekyllConfig::default();

        let config_path = root_dir.join("_config.yml");
        if config_path.exists() {
            let main_config = JekyllConfig::from_file(&config_path)?;
            config = config.merge(&main_config);
        }

        let local_config_path = root_dir.join("_config.local.yml");
        if local_config_path.exists() {
            let local_config = JekyllConfig::from_file(&local_config_path)?;
            config = config.merge(&local_config);
        }

        Ok(config)
    }

    /// 从项目根目录加载配置，支持指定额外的配置文件
    ///
    /// # Arguments
    ///
    /// * `root_dir` - 项目根目录
    /// * `additional_configs` - 额外的配置文件列表（按顺序合并）
    ///
    /// # Errors
    ///
    /// 返回 `JekyllError` 如果配置文件读取或解析失败
    pub fn load_from_dir_with_additional<P: AsRef<Path>>(
        root_dir: P,
        additional_configs: &[String],
    ) -> Result<JekyllConfig, JekyllError> {
        let mut config = Self::load_from_dir(&root_dir)?;

        for config_name in additional_configs {
            let config_path = root_dir.as_ref().join(config_name);
            if config_path.exists() {
                let additional_config = JekyllConfig::from_file(&config_path)?;
                config = config.merge(&additional_config);
            }
        }

        Ok(config)
    }
}

pub mod collection;
pub mod data;
pub mod defaults;
pub mod liquid;
pub mod markdown;
pub mod post;
pub mod static_files;

pub use collection::{Collection, CollectionConfig, CollectionItem, CollectionManager};
pub use data::{DataFile, DataFormat, DataManager};
pub use defaults::{DefaultConfig, DefaultsError, DefaultsManager, Scope};
pub use liquid::LiquidEngine;
pub use markdown::{MarkdownConverter, MarkdownOptions, MarkdownProcessor};
pub use post::{Post, PostManager};
pub use static_files::{StaticFile, StaticFileProcessor};
