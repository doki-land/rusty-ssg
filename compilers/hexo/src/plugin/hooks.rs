//! 插件钩子系统

use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;

use crate::types::Result;

/// 钩子类型
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum HookType {
    /// 初始化
    Init,
    /// 生成前
    BeforeGenerate,
    /// 生成后
    AfterGenerate,
    /// 部署前
    BeforeDeploy,
    /// 部署后
    AfterDeploy,
    /// 新文章
    NewPost,
    /// 渲染前
    BeforeRender,
    /// 渲染后
    AfterRender,
    /// 处理文章
    ProcessPost,
    /// 处理页面
    ProcessPage,
    /// 其他自定义钩子
    Custom(String),
}

/// 钩子回调
pub type HookCallback = Arc<dyn Fn(&mut HookContext) -> Result<()> + Send + Sync>;

/// 钩子上下文
#[derive(Debug)]
pub struct HookContext {
    /// 上下文数据
    pub data: HashMap<String, serde_json::Value>,
}

impl HookContext {
    /// 创建钩子上下文
    pub fn new() -> Self {
        Self { data: HashMap::new() }
    }

    /// 添加数据
    pub fn add_data(&mut self, key: &str, value: serde_json::Value) {
        self.data.insert(key.to_string(), value);
    }

    /// 获取数据
    pub fn get_data(&self, key: &str) -> Option<&serde_json::Value> {
        self.data.get(key)
    }
}

/// 钩子管理器
pub struct HookManager {
    /// 钩子映射
    hooks: Arc<RwLock<HashMap<HookType, Vec<HookCallback>>>>,
}

impl HookManager {
    /// 创建钩子管理器
    pub fn new() -> Self {
        Self { hooks: Arc::new(RwLock::new(HashMap::new())) }
    }

    /// 注册钩子
    pub async fn register_hook(&self, hook_type: HookType, callback: HookCallback) {
        let mut hooks = self.hooks.write().await;
        hooks.entry(hook_type).or_insert_with(Vec::new).push(callback);
    }

    /// 触发钩子
    pub async fn trigger_hook(&self, hook_type: HookType, context: &mut HookContext) -> Result<()> {
        let hooks = self.hooks.read().await;
        if let Some(callbacks) = hooks.get(&hook_type) {
            for callback in callbacks {
                callback(context)?;
            }
        }
        Ok(())
    }

    /// 移除钩子
    pub async fn remove_hook(&self, hook_type: HookType, callback: HookCallback) {
        let mut hooks = self.hooks.write().await;
        if let Some(callbacks) = hooks.get_mut(&hook_type) {
            callbacks.retain(|cb| !Arc::ptr_eq(cb, &callback));
        }
    }

    /// 获取钩子数量
    pub async fn get_hook_count(&self, hook_type: &HookType) -> usize {
        let hooks = self.hooks.read().await;
        hooks.get(hook_type).map_or(0, |callbacks| callbacks.len())
    }
}
