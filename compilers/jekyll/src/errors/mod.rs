use std::fmt;
use notify::Error as NotifyError;

/// 通用结果类型
pub type Result<T> = std::result::Result<T, JekyllError>;

/// Jekyll 目录结构相关错误
#[derive(Debug)]
pub enum JekyllError {
    /// 文件系统错误
    FileSystemError(std::io::Error),

    /// VFS 错误
    VfsError(String),

    /// 目录不存在错误
    DirectoryNotFound(String),

    /// Front Matter 解析错误
    FrontMatterParseError(String),

    /// YAML 解析错误
    YamlParseError(String),

    /// 无效的 Front Matter 格式
    InvalidFrontMatterFormat,

    /// 配置文件读取错误
    ConfigReadError(String),

    /// 配置合并错误
    ConfigMergeError(String),

    /// 帖子文件名解析错误
    InvalidPostFilename(String),

    /// 日期解析错误
    DateParseError(String),

    /// 永久链接生成错误
    PermalinkError(String),
}

impl JekyllError {
    /// 获取错误的 i18n 键
    pub fn i18n_key(&self) -> &'static str {
        match self {
            JekyllError::FileSystemError(_) => "jekyll.error.file_system",
            JekyllError::VfsError(_) => "jekyll.error.vfs",
            JekyllError::DirectoryNotFound(_) => "jekyll.error.directory_not_found",
            JekyllError::FrontMatterParseError(_) => "jekyll.error.front_matter_parse",
            JekyllError::YamlParseError(_) => "jekyll.error.yaml_parse",
            JekyllError::InvalidFrontMatterFormat => "jekyll.error.invalid_front_matter_format",
            JekyllError::ConfigReadError(_) => "jekyll.error.config_read",
            JekyllError::ConfigMergeError(_) => "jekyll.error.config_merge",
            JekyllError::InvalidPostFilename(_) => "jekyll.error.invalid_post_filename",
            JekyllError::DateParseError(_) => "jekyll.error.date_parse",
            JekyllError::PermalinkError(_) => "jekyll.error.permalink",
        }
    }
}

impl fmt::Display for JekyllError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            JekyllError::FileSystemError(error) => write!(f, "File system error: {}", error),
            JekyllError::VfsError(error) => write!(f, "VFS error: {}", error),
            JekyllError::DirectoryNotFound(path) => write!(f, "Directory not found: {}", path),
            JekyllError::FrontMatterParseError(error) => write!(f, "Front Matter parse error: {}", error),
            JekyllError::YamlParseError(error) => write!(f, "YAML parse error: {}", error),
            JekyllError::InvalidFrontMatterFormat => write!(f, "Invalid Front Matter format"),
            JekyllError::ConfigReadError(error) => write!(f, "Failed to read config file: {}", error),
            JekyllError::ConfigMergeError(error) => write!(f, "Failed to merge configurations: {}", error),
            JekyllError::InvalidPostFilename(error) => write!(f, "Invalid post filename format: {}", error),
            JekyllError::DateParseError(error) => write!(f, "Failed to parse date: {}", error),
            JekyllError::PermalinkError(error) => write!(f, "Failed to generate permalink: {}", error),
        }
    }
}

impl From<std::io::Error> for JekyllError {
    fn from(error: std::io::Error) -> Self {
        JekyllError::FileSystemError(error)
    }
}

/// 集合相关错误
#[derive(Debug)]
pub enum CollectionError {
    /// 集合配置错误
    ConfigError(String),

    /// 集合文件解析错误
    FileParseError(String),

    /// Jekyll 相关错误
    JekyllError(JekyllError),

    /// 文件系统错误
    FileSystemError(std::io::Error),
}

impl CollectionError {
    /// 获取错误的 i18n 键
    pub fn i18n_key(&self) -> &'static str {
        match self {
            CollectionError::ConfigError(_) => "jekyll.collection.error.config",
            CollectionError::FileParseError(_) => "jekyll.collection.error.file_parse",
            CollectionError::JekyllError(error) => error.i18n_key(),
            CollectionError::FileSystemError(_) => "jekyll.collection.error.file_system",
        }
    }
}

impl fmt::Display for CollectionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CollectionError::ConfigError(error) => write!(f, "Collection config error: {}", error),
            CollectionError::FileParseError(error) => write!(f, "Collection file parse error: {}", error),
            CollectionError::JekyllError(error) => write!(f, "Jekyll error: {}", error),
            CollectionError::FileSystemError(error) => write!(f, "File system error: {}", error),
        }
    }
}

impl From<JekyllError> for CollectionError {
    fn from(error: JekyllError) -> Self {
        CollectionError::JekyllError(error)
    }
}

impl From<std::io::Error> for CollectionError {
    fn from(error: std::io::Error) -> Self {
        CollectionError::FileSystemError(error)
    }
}

/// Liquid 模板引擎相关错误
#[derive(Debug)]
pub enum LiquidError {
    /// 模板解析错误
    ParseError(String),

    /// 模板渲染错误
    RenderError(String),

    /// 模板文件未找到错误
    TemplateNotFound(String),

    /// Jekyll 相关错误
    JekyllError(JekyllError),

    /// 文件系统错误
    FileSystemError(std::io::Error),
}

impl LiquidError {
    /// 获取错误的 i18n 键
    pub fn i18n_key(&self) -> &'static str {
        match self {
            LiquidError::ParseError(_) => "jekyll.error.liquid.parse",
            LiquidError::RenderError(_) => "jekyll.error.liquid.render",
            LiquidError::TemplateNotFound(_) => "jekyll.error.liquid.template_not_found",
            LiquidError::JekyllError(error) => error.i18n_key(),
            LiquidError::FileSystemError(_) => "jekyll.error.liquid.filesystem",
        }
    }

    /// 创建解析错误
    pub fn parse_error(error: String) -> Self {
        LiquidError::ParseError(error)
    }

    /// 创建渲染错误
    pub fn render_error(error: String) -> Self {
        LiquidError::RenderError(error)
    }

    /// 创建模板未找到错误
    pub fn template_not_found(template: String) -> Self {
        LiquidError::TemplateNotFound(template)
    }
}

impl fmt::Display for LiquidError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LiquidError::ParseError(error) => write!(f, "Liquid parse error: {}", error),
            LiquidError::RenderError(error) => write!(f, "Liquid render error: {}", error),
            LiquidError::TemplateNotFound(template) => write!(f, "Template not found: {}", template),
            LiquidError::JekyllError(error) => write!(f, "Jekyll error: {}", error),
            LiquidError::FileSystemError(error) => write!(f, "File system error: {}", error),
        }
    }
}

impl From<JekyllError> for LiquidError {
    fn from(error: JekyllError) -> Self {
        LiquidError::JekyllError(error)
    }
}

impl From<std::io::Error> for LiquidError {
    fn from(error: std::io::Error) -> Self {
        LiquidError::FileSystemError(error)
    }
}

impl std::error::Error for LiquidError {}

/// Markdown 处理相关错误
#[derive(Debug)]
pub enum MarkdownError {
    /// 无效的 Markdown 处理器
    InvalidProcessor(String),

    /// 代码高亮错误
    HighlightError(String),
}

impl MarkdownError {
    /// 获取错误的 i18n 键
    pub fn i18n_key(&self) -> &'static str {
        match self {
            MarkdownError::InvalidProcessor(_) => "jekyll.error.markdown.invalid_processor",
            MarkdownError::HighlightError(_) => "jekyll.error.markdown.highlight",
        }
    }

    /// 创建无效处理器错误
    pub fn invalid_processor(processor: String) -> Self {
        MarkdownError::InvalidProcessor(processor)
    }

    /// 创建代码高亮错误
    pub fn highlight_error(error: String) -> Self {
        MarkdownError::HighlightError(error)
    }
}

impl fmt::Display for MarkdownError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MarkdownError::InvalidProcessor(processor) => write!(f, "Invalid Markdown processor: {}", processor),
            MarkdownError::HighlightError(error) => write!(f, "Code highlighting error: {}", error),
        }
    }
}

impl std::error::Error for MarkdownError {}

/// 帖子相关错误
#[derive(Debug)]
pub enum PostError {
    /// 无效的帖子文件名
    InvalidPostFilename(String),

    /// 日期解析错误
    DateParseError(String),

    /// 永久链接生成错误
    PermalinkError(String),

    /// Jekyll 相关错误
    JekyllError(JekyllError),

    /// 文件系统错误
    FileSystemError(std::io::Error),
}

/// 静态文件处理相关错误
#[derive(Debug)]
pub enum StaticFileError {
    /// 文件复制错误
    CopyError(String),

    /// 目录创建错误
    DirectoryCreateError(String),

    /// 路径匹配错误
    PathMatchError(String),

    /// Jekyll 相关错误
    JekyllError(JekyllError),

    /// 文件系统错误
    FileSystemError(std::io::Error),
}

impl StaticFileError {
    /// 获取错误的 i18n 键
    pub fn i18n_key(&self) -> &'static str {
        match self {
            StaticFileError::CopyError(_) => "jekyll.static_file.error.copy",
            StaticFileError::DirectoryCreateError(_) => "jekyll.static_file.error.directory_create",
            StaticFileError::PathMatchError(_) => "jekyll.static_file.error.path_match",
            StaticFileError::JekyllError(error) => error.i18n_key(),
            StaticFileError::FileSystemError(_) => "jekyll.static_file.error.file_system",
        }
    }
}

impl std::fmt::Display for StaticFileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StaticFileError::CopyError(message) => write!(f, "Static file copy error: {}", message),
            StaticFileError::DirectoryCreateError(message) => write!(f, "Directory creation error: {}", message),
            StaticFileError::PathMatchError(message) => write!(f, "Path match error: {}", message),
            StaticFileError::JekyllError(error) => write!(f, "Jekyll error: {}", error),
            StaticFileError::FileSystemError(error) => write!(f, "File system error: {}", error),
        }
    }
}

impl From<JekyllError> for StaticFileError {
    fn from(error: JekyllError) -> Self {
        StaticFileError::JekyllError(error)
    }
}

impl From<std::io::Error> for StaticFileError {
    fn from(error: std::io::Error) -> Self {
        StaticFileError::FileSystemError(error)
    }
}

impl std::error::Error for StaticFileError {}

/// 数据文件相关错误
#[derive(Debug)]
pub enum DataError {
    /// 不支持的数据文件格式
    UnsupportedFormat(String),

    /// JSON 解析错误
    JsonParseError(String),

    /// YAML 解析错误
    YamlParseError(String),

    /// CSV 解析错误
    CsvParseError(String),

    /// 数据文件读取错误
    ReadError(String),

    /// Jekyll 相关错误
    JekyllError(JekyllError),

    /// 文件系统错误
    FileSystemError(std::io::Error),
}

impl DataError {
    /// 获取错误的 i18n 键
    pub fn i18n_key(&self) -> &'static str {
        match self {
            DataError::UnsupportedFormat(_) => "jekyll.data.error.unsupported_format",
            DataError::JsonParseError(_) => "jekyll.data.error.json_parse",
            DataError::YamlParseError(_) => "jekyll.data.error.yaml_parse",
            DataError::CsvParseError(_) => "jekyll.data.error.csv_parse",
            DataError::ReadError(_) => "jekyll.data.error.read",
            DataError::JekyllError(error) => error.i18n_key(),
            DataError::FileSystemError(_) => "jekyll.data.error.file_system",
        }
    }
}

impl std::fmt::Display for DataError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DataError::UnsupportedFormat(format) => write!(f, "Unsupported data file format: {}", format),
            DataError::JsonParseError(error) => write!(f, "JSON parse error: {}", error),
            DataError::YamlParseError(error) => write!(f, "YAML parse error: {}", error),
            DataError::CsvParseError(error) => write!(f, "CSV parse error: {}", error),
            DataError::ReadError(error) => write!(f, "Failed to read data file: {}", error),
            DataError::JekyllError(error) => write!(f, "Jekyll error: {}", error),
            DataError::FileSystemError(error) => write!(f, "File system error: {}", error),
        }
    }
}

impl From<JekyllError> for DataError {
    fn from(error: JekyllError) -> Self {
        DataError::JekyllError(error)
    }
}

impl From<std::io::Error> for DataError {
    fn from(error: std::io::Error) -> Self {
        DataError::FileSystemError(error)
    }
}

impl std::error::Error for DataError {}

impl PostError {
    /// 获取错误的 i18n 键
    pub fn i18n_key(&self) -> &'static str {
        match self {
            PostError::InvalidFilename(_) => "jekyll.post.error.invalid_filename",
            PostError::DateParseError(_) => "jekyll.post.error.date_parse",
            PostError::PermalinkError(_) => "jekyll.post.error.permalink",
            PostError::JekyllError(error) => error.i18n_key(),
            PostError::FileSystemError(_) => "jekyll.post.error.file_system",
        }
    }
}

impl fmt::Display for PostError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PostError::InvalidFilename(filename) => write!(f, "Invalid post filename: {}", filename),
            PostError::DateParseError(error) => write!(f, "Failed to parse date: {}", error),
            PostError::PermalinkError(error) => write!(f, "Failed to generate permalink: {}", error),
            PostError::JekyllError(error) => write!(f, "Jekyll error: {}", error),
            PostError::FileSystemError(error) => write!(f, "File system error: {}", error),
        }
    }
}

impl From<JekyllError> for PostError {
    fn from(error: JekyllError) -> Self {
        PostError::JekyllError(error)
    }
}

impl From<std::io::Error> for PostError {
    fn from(error: std::io::Error) -> Self {
        PostError::FileSystemError(error)
    }
}

impl From<PostError> for JekyllError {
    fn from(error: PostError) -> Self {
        match error {
            PostError::InvalidFilename(filename) => JekyllError::InvalidPostFilename(filename),
            PostError::DateParseError(error) => JekyllError::DateParseError(error),
            PostError::PermalinkError(error) => JekyllError::PermalinkError(error),
            PostError::JekyllError(error) => error,
            PostError::FileSystemError(error) => JekyllError::FileSystemError(error),
        }
    }
}

impl From<NotifyError> for JekyllError {
    fn from(error: NotifyError) -> Self {
        JekyllError::FileSystemError(std::io::Error::new(std::io::ErrorKind::Other, error.to_string()))
    }
}
