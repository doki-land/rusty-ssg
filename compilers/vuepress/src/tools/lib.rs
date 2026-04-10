#![warn(missing_docs)]
#![doc = "VuePress 文档系统 CLI 工具库"]

pub use crate::{
    CompileResult, CompileSession, PluginHost,
    compiler::VuePressCompiler,
    types::{Result, VuePressConfig, VutexError},
};

use clap::Parser;
use std::path::PathBuf;

/// VuePress CLI 主命令
#[derive(Parser, Debug)]
#[command(name = "vuepress")]
#[command(about = "VuePress 文档系统工具", long_about = None)]
pub struct VuePressCli {
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
    #[arg(short, long, default_value = "8080")]
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
