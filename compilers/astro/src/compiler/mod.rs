//! Astro 编译器模块

pub mod component;
pub mod dependency;
pub mod optimization;
pub mod parser;
pub mod renderer;
pub use component::*;

pub use dependency::DependencyAnalyzer;
pub use optimization::Optimizer;
pub use parser::{AstNode, Lexer, Parser, Token};
pub use renderer::HtmlRenderer;

/// Astro 语言定义
pub struct AstroLanguage;

impl Default for AstroLanguage {
    fn default() -> Self {
        Self
    }
}

/// Astro 词法分析器
pub struct AstroLexer;

impl oak_core::lexer::Lexer for AstroLexer {
    type Language = AstroLanguage;
    type Token = Token;
    type Error = oak_core::error::Error;

    fn lex<'source>(
        &self,
        source: &oak_core::source::SourceText<'source>,
        _config: &[oak_core::config::Config],
        _lexer_cache: &mut oak_core::lexer::LexerCache<Self::Language>,
    ) -> oak_core::lexer::LexResult<'source, Self::Token, Self::Error> {
        let mut lexer = super::parser::lexer::Lexer::new(source.text());
        let tokens: Vec<Self::Token> = std::iter::from_fn(|| {
            let token = lexer.next_token();
            if token != Token::Eof {
                Some(token)
            } else {
                None
            }
        }).collect();
        oak_core::lexer::LexResult::new_ok(tokens)
    }
}

/// Astro 语法分析器
pub struct AstroParser;

impl oak_core::parser::Parser for AstroParser {
    type Language = AstroLanguage;
    type Token = Token;
    type Node = AstNode;
    type Error = oak_core::error::Error;

    fn parse<'source>(
        &self,
        source: &oak_core::source::SourceText<'source>,
        _config: &[oak_core::config::Config],
        _session: &mut oak_core::parser::session::ParseSession<Self::Language>,
    ) -> oak_core::parser::ParseResult<'source, Self::Node, Self::Error> {
        let mut parser = super::parser::parser::Parser::new(source.text());
        let nodes = parser.parse();
        // 将多个节点包装成一个根节点
        let root = AstNode::text("");
        oak_core::parser::ParseResult::new_ok(root)
    }
}
