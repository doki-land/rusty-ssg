#![warn(missing_docs)]
#![doc = "Jekyll 静态站点生成器 CLI 工具库"]

pub use super::{
    site_generator::{ConfigLoader, StaticSiteGenerator},
    template::UnifiedTemplateManager,
    theme::{DefaultTheme, NavItem, PageContext, SidebarGroup, SidebarLink},
};
pub use crate::{
    CompileResult, Document, VutexCompiler,
    types::{
        config::VutexConfig,
        errors::{Result, VutexError},
    },
};

use clap::Parser;
use std::path::PathBuf;

/// Jekyll CLI 主命令
#[derive(Parser, Debug)]
#[command(
    name = "jekyll",
    about = "Jekyll 静态站点生成器",
    long_about = "Rusty Jekyll 是一个用 Rust 编写的 Jekyll 兼容静态站点生成器，提供快速、安全的站点构建体验。"
)]
pub struct JekyllCli {
    /// 启用详细输出
    #[arg(short, long, global = true)]
    pub verbose: bool,

    /// 启用静默模式（减少输出）
    #[arg(short, long, global = true, conflicts_with = "verbose")]
    pub quiet: bool,

    /// 子命令
    #[command(subcommand)]
    pub command: Commands,
}

/// 子命令集合
#[derive(Parser, Debug)]
pub enum Commands {
    /// 编译生成静态站点
    Build(BuildArgs),
    /// 启动开发服务器（监听文件变化并自动重新构建）
    #[cfg(feature = "dev")]
    Dev(DevArgs),
    /// 在当前目录初始化新的 Jekyll 项目
    Init(InitArgs),
    /// 检查站点配置和内容的有效性
    Check(CheckArgs),
}

/// Build 命令参数配置
#[derive(Parser, Debug)]
pub struct BuildArgs {
    /// 源目录（默认为当前目录）
    #[arg(short, long, value_name = "DIR")]
    pub source: Option<PathBuf>,

    /// 输出目录（默认为 _site）
    #[arg(short, long, value_name = "DIR")]
    pub output: Option<PathBuf>,

    /// 构建前清理输出目录
    #[arg(short, long)]
    pub clean: bool,

    /// 增量构建（仅重新构建变更的文件）
    #[arg(short, long)]
    pub incremental: bool,

    /// 显示构建进度
    #[arg(short, long)]
    pub profile: bool,

    /// 配置文件路径（默认为 _config.yml）
    #[arg(short, long, value_name = "FILE")]
    pub config: Option<PathBuf>,

    /// 设置额外的配置选项（格式：key=value）
    #[arg(short = 'c', long = "config-option", value_name = "KEY=VALUE")]
    pub config_options: Vec<String>,
}

/// Dev 命令参数配置
#[cfg(feature = "dev")]
#[derive(Parser, Debug)]
pub struct DevArgs {
    /// 源目录（默认为当前目录）
    #[arg(short, long, value_name = "DIR")]
    pub source: Option<PathBuf>,

    /// 监听端口（默认 4000）
    #[arg(short, long, default_value = "4000", value_name = "PORT")]
    pub port: u16,

    /// 绑定地址（默认 127.0.0.1）
    #[arg(short = 'H', long, default_value = "127.0.0.1", value_name = "HOST")]
    pub host: String,

    /// 自动打开浏览器
    #[arg(short, long)]
    pub open: bool,

    /// 启用 LiveReload
    #[arg(short, long)]
    pub livereload: bool,

    /// LiveReload 端口（默认 35729）
    #[arg(long, default_value = "35729", value_name = "PORT")]
    pub livereload_port: u16,

    /// 输出目录（默认为 _site）
    #[arg(short, long, value_name = "DIR")]
    pub output: Option<PathBuf>,

    /// 配置文件路径（默认为 _config.yml）
    #[arg(short, long, value_name = "FILE")]
    pub config: Option<PathBuf>,
}

/// Init 命令参数配置
#[derive(Parser, Debug)]
pub struct InitArgs {
    /// 项目名称（默认为 my-jekyll-site）
    pub name: Option<String>,

    /// 在当前目录初始化，不创建新目录
    #[arg(short, long)]
    pub force: bool,

    /// 使用空白模板初始化
    #[arg(short, long)]
    pub blank: bool,

    /// 指定初始化的目录
    #[arg(short, long, value_name = "DIR")]
    pub destination: Option<PathBuf>,

    /// 跳过 Git 仓库初始化
    #[arg(long)]
    pub skip_git: bool,

    /// 跳过示例内容创建
    #[arg(long)]
    pub skip_example: bool,
}

/// Check 命令参数配置
#[derive(Parser, Debug)]
pub struct CheckArgs {
    /// 源目录（默认为当前目录）
    #[arg(short, long, value_name = "DIR")]
    pub source: Option<PathBuf>,

    /// 配置文件路径（默认为 _config.yml）
    #[arg(short, long, value_name = "FILE")]
    pub config: Option<PathBuf>,

    /// 检查所有文件，包括草稿
    #[arg(short, long)]
    pub drafts: bool,

    /// 显示详细的检查结果
    #[arg(short, long)]
    pub detailed: bool,

    /// 只检查配置文件
    #[arg(long)]
    pub config_only: bool,

    /// 只检查内容文件
    #[arg(long, conflicts_with = "config_only")]
    pub content_only: bool,
}
