//! VuTeX AST 模块
//! 定义 VuTeX 文档的抽象语法树结构

use nargo_types::{NargoValue, Span};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// VuTeX 文档
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct VutexDocument {
    /// 文档节点
    pub nodes: Vec<DocumentNode>,
    /// 元数据
    pub metadata: HashMap<String, NargoValue>,
    /// 位置信息
    pub span: Span,
}

impl VutexDocument {
    /// 创建新的文档
    pub fn new() -> Self {
        Self::default()
    }

    /// 添加节点
    pub fn add_node(&mut self, node: DocumentNode) {
        self.nodes.push(node);
    }

    /// 设置元数据
    pub fn set_metadata(&mut self, key: String, value: NargoValue) {
        self.metadata.insert(key, value);
    }

    /// 获取元数据
    pub fn get_metadata(&self, key: &str) -> Option<&NargoValue> {
        self.metadata.get(key)
    }
}

/// 文档节点
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DocumentNode {
    /// 元素节点
    Element(ElementNode),
    /// 文本节点
    Text(TextNode),
    /// 注释节点
    Comment(CommentNode),
}

impl DocumentNode {
    /// 获取节点的位置信息
    pub fn span(&self) -> Span {
        match self {
            DocumentNode::Element(e) => e.span,
            DocumentNode::Text(t) => t.span,
            DocumentNode::Comment(c) => c.span,
        }
    }
}

/// 元素节点
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ElementNode {
    /// 标签名
    pub tag: String,
    /// 属性
    pub attributes: HashMap<String, String>,
    /// 子节点
    pub children: Vec<DocumentNode>,
    /// 是否自闭合
    pub self_closing: bool,
    /// 位置信息
    pub span: Span,
}

impl ElementNode {
    /// 创建新的元素节点
    pub fn new(tag: String, span: Span) -> Self {
        Self { tag, attributes: HashMap::new(), children: Vec::new(), self_closing: false, span }
    }

    /// 添加属性
    pub fn add_attribute(&mut self, name: String, value: String) {
        self.attributes.insert(name, value);
    }

    /// 添加子节点
    pub fn add_child(&mut self, child: DocumentNode) {
        self.children.push(child);
    }

    /// 获取属性
    pub fn get_attribute(&self, name: &str) -> Option<&String> {
        self.attributes.get(name)
    }
}

/// 文本节点
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TextNode {
    /// 文本内容
    pub content: String,
    /// 位置信息
    pub span: Span,
}

impl TextNode {
    /// 创建新的文本节点
    pub fn new(content: String, span: Span) -> Self {
        Self { content, span }
    }
}

/// 注释节点
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CommentNode {
    /// 注释内容
    pub content: String,
    /// 位置信息
    pub span: Span,
}

impl CommentNode {
    /// 创建新的注释节点
    pub fn new(content: String, span: Span) -> Self {
        Self { content, span }
    }
}
