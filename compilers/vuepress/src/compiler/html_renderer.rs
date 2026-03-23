//! HTML 渲染器模块
//! 负责将解析后的内容渲染为 HTML

use crate::{Result, types::VuePressConfig};

/// HTML 渲染器配置
pub struct HtmlRendererConfig {
    /// 是否启用压缩
    pub minify: bool,
}

impl Default for HtmlRendererConfig {
    fn default() -> Self {
        Self { minify: false }
    }
}

/// HTML 渲染器
pub struct HtmlRenderer {
    /// 渲染器配置
    config: VuePressConfig,
    /// HTML 渲染器配置
    html_config: HtmlRendererConfig,
}

impl HtmlRenderer {
    /// 创建新的 HTML 渲染器实例
    pub fn new(config: VuePressConfig) -> Self {
        Self { config, html_config: HtmlRendererConfig::default() }
    }

    /// 渲染内容为 HTML
    pub fn render(&self, content: &str) -> Result<String> {
        Ok(content.to_string())
    }
}
