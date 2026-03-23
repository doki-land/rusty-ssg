use async_graphql_value::ConstValue;
use gatsby::{GraphQLService, Node, NodeId, NodeType};

#[test]
fn test_graphql_service_creation() {
    let service = GraphQLService::new();
    assert!(!service.get_schema().types.is_empty());
}

#[test]
fn test_graphql_service_add_node() {
    let mut service = GraphQLService::new();
    let node_id = NodeId::generate();
    let node_type = NodeType::new("MarkdownRemark".to_string());

    let mut node = Node::new(node_id.clone(), node_type, "test-digest".to_string());
    node.set_field("title".to_string(), ConstValue::String("Test Title".to_string()));
    node.set_field("content".to_string(), ConstValue::String("Test Content".to_string()));

    let result = service.add_node(node);
    assert!(result.is_ok());

    let node_store = service.node_store();
    assert!(node_store.has_node(&node_id));
}

#[test]
fn test_graphql_service_generate_schema_sdl() {
    let service = GraphQLService::new();
    let sdl = service.generate_schema_sdl();
    assert!(!sdl.is_empty());
    assert!(sdl.contains("type Query"));
    assert!(sdl.contains("type MarkdownRemark"));
    assert!(sdl.contains("type File"));
}

#[test]
fn test_graphql_service_execute_query() {
    let service = GraphQLService::new();
    let request = gatsby::GraphQLRequest {
        query: "query { site { siteMetadata { title } } }".to_string(),
        operation_name: None,
        variables: None,
    };

    let response = service.execute_query(request);
    assert!(response.data.is_some());
}
