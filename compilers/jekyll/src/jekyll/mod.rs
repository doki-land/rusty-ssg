#![warn(missing_docs)]

//! Jekyll 核心模块
//!
//! 提供 Jekyll 静态站点生成器的核心功能实现，包括：
//! - 目录结构识别和管理
//! - Front Matter 解析
//! - 配置文件加载和处理
//! - 帖子（Posts）管理
//! - 集合（Collections）管理
//! - Liquid 模板引擎集成
//! - Markdown 转换
//! - 静态文件处理
//! - 数据文件处理

mod collection;
mod config;
mod front_matter;
mod liquid;
mod markdown;
mod post;
mod static_file;
mod structure;

pub use collection::{Collection, CollectionConfig, CollectionItem, CollectionManager};
pub use config::{JekyllConfig, JekyllConfigLoader};
pub use front_matter::FrontMatterParser;
pub use liquid::{LiquidEngine, LiquidFilter, LiquidTag};
pub use markdown::MarkdownConverter;
pub use post::{Post, PostManager};
pub use static_file::{StaticFile, StaticFileManager};
pub use structure::{JekyllDirectory, JekyllStructure};

use crate::errors::{JekyllError, Result};
