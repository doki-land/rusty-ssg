//! Astro 语法解析器

use std::collections::HashMap;

use super::lexer::Lexer;
use super::tokens::Token;
use super::ast::{AstNode, DirectiveType};

/// 语法分析器
pub struct Parser {
    /// 词法分析器
    lexer: Lexer,
    /// 当前 token
    current_token: Token,
}

impl Parser {
    /// 创建新的语法分析器
    pub fn new(input: &'static str) -> Self {
        let mut lexer = Lexer::new(input);
        let current_token = lexer.next_token();
        Self {
            lexer,
            current_token,
        }
    }

    /// 解析 Astro 模板，返回抽象语法树
    pub fn parse(&mut self) -> Vec<AstNode> {
        self.parse_nodes()
    }

    /// 解析多个节点
    fn parse_nodes(&mut self) -> Vec<AstNode> {
        let mut nodes = Vec::new();
        while self.current_token != Token::Eof {
            match &self.current_token {
                Token::Text(text) => {
                    nodes.push(AstNode::text(text));
                    self.next_token();
                }
                Token::InterpolationStart(interpolation_type) => {
                    nodes.push(self.parse_interpolation(interpolation_type.clone()));
                }
                Token::DirectiveStart => {
                    nodes.push(self.parse_directive());
                }
                Token::TagStart => {
                    nodes.push(self.parse_component());
                }
                _ => {
                    // 跳过未知 token
                    self.next_token();
                }
            }
        }
        nodes
    }

    /// 解析变量插值
    fn parse_interpolation(&mut self, interpolation_type: super::tokens::InterpolationType) -> AstNode {
        // 跳过 InterpolationStart
        self.next_token();

        // 解析表达式
        let mut expression = String::new();
        while self.current_token != Token::InterpolationEnd(interpolation_type) {
            match &self.current_token {
                Token::Text(text) => expression.push_str(text),
                Token::Identifier(id) => expression.push_str(id),
                Token::Number(num) => expression.push_str(num),
                Token::String(s) => expression.push_str(s),
                Token::Plus => expression.push('+'),
                Token::Minus => expression.push('-'),
                Token::Multiply => expression.push('*'),
                Token::Divide => expression.push('/'),
                Token::Modulo => expression.push('%'),
                Token::Equal => expression.push_str("=="),
                Token::NotEqual => expression.push_str("!="),
                Token::GreaterThan => expression.push('>'),
                Token::LessThan => expression.push('<'),
                Token::GreaterThanOrEqual => expression.push_str(">="),
                Token::LessThanOrEqual => expression.push_str("<="),
                Token::And => expression.push_str("&&"),
                Token::Or => expression.push_str("||"),
                Token::Not => expression.push('!'),
                Token::LeftParen => expression.push('('),
                Token::RightParen => expression.push(')'),
                Token::LeftBracket => expression.push('['),
                Token::RightBracket => expression.push(']'),
                Token::LeftBrace => expression.push('{'),
                Token::RightBrace => expression.push('}'),
                Token::Dot => expression.push('.'),
                Token::Comma => expression.push(','),
                Token::Colon => expression.push(':'),
                Token::Semicolon => expression.push(';'),
                _ => {}
            }
            self.next_token();
        }

        // 跳过 InterpolationEnd
        self.next_token();

        // 转换为 ast::InterpolationType
        let ast_interpolation_type = match interpolation_type {
            super::tokens::InterpolationType::Escaped => super::ast::InterpolationType::Escaped,
            super::tokens::InterpolationType::Unescaped => super::ast::InterpolationType::Unescaped,
        };

        AstNode::interpolation(expression.trim(), ast_interpolation_type)
    }

    /// 解析指令
    fn parse_directive(&mut self) -> AstNode {
        // 跳过 DirectiveStart
        self.next_token();

        // 解析指令类型
        let directive_type = match &self.current_token {
            Token::Identifier(id) => {
                let id_lower = id.to_lowercase();
                match id_lower.as_str() {
                    "for" => DirectiveType::For,
                    "if" => DirectiveType::If,
                    "else" => DirectiveType::Else,
                    "elif" => DirectiveType::Elif,
                    "endif" => DirectiveType::End,
                    "endfor" => DirectiveType::End,
                    "import" => DirectiveType::Import,
                    "layout" => DirectiveType::Layout,
                    _ => DirectiveType::Other(id.to_string()),
                }
            }
            _ => DirectiveType::Other("unknown".to_string()),
        };

        // 解析指令参数
        let mut arguments = Vec::new();
        self.next_token();
        while self.current_token != Token::DirectiveEnd {
            match &self.current_token {
                Token::Text(text) => arguments.push(text.to_string()),
                Token::Identifier(id) => arguments.push(id.to_string()),
                Token::Number(num) => arguments.push(num.to_string()),
                Token::String(s) => arguments.push(s.to_string()),
                _ => {}
            }
            self.next_token();
        }

        // 跳过 DirectiveEnd
        self.next_token();

        // 解析指令内容（如果有）
        let content = match directive_type {
            DirectiveType::For | DirectiveType::If | DirectiveType::Layout => {
                let content = self.parse_nodes();
                // 跳过结束指令
                if let Token::DirectiveStart = self.current_token {
                    self.next_token();
                    match self.current_token.clone() {
                        Token::Identifier(id) => {
                            if id == "endfor" || id == "endif" {
                                self.next_token();
                                if let Token::DirectiveEnd = self.current_token {
                                    self.next_token();
                                }
                            }
                        }
                        _ => {}
                    }
                }
                Some(content)
            }
            _ => None,
        };

        AstNode::directive(directive_type, arguments, content)
    }

    /// 解析组件标签
    fn parse_component(&mut self) -> AstNode {
        // 跳过 TagStart
        self.next_token();

        // 解析组件名称
        let component_name = match &self.current_token {
            Token::Identifier(id) => id.to_string(),
            _ => "unknown".to_string(),
        };
        self.next_token();

        // 解析组件属性
        let mut attributes = HashMap::new();
        while self.current_token != Token::TagEnd && self.current_token != Token::TagClose {
            match self.current_token.clone() {
                Token::Identifier(name) => {
                    self.next_token();
                    if let Token::Equals = self.current_token {
                        self.next_token();
                        match self.current_token.clone() {
                            Token::String(value) => {
                                attributes.insert(name, value);
                                self.next_token();
                            }
                            Token::Identifier(value) => {
                                attributes.insert(name, value);
                                self.next_token();
                            }
                            _ => {}
                        }
                    }
                }
                _ => self.next_token(),
            }
        }

        // 检查是否是自闭合标签
        let self_closing = if self.current_token == Token::TagClose {
            self.next_token();
            true
        } else {
            // 跳过 TagEnd
            self.next_token();
            false
        };

        // 解析组件内容（如果不是自闭合标签）
        let content = if !self_closing {
            let content = self.parse_nodes();
            // 跳过结束标签
            if let Token::TagEndStart = self.current_token {
                self.next_token();
                match self.current_token.clone() {
                    Token::Identifier(id) => {
                        if id == component_name {
                            self.next_token();
                            if let Token::TagEnd = self.current_token {
                                self.next_token();
                            }
                        }
                    }
                    _ => {}
                }
            }
            Some(content)
        } else {
            None
        };

        AstNode::component(&component_name, attributes, content, self_closing)
    }

    /// 获取下一个 token
    fn next_token(&mut self) {
        self.current_token = self.lexer.next_token();
    }
}