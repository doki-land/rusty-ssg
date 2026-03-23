//! 文件监听器模块
//! 提供文件系统监听功能，用于开发模式和热重载

use notify::{RecommendedWatcher, RecursiveMode, Watcher, Event, EventKind, Config};
use std::sync::mpsc;

/// 文件变更事件
#[derive(Debug, Clone)]
pub enum FileChangeEvent {
    /// 文件创建
    Created(String),
    /// 文件修改
    Modified(String),
    /// 文件删除
    Deleted(String),
}

/// 文件监听器
pub struct FileWatcher {
    watcher: RecommendedWatcher,
    rx: mpsc::Receiver<notify::Result<Event>>,
}

impl FileWatcher {
    /// 创建新的文件监听器
    pub fn new() -> notify::Result<Self> {
        let (tx, rx) = mpsc::channel();
        
        let config = Config::default();
        
        let mut watcher = RecommendedWatcher::new(move |res| {
            tx.send(res).unwrap();
        }, config)?;
        
        Ok(Self {
            watcher,
            rx,
        })
    }
    
    /// 开始监听目录
    pub fn watch_directory(&mut self, path: &str) -> notify::Result<()> {
        self.watcher.watch(
            path.as_ref(),
            RecursiveMode::Recursive
        )
    }
    
    /// 监听文件变更并处理事件
    pub fn listen<F>(&self, mut handler: F) where F: FnMut(FileChangeEvent) {
        loop {
            match self.rx.recv() {
                Ok(Ok(event)) => {
                    if let EventKind::Modify(_) = event.kind {
                        for path in event.paths {
                            if let Some(path_str) = path.to_str() {
                                handler(FileChangeEvent::Modified(path_str.to_string()));
                            }
                        }
                    } else if let EventKind::Create(_) = event.kind {
                        for path in event.paths {
                            if let Some(path_str) = path.to_str() {
                                handler(FileChangeEvent::Created(path_str.to_string()));
                            }
                        }
                    } else if let EventKind::Remove(_) = event.kind {
                        for path in event.paths {
                            if let Some(path_str) = path.to_str() {
                                handler(FileChangeEvent::Deleted(path_str.to_string()));
                            }
                        }
                    }
                },
                Ok(Err(e)) => {
                    eprintln!("File watcher error: {:?}", e);
                },
                Err(e) => {
                    eprintln!("File watcher receive error: {:?}", e);
                    break;
                },
            }
        }
    }
}
