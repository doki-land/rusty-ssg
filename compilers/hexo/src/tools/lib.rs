//! Rusty Hexo 工具库

#![warn(missing_docs)]

use clap::Parser;
use std::path::PathBuf;

/// Hexo CLI 主命令
#[derive(Parser, Debug)]
#[command(name = "hexo")]
#[command(about = "Rusty Hexo 博客框架", long_about = None)]
pub struct HexoCli {
    /// 子命令
    #[command(subcommand)]
    pub command: Commands,
}

/// 子命令
#[derive(Parser, Debug)]
pub enum Commands {
    /// 初始化博客
    Init(InitArgs),
    /// 新建文章
    New(NewArgs),
    /// 生成静态文件
    Generate(GenerateArgs),
    /// 启动服务器
    Server(ServerArgs),
    /// 部署博客
    Deploy(DeployArgs),
    /// 清理缓存
    Clean(CleanArgs),
    /// 列出插件
    Plugin(PluginArgs),
}

/// 初始化参数
#[derive(Parser, Debug)]
pub struct InitArgs {
    /// 博客名称
    pub name: Option<String>,
    /// 博客目录
    #[arg(short, long)]
    pub dir: Option<PathBuf>,
}

/// 新建文章参数
#[derive(Parser, Debug)]
pub struct NewArgs {
    /// 文章标题
    pub title: String,
    /// 文章布局
    #[arg(short, long, default_value = "post")]
    pub layout: String,
    /// 文章路径
    #[arg(short, long)]
    pub path: Option<PathBuf>,
    /// 是否草稿
    #[arg(short, long)]
    pub draft: bool,
    /// 是否发布
    #[arg(short, long)]
    pub publish: bool,
}

/// 生成参数
#[derive(Parser, Debug)]
pub struct GenerateArgs {
    /// 源目录
    #[arg(short, long)]
    pub source: Option<PathBuf>,
    /// 输出目录
    #[arg(short, long)]
    pub output: Option<PathBuf>,
    /// 是否清理输出目录
    #[arg(short, long)]
    pub clean: bool,
    /// 是否监视文件变化
    #[arg(short, long)]
    pub watch: bool,
}

/// 服务器参数
#[derive(Parser, Debug)]
pub struct ServerArgs {
    /// 端口
    #[arg(short, long, default_value = "4000")]
    pub port: u16,
    /// 主机
    #[arg(short, long, default_value = "localhost")]
    pub host: String,
    /// 是否打开浏览器
    #[arg(short, long)]
    pub open: bool,
    /// 是否监视文件变化
    #[arg(short, long)]
    pub watch: bool,
}

/// 部署参数
#[derive(Parser, Debug)]
pub struct DeployArgs {
    /// 部署环境
    #[arg(short, long)]
    pub env: Option<String>,
}

/// 清理参数
#[derive(Parser, Debug)]
pub struct CleanArgs {
    /// 是否清理数据库
    #[arg(short, long)]
    pub database: bool,
}

/// 插件参数
#[derive(Parser, Debug)]
pub struct PluginArgs {
    /// 插件子命令
    #[command(subcommand)]
    pub subcommand: PluginSubcommand,
}

/// 插件子命令
#[derive(Parser, Debug)]
pub enum PluginSubcommand {
    /// 列出所有插件
    List,
    /// 安装插件
    Install(PluginInstallArgs),
    /// 卸载插件
    Uninstall(PluginUninstallArgs),
}

/// 插件安装参数
#[derive(Parser, Debug)]
pub struct PluginInstallArgs {
    /// 插件名称
    pub name: String,
}

/// 插件卸载参数
#[derive(Parser, Debug)]
pub struct PluginUninstallArgs {
    /// 插件名称
    pub name: String,
}
