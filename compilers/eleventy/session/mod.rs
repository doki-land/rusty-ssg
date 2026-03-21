//! 会话模块

/// 编译会话
pub struct Session {
    /// 会话 ID
    id: String,
}

impl Session {
    /// 创建新的会话
    pub fn new(id: &str) -> Self {
        Self { id: id.to_string() }
    }
}
