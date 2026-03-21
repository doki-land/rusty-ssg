//! Astro 语法抽象语法树

use std::collections::HashMap;

/// 抽象语法树节点
#[derive(Debug, Clone)]
pub enum AstNode {
    /// 文本节点
    Text(String),
    /// 变量插值节点
    Interpolation {
        /// 插值内容
        expression: String,
        /// 插值类型
        interpolation_type: InterpolationType,
    },
    /// 指令节点
    Directive {
        /// 指令类型
        directive_type: DirectiveType,
        /// 指令参数
        arguments: Vec<String>,
        /// 指令内容
        content: Option<Vec<AstNode>>,
    },
    /// 组件标签节点
    Component {
        /// 组件名称
        name: String,
        /// 组件属性
        attributes: HashMap<String, String>,
        /// 组件内容
        content: Option<Vec<AstNode>>,
        /// 是否是自闭合标签
        self_closing: bool,
    },
    /// 布局组件节点
    Layout {
        /// 布局名称
        name: String,
        /// 布局属性
        attributes: HashMap<String, String>,
        /// 布局内容
        content: Vec<AstNode>,
    },
    /// 插槽节点
    Slot {
        /// 插槽名称
        name: Option<String>,
        /// 插槽内容
        content: Vec<AstNode>,
    },
}

/// 插值类型
#[derive(Debug, Clone, PartialEq)]
pub enum InterpolationType {
    /// 转义插值: {{}}
    Escaped,
    /// 非转义插值: {{{}}}
    Unescaped,
}

/// 指令类型
#[derive(Debug, Clone, PartialEq)]
pub enum DirectiveType {
    /// 循环指令: {% for %}
    For,
    /// 条件指令: {% if %}
    If,
    /// 否则指令: {% else %}
    Else,
    /// 否则如果指令: {% elif %}
    Elif,
    /// 结束指令: {% endif %}, {% endfor %}
    End,
    /// 组件导入指令: {% import %}
    Import,
    /// 布局指令: {% layout %}
    Layout,
    /// 其他指令
    Other(String),
}

impl AstNode {
    /// 创建文本节点
    pub fn text(content: &str) -> Self {
        AstNode::Text(content.to_string())
    }

    /// 创建变量插值节点
    pub fn interpolation(expression: &str, interpolation_type: InterpolationType) -> Self {
        AstNode::Interpolation { expression: expression.to_string(), interpolation_type }
    }

    /// 创建指令节点
    pub fn directive(directive_type: DirectiveType, arguments: Vec<String>, content: Option<Vec<AstNode>>) -> Self {
        AstNode::Directive { directive_type, arguments, content }
    }

    /// 创建组件标签节点
    pub fn component(
        name: &str,
        attributes: HashMap<String, String>,
        content: Option<Vec<AstNode>>,
        self_closing: bool,
    ) -> Self {
        AstNode::Component { name: name.to_string(), attributes, content, self_closing }
    }

    /// 创建布局组件节点
    pub fn layout(name: &str, attributes: HashMap<String, String>, content: Vec<AstNode>) -> Self {
        AstNode::Layout { name: name.to_string(), attributes, content }
    }

    /// 创建插槽节点
    pub fn slot(name: Option<&str>, content: Vec<AstNode>) -> Self {
        AstNode::Slot { name: name.map(|s| s.to_string()), content }
    }
}
