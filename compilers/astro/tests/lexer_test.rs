//! 词法分析器测试

use astro::compiler::parser::{lexer::Lexer, tokens::{Token, InterpolationType}};

#[test]
fn test_lexer_basic_tokens() {
    let input = r#"<div>{{ variable }}</div>"#;
    let mut lexer = Lexer::new(input);
    
    assert_eq!(lexer.next_token(), Token::TagStart);
    assert_eq!(lexer.next_token(), Token::Identifier("div".to_string()));
    assert_eq!(lexer.next_token(), Token::TagEnd);
    assert_eq!(lexer.next_token(), Token::InterpolationStart(InterpolationType::Escaped));
    assert_eq!(lexer.next_token(), Token::Whitespace(" ".to_string()));
    assert_eq!(lexer.next_token(), Token::Identifier("variable".to_string()));
    assert_eq!(lexer.next_token(), Token::Whitespace(" ".to_string()));
    assert_eq!(lexer.next_token(), Token::InterpolationEnd(InterpolationType::Escaped));
    assert_eq!(lexer.next_token(), Token::TagEndStart);
    assert_eq!(lexer.next_token(), Token::Identifier("div".to_string()));
    assert_eq!(lexer.next_token(), Token::TagEnd);
    assert_eq!(lexer.next_token(), Token::Eof);
}

#[test]
fn test_lexer_directives() {
    let input = r#"{% if condition %}{% endif %}"#;
    let mut lexer = Lexer::new(input);
    
    assert_eq!(lexer.next_token(), Token::DirectiveStart);
    assert_eq!(lexer.next_token(), Token::Identifier("if".to_string()));
    assert_eq!(lexer.next_token(), Token::Whitespace(" ".to_string()));
    assert_eq!(lexer.next_token(), Token::Identifier("condition".to_string()));
    assert_eq!(lexer.next_token(), Token::DirectiveEnd);
    assert_eq!(lexer.next_token(), Token::DirectiveStart);
    assert_eq!(lexer.next_token(), Token::Identifier("endif".to_string()));
    assert_eq!(lexer.next_token(), Token::DirectiveEnd);
    assert_eq!(lexer.next_token(), Token::Eof);
}

#[test]
fn test_lexer_comments() {
    let input = r#"<div>{# This is a comment #}</div>"#;
    let mut lexer = Lexer::new(input);
    
    assert_eq!(lexer.next_token(), Token::TagStart);
    assert_eq!(lexer.next_token(), Token::Identifier("div".to_string()));
    assert_eq!(lexer.next_token(), Token::TagEnd);
    assert_eq!(lexer.next_token(), Token::Comment(" This is a comment ".to_string()));
    assert_eq!(lexer.next_token(), Token::TagEndStart);
    assert_eq!(lexer.next_token(), Token::Identifier("div".to_string()));
    assert_eq!(lexer.next_token(), Token::TagEnd);
    assert_eq!(lexer.next_token(), Token::Eof);
}

#[test]
fn test_lexer_html_comments() {
    let input = r#"<div><!-- This is an HTML comment --></div>"#;
    let mut lexer = Lexer::new(input);
    
    assert_eq!(lexer.next_token(), Token::TagStart);
    assert_eq!(lexer.next_token(), Token::Identifier("div".to_string()));
    assert_eq!(lexer.next_token(), Token::TagEnd);
    assert_eq!(lexer.next_token(), Token::Comment(" This is an HTML comment ".to_string()));
    assert_eq!(lexer.next_token(), Token::TagEndStart);
    assert_eq!(lexer.next_token(), Token::Identifier("div".to_string()));
    assert_eq!(lexer.next_token(), Token::TagEnd);
    assert_eq!(lexer.next_token(), Token::Eof);
}

#[test]
fn test_lexer_expressions() {
    let input = r#"{{ x + y * z }}  {{ a > b && c < d }}"#;
    let mut lexer = Lexer::new(input);
    
    assert_eq!(lexer.next_token(), Token::InterpolationStart(InterpolationType::Escaped));
    assert_eq!(lexer.next_token(), Token::Whitespace(" ".to_string()));
    assert_eq!(lexer.next_token(), Token::Identifier("x".to_string()));
    assert_eq!(lexer.next_token(), Token::Whitespace(" ".to_string()));
    assert_eq!(lexer.next_token(), Token::Plus);
    assert_eq!(lexer.next_token(), Token::Whitespace(" ".to_string()));
    assert_eq!(lexer.next_token(), Token::Identifier("y".to_string()));
    assert_eq!(lexer.next_token(), Token::Whitespace(" ".to_string()));
    assert_eq!(lexer.next_token(), Token::Multiply);
    assert_eq!(lexer.next_token(), Token::Whitespace(" ".to_string()));
    assert_eq!(lexer.next_token(), Token::Identifier("z".to_string()));
    assert_eq!(lexer.next_token(), Token::Whitespace(" ".to_string()));
    assert_eq!(lexer.next_token(), Token::InterpolationEnd(InterpolationType::Escaped));
    assert_eq!(lexer.next_token(), Token::Whitespace("  ".to_string()));
    assert_eq!(lexer.next_token(), Token::InterpolationStart(InterpolationType::Escaped));
    assert_eq!(lexer.next_token(), Token::Whitespace(" ".to_string()));
    assert_eq!(lexer.next_token(), Token::Identifier("a".to_string()));
    assert_eq!(lexer.next_token(), Token::Whitespace(" ".to_string()));
    assert_eq!(lexer.next_token(), Token::GreaterThan);
    assert_eq!(lexer.next_token(), Token::Whitespace(" ".to_string()));
    assert_eq!(lexer.next_token(), Token::Identifier("b".to_string()));
    assert_eq!(lexer.next_token(), Token::Whitespace(" ".to_string()));
    assert_eq!(lexer.next_token(), Token::And);
    assert_eq!(lexer.next_token(), Token::Whitespace(" ".to_string()));
    assert_eq!(lexer.next_token(), Token::Identifier("c".to_string()));
    assert_eq!(lexer.next_token(), Token::Whitespace(" ".to_string()));
    assert_eq!(lexer.next_token(), Token::LessThan);
    assert_eq!(lexer.next_token(), Token::Whitespace(" ".to_string()));
    assert_eq!(lexer.next_token(), Token::Identifier("d".to_string()));
    assert_eq!(lexer.next_token(), Token::Whitespace(" ".to_string()));
    assert_eq!(lexer.next_token(), Token::InterpolationEnd(InterpolationType::Escaped));
    assert_eq!(lexer.next_token(), Token::Eof);
}