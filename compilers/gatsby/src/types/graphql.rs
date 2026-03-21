//! GraphQL 数据层模块
//! 提供 Gatsby 兼容的 GraphQL 数据层实现

use std::collections::{HashMap, HashSet};

use async_graphql_value::ConstValue;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;

/// GraphQL 错误类型
#[derive(Error, Debug, Clone, Serialize, Deserialize)]
pub enum GraphQLError {
    /// 节点未找到错误
    #[error("Node not found: {0}")]
    NodeNotFound(String),

    /// 类型未找到错误
    #[error("Type not found: {0}")]
    TypeNotFound(String),

    /// 字段未找到错误
    #[error("Field not found: {0}")]
    FieldNotFound(String),

    /// 查询解析错误
    #[error("Query parse error: {0}")]
    QueryParseError(String),

    /// 执行错误
    #[error("Execution error: {0}")]
    ExecutionError(String),

    /// 验证错误
    #[error("Validation error: {0}")]
    ValidationError(String),

    /// 重复节点错误
    #[error("Duplicate node with ID: {0}")]
    DuplicateNode(String),

    /// 类型冲突错误
    #[error("Type conflict for node {0}: expected {1}, got {2}")]
    TypeConflict(String, String, String),
}

/// GraphQL 结果类型
pub type GraphQLResult<T> = std::result::Result<T, GraphQLError>;

/// 节点 ID 类型
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct NodeId(String);

impl NodeId {
    /// 创建新的节点 ID
    pub fn new(id: String) -> Self {
        NodeId(id)
    }

    /// 生成随机节点 ID
    pub fn generate() -> Self {
        NodeId(Uuid::new_v4().to_string())
    }

    /// 获取节点 ID 的字符串表示
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<String> for NodeId {
    fn from(id: String) -> Self {
        NodeId::new(id)
    }
}

impl From<&str> for NodeId {
    fn from(id: &str) -> Self {
        NodeId::new(id.to_string())
    }
}

impl std::fmt::Display for NodeId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// 节点类型名称
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct NodeType(String);

impl NodeType {
    /// 创建新的节点类型
    pub fn new(name: String) -> Self {
        NodeType(name)
    }

    /// 获取节点类型的字符串表示
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<String> for NodeType {
    fn from(name: String) -> Self {
        NodeType::new(name)
    }
}

impl From<&str> for NodeType {
    fn from(name: &str) -> Self {
        NodeType::new(name.to_string())
    }
}

impl std::fmt::Display for NodeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Gatsby 节点
/// 代表 GraphQL 数据层中的一个数据节点
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Node {
    /// 节点唯一标识符
    pub id: NodeId,

    /// 节点类型
    pub internal: Internal,

    /// 节点字段数据
    #[serde(flatten)]
    pub fields: HashMap<String, ConstValue>,

    /// 子节点 ID 列表
    pub children: Vec<NodeId>,

    /// 父节点 ID
    pub parent: Option<NodeId>,
}

/// 节点内部元数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Internal {
    /// 节点类型名称
    pub type_name: NodeType,

    /// 节点内容摘要
    pub content_digest: String,

    /// 节点内容（可选）
    pub content: Option<String>,

    /// 节点描述（可选）
    pub description: Option<String>,

    /// 节点所有者（可选）
    pub owner: Option<String>,
}

impl Node {
    /// 创建新的节点
    ///
    /// # Arguments
    ///
    /// * `id` - 节点 ID
    /// * `type_name` - 节点类型名称
    /// * `content_digest` - 内容摘要
    pub fn new(id: NodeId, type_name: NodeType, content_digest: String) -> Self {
        Node {
            id,
            internal: Internal {
                type_name,
                content_digest,
                content: None,
                description: None,
                owner: None,
            },
            fields: HashMap::new(),
            children: Vec::new(),
            parent: None,
        }
    }

    /// 设置节点字段
    ///
    /// # Arguments
    ///
    /// * `key` - 字段键
    /// * `value` - 字段值
    pub fn set_field(&mut self, key: String, value: ConstValue) {
        self.fields.insert(key, value);
    }

    /// 获取节点字段
    ///
    /// # Arguments
    ///
    /// * `key` - 字段键
    pub fn get_field(&self, key: &str) -> Option<&ConstValue> {
        self.fields.get(key)
    }

    /// 添加子节点
    ///
    /// # Arguments
    ///
    /// * `child_id` - 子节点 ID
    pub fn add_child(&mut self, child_id: NodeId) {
        if !self.children.contains(&child_id) {
            self.children.push(child_id);
        }
    }

    /// 设置父节点
    ///
    /// # Arguments
    ///
    /// * `parent_id` - 父节点 ID
    pub fn set_parent(&mut self, parent_id: NodeId) {
        self.parent = Some(parent_id);
    }

    /// 设置节点内容
    ///
    /// # Arguments
    ///
    /// * `content` - 节点内容
    pub fn with_content(mut self, content: String) -> Self {
        self.internal.content = Some(content);
        self
    }

    /// 设置节点描述
    ///
    /// # Arguments
    ///
    /// * `description` - 节点描述
    pub fn with_description(mut self, description: String) -> Self {
        self.internal.description = Some(description);
        self
    }

    /// 设置节点所有者
    ///
    /// # Arguments
    ///
    /// * `owner` - 节点所有者
    pub fn with_owner(mut self, owner: String) -> Self {
        self.internal.owner = Some(owner);
        self
    }
}

/// 节点操作上下文
#[derive(Debug, Clone, Default)]
pub struct NodeActions {
    /// 创建的节点
    created_nodes: Vec<Node>,

    /// 删除的节点 ID
    deleted_nodes: Vec<NodeId>,

    /// 创建的节点类型
    created_types: Vec<NodeType>,
}

impl NodeActions {
    /// 创建新的节点操作上下文
    pub fn new() -> Self {
        Self::default()
    }

    /// 创建节点
    ///
    /// # Arguments
    ///
    /// * `node` - 要创建的节点
    pub fn create_node(&mut self, node: Node) {
        self.created_nodes.push(node);
    }

    /// 删除节点
    ///
    /// # Arguments
    ///
    /// * `node_id` - 要删除的节点 ID
    pub fn delete_node(&mut self, node_id: NodeId) {
        self.deleted_nodes.push(node_id);
    }

    /// 创建节点类型
    ///
    /// # Arguments
    ///
    /// * `type_name` - 要创建的节点类型
    pub fn create_node_type(&mut self, type_name: NodeType) {
        self.created_types.push(type_name);
    }

    /// 获取创建的节点
    pub fn created_nodes(&self) -> &[Node] {
        &self.created_nodes
    }

    /// 获取删除的节点 ID
    pub fn deleted_nodes(&self) -> &[NodeId] {
        &self.deleted_nodes
    }

    /// 获取创建的节点类型
    pub fn created_types(&self) -> &[NodeType] {
        &self.created_types
    }
}

/// GraphQL 字段类型
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GraphQLFieldType {
    /// 字符串类型
    String,

    /// 整数类型
    Int,

    /// 浮点数类型
    Float,

    /// 布尔类型
    Boolean,

    /// ID 类型
    ID,

    /// 自定义类型
    Custom(String),

    /// 列表类型
    List(Box<GraphQLFieldType>),

    /// 非空类型
    NonNull(Box<GraphQLFieldType>),

    /// JSON 类型
    JSON,

    /// 日期类型
    Date,

    /// 时间类型
    DateTime,
}

/// GraphQL 字段定义
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphQLField {
    /// 字段名称
    pub name: String,

    /// 字段类型
    pub field_type: GraphQLFieldType,

    /// 字段描述
    pub description: Option<String>,

    /// 字段参数
    pub arguments: Vec<GraphQLArgument>,

    /// 是否弃用
    pub is_deprecated: bool,

    /// 弃用原因
    pub deprecation_reason: Option<String>,
}

impl GraphQLField {
    /// 创建新的字段定义
    ///
    /// # Arguments
    ///
    /// * `name` - 字段名称
    /// * `field_type` - 字段类型
    pub fn new(name: String, field_type: GraphQLFieldType) -> Self {
        GraphQLField {
            name,
            field_type,
            description: None,
            arguments: Vec::new(),
            is_deprecated: false,
            deprecation_reason: None,
        }
    }

    /// 设置字段描述
    ///
    /// # Arguments
    ///
    /// * `description` - 字段描述
    pub fn with_description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    /// 添加字段参数
    ///
    /// # Arguments
    ///
    /// * `argument` - 字段参数
    pub fn with_argument(mut self, argument: GraphQLArgument) -> Self {
        self.arguments.push(argument);
        self
    }

    /// 标记为已弃用
    ///
    /// # Arguments
    ///
    /// * `reason` - 弃用原因
    pub fn deprecated(mut self, reason: String) -> Self {
        self.is_deprecated = true;
        self.deprecation_reason = Some(reason);
        self
    }
}

/// GraphQL 参数定义
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphQLArgument {
    /// 参数名称
    pub name: String,

    /// 参数类型
    pub argument_type: GraphQLFieldType,

    /// 参数描述
    pub description: Option<String>,

    /// 默认值
    pub default_value: Option<ConstValue>,
}

impl GraphQLArgument {
    /// 创建新的参数定义
    ///
    /// # Arguments
    ///
    /// * `name` - 参数名称
    /// * `argument_type` - 参数类型
    pub fn new(name: String, argument_type: GraphQLFieldType) -> Self {
        GraphQLArgument {
            name,
            argument_type,
            description: None,
            default_value: None,
        }
    }

    /// 设置参数描述
    ///
    /// # Arguments
    ///
    /// * `description` - 参数描述
    pub fn with_description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    /// 设置默认值
    ///
    /// # Arguments
    ///
    /// * `value` - 默认值
    pub fn with_default(mut self, value: ConstValue) -> Self {
        self.default_value = Some(value);
        self
    }
}

/// GraphQL 对象类型定义
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphQLObjectType {
    /// 类型名称
    pub name: String,

    /// 类型描述
    pub description: Option<String>,

    /// 类型字段
    pub fields: Vec<GraphQLField>,

    /// 实现的接口
    pub interfaces: Vec<String>,

    /// 是否为接口
    pub is_interface: bool,

    /// 是否为联合类型
    pub is_union: bool,

    /// 联合类型的成员
    pub union_types: Vec<String>,
}

impl GraphQLObjectType {
    /// 创建新的对象类型
    ///
    /// # Arguments
    ///
    /// * `name` - 类型名称
    pub fn new(name: String) -> Self {
        GraphQLObjectType {
            name,
            description: None,
            fields: Vec::new(),
            interfaces: Vec::new(),
            is_interface: false,
            is_union: false,
            union_types: Vec::new(),
        }
    }

    /// 设置类型描述
    ///
    /// # Arguments
    ///
    /// * `description` - 类型描述
    pub fn with_description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    /// 添加字段
    ///
    /// # Arguments
    ///
    /// * `field` - 字段定义
    pub fn with_field(mut self, field: GraphQLField) -> Self {
        self.fields.push(field);
        self
    }

    /// 实现接口
    ///
    /// # Arguments
    ///
    /// * `interface` - 接口名称
    pub fn implements(mut self, interface: String) -> Self {
        self.interfaces.push(interface);
        self
    }

    /// 标记为接口类型
    pub fn as_interface(mut self) -> Self {
        self.is_interface = true;
        self
    }

    /// 标记为联合类型
    ///
    /// # Arguments
    ///
    /// * `types` - 联合类型成员
    pub fn as_union(mut self, types: Vec<String>) -> Self {
        self.is_union = true;
        self.union_types = types;
        self
    }
}

/// GraphQL Schema 定义
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphQLSchema {
    /// 查询类型
    pub query_type: String,

    /// 变更类型（可选）
    pub mutation_type: Option<String>,

    /// 订阅类型（可选）
    pub subscription_type: Option<String>,

    /// 所有类型定义
    pub types: Vec<GraphQLObjectType>,

    /// 指令定义
    pub directives: Vec<String>,
}

impl GraphQLSchema {
    /// 创建新的 Schema
    ///
    /// # Arguments
    ///
    /// * `query_type` - 查询类型名称
    pub fn new(query_type: String) -> Self {
        GraphQLSchema {
            query_type,
            mutation_type: None,
            subscription_type: None,
            types: Vec::new(),
            directives: Vec::new(),
        }
    }

    /// 设置变更类型
    ///
    /// # Arguments
    ///
    /// * `mutation_type` - 变更类型名称
    pub fn with_mutation(mut self, mutation_type: String) -> Self {
        self.mutation_type = Some(mutation_type);
        self
    }

    /// 设置订阅类型
    ///
    /// # Arguments
    ///
    /// * `subscription_type` - 订阅类型名称
    pub fn with_subscription(mut self, subscription_type: String) -> Self {
        self.subscription_type = Some(subscription_type);
        self
    }

    /// 添加类型定义
    ///
    /// # Arguments
    ///
    /// * `type_def` - 类型定义
    pub fn with_type(mut self, type_def: GraphQLObjectType) -> Self {
        self.types.push(type_def);
        self
    }
}

/// 查询选择集
#[derive(Debug, Clone)]
pub struct SelectionSet {
    /// 选择的字段
    pub selections: Vec<Selection>,
}

/// 查询选择
#[derive(Debug, Clone)]
pub enum Selection {
    /// 字段选择
    Field(FieldSelection),

    /// 片段展开
    FragmentSpread(String),

    /// 内联片段
    InlineFragment(Option<String>, SelectionSet),
}

/// 字段选择
#[derive(Debug, Clone)]
pub struct FieldSelection {
    /// 字段别名
    pub alias: Option<String>,

    /// 字段名称
    pub name: String,

    /// 字段参数
    pub arguments: HashMap<String, ConstValue>,

    /// 指令
    pub directives: Vec<Directive>,

    /// 子选择集
    pub selection_set: Option<SelectionSet>,
}

/// 查询指令
#[derive(Debug, Clone)]
pub struct Directive {
    /// 指令名称
    pub name: String,

    /// 指令参数
    pub arguments: HashMap<String, ConstValue>,
}

/// GraphQL 查询请求
#[derive(Debug, Clone)]
pub struct GraphQLRequest {
    /// 查询字符串
    pub query: String,

    /// 操作名称（可选）
    pub operation_name: Option<String>,

    /// 变量（可选）
    pub variables: Option<HashMap<String, ConstValue>>,
}

/// GraphQL 响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphQLResponse {
    /// 响应数据
    pub data: Option<ConstValue>,

    /// 错误列表
    pub errors: Option<Vec<GraphQLError>>,

    /// 扩展信息
    pub extensions: Option<HashMap<String, ConstValue>>,
}

impl GraphQLResponse {
    /// 创建成功响应
    ///
    /// # Arguments
    ///
    /// * `data` - 响应数据
    pub fn success(data: ConstValue) -> Self {
        GraphQLResponse {
            data: Some(data),
            errors: None,
            extensions: None,
        }
    }

    /// 创建错误响应
    ///
    /// # Arguments
    ///
    /// * `errors` - 错误列表
    pub fn error(errors: Vec<GraphQLError>) -> Self {
        GraphQLResponse {
            data: None,
            errors: Some(errors),
            extensions: None,
        }
    }

    /// 添加扩展信息
    ///
    /// # Arguments
    ///
    /// * `key` - 扩展键
    /// * `value` - 扩展值
    pub fn with_extension(mut self, key: String, value: ConstValue) -> Self {
        let extensions = self.extensions.get_or_insert_with(HashMap::new);
        extensions.insert(key, value);
        self
    }
}

/// 节点存储
/// 存储和管理所有节点
#[derive(Debug, Clone, Default)]
pub struct NodeStore {
    /// 节点映射（ID -> 节点）
    nodes: HashMap<NodeId, Node>,

    /// 类型索引（类型名称 -> 节点 ID 集合）
    type_index: HashMap<NodeType, HashSet<NodeId>>,

    /// 父节点索引（父节点 ID -> 子节点 ID 集合）
    parent_index: HashMap<NodeId, HashSet<NodeId>>,
}

impl NodeStore {
    /// 创建新的节点存储
    pub fn new() -> Self {
        Self::default()
    }

    /// 添加节点
    ///
    /// # Arguments
    ///
    /// * `node` - 要添加的节点
    ///
    /// # Errors
    ///
    /// 如果节点 ID 已存在，返回 `GraphQLError::DuplicateNode`
    pub fn add_node(&mut self, node: Node) -> GraphQLResult<()> {
        if self.nodes.contains_key(&node.id) {
            return Err(GraphQLError::DuplicateNode(node.id.to_string()));
        }

        let node_id = node.id.clone();
        let type_name = node.internal.type_name.clone();

        self.nodes.insert(node_id.clone(), node);

        self.type_index
            .entry(type_name)
            .or_insert_with(HashSet::new)
            .insert(node_id.clone());

        if let Some(parent_id) = &self.nodes[&node_id].parent {
            self.parent_index
                .entry(parent_id.clone())
                .or_insert_with(HashSet::new)
                .insert(node_id);
        }

        Ok(())
    }

    /// 获取节点
    ///
    /// # Arguments
    ///
    /// * `id` - 节点 ID
    pub fn get_node(&self, id: &NodeId) -> Option<&Node> {
        self.nodes.get(id)
    }

    /// 获取可变节点
    ///
    /// # Arguments
    ///
    /// * `id` - 节点 ID
    pub fn get_node_mut(&mut self, id: &NodeId) -> Option<&mut Node> {
        self.nodes.get_mut(id)
    }

    /// 根据类型获取节点
    ///
    /// # Arguments
    ///
    /// * `type_name` - 节点类型名称
    pub fn get_nodes_by_type(&self, type_name: &NodeType) -> Vec<&Node> {
        self.type_index
            .get(type_name)
            .map(|ids| ids.iter().filter_map(|id| self.nodes.get(id)).collect())
            .unwrap_or_default()
    }

    /// 获取所有类型
    pub fn get_all_types(&self) -> Vec<&NodeType> {
        self.type_index.keys().collect()
    }

    /// 获取所有节点
    pub fn get_all_nodes(&self) -> Vec<&Node> {
        self.nodes.values().collect()
    }

    /// 删除节点
    ///
    /// # Arguments
    ///
    /// * `id` - 节点 ID
    pub fn remove_node(&mut self, id: &NodeId) -> Option<Node> {
        let node = self.nodes.remove(id)?;

        if let Some(ids) = self.type_index.get_mut(&node.internal.type_name) {
            ids.remove(id);
        }

        if let Some(parent_id) = &node.parent {
            if let Some(children) = self.parent_index.get_mut(parent_id) {
                children.remove(id);
            }
        }

        if let Some(children) = self.parent_index.remove(id) {
            for child_id in children {
                if let Some(child) = self.nodes.get_mut(&child_id) {
                    child.parent = None;
                }
            }
        }

        Some(node)
    }

    /// 检查节点是否存在
    ///
    /// # Arguments
    ///
    /// * `id` - 节点 ID
    pub fn has_node(&self, id: &NodeId) -> bool {
        self.nodes.contains_key(id)
    }

    /// 获取节点数量
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    /// 清空存储
    pub fn clear(&mut self) {
        self.nodes.clear();
        self.type_index.clear();
        self.parent_index.clear();
    }
}
