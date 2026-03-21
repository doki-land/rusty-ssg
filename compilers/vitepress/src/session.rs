//! 会话模块
//! 用于管理编译会话

use crate::types::VutexConfig;

/// 编译会话
/// 管理编译过程中的状态
pub struct CompileSession {
    /// 会话配置
    config: VutexConfig,
}

impl CompileSession {
    /// 创建新的会话
    pub fn new(config: VutexConfig) -> Self {
        Self { config }
    }

    /// 获取会话配置
    pub fn config(&self) -> &VutexConfig {
        &self.config
    }

    /// 获取可变的会话配置
    pub fn config_mut(&mut self) -> &mut VutexConfig {
        &mut self.config
    }
}

impl Default for CompileSession {
    fn default() -> Self {
        Self::new(VutexConfig::new())
    }
}
