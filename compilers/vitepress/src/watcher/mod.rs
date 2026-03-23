//! 文件监听器模块
//! 实现文件系统监听器，支持开发模式和热模块替换功能

use notify::{RecommendedWatcher, RecursiveMode, Watcher, Event, EventKind, EventResult};
use std::sync::mpsc::channel;
use std::time::Duration;
use std::path::PathBuf;

/// 文件监听器
/// 用于监控文件变化并触发相应的编译和更新
pub struct FileWatcher {
    /// 监听器实例
    watcher: RecommendedWatcher,
    /// 监控的目录
    watch_dirs: Vec<PathBuf>,
    /// 是否正在运行
    running: bool,
}

/// 文件变化事件
#[derive(Debug)]
pub struct FileChangeEvent {
    /// 变化的文件路径
    pub path: PathBuf,
    /// 变化类型
    pub change_type: FileChangeType,
}

/// 文件变化类型
#[derive(Debug, PartialEq, Eq)]
pub enum FileChangeType {
    /// 文件创建
    Create,
    /// 文件修改
    Modify,
    /// 文件删除
    Remove,
    /// 其他变化
    Other,
}

impl FileWatcher {
    /// 创建新的文件监听器
    pub fn new() -> Result<Self, notify::Error> {
        let (tx, rx) = channel();
        let watcher = RecommendedWatcher::new(move |res| {
            match res {
                Ok(event) => {
                    if let Err(e) = tx.send(event) {
                        eprintln!("Failed to send event: {}", e);
                    }
                }
                Err(e) => eprintln!("Watch error: {}", e),
            }
        }, Duration::from_secs(1))?;

        Ok(Self {
            watcher,
            watch_dirs: Vec::new(),
            running: false,
        })
    }

    /// 添加监控目录
    ///
    /// # Arguments
    ///
    /// * `dir` - 要监控的目录路径
    pub fn add_watch_dir(&mut self, dir: &str) -> Result<(), notify::Error> {
        let path = PathBuf::from(dir);
        self.watcher.watch(&path, RecursiveMode::Recursive)?;
        self.watch_dirs.push(path);
        Ok(())
    }

    /// 开始监控
    ///
    /// # Arguments
    ///
    /// * `callback` - 文件变化回调函数
    pub fn start<F>(&mut self, mut callback: F) -> Result<(), notify::Error>
    where
        F: FnMut(FileChangeEvent) -> (),
    {
        self.running = true;

        while self.running {
            match self.watcher.receiver().recv_timeout(Duration::from_secs(1)) {
                Ok(event) => {
                    if let Some(event) = self.process_event(event) {
                        callback(event);
                    }
                }
                Err(std::sync::mpsc::RecvTimeoutError::Timeout) => {
                    // 超时，继续循环
                }
                Err(e) => {
                    eprintln!("Error receiving event: {}", e);
                    break;
                }
            }
        }

        Ok(())
    }

    /// 停止监控
    pub fn stop(&mut self) {
        self.running = false;
    }

    /// 处理文件变化事件
    ///
    /// # Arguments
    ///
    /// * `event` - 原始事件
    ///
    /// # Returns
    ///
    /// 处理后的文件变化事件
    fn process_event(&self, event: Event) -> Option<FileChangeEvent> {
        let change_type = match event.kind {
            EventKind::Create(_) => FileChangeType::Create,
            EventKind::Modify(_) => FileChangeType::Modify,
            EventKind::Remove(_) => FileChangeType::Remove,
            _ => FileChangeType::Other,
        };

        // 只处理 Markdown 文件和配置文件的变化
        for path in event.paths {
            if self.is_watched_file(&path) {
                return Some(FileChangeEvent {
                    path,
                    change_type,
                });
            }
        }

        None
    }

    /// 检查文件是否是需要监控的文件
    ///
    /// # Arguments
    ///
    /// * `path` - 文件路径
    ///
    /// # Returns
    ///
    /// 是否是需要监控的文件
    fn is_watched_file(&self, path: &PathBuf) -> bool {
        // 检查文件扩展名
        if let Some(ext) = path.extension() {
            match ext.to_str() {
                Some("md") | Some("markdown") | Some("txt") => return true,
                Some("js") | Some("ts") | Some("json") | Some("toml") => return true,
                _ => {}
            }
        }

        // 检查是否是配置文件
        if let Some(file_name) = path.file_name() {
            if file_name.to_str() == Some("vitepress.config.js") ||
               file_name.to_str() == Some("vitepress.config.ts") ||
               file_name.to_str() == Some("vitepress.config.json") ||
               file_name.to_str() == Some("vitepress.config.toml") {
                return true;
            }
        }

        false
    }
}

impl Drop for FileWatcher {
    fn drop(&mut self) {
        self.stop();
    }
}
