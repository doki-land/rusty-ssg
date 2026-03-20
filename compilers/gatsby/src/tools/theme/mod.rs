//! 主题系统模块
//! 提供主题管理和模板渲染功能

pub mod default_theme;

pub use default_theme::{
    DefaultTheme, LocaleInfo, NavItem, PageContext, SidebarGroup, SidebarLink,
    TemplateEngineType,
};
