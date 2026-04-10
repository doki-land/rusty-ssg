//! 工具模块
//! 提供 CLI 命令、站点生成和主题系统等工具功能

pub mod cmd;
pub mod site_generator;
pub mod template;
pub mod theme;

#[cfg(feature = "dev")]
pub use cmd::DevCommand;
pub use cmd::{BuildCommand, CheckCommand, InitCommand};
pub use site_generator::{ConfigLoader, StaticSiteGenerator};
pub use template::UnifiedTemplateManager;
pub use theme::{DefaultTheme, LocaleInfo, NavItem, PageContext, SidebarGroup, SidebarLink, TemplateEngineType};

mod lib;
pub use lib::*;
