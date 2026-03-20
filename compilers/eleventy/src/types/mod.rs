//! 类型定义模块

use serde::{Deserialize, Serialize};

/// 配置选项
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    /// 输入目录
    pub input_dir: String,
    /// 输出目录
    pub output_dir: String,
    /// 模板目录
    pub template_dir: String,
    /// 数据目录
    pub data_dir: String,
    /// 插件配置
    pub plugins: Vec<PluginConfig>,
}

/// 插件配置
#[derive(Debug, Serialize, Deserialize)]
pub struct PluginConfig {
    /// 插件名称
    pub name: String,
    /// 插件选项
    pub options: serde_json::Value,
}

/// 页面数据
#[derive(Debug, Serialize, Deserialize)]
pub struct PageData {
    /// 页面标题
    pub title: String,
    /// 页面描述
    pub description: Option<String>,
    /// 页面日期
    pub date: Option<String>,
    /// 页面标签
    pub tags: Vec<String>,
    /// 页面前置数据
    pub frontmatter: serde_json::Value,
}

/// 构建结果
#[derive(Debug)]
pub struct BuildResult {
    /// 成功构建的页面数量
    pub success_count: usize,
    /// 失败的页面数量
    pub error_count: usize,
    /// 构建时间（毫秒）
    pub build_time: u64,
}

/// 命令行参数
#[derive(Debug, clap::Parser)]
pub struct Cli {
    /// 命令
    #[clap(subcommand)]
    pub command: Command,
}

/// 命令
#[derive(Debug, clap::Subcommand)]
pub enum Command {
    /// 构建站点
    Build {
        /// 输入目录
        #[clap(short, long)]
        input: Option<String>,
        
        /// 输出目录
        #[clap(short, long)]
        output: Option<String>,
    },
    
    /// 启动开发服务器
    Serve {
        /// 端口
        #[clap(short, long, default_value = "8080")]
        port: u16,
        
        /// 输入目录
        #[clap(short, long)]
        input: Option<String>,
    },
    
    /// 监视文件变更
    Watch {
        /// 输入目录
        #[clap(short, long)]
        input: Option<String>,
    },
}