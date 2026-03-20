//! IPC 通信协议模块
//!
//! 定义 VuTeX Rust 端和 Node.js 端之间的 IPC 通信协议类型，
//! 用于插件调用和数据交换。

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 插件调用上下文
///
/// 包含执行插件时所需的文档信息和元数据
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PluginContext {
    /// 文档内容
    pub content: String,

    /// 文档元数据
    pub frontmatter: HashMap<String, String>,

    /// 文档文件路径
    pub path: String,
}

impl PluginContext {
    /// 创建新的插件调用上下文
    ///
    /// # Arguments
    ///
    /// * `content` - 文档内容
    /// * `frontmatter` - 文档元数据
    /// * `path` - 文档文件路径
    pub fn new(content: String, frontmatter: HashMap<String, String>, path: String) -> Self {
        Self { content, frontmatter, path }
    }
}

/// 插件调用请求
///
/// 从 Rust 端发送到 Node.js 端，用于请求执行特定插件
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InvokePluginRequest {
    /// 钩子名称（before_render 或 after_render）
    pub hook_name: String,

    /// 插件名称
    pub plugin_name: String,

    /// 插件调用上下文
    pub context: PluginContext,
}

impl InvokePluginRequest {
    /// 创建新的插件调用请求
    ///
    /// # Arguments
    ///
    /// * `hook_name` - 钩子名称
    /// * `plugin_name` - 插件名称
    /// * `context` - 插件调用上下文
    pub fn new(hook_name: String, plugin_name: String, context: PluginContext) -> Self {
        Self { hook_name, plugin_name, context }
    }
}

/// 插件调用响应
///
/// 从 Node.js 端返回给 Rust 端，包含插件执行结果
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InvokePluginResponse {
    /// 是否成功执行
    pub success: bool,

    /// 处理后的内容（如果成功）
    pub content: Option<String>,

    /// 错误信息（如果失败）
    pub error: Option<String>,
}

impl InvokePluginResponse {
    /// 创建成功的响应
    ///
    /// # Arguments
    ///
    /// * `content` - 处理后的内容
    pub fn success(content: String) -> Self {
        Self { success: true, content: Some(content), error: None }
    }

    /// 创建失败的响应
    ///
    /// # Arguments
    ///
    /// * `error` - 错误信息
    pub fn error(error: String) -> Self {
        Self { success: false, content: None, error: Some(error) }
    }
}

/// IPC 消息类型
///
/// 用于在 Rust 端和 Node.js 端之间传输请求和响应
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", content = "payload")]
pub enum IpcMessage {
    /// 请求消息
    Request(InvokePluginRequest),

    /// 响应消息
    Response(InvokePluginResponse),
}

impl IpcMessage {
    /// 创建请求消息
    ///
    /// # Arguments
    ///
    /// * `request` - 插件调用请求
    pub fn request(request: InvokePluginRequest) -> Self {
        Self::Request(request)
    }

    /// 创建响应消息
    ///
    /// # Arguments
    ///
    /// * `response` - 插件调用响应
    pub fn response(response: InvokePluginResponse) -> Self {
        Self::Response(response)
    }

    /// 将消息序列化为 JSON 字符串
    ///
    /// # Errors
    ///
    /// 返回 `serde_json::Error` 如果序列化失败
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }

    /// 从 JSON 字符串反序列化消息
    ///
    /// # Arguments
    ///
    /// * `json` - JSON 格式的字符串
    ///
    /// # Errors
    ///
    /// 返回 `serde_json::Error` 如果反序列化失败
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }
}
