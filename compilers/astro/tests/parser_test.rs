//! 语法分析器测试

use astro::compiler::parser::{parser::Parser, ast::{AstNode, InterpolationType, DirectiveType}};

#[test]
fn test_parser_basic_template() {
    let input = r#"<div>{{ variable }}</div>"#;
    let mut parser = Parser::new(input);
    let nodes = parser.parse();
    
    assert_eq!(nodes.len(), 1);
    match &nodes[0] {
        AstNode::Component { name, attributes, content, self_closing } => {
            assert_eq!(name, "div");
            assert!(attributes.is_empty());
            assert!(!self_closing);
            assert!(content.is_some());
            let content_nodes = content.as_ref().unwrap();
            assert_eq!(content_nodes.len(), 1);
            match &content_nodes[0] {
                AstNode::Interpolation { expression, interpolation_type } => {
                    assert_eq!(expression, "variable");
                    assert_eq!(interpolation_type, &InterpolationType::Escaped);
                }
                _ => panic!("Expected Interpolation node"),
            }
        }
        _ => panic!("Expected Component node"),
    }
}

#[test]
fn test_parser_directives() {
    let input = r#"{% if condition %}Hello{% endif %}"#;
    let mut parser = Parser::new(input);
    let nodes = parser.parse();
    
    assert_eq!(nodes.len(), 1);
    match &nodes[0] {
        AstNode::Directive { directive_type, arguments, content } => {
            assert_eq!(directive_type, &DirectiveType::If);
            assert_eq!(arguments, &vec!["condition"]);
            assert!(content.is_some());
            let content_nodes = content.as_ref().unwrap();
            assert_eq!(content_nodes.len(), 1);
            match &content_nodes[0] {
                AstNode::Text(text) => {
                    assert_eq!(text, "Hello");
                }
                _ => panic!("Expected Text node"),
            }
        }
        _ => panic!("Expected Directive node"),
    }
}

#[test]
fn test_parser_for_directive() {
    let input = r#"{% for item in items %}<div>{{ item }}</div>{% endfor %}"#;
    let mut parser = Parser::new(input);
    let nodes = parser.parse();
    
    assert_eq!(nodes.len(), 1);
    match &nodes[0] {
        AstNode::Directive { directive_type, arguments, content } => {
            assert_eq!(directive_type, &DirectiveType::For);
            assert_eq!(arguments, &vec!["item", "in", "items"]);
            assert!(content.is_some());
            let content_nodes = content.as_ref().unwrap();
            assert_eq!(content_nodes.len(), 1);
            match &content_nodes[0] {
                AstNode::Component { name, attributes, content: component_content, self_closing } => {
                    assert_eq!(name, "div");
                    assert!(attributes.is_empty());
                    assert!(!self_closing);
                    assert!(component_content.is_some());
                    let component_content_nodes = component_content.as_ref().unwrap();
                    assert_eq!(component_content_nodes.len(), 1);
                    match &component_content_nodes[0] {
                        AstNode::Interpolation { expression, interpolation_type } => {
                            assert_eq!(expression, "item");
                            assert_eq!(interpolation_type, &InterpolationType::Escaped);
                        }
                        _ => panic!("Expected Interpolation node"),
                    }
                }
                _ => panic!("Expected Component node"),
            }
        }
        _ => panic!("Expected Directive node"),
    }
}

#[test]
fn test_parser_components() {
    let input = r#"<MyComponent title="Hello" count={42}>
  <div>Content</div>
</MyComponent>"#;
    let mut parser = Parser::new(input);
    let nodes = parser.parse();
    
    assert_eq!(nodes.len(), 1);
    match &nodes[0] {
        AstNode::Component { name, attributes, content, self_closing } => {
            assert_eq!(name, "MyComponent");
            assert_eq!(attributes.get("title"), Some(&"Hello".to_string()));
            assert_eq!(attributes.get("count"), Some(&"42".to_string()));
            assert!(!self_closing);
            assert!(content.is_some());
            let content_nodes = content.as_ref().unwrap();
            assert_eq!(content_nodes.len(), 1);
            match &content_nodes[0] {
                AstNode::Component { name: inner_name, attributes: inner_attributes, content: inner_content, self_closing: inner_self_closing } => {
                    assert_eq!(inner_name, "div");
                    assert!(inner_attributes.is_empty());
                    assert!(!inner_self_closing);
                    assert!(inner_content.is_some());
                    let inner_content_nodes = inner_content.as_ref().unwrap();
                    assert_eq!(inner_content_nodes.len(), 1);
                    match &inner_content_nodes[0] {
                        AstNode::Text(text) => {
                            assert_eq!(text, "Content");
                        }
                        _ => panic!("Expected Text node"),
                    }
                }
                _ => panic!("Expected Component node"),
            }
        }
        _ => panic!("Expected Component node"),
    }
}

#[test]
fn test_parser_comments() {
    let input = r#"<div>{# This is a comment #}</div>"#;
    let mut parser = Parser::new(input);
    let nodes = parser.parse();
    
    assert_eq!(nodes.len(), 1);
    match &nodes[0] {
        AstNode::Component { name, attributes, content, self_closing } => {
            assert_eq!(name, "div");
            assert!(attributes.is_empty());
            assert!(!self_closing);
            assert!(content.is_some());
            let content_nodes = content.as_ref().unwrap();
            assert_eq!(content_nodes.len(), 0); // 注释应该被跳过
        }
        _ => panic!("Expected Component node"),
    }
}