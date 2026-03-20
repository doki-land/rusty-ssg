
//! 默认值配置模块
//!
//! 该模块提供 Jekyll defaults 配置的解析和应用功能，支持为不同类型和路径的文档设置默认值。
//! 支持的筛选条件包括：路径模式、集合类型、布局类型等。

use serde_json::Value;
use std::collections::HashMap;
use std::path::Path;

use super::{FrontMatter, JekyllConfig};

/// 默认值配置项
///
/// 表示 Jekyll 配置中的一个 defaults 条目，包含作用域筛选和默认值
#[derive(Debug, Clone, PartialEq)]
pub struct DefaultConfig {
    /// 作用域配置，用于筛选要应用默认值的文档
    pub scope: Scope,
    /// 要应用的默认值
    pub values: HashMap&lt;String, Value&gt;,
}

/// 作用域配置
///
/// 定义用于筛选文档的条件，可以组合多个条件
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Scope {
    /// 路径模式，用于匹配文档路径
    pub path: Option&lt;String&gt;,
    /// 集合类型，用于匹配特定集合的文档
    pub collection: Option&lt;String&gt;,
    /// 布局类型，用于匹配特定布局的文档
    pub layout: Option&lt;String&gt;,
}

/// 默认值管理器
///
/// 负责解析和应用 Jekyll 配置中的 defaults 配置
pub struct DefaultsManager {
    /// 所有默认值配置项列表
    defaults: Vec&lt;DefaultConfig&gt;,
}

impl DefaultConfig {
    /// 创建新的默认值配置项
    ///
    /// # Arguments
    ///
    /// * `scope` - 作用域配置
    /// * `values` - 要应用的默认值
    pub fn new(scope: Scope, values: HashMap&lt;String, Value&gt;) -&gt; Self {
        Self { scope, values }
    }

    /// 从 HashMap 创建默认值配置项
    ///
    /// # Arguments
    ///
    /// * `map` - 包含 scope 和 values 的 HashMap
    ///
    /// # Errors
    ///
    /// 返回 `DefaultsError` 如果解析失败
    pub fn from_map(map: &amp;HashMap&lt;String, Value&gt;) -&gt; Result&lt;Self, DefaultsError&gt; {
        let scope = map
            .get("scope")
            .and_then(|v| v.as_object())
            .map(Scope::from_map)
            .unwrap_or_else(|| Ok(Scope::default()))?;

        let values = map
            .get("values")
            .and_then(|v| v.as_object())
            .map(|obj| {
                obj.iter()
                    .map(|(k, v)| (k.clone(), v.clone()))
                    .collect()
            })
            .unwrap_or_default();

        Ok(Self { scope, values })
    }
}

impl Scope {
    /// 创建新的作用域配置
    pub fn new() -&gt; Self {
        Self::default()
    }

    /// 设置路径模式
    pub fn with_path(mut self, path: String) -&gt; Self {
        self.path = Some(path);
        self
    }

    /// 设置集合类型
    pub fn with_collection(mut self, collection: String) -&gt; Self {
        self.collection = Some(collection);
        self
    }

    /// 设置布局类型
    pub fn with_layout(mut self, layout: String) -&gt; Self {
        self.layout = Some(layout);
        self
    }

    /// 从 HashMap 创建作用域配置
    ///
    /// # Arguments
    ///
    /// * `map` - 包含作用域配置的 HashMap
    ///
    /// # Errors
    ///
    /// 返回 `DefaultsError` 如果解析失败
    pub fn from_map(map: &amp;serde_json::Map&lt;String, Value&gt;) -&gt; Result&lt;Self, DefaultsError&gt; {
        let path = map
            .get("path")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());

        let collection = map
            .get("collection")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());

        let layout = map
            .get("layout")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());

        Ok(Self {
            path,
            collection,
            layout,
        })
    }

    /// 检查文档是否匹配该作用域
    ///
    /// # Arguments
    ///
    /// * `document_path` - 文档的相对路径
    /// * `collection` - 文档所属的集合（如果有）
    /// * `current_layout` - 文档当前使用的布局（如果有）
    ///
    /// # Returns
    ///
    /// 如果文档匹配该作用域返回 true
    pub fn matches(
        &amp;self,
        document_path: &amp;Path,
        collection: Option&lt;&amp;str&gt;,
        current_layout: Option&lt;&amp;str&gt;,
    ) -&gt; bool {
        let mut matches = true;

        if let Some(ref pattern) = self.path {
            matches = matches &amp;&amp; Self::match_path(document_path, pattern);
        }

        if let Some(ref target_collection) = self.collection {
            matches = matches &amp;&amp; collection.map(|c| c == target_collection).unwrap_or(false);
        }

        if let Some(ref target_layout) = self.layout {
            matches = matches &amp;&amp; current_layout.map(|l| l == target_layout).unwrap_or(false);
        }

        matches
    }

    /// 检查路径是否匹配给定的模式
    ///
    /// 支持的模式语法：
    /// - `*` 匹配任意字符（除路径分隔符）
    /// - `**` 匹配任意字符（包括路径分隔符）
    ///
    /// # Arguments
    ///
    /// * `path` - 要检查的路径
    /// * `pattern` - 匹配模式
    ///
    /// # Returns
    ///
    /// 如果路径匹配模式返回 true
    fn match_path(path: &amp;Path, pattern: &amp;str) -&gt; bool {
        let path_str = path.to_string_lossy();
        let path_str = path_str.replace(std::path::MAIN_SEPARATOR, "/");
        let pattern = pattern.replace(std::path::MAIN_SEPARATOR, "/");

        let regex_pattern = Self::glob_to_regex(&amp;pattern);
        let regex = match regex::Regex::new(&amp;regex_pattern) {
            Ok(re) =&gt; re,
            Err(_) =&gt; return false,
        };

        regex.is_match(&amp;path_str)
    }

    /// 将 glob 模式转换为正则表达式
    fn glob_to_regex(pattern: &amp;str) -&gt; String {
        let mut result = String::from("^");
        let mut chars = pattern.chars().peekable();

        while let Some(c) = chars.next() {
            match c {
                '*' =&gt; {
                    if let Some('*') = chars.peek() {
                        chars.next();
                        result.push_str(".*");
                    } else {
                        result.push_str("[^/]*");
                    }
                }
                '.' =&gt; result.push_str("\\."),
                '?' =&gt; result.push('.'),
                '\\' =&gt; {
                    if let Some(next) = chars.next() {
                        result.push('\\');
                        result.push(next);
                    }
                }
                _ =&gt; result.push(c),
            }
        }

        result.push('$');
        result
    }
}

impl DefaultsManager {
    /// 创建新的默认值管理器
    pub fn new() -&gt; Self {
        Self {
            defaults: Vec::new(),
        }
    }

    /// 从 Jekyll 配置创建默认值管理器
    ///
    /// # Arguments
    ///
    /// * `config` - Jekyll 配置
    ///
    /// # Errors
    ///
    /// 返回 `DefaultsError` 如果解析配置中的 defaults 失败
    pub fn from_config(config: &amp;JekyllConfig) -&gt; Result&lt;Self, DefaultsError&gt; {
        let mut manager = Self::new();

        if let Some(defaults) = &amp;config.defaults {
            for default_map in defaults {
                let default_config = DefaultConfig::from_map(default_map)?;
                manager.add_default(default_config);
            }
        }

        Ok(manager)
    }

    /// 添加一个默认值配置项
    ///
    /// # Arguments
    ///
    /// * `default` - 要添加的默认值配置项
    pub fn add_default(&amp;mut self, default: DefaultConfig) {
        self.defaults.push(default);
    }

    /// 为文档应用所有匹配的默认值
    ///
    /// 默认值按配置顺序应用，后面的配置会覆盖前面的配置
    ///
    /// # Arguments
    ///
    /// * `document_path` - 文档的相对路径
    /// * `collection` - 文档所属的集合（如果有）
    /// * `front_matter` - 要应用默认值的 Front Matter（会被修改）
    pub fn apply_defaults(
        &amp;self,
        document_path: &amp;Path,
        collection: Option&lt;&amp;str&gt;,
        front_matter: &amp;mut FrontMatter,
    ) {
        let current_layout = front_matter.get_str("layout");

        for default_config in &amp;self.defaults {
            if default_config.scope.matches(document_path, collection, current_layout) {
                Self::merge_values(&amp;mut front_matter.variables, &amp;default_config.values);
            }
        }
    }

    /// 合并两个值映射
    ///
    /// 源映射中的值会合并到目标映射中，源映射中的值不会覆盖目标映射中已存在的值
    ///
    /// # Arguments
    ///
    /// * `target` - 目标映射（会被修改）
    /// * `source` - 源映射
    fn merge_values(target: &amp;mut HashMap&lt;String, Value&gt;, source: &amp;HashMap&lt;String, Value&gt;) {
        for (key, value) in source {
            if !target.contains_key(key) {
                target.insert(key.clone(), value.clone());
            }
        }
    }

    /// 获取所有默认值配置项
    pub fn defaults(&amp;self) -&gt; &amp;[DefaultConfig] {
        &amp;self.defaults
    }
}

impl Default for DefaultsManager {
    fn default() -&gt; Self {
        Self::new()
    }
}

/// 默认值配置相关的错误类型
#[derive(Debug)]
pub enum DefaultsError {
    /// 无效的默认值配置格式
    InvalidFormat(String),
    /// 路径匹配错误
    PathMatchError(String),
}

impl std::fmt::Display for DefaultsError {
    fn fmt(&amp;self, f: &amp;mut std::fmt::Formatter&lt;'_&gt;) -&gt; std::fmt::Result {
        match self {
            DefaultsError::InvalidFormat(msg) =&gt; write!(f, "Invalid defaults format: {}", msg),
            DefaultsError::PathMatchError(msg) =&gt; write!(f, "Path match error: {}", msg),
        }
    }
}

impl std::error::Error for DefaultsError {}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_scope_from_map() {
        let map = serde_json::Map::from_iter(vec![
            ("path".to_string(), json!("posts/")),
            ("collection".to_string(), json!("posts")),
            ("layout".to_string(), json!("post")),
        ]);

        let scope = Scope::from_map(&amp;map).unwrap();
        assert_eq!(scope.path, Some("posts/".to_string()));
        assert_eq!(scope.collection, Some("posts".to_string()));
        assert_eq!(scope.layout, Some("post".to_string()));
    }

    #[test]
    fn test_scope_matches_path() {
        let scope = Scope::new().with_path("posts/".to_string());
        let path = Path::new("posts/2024-01-01-test.md");
        assert!(scope.matches(path, None, None));
    }

    #[test]
    fn test_scope_matches_collection() {
        let scope = Scope::new().with_collection("posts".to_string());
        let path = Path::new("any/path.md");
        assert!(scope.matches(path, Some("posts"), None));
        assert!(!scope.matches(path, Some("drafts"), None));
    }

    #[test]
    fn test_apply_defaults() {
        let mut manager = DefaultsManager::new();

        let values = HashMap::from_iter(vec![
            ("layout".to_string(), json!("default")),
            ("author".to_string(), json!("Test Author")),
        ]);

        manager.add_default(DefaultConfig::new(Scope::new(), values));

        let mut front_matter = FrontMatter::new(
            String::new(),
            HashMap::new(),
            String::new(),
        );

        manager.apply_defaults(Path::new("test.md"), None, &amp;mut front_matter);

        assert_eq!(front_matter.get_str("layout"), Some("default"));
        assert_eq!(front_matter.get_str("author"), Some("Test Author"));
    }

    #[test]
    fn test_apply_defaults_no_override() {
        let mut manager = DefaultsManager::new();

        let values = HashMap::from_iter(vec![
            ("layout".to_string(), json!("default")),
        ]);

        manager.add_default(DefaultConfig::new(Scope::new(), values));

        let mut front_matter = FrontMatter::new(
            String::new(),
            HashMap::from_iter(vec![
                ("layout".to_string(), json!("custom")),
            ]),
            String::new(),
        );

        manager.apply_defaults(Path::new("test.md"), None, &amp;mut front_matter);

        assert_eq!(front_matter.get_str("layout"), Some("