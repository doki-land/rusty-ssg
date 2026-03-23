#![warn(missing_docs)]
#![doc = "Gatsby 编译器 CLI 工具库"]

pub mod cmd;
pub mod site_generator;
pub mod template;
pub mod theme;

pub use site_generator::{ConfigLoader, StaticSiteGenerator};
pub use template::UnifiedTemplateManager;
pub use theme::{DefaultTheme, LocaleInfo, NavItem, PageContext, SidebarGroup, SidebarLink, TemplateEngineType};

use clap::Parser;
use std::path::PathBuf;

/// Gatsby CLI 主命令
#[derive(Parser, Debug)]
#[command(name = "gatsby")]
#[command(about = "Gatsby 兼容静态站点生成器", long_about = None)]
pub struct GatsbyCli {
    /// 子命令
    #[command(subcommand)]
    pub command: GatsbyCommands,
}

/// Gatsby 子命令
#[derive(Parser, Debug)]
pub enum GatsbyCommands {
    /// 创建新内容
    New(NewArgs),
    /// 构建整个静态站点
    Build(BuildArgs),
    /// 开发服务器（支持热重载）
    #[cfg(feature = "dev")]
    Develop(DevelopArgs),
    /// 显示版本信息
    Version,
    /// 初始化项目
    Init(InitArgs),
    /// 检查文档
    Check(CheckArgs),
    /// 清理缓存和构建文件
    Clean(CleanArgs),
    /// 显示项目信息
    Info,
    /// 插件相关命令
    Plugin(PluginArgs),
    /// 遥测相关命令
    Telemetry(TelemetryArgs),
}

/// new 命令参数
#[derive(Parser, Debug)]
pub struct NewArgs {
    /// 内容路径（例如：posts/my-first-post.md）
    pub path: PathBuf,

    /// 使用的 archetype
    #[arg(short, long)]
    pub kind: Option<String>,

    /// 源目录
    #[arg(short, long)]
    pub source: Option<PathBuf>,
}

/// build 命令参数
#[derive(Parser, Debug)]
pub struct BuildArgs {
    /// 源目录
    #[arg(short, long)]
    pub source: Option<PathBuf>,

    /// 输出目录
    #[arg(short, long, default_value = "public")]
    pub destination: PathBuf,

    /// 是否清理输出目录
    #[arg(short, long)]
    pub clean_destination_dir: bool,

    /// 是否压缩输出
    #[arg(long)]
    pub minify: bool,
}

/// develop 命令参数
#[cfg(feature = "dev")]
#[derive(Parser, Debug)]
pub struct DevelopArgs {
    /// 源目录
    #[arg(short, long)]
    pub source: Option<PathBuf>,

    /// 端口
    #[arg(short, long, default_value = "8000")]
    pub port: u16,

    /// 是否禁用浏览器自动打开
    #[arg(long)]
    pub no_browser: bool,

    /// 绑定地址
    #[arg(long, default_value = "127.0.0.1")]
    pub bind: String,
}

/// 初始化参数
#[derive(Parser, Debug)]
pub struct InitArgs {
    /// 项目名称
    pub name: Option<String>,
}

/// 检查参数
#[derive(Parser, Debug)]
pub struct CheckArgs {
    /// 源目录
    #[arg(short, long)]
    pub source: Option<PathBuf>,
}

/// clean 命令参数
#[derive(Parser, Debug)]
pub struct CleanArgs {
    /// 是否强制清理
    #[arg(short, long)]
    pub force: bool,
}

/// plugin 命令参数
#[derive(Parser, Debug)]
pub struct PluginArgs {
    /// 插件子命令
    #[command(subcommand)]
    pub subcommand: PluginSubCommand,
}

/// 插件子命令
#[derive(Parser, Debug)]
pub enum PluginSubCommand {
    /// 安装插件
    Install(InstallArgs),
    /// 卸载插件
    Uninstall(UninstallArgs),
    /// 列出已安装插件
    List,
}

/// 安装插件参数
#[derive(Parser, Debug)]
pub struct InstallArgs {
    /// 插件名称
    pub plugins: Vec<String>,
}

/// 卸载插件参数
#[derive(Parser, Debug)]
pub struct UninstallArgs {
    /// 插件名称
    pub plugins: Vec<String>,
}

/// telemetry 命令参数
#[derive(Parser, Debug)]
pub struct TelemetryArgs {
    /// 遥测子命令
    #[command(subcommand)]
    pub subcommand: TelemetrySubCommand,
}

/// 遥测子命令
#[derive(Parser, Debug)]
pub enum TelemetrySubCommand {
    /// 启用遥测
    Enable,
    /// 禁用遥测
    Disable,
    /// 显示遥测状态
    Status,
}
