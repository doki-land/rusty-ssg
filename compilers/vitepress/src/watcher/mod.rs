//! 文件监听模块
//! 提供文件系统监听功能，用于开发模式的自动重新构建

use notify::{Config, Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::time::Duration;

/// 文件变化事件
#[derive(Debug, Clone)]
pub struct FileChange {
    /// 变化类型
    pub kind: FileChangeKind,
    /// 变化的路径
    pub path: PathBuf,
}

/// 文件变化类型
#[derive(Debug, Clone, PartialEq)]
pub enum FileChangeKind {
    /// 文件创建
    Created,
    /// 文件修改
    Modified,
    /// 文件删除
    Removed,
    /// 目录创建
    DirCreated,
    /// 目录删除
    DirRemoved,
}

impl From<&EventKind> for FileChangeKind {
    fn from(kind: &EventKind) -> Self {
        match kind {
            EventKind::Create(_) => FileChangeKind::Created,
            EventKind::Modify(_) => FileChangeKind::Modified,
            EventKind::Remove(_) => FileChangeKind::Removed,
            _ => FileChangeKind::Modified,
        }
    }
}

impl FileChange {
    /// 从 notify::Event 创建
    pub fn from_event(event: &Event) -> Vec<Self> {
        event.paths.iter().map(|path| FileChange { kind: FileChangeKind::from(&event.kind), path: path.clone() }).collect()
    }
}

/// 文件监听器
/// 用于监听文件变化并触发回调
pub struct FileWatcher {
    watcher: Option<RecommendedWatcher>,
    watched_paths: Arc<Mutex<HashMap<PathBuf, bool>>>,
    event_receiver: Option<Receiver<Result<Event, notify::Error>>>,
}

impl FileWatcher {
    /// 创建新的文件监听器
    pub fn new() -> Result<Self, WatcherError> {
        let (tx, rx) = channel();

        let watcher = RecommendedWatcher::new(
            move |res| {
                let _ = tx.send(res);
            },
            Config::default().with_poll_interval(Duration::from_secs(2)),
        )
        .map_err(|e| WatcherError::CreateError { message: e.to_string() })?;

        Ok(Self { watcher: Some(watcher), watched_paths: Arc::new(Mutex::new(HashMap::new())), event_receiver: Some(rx) })
    }

    /// 监听指定路径
    pub fn watch<P: AsRef<Path>>(&mut self, path: P, recursive: bool) -> Result<(), WatcherError> {
        let path = path.as_ref().to_path_buf();

        if let Some(ref mut watcher) = self.watcher {
            let mode = if recursive { RecursiveMode::Recursive } else { RecursiveMode::NonRecursive };

            watcher.watch(&path, mode).map_err(|e| WatcherError::WatchError { message: e.to_string() })?;

            self.watched_paths.lock().unwrap().insert(path, recursive);
        }

        Ok(())
    }

    /// 取消监听指定路径
    pub fn unwatch<P: AsRef<Path>>(&mut self, path: P) -> Result<(), WatcherError> {
        let path = path.as_ref().to_path_buf();

        if let Some(ref mut watcher) = self.watcher {
            watcher.unwatch(&path).map_err(|e| WatcherError::UnwatchError { message: e.to_string() })?;
            self.watched_paths.lock().unwrap().remove(&path);
        }

        Ok(())
    }

    /// 获取文件变化事件（阻塞）
    pub fn wait_for_event(&self, timeout: Duration) -> Result<FileChange, WatcherError> {
        if let Some(ref receiver) = self.event_receiver {
            let start = std::time::Instant::now();

            while start.elapsed() < timeout {
                if let Ok(result) = receiver.recv_timeout(Duration::from_millis(100)) {
                    match result {
                        Ok(event) => {
                            if let Some(change) = FileChange::from_event(&event).pop() {
                                return Ok(change);
                            }
                        }
                        Err(e) => return Err(WatcherError::ReceiveError { message: e.to_string() }),
                    }
                }
            }

            return Err(WatcherError::Timeout);
        }

        Err(WatcherError::NotRunning)
    }

    /// 获取所有待处理的事件
    pub fn try_recv_events(&self) -> Vec<FileChange> {
        let mut changes = Vec::new();

        if let Some(ref receiver) = self.event_receiver {
            while let Ok(result) = receiver.try_recv() {
                if let Ok(event) = result {
                    changes.extend(FileChange::from_event(&event));
                }
            }
        }

        changes
    }

    /// 获取所有监听的路径
    pub fn watched_paths(&self) -> Vec<PathBuf> {
        self.watched_paths.lock().unwrap().keys().cloned().collect()
    }
}

impl Default for FileWatcher {
    fn default() -> Self {
        Self::new().expect("Failed to create file watcher")
    }
}

/// 监听器错误
#[derive(Debug, Clone)]
pub enum WatcherError {
    /// 创建监听器失败
    CreateError { message: String },
    /// 监听路径失败
    WatchError { message: String },
    /// 取消监听失败
    UnwatchError { message: String },
    /// 接收事件失败
    ReceiveError { message: String },
    /// 超时
    Timeout,
    /// 监听器未运行
    NotRunning,
}

impl std::fmt::Display for WatcherError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WatcherError::CreateError { message } => write!(f, "Failed to create watcher: {}", message),
            WatcherError::WatchError { message } => write!(f, "Failed to watch path: {}", message),
            WatcherError::UnwatchError { message } => write!(f, "Failed to unwatch path: {}", message),
            WatcherError::ReceiveError { message } => write!(f, "Failed to receive event: {}", message),
            WatcherError::Timeout => write!(f, "Watch timeout"),
            WatcherError::NotRunning => write!(f, "Watcher not running"),
        }
    }
}

impl std::error::Error for WatcherError {}

/// 监听器结果类型
pub type WatcherResult<T> = Result<T, WatcherError>;

/// 文件过滤器
/// 用于过滤需要监听的文件类型
pub struct FileFilter {
    /// 包含的扩展名
    include_extensions: Vec<String>,
    /// 排除的文件名
    exclude_names: Vec<String>,
    /// 包含的文件名
    include_names: Vec<String>,
}

impl FileFilter {
    /// 创建新的文件过滤器
    pub fn new() -> Self {
        Self { include_extensions: Vec::new(), exclude_names: Vec::new(), include_names: Vec::new() }
    }

    /// 添加需要包含的扩展名
    pub fn with_extension(mut self, ext: &str) -> Self {
        self.include_extensions.push(ext.to_lowercase());
        self
    }

    /// 添加需要排除的文件名
    pub fn exclude_name(mut self, name: &str) -> Self {
        self.exclude_names.push(name.to_string());
        self
    }

    /// 添加需要包含的文件名
    pub fn include_name(mut self, name: &str) -> Self {
        self.include_names.push(name.to_string());
        self
    }

    /// 设置默认的 Markdown 和配置文件扩展名
    pub fn with_default_extensions(mut self) -> Self {
        self.include_extensions = vec!["md".to_string(), "html".to_string(), "css".to_string(), "js".to_string(), "ts".to_string(), "toml".to_string(), "json".to_string(), "yaml".to_string(), "yml".to_string()];
        self
    }

    /// 检查路径是否应该被监听
    pub fn should_watch(&self, path: &Path) -> bool {
        let file_name = path.file_name().and_then(|s| s.to_str()).unwrap_or("");
        let ext = path.extension().and_then(|s| s.to_str()).unwrap_or("").to_lowercase();

        if !self.include_names.is_empty() && !self.include_names.iter().any(|n| file_name.contains(n)) {
            return false;
        }

        if self.exclude_names.iter().any(|n| file_name == *n) {
            return false;
        }

        if self.include_extensions.is_empty() {
            return true;
        }

        self.include_extensions.contains(&ext)
    }
}

impl Default for FileFilter {
    fn default() -> Self {
        Self::new().with_default_extensions()
    }
}

/// 带过滤的文件监听器
pub struct FilteredFileWatcher {
    watcher: FileWatcher,
    filter: FileFilter,
}

impl FilteredFileWatcher {
    /// 创建新的带过滤的文件监听器
    pub fn new(filter: FileFilter) -> Result<Self, WatcherError> {
        Ok(Self { watcher: FileWatcher::new()?, filter })
    }

    /// 监听目录
    pub fn watch_dir<P: AsRef<Path>>(&mut self, dir: P) -> Result<(), WatcherError> {
        self.watcher.watch(dir, true)
    }

    /// 获取过滤后的文件变化事件
    pub fn wait_for_filtered_event(&self, timeout: Duration) -> Result<FileChange, WatcherError> {
        loop {
            let event = self.watcher.wait_for_event(timeout)?;

            if self.filter.should_watch(&event.path) {
                return Ok(event);
            }
        }
    }

    /// 获取所有过滤后的待处理事件
    pub fn try_recv_filtered_events(&self) -> Vec<FileChange> {
        self.watcher.try_recv_events().into_iter().filter(|c| self.filter.should_watch(&c.path)).collect()
    }
}
