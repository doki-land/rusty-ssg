//! 主题模块
//! 提供 MkDocs 主题相关的功能

pub mod default_theme;

pub use default_theme::{
    DefaultTheme, PageContext, ThemeNavItem as NavItem, ThemeSidebarGroup as SidebarGroup,
    ThemeSidebarLink as SidebarLink,
};
