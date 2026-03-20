//! 工具模块
//! 提供站点生成、命令行工具和主题功能

pub mod cmd;
pub mod site_generator;
pub mod theme;

pub use site_generator::{ConfigLoader, StaticSiteGenerator, LanguageDocuments};
pub use theme::{DefaultTheme, LocaleInfo, NavItem, PageContext, SidebarGroup, SidebarLink, SocialLink};

use clap::Parser;
use std::path::PathBuf;

/// VuTeX CLI 主命令
#[derive(Parser, Debug)]
#[command(name = "vutex")]
#[command(about = "VuTeX 文档系统工具", long_about = None)]
pub struct VutexCli {
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
    #[arg(short, long)]
    pub clean: bool,
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
