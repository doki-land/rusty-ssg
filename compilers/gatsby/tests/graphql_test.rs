use gatsby::{
    GraphQLExecutor, SchemaBuilder, NodeBuilder, ContentDigest,
    GraphQLError, GraphQLResult, NodeId, NodeType, Node, NodeStore,
    GraphQLRequest,
};

#[test]
fn test_node_id_creation() {
    let id = NodeId::new("test-id".to_string());
    assert_eq!(id.as_str(), "test-id");
}

#[test]
fn test_node_id_generation() {
    let id1 = NodeId::generate();
    let id2 = NodeId::generate();
    assert_ne!(id1, id2);
}

#[test]
fn test_node_id_from_string() {
    let id: NodeId = "test-id".into();
    assert_eq!(id.as_str(), "test-id");
}

#[test]
fn test_node_type_creation() {
    let node_type = NodeType::new("TestType".to_string());
    assert_eq!(node_type.as_str(), "TestType");
}

#[test]
fn test_node_creation() {
    let id = NodeId::new("test-node".to_string());
    let type_name = NodeType::new("TestType".to_string());
    let digest = ContentDigest::generate("test content");
    
    let node = Node::new(id.clone(), type_name.clone(), digest.clone());
    
    assert_eq!(node.id, id);
    assert_eq!(node.internal.type_name, type_name);
    assert_eq!(node.internal.content_digest, digest);
}

#[test]
fn test_node_builder() {
    let id = NodeId::new("test-node".to_string());
    let type_name = NodeType::new("TestType".to_string());
    let digest = ContentDigest::generate("test content");
    
    let result = NodeBuilder::new()
        .id(id.clone())
        .type_name(type_name.clone())
        .content_digest(digest)
        .build();
    
    assert!(result.is_ok());
    let node = result.unwrap();
    assert_eq!(node.id, id);
    assert_eq!(node.internal.type_name, type_name);
}

#[test]
fn test_node_builder_missing_id() {
    let type_name = NodeType::new("TestType".to_string());
    let digest = ContentDigest::generate("test content");
    
    let result = NodeBuilder::new()
        .type_name(type_name)
        .content_digest(digest)
        .build();
    
    assert!(result.is_err());
}

#[test]
fn test_node_store_operations() {
    let mut store = NodeStore::new();
    let id = NodeId::new("test-node".to_string());
    let type_name = NodeType::new("TestType".to_string());
    let digest = ContentDigest::generate("test content");
    let node = Node::new(id.clone(), type_name.clone(), digest);
    
    let result = store.add_node(node);
    assert!(result.is_ok());
    assert_eq!(store.node_count(), 1);
    assert!(store.has_node(&id));
    
    let retrieved = store.get_node(&id);
    assert!(retrieved.is_some());
    assert_eq!(retrieved.unwrap().id, id);
}

#[test]
fn test_node_store_duplicate_node() {
    let mut store = NodeStore::new();
    let id = NodeId::new("test-node".to_string());
    let type_name = NodeType::new("TestType".to_string());
    let digest = ContentDigest::generate("test content");
    let node1 = Node::new(id.clone(), type_name.clone(), digest.clone());
    let node2 = Node::new(id.clone(), type_name.clone(), digest);
    
    let result1 = store.add_node(node1);
    assert!(result1.is_ok());
    
    let result2 = store.add_node(node2);
    assert!(result2.is_err());
    match result2 {
        Err(GraphQLError::DuplicateNode(_)) => {},
        _ => panic!("Expected DuplicateNode error"),
    }
}

#[test]
fn test_schema_builder() {
    let result = SchemaBuilder::new()
        .query_type("Query".to_string())
        .build();
    
    assert!(result.is_ok());
    let schema = result.unwrap();
    assert_eq!(schema.query_type, "Query");
}

#[test]
fn test_schema_builder_missing_query_type() {
    let result = SchemaBuilder::new().build();
    assert!(result.is_err());
    match result {
        Err(GraphQLError::ValidationError(_)) => {},
        _ => panic!("Expected ValidationError"),
    }
}

#[test]
fn test_graphql_executor_creation() {
    let schema = SchemaBuilder::new()
        .query_type("Query".to_string())
        .build()
        .unwrap();
    
    let executor = GraphQLExecutor::new(schema);
    assert_eq!(executor.schema().query_type, "Query");
}

#[test]
fn test_content_digest_generation() {
    let content = "test content";
    let digest1 = ContentDigest::generate(content);
    let digest2 = ContentDigest::generate(content);
    
    assert_eq!(digest1, digest2);
    
    let different_content = "different content";
    let digest3 = ContentDigest::generate(different_content);
    assert_ne!(digest1, digest3);
}

#[test]
fn test_graphql_request_execution() {
    let schema = SchemaBuilder::new()
        .query_type("Query".to_string())
        .build()
        .unwrap();
    
    let executor = GraphQLExecutor::new(schema);
    
    let request = GraphQLRequest {
        query: "{ allNodes { id } }".to_string(),
        operation_name: None,
        variables: None,
    };
    
    let response = executor.execute(&request);
    assert!(response.data.is_some());
}
