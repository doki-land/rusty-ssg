#![warn(missing_docs)]
#![doc = "MkDocs 文档系统 CLI 工具库"]

use clap::Parser;
use std::path::PathBuf;

pub mod cmd;
pub mod site_generator;
pub mod template;
pub mod theme;

pub use crate::{
    MkDocsCompiler,
    types::{MkDocsConfig, Result, MkDocsError},
};
pub use site_generator::{ConfigLoader, StaticSiteGenerator};
pub use template::UnifiedTemplateManager;
pub use theme::{DefaultTheme, NavItem, PageContext, SidebarGroup, SidebarLink};

pub use cmd::{
    BuildCommand,
    CheckCommand,
    InitCommand,
    NewCommand,
    ServeCommand,
    VersionCommand,
};

/// MkDocs 兼容 CLI 主命令
#[derive(Parser, Debug)]
#[command(name = "mkdocs")]
#[command(about = "MkDocs 兼容静态站点生成器", long_about = None)]
pub struct MkDocsCli {
    /// 子命令
    #[command(subcommand)]
    pub command: MkDocsCommands,
}

/// MkDocs 子命令
#[derive(Parser, Debug)]
pub enum MkDocsCommands {
    /// 创建新内容
    New(NewArgs),
    /// 构建整个静态站点
    Build(BuildArgs),
    /// 开发服务器（支持热重载）
    Serve(ServeArgs),
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
    /// 内容路径（例如：docs/my-page.md）
    pub path: PathBuf,

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
    #[arg(short, long, default_value = "site")]
    pub destination: PathBuf,

    /// 是否清理输出目录
    #[arg(short, long)]
    pub clean: bool,
}

/// serve 命令参数
#[derive(Parser, Debug)]
pub struct ServeArgs {
    /// 源目录
    #[arg(short, long)]
    pub source: Option<PathBuf>,

    /// 端口
    #[arg(short, long, default_value = "8000")]
    pub port: Option<u16>,

    /// 开发地址
    #[arg(long)]
    pub dev_addr: Option<String>,
}

/// 初始化参数
#[derive(Parser, Debug)]
pub struct InitArgs {
    /// 项目名称
    pub name: Option<String>,

    /// 目标目录
    #[arg(short, long)]
    pub directory: Option<PathBuf>,
}

/// 检查参数
#[derive(Parser, Debug)]
pub struct CheckArgs {
    /// 源目录
    #[arg(short, long)]
    pub source: Option<PathBuf>,
}
