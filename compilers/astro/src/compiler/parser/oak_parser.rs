//! Astro 语法分析器（基于 oaks 框架）

use oak_core::{language::Language, parser::{Parser, ParseOutput, ParseSession, TreeSink}, lexer::Token};
use oak_core::source::Source;
use oak_core::TextEdit;

use super::oak_lexer::{AstroLanguage, AstroTokenType, AstroElementType};

/// Astro 语法分析器
pub struct AstroParser;

impl Parser<AstroLanguage> for AstroParser {
    fn parse<'a, S: Source + ?Sized>(&self, text: &S, edits: &[TextEdit], session: &'a mut ParseSession<AstroLanguage>) -> ParseOutput<AstroLanguage> {
        let lexer = super::oak_lexer::AstroLexer;
        let tokens = lexer.lex(text, edits, &mut session.lexer_cache).into_result().unwrap();
        let mut sink = session.tree_sink();
        
        self.parse_root(&tokens, &mut sink);
        
        ParseOutput::ok(sink.finish())
    }
}

impl AstroParser {
    /// 解析根节点
    fn parse_root(&self, tokens: &oak_core::lexer::Tokens<AstroLanguage>, sink: &mut impl TreeSink<AstroLanguage>) {
        sink.start_node(AstroElementType::Root);
        
        let mut pos = 0;
        while pos < tokens.len() {
            let token = &tokens[pos];
            
            match token.token_type() {
                AstroTokenType::ScriptStart => {
                    pos = self.parse_script(tokens, pos, sink);
                }
                AstroTokenType::Text => {
                    sink.token(token.token_type().clone(), token.range());
                    pos += 1;
                }
                AstroTokenType::InterpolationStart => {
                    pos = self.parse_interpolation(tokens, pos, sink, false);
                }
                AstroTokenType::UnescapedInterpolationStart => {
                    pos = self.parse_interpolation(tokens, pos, sink, true);
                }
                AstroTokenType::DirectiveStart => {
                    pos = self.parse_directive(tokens, pos, sink);
                }
                AstroTokenType::TagStart => {
                    pos = self.parse_component(tokens, pos, sink);
                }
                AstroTokenType::Eof => {
                    break;
                }
                _ => {
                    pos += 1;
                }
            }
        }
        
        sink.finish_node();
    }
    
    /// 解析组件脚本
    fn parse_script(&self, tokens: &oak_core::lexer::Tokens<AstroLanguage>, mut pos: usize, sink: &mut impl TreeSink<AstroLanguage>) -> usize {
        sink.start_node(AstroElementType::Script);
        sink.token(AstroTokenType::ScriptStart, tokens[pos].range());
        pos += 1;
        
        // 解析脚本内容直到 ScriptEnd
        while pos < tokens.len() {
            let token = &tokens[pos];
            
            if token.token_type() == &AstroTokenType::ScriptStart {
                // 找到脚本结束
                sink.token(AstroTokenType::ScriptEnd, token.range());
                pos += 1;
                break;
            }
            else {
                sink.token(token.token_type().clone(), token.range());
                pos += 1;
            }
        }
        
        sink.finish_node();
        pos
    }
    
    /// 解析插值
    fn parse_interpolation(&self, tokens: &oak_core::lexer::Tokens<AstroLanguage>, mut pos: usize, sink: &mut impl TreeSink<AstroLanguage>, unescaped: bool) -> usize {
        let element_type = if unescaped {
            AstroElementType::UnescapedInterpolation
        } else {
            AstroElementType::Interpolation
        };
        
        sink.start_node(element_type);
        sink.token(tokens[pos].token_type().clone(), tokens[pos].range());
        pos += 1;
        
        // 解析插值内容直到 InterpolationEnd
        while pos < tokens.len() {
            let token = &tokens[pos];
            
            if (unescaped && token.token_type() == &AstroTokenType::UnescapedInterpolationEnd) || 
               (!unescaped && token.token_type() == &AstroTokenType::InterpolationEnd) {
                sink.token(token.token_type().clone(), token.range());
                pos += 1;
                break;
            }
            else {
                sink.token(token.token_type().clone(), token.range());
                pos += 1;
            }
        }
        
        sink.finish_node();
        pos
    }
    
    /// 解析指令
    fn parse_directive(&self, tokens: &oak_core::lexer::Tokens<AstroLanguage>, mut pos: usize, sink: &mut impl TreeSink<AstroLanguage>) -> usize {
        sink.start_node(AstroElementType::Directive);
        sink.token(AstroTokenType::DirectiveStart, tokens[pos].range());
        pos += 1;
        
        // 解析指令内容直到 DirectiveEnd
        while pos < tokens.len() {
            let token = &tokens[pos];
            
            if token.token_type() == &AstroTokenType::DirectiveEnd {
                sink.token(AstroTokenType::DirectiveEnd, token.range());
                pos += 1;
                break;
            }
            else {
                sink.token(token.token_type().clone(), token.range());
                pos += 1;
            }
        }
        
        sink.finish_node();
        pos
    }
    
    /// 解析组件
    fn parse_component(&self, tokens: &oak_core::lexer::Tokens<AstroLanguage>, mut pos: usize, sink: &mut impl TreeSink<AstroLanguage>) -> usize {
        sink.start_node(AstroElementType::Component);
        sink.token(AstroTokenType::TagStart, tokens[pos].range());
        pos += 1;
        
        // 解析组件名称
        if pos < tokens.len() && tokens[pos].token_type() == &AstroTokenType::Identifier {
            sink.token(AstroTokenType::Identifier, tokens[pos].range());
            pos += 1;
        }
        
        // 解析组件属性
        while pos < tokens.len() {
            let token = &tokens[pos];
            
            if token.token_type() == &AstroTokenType::TagEnd {
                sink.token(AstroTokenType::TagEnd, token.range());
                pos += 1;
                break;
            }
            else if token.token_type() == &AstroTokenType::TagClose {
                sink.token(AstroTokenType::TagClose, token.range());
                pos += 1;
                sink.finish_node();
                return pos;
            }
            else {
                // 解析属性
                pos = self.parse_attribute(tokens, pos, sink);
            }
        }
        
        // 解析组件内容
        while pos < tokens.len() {
            let token = &tokens[pos];
            
            if token.token_type() == &AstroTokenType::TagEndStart {
                sink.token(AstroTokenType::TagEndStart, token.range());
                pos += 1;
                
                // 解析结束标签名称
                if pos < tokens.len() && tokens[pos].token_type() == &AstroTokenType::Identifier {
                    sink.token(AstroTokenType::Identifier, tokens[pos].range());
                    pos += 1;
                }
                
                // 解析标签结束
                if pos < tokens.len() && tokens[pos].token_type() == &AstroTokenType::TagEnd {
                    sink.token(AstroTokenType::TagEnd, token.range());
                    pos += 1;
                }
                
                break;
            }
            else if token.token_type() == &AstroTokenType::Text {
                sink.token(AstroTokenType::Text, token.range());
                pos += 1;
            }
            else if token.token_type() == &AstroTokenType::InterpolationStart {
                pos = self.parse_interpolation(tokens, pos, sink, false);
            }
            else if token.token_type() == &AstroTokenType::UnescapedInterpolationStart {
                pos = self.parse_interpolation(tokens, pos, sink, true);
            }
            else if token.token_type() == &AstroTokenType::DirectiveStart {
                pos = self.parse_directive(tokens, pos, sink);
            }
            else if token.token_type() == &AstroTokenType::TagStart {
                pos = self.parse_component(tokens, pos, sink);
            }
            else {
                pos += 1;
            }
        }
        
        sink.finish_node();
        pos
    }
    
    /// 解析属性
    fn parse_attribute(&self, tokens: &oak_core::lexer::Tokens<AstroLanguage>, mut pos: usize, sink: &mut impl TreeSink<AstroLanguage>) -> usize {
        sink.start_node(AstroElementType::Attribute);
        
        // 解析属性名称
        if pos < tokens.len() && tokens[pos].token_type() == &AstroTokenType::Identifier {
            sink.token(AstroTokenType::Identifier, tokens[pos].range());
            pos += 1;
        }
        
        // 解析等号
        if pos < tokens.len() && tokens[pos].token_type() == &AstroTokenType::Equals {
            sink.token(AstroTokenType::Equals, tokens[pos].range());
            pos += 1;
        }
        
        // 解析属性值
        if pos < tokens.len() {
            let token = &tokens[pos];
            match token.token_type() {
                AstroTokenType::String | AstroTokenType::Number | AstroTokenType::Identifier => {
                    sink.token(token.token_type().clone(), token.range());
                    pos += 1;
                }
                _ => {}
            }
        }
        
        sink.finish_node();
        pos
    }
}
