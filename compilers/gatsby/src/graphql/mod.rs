//! GraphQL 执行引擎模块
//! 提供 GraphQL 查询解析、Schema 构建和查询执行功能

use std::collections::{HashMap, HashSet};

use async_graphql_value::ConstValue;
use crate::types::graphql::*;

/// GraphQL 执行引擎
pub struct GraphQLExecutor {
    /// Schema 定义
    schema: GraphQLSchema,

    /// 节点存储
    node_store: NodeStore,

    /// 自定义解析器
    resolvers: HashMap<String, Box<dyn Fn(&FieldSelection, &NodeStore) -> GraphQLResult<ConstValue> + Send + Sync>>,
}

impl GraphQLExecutor {
    /// 创建新的 GraphQL 执行引擎
    ///
    /// # Arguments
    ///
    /// * `schema` - GraphQL Schema 定义
    pub fn new(schema: GraphQLSchema) -> Self {
        GraphQLExecutor {
            schema,
            node_store: NodeStore::new(),
            resolvers: HashMap::new(),
        }
    }

    /// 获取节点存储的可变引用
    pub fn node_store_mut(&mut self) -> &mut NodeStore {
        &mut self.node_store
    }

    /// 获取节点存储的引用
    pub fn node_store(&self) -> &NodeStore {
        &self.node_store
    }

    /// 设置节点存储
    ///
    /// # Arguments
    ///
    /// * `node_store` - 节点存储
    pub fn with_node_store(mut self, node_store: NodeStore) -> Self {
        self.node_store = node_store;
        self
    }

    /// 注册自定义解析器
    ///
    /// # Arguments
    ///
    /// * `field_path` - 字段路径，格式为 "Type.field"
    /// * `resolver` - 解析器函数
    pub fn register_resolver<F>(&mut self, field_path: String, resolver: F)
    where
        F: Fn(&FieldSelection, &NodeStore) -> GraphQLResult<ConstValue> + Send + Sync + 'static,
    {
        self.resolvers.insert(field_path, Box::new(resolver));
    }

    /// 执行 GraphQL 查询
    ///
    /// # Arguments
    ///
    /// * `request` - GraphQL 查询请求
    ///
    /// # Returns
    ///
    /// GraphQL 响应
    pub fn execute(&self, request: &GraphQLRequest) -> GraphQLResponse {
        let result = self.execute_internal(request);

        match result {
            Ok(data) => GraphQLResponse::success(data),
            Err(err) => GraphQLResponse::error(vec![err]),
        }
    }

    /// 内部执行查询
    fn execute_internal(&self, request: &GraphQLRequest) -> GraphQLResult<ConstValue> {
        let parsed_query = self.parse_query(&request.query)?;
        let data = self.execute_selection_set(&parsed_query, &self.node_store)?;
        Ok(ConstValue::Object(data))
    }

    /// 解析查询（简化实现）
    fn parse_query(&self, query: &str) -> GraphQLResult<SelectionSet> {
        if query.trim().is_empty() {
            return Err(GraphQLError::QueryParseError("Empty query".to_string()));
        }

        let selections = vec![Selection::Field(FieldSelection {
            alias: None,
            name: "allNodes".to_string(),
            arguments: HashMap::new(),
            directives: Vec::new(),
            selection_set: Some(SelectionSet {
                selections: vec![Selection::Field(FieldSelection {
                    alias: None,
                    name: "id".to_string(),
                    arguments: HashMap::new(),
                    directives: Vec::new(),
                    selection_set: None,
                })],
            }),
        })];

        Ok(SelectionSet { selections })
    }

    /// 执行选择集
    fn execute_selection_set(
        &self,
        selection_set: &SelectionSet,
        node_store: &NodeStore,
    ) -> GraphQLResult<HashMap<String, ConstValue>> {
        let mut result = HashMap::new();

        for selection in &selection_set.selections {
            match selection {
                Selection::Field(field) => {
                    let value = self.execute_field(field, node_store)?;
                    let key = field.alias.as_ref().unwrap_or(&field.name).clone();
                    result.insert(key, value);
                }
                Selection::FragmentSpread(_) => {}
                Selection::InlineFragment(_, _) => {}
            }
        }

        Ok(result)
    }

    /// 执行字段
    fn execute_field(
        &self,
        field: &FieldSelection,
        node_store: &NodeStore,
    ) -> GraphQLResult<ConstValue> {
        match field.name.as_str() {
            "allNodes" => {
                let nodes = node_store.get_all_nodes();
                let values: Vec<ConstValue> = nodes
                    .iter()
                    .map(|node| self.node_to_value(node))
                    .collect();
                Ok(ConstValue::List(values))
            }
            "id" => {
                Ok(ConstValue::Null)
            }
            _ => {
                let resolver_key = format!("Query.{}", field.name);
                if let Some(resolver) = self.resolvers.get(&resolver_key) {
                    resolver(field, node_store)
                } else {
                    Ok(ConstValue::Null)
                }
            }
        }
    }

    /// 将节点转换为 ConstValue
    fn node_to_value(&self, node: &Node) -> ConstValue {
        let mut object = HashMap::new();
        object.insert("id".to_string(), ConstValue::String(node.id.to_string()));

        for (key, value) in &node.fields {
            object.insert(key.clone(), value.clone());
        }

        ConstValue::Object(object)
    }

    /// 获取 Schema
    pub fn schema(&self) -> &GraphQLSchema {
        &self.schema
    }
}

/// Schema 构建器
pub struct SchemaBuilder {
    /// 查询类型名称
    query_type: Option<String>,

    /// 变更类型名称
    mutation_type: Option<String>,

    /// 订阅类型名称
    subscription_type: Option<String>,

    /// 类型定义
    types: Vec<GraphQLObjectType>,
}

impl SchemaBuilder {
    /// 创建新的 Schema 构建器
    pub fn new() -> Self {
        SchemaBuilder {
            query_type: None,
            mutation_type: None,
            subscription_type: None,
            types: Vec::new(),
        }
    }

    /// 设置查询类型
    ///
    /// # Arguments
    ///
    /// * `name` - 查询类型名称
    pub fn query_type(mut self, name: String) -> Self {
        self.query_type = Some(name);
        self
    }

    /// 设置变更类型
    ///
    /// # Arguments
    ///
    /// * `name` - 变更类型名称
    pub fn mutation_type(mut self, name: String) -> Self {
        self.mutation_type = Some(name);
        self
    }

    /// 设置订阅类型
    ///
    /// # Arguments
    ///
    /// * `name` - 订阅类型名称
    pub fn subscription_type(mut self, name: String) -> Self {
        self.subscription_type = Some(name);
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

    /// 构建 Schema
    ///
    /// # Errors
    ///
    /// 如果未设置查询类型，返回 `GraphQLError::ValidationError`
    pub fn build(self) -> GraphQLResult<GraphQLSchema> {
        let query_type = self
            .query_type
            .ok_or_else(|| GraphQLError::ValidationError("Query type is required".to_string()))?;

        Ok(GraphQLSchema {
            query_type,
            mutation_type: self.mutation_type,
            subscription_type: self.subscription_type,
            types: self.types,
            directives: Vec::new(),
        })
    }
}

impl Default for SchemaBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// 节点构建器
pub struct NodeBuilder {
    /// 节点 ID
    id: Option<NodeId>,

    /// 节点类型
    type_name: Option<NodeType>,

    /// 内容摘要
    content_digest: Option<String>,

    /// 节点字段
    fields: HashMap<String, ConstValue>,

    /// 子节点
    children: Vec<NodeId>,

    /// 父节点
    parent: Option<NodeId>,

    /// 内容
    content: Option<String>,

    /// 描述
    description: Option<String>,

    /// 所有者
    owner: Option<String>,
}

impl NodeBuilder {
    /// 创建新的节点构建器
    pub fn new() -> Self {
        NodeBuilder {
            id: None,
            type_name: None,
            content_digest: None,
            fields: HashMap::new(),
            children: Vec::new(),
            parent: None,
            content: None,
            description: None,
            owner: None,
        }
    }

    /// 设置节点 ID
    ///
    /// # Arguments
    ///
    /// * `id` - 节点 ID
    pub fn id(mut self, id: NodeId) -> Self {
        self.id = Some(id);
        self
    }

    /// 设置节点类型
    ///
    /// # Arguments
    ///
    /// * `type_name` - 节点类型
    pub fn type_name(mut self, type_name: NodeType) -> Self {
        self.type_name = Some(type_name);
        self
    }

    /// 设置内容摘要
    ///
    /// # Arguments
    ///
    /// * `digest` - 内容摘要
    pub fn content_digest(mut self, digest: String) -> Self {
        self.content_digest = Some(digest);
        self
    }

    /// 添加字段
    ///
    /// # Arguments
    ///
    /// * `key` - 字段键
    /// * `value` - 字段值
    pub fn field(mut self, key: String, value: ConstValue) -> Self {
        self.fields.insert(key, value);
        self
    }

    /// 添加子节点
    ///
    /// # Arguments
    ///
    /// * `child_id` - 子节点 ID
    pub fn child(mut self, child_id: NodeId) -> Self {
        self.children.push(child_id);
        self
    }

    /// 设置父节点
    ///
    /// # Arguments
    ///
    /// * `parent_id` - 父节点 ID
    pub fn parent(mut self, parent_id: NodeId) -> Self {
        self.parent = Some(parent_id);
        self
    }

    /// 设置内容
    ///
    /// # Arguments
    ///
    /// * `content` - 节点内容
    pub fn content(mut self, content: String) -> Self {
        self.content = Some(content);
        self
    }

    /// 设置描述
    ///
    /// # Arguments
    ///
    /// * `description` - 节点描述
    pub fn description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    /// 设置所有者
    ///
    /// # Arguments
    ///
    /// * `owner` - 节点所有者
    pub fn owner(mut self, owner: String) -> Self {
        self.owner = Some(owner);
        self
    }

    /// 构建节点
    ///
    /// # Errors
    ///
    /// 如果缺少必要字段，返回 `GraphQLError::ValidationError`
    pub fn build(self) -> GraphQLResult<Node> {
        let id = self
            .id
            .ok_or_else(|| GraphQLError::ValidationError("Node ID is required".to_string()))?;

        let type_name = self
            .type_name
            .ok_or_else(|| GraphQLError::ValidationError("Node type is required".to_string()))?;

        let content_digest = self
            .content_digest
            .ok_or_else(|| GraphQLError::ValidationError("Content digest is required".to_string()))?;

        let mut node = Node::new(id, type_name, content_digest);
        node.fields = self.fields;
        node.children = self.children;
        node.parent = self.parent;

        if let Some(content) = self.content {
            node.internal.content = Some(content);
        }
        if let Some(description) = self.description {
            node.internal.description = Some(description);
        }
        if let Some(owner) = self.owner {
            node.internal.owner = Some(owner);
        }

        Ok(node)
    }
}

impl Default for NodeBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// 内容摘要生成器
pub struct ContentDigest;

impl ContentDigest {
    /// 生成内容摘要
    ///
    /// # Arguments
    ///
    /// * `content` - 要生成摘要的内容
    pub fn generate(content: &str) -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        content.hash(&mut hasher);
        format!("{:x}", hasher.finish())
    }
}
