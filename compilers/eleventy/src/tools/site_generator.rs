//! 站点生成器模块

/// 站点生成器
pub struct SiteGenerator {
    /// 生成器配置
    config: String,
}

impl SiteGenerator {
    /// 创建新的站点生成器
    pub fn new(config: String) -> Self {
        Self {
            config,
        }
    }
    
    /// 生成站点
    pub fn generate(&self) -> Result<(), String> {
        Ok(())
    }
}
