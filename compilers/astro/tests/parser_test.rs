//! Astro 语法解析器测试

use super::parser::Parser;

#[test]
fn test_parse_text() {
    let input = "Hello, World!";
    let mut parser = Parser::new(input);
    let nodes = parser.parse();
    assert_eq!(nodes.len(), 1);
}

#[test]
fn test_parse_interpolation() {
    let input = "Hello, {{ name }}!";
    let mut parser = Parser::new(input);
    let nodes = parser.parse();
    assert_eq!(nodes.len(), 3); // Text, Interpolation, Text
}

#[test]
fn test_parse_directive() {
    let input = "{% if condition %}Hello{% endif %}";
    let mut parser = Parser::new(input);
    let nodes = parser.parse();
    assert_eq!(nodes.len(), 1); // Directive
}

#[test]
fn test_parse_component() {
    let input = "<Component prop=\"value\">Content</Component>";
    let mut parser = Parser::new(input);
    let nodes = parser.parse();
    assert_eq!(nodes.len(), 1); // Component
}

#[test]
fn test_parse_self_closing_component() {
    let input = "<Component prop=\"value\" />";
    let mut parser = Parser::new(input);
    let nodes = parser.parse();
    assert_eq!(nodes.len(), 1); // Component
}
