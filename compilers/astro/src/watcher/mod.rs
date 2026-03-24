//! 文件监听器模块

use notify::{RecommendedWatcher, RecursiveMode, Watcher, Event, EventKind};
use std::sync::mpsc::channel;
use std::time::Duration;
use std::path::Path;

/// 文件监听器
pub struct FileWatcher {
    /// 监听器实例
    watcher: RecommendedWatcher,
}

impl FileWatcher {
    /// 创建新的文件监听器
    pub fn new<F>(callback: F) -> Result<Self, notify::Error>
    where
        F: Fn(&Path) + Send + 'static,
    {
        // 创建通道用于接收文件变更事件
        let (tx, rx) = channel();

        // 创建监听器
        let mut watcher: RecommendedWatcher = RecommendedWatcher::new(tx, Duration::from_secs(1))?;

        // 启动一个线程来处理事件
        std::thread::spawn(move || {
            for res in rx {
                match res {
                    Ok(event) => {
                        // 处理文件变更事件
                        if let Event { kind: EventKind::Modify(_), paths, .. } = event {
                            for path in paths {
                                callback(&path);
                            }
                        }
                        else if let Event { kind: EventKind::Create(_), paths, .. } = event {
                            for path in paths {
                                callback(&path);
                            }
                        }
                        else if let Event { kind: EventKind::Remove(_), paths, .. } = event {
                            for path in paths {
                                callback(&path);
                            }
                        }
                    }
                    Err(e) => println!("File watcher error: {:?}", e),
                }
            }
        });

        Ok(Self { watcher })
    }

    /// 开始监听目录
    pub fn watch(&mut self, path: &Path) -> Result<(), notify::Error> {
        self.watcher.watch(path, RecursiveMode::Recursive)
    }

    /// 停止监听
    pub fn unwatch(&mut self, path: &Path) -> Result<(), notify::Error> {
        self.watcher.unwatch(path)
    }
}
