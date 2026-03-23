//! 工具模块
//! 提供站点生成、命令行工具和主题功能

pub mod cmd;
pub mod site_generator;
pub mod template;
pub mod theme;

pub use site_generator::{ConfigLoader, LanguageDocuments, StaticSiteGenerator};
pub use template::UnifiedTemplateManager;
pub use theme::{DefaultTheme, LocaleInfo, NavItem, PageContext, SidebarGroup, SidebarLink, SocialLink};

use clap::Parser;
use std::path::PathBuf;

/// VitePress CLI 主命令
#[derive(Parser, Debug)]
#[command(name = "vitepress")]
#[command(about = "VitePress 文档系统工具", long_about = None)]
pub struct VitePressCli {
    /// 子命令
    #[command(subcommand)]
    pub command: Commands,
}

/// 子命令
#[derive(Parser, Debug)]
pub enum Commands {
    /// 编译文档
    Build(BuildArgs),
    /// 开发模式（监听文件变化）
    #[cfg(feature = "dev")]
    Dev(DevArgs),
    /// 初始化项目
    Init(InitArgs),
    /// 检查文档
    Check(CheckArgs),
}

/// 编译参数
#[derive(Parser, Debug)]
pub struct BuildArgs {
    /// 源目录
    #[arg(short, long)]
    pub source: Option<PathBuf>,

    /// 输出目录
    #[arg(short, long)]
    pub output: Option<PathBuf>,

    /// 是否清理输出目录
    #[arg(short, long, default_value = "true")]
    pub clean: bool,

    /// 是否压缩输出
    #[arg(short, long, default_value = "false")]
    pub minify: bool,

    /// 是否启用调试模式
    #[arg(short, long, default_value = "false")]
    pub debug: bool,

    /// 配置文件路径
    #[arg(long)]
    pub config: Option<PathBuf>,
}

/// 开发模式参数
#[cfg(feature = "dev")]
#[derive(Parser, Debug)]
pub struct DevArgs {
    /// 源目录
    #[arg(short, long)]
    pub source: Option<PathBuf>,

    /// 端口
    #[arg(short, long, default_value = "5173")]
    pub port: u16,

    /// 主机地址
    #[arg(short, long, default_value = "0.0.0.0")]
    pub host: String,

    /// 是否自动打开浏览器
    #[arg(short, long, default_value = "false")]
    pub open: bool,

    /// 配置文件路径
    #[arg(long)]
    pub config: Option<PathBuf>,
}

/// 初始化参数
#[derive(Parser, Debug)]
pub struct InitArgs {
    /// 项目名称
    pub name: Option<String>,

    /// 模板名称
    #[arg(short, long, default_value = "default")]
    pub template: String,

    /// 是否强制覆盖现有文件
    #[arg(short, long, default_value = "false")]
    pub force: bool,

    /// 初始化路径
    #[arg(short, long)]
    pub path: Option<PathBuf>,
}

/// 检查参数
#[derive(Parser, Debug)]
pub struct CheckArgs {
    /// 源目录
    #[arg(short, long)]
    pub source: Option<PathBuf>,

    /// 是否启用严格模式
    #[arg(short, long, default_value = "false")]
    pub strict: bool,

    /// 是否自动修复问题
    #[arg(short, long, default_value = "false")]
    pub fix: bool,

    /// 配置文件路径
    #[arg(long)]
    pub config: Option<PathBuf>,
}
