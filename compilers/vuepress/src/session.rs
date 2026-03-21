//! 会话模块
//! 管理编译会话状态

use crate::{Result, types::VutexConfig};

/// 编译会话
pub struct CompileSession {
    /// 会话配置
    config: VutexConfig,
}

impl CompileSession {
    /// 创建新的会话实例
    pub fn new(config: VutexConfig) -> Self {
        Self { config }
    }

    /// 初始化会话
    pub fn initialize(&mut self) -> Result<()> {
        Ok(())
    }

    /// 关闭会话
    pub fn shutdown(&mut self) -> Result<()> {
        Ok(())
    }
}

/// 会话
pub struct Session {
    /// 会话配置
    config: VutexConfig,
}

impl Session {
    /// 创建新的会话实例
    pub fn new(config: VutexConfig) -> Self {
        Self { config }
    }

    /// 初始化会话
    pub fn initialize(&mut self) -> Result<()> {
        Ok(())
    }

    /// 关闭会话
    pub fn shutdown(&mut self) -> Result<()> {
        Ok(())
    }
}
