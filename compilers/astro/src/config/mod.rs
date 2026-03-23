//! 配置模块

use serde::Deserialize;
use std::{fs, path::Path};

/// Astro 配置结构
#[derive(Debug, Deserialize, Default, Clone)]
pub struct AstroConfig {
    /// 站点最终部署的链接
    pub site: Option<String>,
    
    /// 部署到的基本路径
    pub base: Option<String>,
    
    /// 末尾斜杠设置
    #[serde(default = "default_trailing_slash")]
    pub trailing_slash: String,
    
    /// 重定向配置
    #[serde(default)]
    pub redirects: std::collections::HashMap<String, RedirectConfig>,
    
    /// 构建输出目标
    #[serde(default = "default_output")]
    pub output: String,
    
    /// 构建适配器
    pub adapter: Option<serde_json::Value>,
    
    /// 集成配置
    #[serde(default)]
    pub integrations: Vec<serde_json::Value>,
    
    /// 项目根目录
    pub root: Option<String>,
    
    /// 源码目录
    #[serde(default = "default_src_dir")]
    pub src_dir: String,
    
    /// 静态资源目录
    #[serde(default = "default_public_dir")]
    pub public_dir: String,
    
    /// 输出目录
    #[serde(default = "default_out_dir")]
    pub out_dir: String,
    
    /// 缓存目录
    #[serde(default = "default_cache_dir")]
    pub cache_dir: String,
    
    /// 是否压缩HTML
    #[serde(default = "default_compress_html")]
    pub compress_html: bool,
    
    /// 样式作用范围策略
    #[serde(default = "default_scoped_style_strategy")]
    pub scoped_style_strategy: String,
    
    /// 安全设置
    #[serde(default)]
    pub security: Option<SecurityConfig>,
    
    /// Vite配置
    #[serde(default)]
    pub vite: Option<serde_json::Value>,
    
    /// 构建配置
    #[serde(default)]
    pub build: BuildConfig,
    
    /// 服务器配置
    #[serde(default)]
    pub server: ServerConfig,
    
    /// 开发者工具栏配置
    #[serde(default)]
    pub dev_toolbar: DevToolbarConfig,
    
    /// 预获取配置
    #[serde(default)]
    pub prefetch: PrefetchConfig,
    
    /// 图像配置
    #[serde(default)]
    pub image: ImageConfig,
    
    /// Markdown配置
    #[serde(default)]
    pub markdown: MarkdownConfig,
    
    /// 国际化配置
    #[serde(default)]
    pub i18n: Option<I18nConfig>,
    
    /// 环境变量配置
    #[serde(default)]
    pub env: EnvConfig,
}

/// 重定向配置
#[derive(Debug, Deserialize, Default, Clone)]
pub struct RedirectConfig {
    /// 重定向状态码
    #[serde(default = "default_redirect_status")]
    pub status: u16,
    
    /// 重定向目标
    pub destination: String,
}

/// 安全配置
#[derive(Debug, Deserialize, Default, Clone)]
pub struct SecurityConfig {
    /// 是否检查Origin头
    #[serde(default = "default_check_origin")]
    pub check_origin: bool,
    
    /// 允许的域名
    #[serde(default)]
    pub allowed_domains: Vec<RemotePattern>,
}

/// 远程模式
#[derive(Debug, Deserialize, Default, Clone)]
pub struct RemotePattern {
    /// 协议
    pub protocol: Option<String>,
    
    /// 主机名
    pub hostname: String,
    
    /// 端口
    pub port: Option<String>,
    
    /// 路径名
    pub pathname: Option<String>,
}

/// 构建配置
#[derive(Debug, Deserialize, Default, Clone)]
pub struct BuildConfig {
    /// 构建格式
    #[serde(default = "default_build_format")]
    pub format: String,
    
    /// 客户端输出目录
    #[serde(default = "default_build_client")]
    pub client: String,
    
    /// 服务器输出目录
    #[serde(default = "default_build_server")]
    pub server: String,
    
    /// 资源目录
    #[serde(default = "default_build_assets")]
    pub assets: String,
    
    /// 资源链接前缀
    pub assets_prefix: Option<serde_json::Value>,
    
    /// 服务器入口文件名
    #[serde(default = "default_build_server_entry")]
    pub server_entry: String,
    
    /// 是否输出重定向到HTML
    #[serde(default = "default_build_redirects")]
    pub redirects: bool,
    
    /// 样式表内联设置
    #[serde(default = "default_build_inline_stylesheets")]
    pub inline_stylesheets: String,
    
    /// 并行构建页面数
    #[serde(default = "default_build_concurrency")]
    pub concurrency: usize,
}

/// 服务器配置
#[derive(Debug, Deserialize, Default, Clone)]
pub struct ServerConfig {
    /// 服务器监听地址
    pub host: Option<serde_json::Value>,
    
    /// 服务器端口
    #[serde(default = "default_server_port")]
    pub port: u16,
    
    /// 允许的主机名
    #[serde(default)]
    pub allowed_hosts: Vec<String>,
    
    /// 是否在浏览器中打开
    pub open: Option<serde_json::Value>,
    
    /// 自定义HTTP响应头
    #[serde(default)]
    pub headers: std::collections::HashMap<String, String>,
    
    /// 会话存储配置
    #[serde(default)]
    pub session: Option<SessionConfig>,
}

/// 会话存储配置
#[derive(Debug, Deserialize, Default, Clone)]
pub struct SessionConfig {
    /// 会话驱动
    pub driver: Option<String>,
    
    /// 驱动配置选项
    #[serde(default)]
    pub options: serde_json::Value,
    
    /// Cookie配置
    pub cookie: Option<serde_json::Value>,
    
    /// 会话过期时间（秒）
    pub ttl: Option<u64>,
}

/// 开发者工具栏配置
#[derive(Debug, Deserialize, Default, Clone)]
pub struct DevToolbarConfig {
    /// 是否启用开发者工具栏
    #[serde(default = "default_dev_toolbar_enabled")]
    pub enabled: bool,
    
    /// 工具栏位置
    #[serde(default = "default_dev_toolbar_placement")]
    pub placement: String,
}

/// 预获取配置
#[derive(Debug, Deserialize, Default, Clone)]
pub struct PrefetchConfig {
    /// 是否为所有链接启用预获取
    pub prefetch_all: Option<bool>,
    
    /// 默认预获取策略
    #[serde(default = "default_prefetch_default_strategy")]
    pub default_strategy: String,
}

/// 图像配置
#[derive(Debug, Deserialize, Default, Clone)]
pub struct ImageConfig {
    /// 图像优化端点
    #[serde(default)]
    pub endpoint: ImageEndpointConfig,
    
    /// 图像服务配置
    #[serde(default)]
    pub service: ImageServiceConfig,
    
    /// 允许的远程图像域名
    #[serde(default)]
    pub domains: Vec<String>,
    
    /// 允许的远程图像URL模式
    #[serde(default)]
    pub remote_patterns: Vec<RemotePattern>,
    
    /// 是否为响应式图片添加全局样式
    #[serde(default = "default_image_responsive_styles")]
    pub responsive_styles: bool,
    
    /// 响应式图像的默认布局类型
    pub layout: Option<String>,
    
    /// 响应式图像的object-fit属性
    #[serde(default = "default_image_fit")]
    pub fit: String,
    
    /// 响应式图像的object-position属性
    #[serde(default = "default_image_object_position")]
    pub object_position: String,
    
    /// 用于生成响应式图像的断点
    pub breakpoints: Option<Vec<u32>>,
}

/// 图像端点配置
#[derive(Debug, Deserialize, Default, Clone)]
pub struct ImageEndpointConfig {
    /// 路由
    #[serde(default = "default_image_endpoint_route")]
    pub route: String,
    
    /// 入口点
    pub entrypoint: Option<String>,
}

/// 图像服务配置
#[derive(Debug, Deserialize, Default, Clone)]
pub struct ImageServiceConfig {
    /// 入口点
    #[serde(default = "default_image_service_entrypoint")]
    pub entrypoint: String,
    
    /// 配置
    #[serde(default)]
    pub config: serde_json::Value,
}

/// Markdown配置
#[derive(Debug, Deserialize, Default, Clone)]
pub struct MarkdownConfig {
    /// Shiki配置
    #[serde(default)]
    pub shiki_config: serde_json::Value,
    
    /// 语法高亮配置
    pub syntax_highlight: Option<serde_json::Value>,
    
    /// Remark插件
    #[serde(default)]
    pub remark_plugins: Vec<serde_json::Value>,
    
    /// Rehype插件
    #[serde(default)]
    pub rehype_plugins: Vec<serde_json::Value>,
    
    /// 是否使用GitHub-flavored Markdown
    #[serde(default = "default_markdown_gfm")]
    pub gfm: bool,
    
    /// 是否使用SmartyPants formatter
    #[serde(default = "default_markdown_smartypants")]
    pub smartypants: bool,
    
    /// remark-rehype选项
    pub remark_rehype: Option<serde_json::Value>,
}

/// 国际化配置
#[derive(Debug, Deserialize, Default, Clone)]
pub struct I18nConfig {
    /// 支持的语言环境列表
    pub locales: serde_json::Value,
    
    /// 默认语言环境
    pub default_locale: String,
    
    /// 回退策略
    #[serde(default)]
    pub fallback: std::collections::HashMap<String, String>,
    
    /// 路由策略
    pub routing: Option<serde_json::Value>,
    
    /// 语言环境的域名映射
    #[serde(default)]
    pub domains: std::collections::HashMap<String, String>,
}

/// 环境变量配置
#[derive(Debug, Deserialize, Default, Clone)]
pub struct EnvConfig {
    /// 环境变量模式
    #[serde(default)]
    pub schema: serde_json::Value,
    
    /// 是否验证私密环境变量
    #[serde(default = "default_env_validate_secrets")]
    pub validate_secrets: bool,
}

/// 默认末尾斜杠设置
fn default_trailing_slash() -> String {
    "ignore".to_string()
}

/// 默认输出目标
fn default_output() -> String {
    "static".to_string()
}

/// 默认源码目录
fn default_src_dir() -> String {
    "./src".to_string()
}

/// 默认静态资源目录
fn default_public_dir() -> String {
    "./public".to_string()
}

/// 默认输出目录
fn default_out_dir() -> String {
    "dist".to_string()
}

/// 默认缓存目录
fn default_cache_dir() -> String {
    "./node_modules/.astro".to_string()
}

/// 默认压缩HTML设置
fn default_compress_html() -> bool {
    true
}

/// 默认样式作用范围策略
fn default_scoped_style_strategy() -> String {
    "attribute".to_string()
}

/// 默认检查Origin头设置
fn default_check_origin() -> bool {
    true
}

/// 默认构建格式
fn default_build_format() -> String {
    "directory".to_string()
}

/// 默认客户端输出目录
fn default_build_client() -> String {
    "./client".to_string()
}

/// 默认服务器输出目录
fn default_build_server() -> String {
    "./server".to_string()
}

/// 默认资源目录
fn default_build_assets() -> String {
    "_astro".to_string()
}

/// 默认服务器入口文件名
fn default_build_server_entry() -> String {
    "entry.mjs".to_string()
}

/// 默认输出重定向到HTML设置
fn default_build_redirects() -> bool {
    true
}

/// 默认样式表内联设置
fn default_build_inline_stylesheets() -> String {
    "auto".to_string()
}

/// 默认并行构建页面数
fn default_build_concurrency() -> usize {
    1
}

/// 默认服务器端口
fn default_server_port() -> u16 {
    4321
}

/// 默认重定向状态码
fn default_redirect_status() -> u16 {
    301
}

/// 默认开发者工具栏启用状态
fn default_dev_toolbar_enabled() -> bool {
    true
}

/// 默认开发者工具栏位置
fn default_dev_toolbar_placement() -> String {
    "bottom-center".to_string()
}

/// 默认预获取策略
fn default_prefetch_default_strategy() -> String {
    "hover".to_string()
}

/// 默认图像端点路由
fn default_image_endpoint_route() -> String {
    "/_image".to_string()
}

/// 默认图像服务入口点
fn default_image_service_entrypoint() -> String {
    "astro/assets/services/sharp".to_string()
}

/// 默认响应式图片全局样式设置
fn default_image_responsive_styles() -> bool {
    false
}

/// 默认响应式图像的object-fit属性
fn default_image_fit() -> String {
    "cover".to_string()
}

/// 默认响应式图像的object-position属性
fn default_image_object_position() -> String {
    "center".to_string()
}

/// 默认GitHub-flavored Markdown设置
fn default_markdown_gfm() -> bool {
    true
}

/// 默认SmartyPants formatter设置
fn default_markdown_smartypants() -> bool {
    true
}

/// 默认环境变量验证私密设置
fn default_env_validate_secrets() -> bool {
    false
}

/// 配置管理器
pub struct ConfigManager {
    config: AstroConfig,
}

impl ConfigManager {
    /// 创建新的配置管理器
    pub fn new() -> Self {
        Self {
            config: AstroConfig::default(),
        }
    }
    
    /// 从项目目录加载配置
    pub fn load_from_project(&mut self, project_path: &Path) -> Result<AstroConfig, String> {
        // 尝试加载不同格式的配置文件
        let config_files = [
            "astro.config.mjs",
            "astro.config.js",
            "astro.config.ts",
            "astro.config.json",
        ];
        
        for config_file in &config_files {
            let config_path = project_path.join(config_file);
            if config_path.exists() {
                return self.load_config(&config_path);
            }
        }
        
        // 如果没有找到配置文件，返回默认配置
        Ok(self.config.clone())
    }
    
    /// 加载配置文件
    fn load_config(&mut self, config_path: &Path) -> Result<AstroConfig, String> {
        // 读取配置文件内容
        let content = fs::read_to_string(config_path)
            .map_err(|e| format!("Failed to read config file: {}", e))?;
        
        // 这里简化处理，实际应该根据文件类型使用不同的解析方法
        // 对于 JSON 文件，直接使用 serde_json 解析
        if config_path.extension().unwrap_or_default() == "json" {
            self.config = serde_json::from_str(&content)
                .map_err(|e| format!("Failed to parse JSON config: {}", e))?;
        } else {
            // 对于 JavaScript/TypeScript 文件，这里使用简化处理
            // 实际应该使用 Node.js 来执行配置文件
            // 这里暂时返回默认配置
            println!("Warning: Using default config for non-JSON config file");
        }
        
        Ok(self.config.clone())
    }
    
    /// 获取配置
    pub fn config(&self) -> &AstroConfig {
        &self.config
    }
}

impl Default for ConfigManager {
    fn default() -> Self {
        Self::new()
    }
}
