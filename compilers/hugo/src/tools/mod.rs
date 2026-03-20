#![warn(missing_docs)]
#![doc = "VuTeX 文档系统 CLI 工具库"]

pub mod cmd;
pub mod site_generator;
pub mod template;
pub mod theme;

pub use crate::{
    CompileResult, CompileSession, Document, VutexCompiler,
    types::{HugoConfig, Result, VutexError},
};
pub use site_generator::{ConfigLoader, StaticSiteGenerator};
pub use template::UnifiedTemplateManager;
pub use theme::{DefaultTheme, NavItem, PageContext, SidebarGroup, SidebarLink};

use clap::Parser;
use std::path::PathBuf;

/// Hugo 兼容 CLI 主命令
#[derive(Parser, Debug)]
#[command(name = "hugo")]
#[command(about = "Hugo 兼容静态站点生成器", long_about = None)]
pub struct HugoCli {
    /// 子命令
    #[command(subcommand)]
    pub command: HugoCommands,
}

/// Hugo 子命令
#[derive(Parser, Debug)]
pub enum HugoCommands {
    /// 创建新内容
    New(NewArgs),
    /// 构建整个静态站点
    Build(BuildArgs),
    /// 开发服务器（支持热重载）
    #[cfg(feature = "dev")]
    Server(ServerArgs),
    /// 显示版本信息
    Version,
    /// 初始化项目
    Init(InitArgs),
    /// 检查文档
    Check(CheckArgs),
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

/// server 命令参数
#[cfg(feature = "dev")]
#[derive(Parser, Debug)]
pub struct ServerArgs {
    /// 源目录
    #[arg(short, long)]
    pub source: Option<PathBuf>,

    /// 端口
    #[arg(short, long, default_value = "1313")]
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

/// 向后兼容 - VuTeX CLI 主命令
#[derive(Parser, Debug)]
#[command(name = "vutex")]
#[command(about = "VuTeX 文档系统工具", long_about = None)]
pub struct VutexCli {
    /// 子命令
    #[command(subcommand)]
    pub command: Commands,
}

/// 向后兼容 - 子命令
#[derive(Parser, Debug)]
pub enum Commands {
    /// 编译文档
    Build(VutexBuildArgs),
    /// 开发模式（监听文件变化）
    #[cfg(feature = "dev")]
    Dev(DevArgs),
    /// 初始化项目
    Init(InitArgs),
    /// 检查文档
    Check(CheckArgs),
}

/// 向后兼容 - 编译参数
#[derive(Parser, Debug)]
pub struct VutexBuildArgs {
    /// 源目录
    #[arg(short, long)]
    pub source: Option<PathBuf>,

    /// 输出目录
    #[arg(short, long)]
    pub output: Option<PathBuf>,

    /// 是否清理输出目录
    #[arg(short, long)]
    pub clean: bool,
}

/// 向后兼容 - 开发模式参数
#[cfg(feature = "dev")]
#[derive(Parser, Debug)]
pub struct DevArgs {
    /// 源目录
    #[arg(short, long)]
    pub source: Option<PathBuf>,

    /// 端口
    #[arg(short, long, default_value = "5173")]
    pub port: u16,
}
