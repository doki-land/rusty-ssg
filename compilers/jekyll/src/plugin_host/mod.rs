//! 插件宿主模块
//!
//! 提供与 Node.js 插件容器通信的功能，用于在 Rust 端调用 Node.js 插件。

use crate::types::ipc::{InvokePluginRequest, InvokePluginResponse, IpcMessage};
use serde_json;
use std::{
    fmt,
    io::{BufRead, BufReader, Write},
    process::{Command, Stdio},
    sync::{Arc, Mutex},
    time::Duration,
};

/// 插件宿主错误类型
///
/// 表示在插件宿主操作过程中可能发生的各种错误
#[derive(Debug)]
pub enum PluginHostError {
    /// 子进程启动失败
    ProcessStartError(String),

    /// IO 操作错误
    IoError(std::io::Error),

    /// JSON 序列化/反序列化错误
    JsonError(serde_json::Error),

    /// 通信超时错误
    Timeout(Duration),

    /// 子进程已关闭错误
    ProcessShutdown,

    /// 响应格式错误
    InvalidResponse,
}

impl fmt::Display for PluginHostError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PluginHostError::ProcessStartError(error) => write!(f, "Failed to start Node.js process: {}", error),
            PluginHostError::IoError(error) => write!(f, "IO error: {}", error),
            PluginHostError::JsonError(error) => write!(f, "JSON serialization error: {}", error),
            PluginHostError::Timeout(duration) => write!(f, "Communication timeout after {:?}", duration),
            PluginHostError::ProcessShutdown => write!(f, "Plugin host process has been shutdown"),
            PluginHostError::InvalidResponse => write!(f, "Invalid response format"),
        }
    }
}

impl From<std::io::Error> for PluginHostError {
    fn from(error: std::io::Error) -> Self {
        PluginHostError::IoError(error)
    }
}

impl From<serde_json::Error> for PluginHostError {
    fn from(error: serde_json::Error) -> Self {
        PluginHostError::JsonError(error)
    }
}

/// 插件宿主结果类型
pub type Result<T> = std::result::Result<T, PluginHostError>;

/// 插件宿主
///
/// 负责管理 Node.js 子进程，并通过 stdin/stdout 进行 JSON 通信来调用插件
pub struct PluginHost {
    /// 子进程句柄
    process: Option<std::process::Child>,

    /// 子进程 stdin 写入器
    stdin: Option<std::process::ChildStdin>,

    /// 子进程 stdout 读取器（通过 Mutex 保护，确保线程安全）
    stdout: Arc<Mutex<Option<BufReader<std::process::ChildStdout>>>>,

    /// 是否已关闭
    shutdown: bool,
}

impl PluginHost {
    /// 创建新的 PluginHost 实例
    ///
    /// 启动 Node.js 子进程并准备进行 IPC 通信
    ///
    /// # Arguments
    ///
    /// * `node_path` - Node.js 可执行文件的路径
    /// * `server_script` - IPC 服务器脚本的路径
    ///
    /// # Errors
    ///
    /// 返回 `PluginHostError::ProcessStartError` 如果子进程启动失败
    pub fn new(node_path: &str, server_script: &str) -> Result<Self> {
        let mut process = Command::new(node_path)
            .arg(server_script)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit())
            .spawn()
            .map_err(|e| PluginHostError::ProcessStartError(e.to_string()))?;

        let stdin =
            process.stdin.take().ok_or_else(|| PluginHostError::ProcessStartError("Failed to capture stdin".to_string()))?;

        let stdout =
            process.stdout.take().ok_or_else(|| PluginHostError::ProcessStartError("Failed to capture stdout".to_string()))?;

        Ok(Self {
            process: Some(process),
            stdin: Some(stdin),
            stdout: Arc::new(Mutex::new(Some(BufReader::new(stdout)))),
            shutdown: false,
        })
    }

    /// 调用插件
    ///
    /// 发送插件调用请求到 Node.js 端并等待响应
    ///
    /// # Arguments
    ///
    /// * `request` - 插件调用请求
    /// * `timeout` - 等待响应的超时时间
    ///
    /// # Errors
    ///
    /// 返回相应的 `PluginHostError` 如果发生错误
    pub fn invoke_plugin(&mut self, request: InvokePluginRequest, timeout: Duration) -> Result<InvokePluginResponse> {
        if self.shutdown {
            return Err(PluginHostError::ProcessShutdown);
        }

        let stdin = self.stdin.as_mut().ok_or(PluginHostError::ProcessShutdown)?;
        let message = IpcMessage::request(request);
        let json = message.to_json()?;

        writeln!(stdin, "{}", json)?;
        stdin.flush()?;

        self.read_response(timeout)
    }

    /// 读取响应
    ///
    /// 从子进程 stdout 读取并解析响应
    ///
    /// # Arguments
    ///
    /// * `timeout` - 等待响应的超时时间
    ///
    /// # Errors
    ///
    /// 返回相应的 `PluginHostError` 如果发生错误
    fn read_response(&self, timeout: Duration) -> Result<InvokePluginResponse> {
        let start = std::time::Instant::now();
        let mut stdout_guard = self.stdout.lock().unwrap();
        let stdout = stdout_guard.as_mut().ok_or(PluginHostError::ProcessShutdown)?;

        let mut line = String::new();
        loop {
            if start.elapsed() > timeout {
                return Err(PluginHostError::Timeout(timeout));
            }

            match stdout.read_line(&mut line) {
                Ok(0) => {
                    return Err(PluginHostError::ProcessShutdown);
                }
                Ok(_) => {
                    if line.trim().is_empty() {
                        line.clear();
                        continue;
                    }
                    break;
                }
                Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                    std::thread::sleep(Duration::from_millis(10));
                    continue;
                }
                Err(e) => {
                    return Err(PluginHostError::IoError(e));
                }
            }
        }

        let message = IpcMessage::from_json(line.trim())?;

        match message {
            IpcMessage::Response(response) => Ok(response),
            _ => Err(PluginHostError::InvalidResponse),
        }
    }

    /// 关闭插件宿主
    ///
    /// 优雅地关闭 Node.js 子进程
    ///
    /// # Errors
    ///
    /// 返回 `PluginHostError::IoError` 如果关闭过程中发生 IO 错误
    pub fn shutdown(&mut self) -> Result<()> {
        if self.shutdown {
            return Ok(());
        }

        self.shutdown = true;

        if let Some(mut process) = self.process.take() {
            drop(self.stdin.take());
            drop(self.stdout.lock().unwrap().take());

            process.kill()?;
            process.wait()?;
        }

        Ok(())
    }
}

impl Drop for PluginHost {
    /// 析构函数，确保子进程被正确关闭
    fn drop(&mut self) {
        let _ = self.shutdown();
    }
}
