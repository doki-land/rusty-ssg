//! Astro 语法词法分析器（基于 oaks 框架）

use oak_core::{language::Language, lexer::{Lexer, LexOutput, Tokens, Token, LexerCache}};
use oak_core::source::Source;
use oak_core::TextEdit;

/// Astro 语言定义
#[derive(Debug, Default)]
pub struct AstroLanguage;

impl Language for AstroLanguage {
    type TokenType = AstroTokenType;
    type ElementType = AstroElementType;
}

/// Astro 令牌类型
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AstroTokenType {
    // 文本
    Text,
    // 标识符
    Identifier,
    // 数字
    Number,
    // 字符串
    String,
    // 插值开始
    InterpolationStart,
    // 无转义插值开始
    UnescapedInterpolationStart,
    // 插值结束
    InterpolationEnd,
    // 无转义插值结束
    UnescapedInterpolationEnd,
    // 指令开始
    DirectiveStart,
    // 指令结束
    DirectiveEnd,
    // 标签开始
    TagStart,
    // 标签结束开始
    TagEndStart,
    // 标签结束
    TagEnd,
    // 标签闭合
    TagClose,
    // 等号
    Equals,
    // 引号
    Quote(char),
    // 逗号
    Comma,
    // 点
    Dot,
    // 左括号
    LeftParen,
    // 右括号
    RightParen,
    // 左中括号
    LeftBracket,
    // 右中括号
    RightBracket,
    // 左大括号
    LeftBrace,
    // 右大括号
    RightBrace,
    // 加号
    Plus,
    // 减号
    Minus,
    // 乘号
    Multiply,
    // 除号
    Divide,
    // 冒号
    Colon,
    // 分号
    Semicolon,
    // 注释
    Comment,
    // 组件脚本开始
    ScriptStart,
    // 组件脚本结束
    ScriptEnd,
    // 结束符
    Eof,
}

/// Astro 元素类型
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AstroElementType {
    // 根元素
    Root,
    // 文本
    Text,
    // 插值
    Interpolation,
    // 无转义插值
    UnescapedInterpolation,
    // 指令
    Directive,
    // 组件
    Component,
    // 组件脚本
    Script,
    // 标签
    Tag,
    // 属性
    Attribute,
    // 插槽
    Slot,
    // 命名插槽
    NamedSlot,
}

/// Astro 词法分析器
pub struct AstroLexer;

impl Lexer<AstroLanguage> for AstroLexer {
    fn lex<'a, S: Source + ?Sized>(&self, text: &S, edits: &[TextEdit], cache: &'a mut impl LexerCache<AstroLanguage>) -> LexOutput<AstroLanguage> {
        let mut tokens = Vec::new();
        let mut chars = text.as_str().chars().peekable();
        let mut position = 0;
        
        while let Some(&c) = chars.peek() {
            let start = position;
            
            match c {
                // 处理组件脚本开始和结束
                '-' => {
                    if chars.clone().take(3).collect::<String>() == "---" {
                        for _ in 0..3 {
                            chars.next();
                            position += 1;
                        }
                        tokens.push(Token::new(AstroTokenType::ScriptStart, start..position));
                        continue;
                    }
                }
                // 处理 '{' 字符，需要区分插值开始和指令开始
                '{' => {
                    // 检查是否是指令开始: {%
                    if let Some(&'%') = chars.clone().nth(1) {
                        chars.next();
                        chars.next();
                        position += 2;
                        tokens.push(Token::new(AstroTokenType::DirectiveStart, start..position));
                        continue;
                    }
                    else {
                        // 处理插值开始
                        let mut count = 1;
                        chars.next();
                        position += 1;
                        while chars.peek() == Some(&'{') {
                            chars.next();
                            position += 1;
                            count += 1;
                        }
                        
                        match count {
                            2 => tokens.push(Token::new(AstroTokenType::InterpolationStart, start..position)),
                            3 => tokens.push(Token::new(AstroTokenType::UnescapedInterpolationStart, start..position)),
                            _ => {
                                // 不是有效的插值开始，返回普通文本
                                let mut text = String::new();
                                for _ in 0..count {
                                    text.push('{');
                                }
                                tokens.push(Token::new(AstroTokenType::Text, start..position));
                            }
                        }
                        continue;
                    }
                }
                // 处理插值结束
                '}' => {
                    let mut count = 0;
                    let mut temp_chars = chars.clone();
                    while temp_chars.peek() == Some(&'}') {
                        temp_chars.next();
                        count += 1;
                    }
                    
                    match count {
                        2 => {
                            for _ in 0..2 {
                                chars.next();
                                position += 1;
                            }
                            tokens.push(Token::new(AstroTokenType::InterpolationEnd, start..position));
                        }
                        3 => {
                            for _ in 0..3 {
                                chars.next();
                                position += 1;
                            }
                            tokens.push(Token::new(AstroTokenType::UnescapedInterpolationEnd, start..position));
                        }
                        _ => {
                            // 不是有效的插值结束，返回普通文本
                            for _ in 0..count {
                                chars.next();
                                position += 1;
                            }
                            tokens.push(Token::new(AstroTokenType::Text, start..position));
                        }
                    }
                    continue;
                }
                // 处理指令结束: %}
                '%' => {
                    if let Some(&'}') = chars.clone().nth(1) {
                        chars.next();
                        chars.next();
                        position += 2;
                        tokens.push(Token::new(AstroTokenType::DirectiveEnd, start..position));
                        continue;
                    }
                }
                // 处理标签开始: <
                '<' => {
                    if let Some(&'/') = chars.clone().nth(1) {
                        // 处理标签结束开始: </
                        chars.next();
                        chars.next();
                        position += 2;
                        tokens.push(Token::new(AstroTokenType::TagEndStart, start..position));
                    }
                    else {
                        chars.next();
                        position += 1;
                        tokens.push(Token::new(AstroTokenType::TagStart, start..position));
                    }
                    continue;
                }
                // 处理标签结束: >
                '>' => {
                    chars.next();
                    position += 1;
                    tokens.push(Token::new(AstroTokenType::TagEnd, start..position));
                    continue;
                }
                // 处理标签闭合: />
                '/' => {
                    if let Some(&'>') = chars.clone().nth(1) {
                        chars.next();
                        chars.next();
                        position += 2;
                        tokens.push(Token::new(AstroTokenType::TagClose, start..position));
                        continue;
                    }
                }
                // 处理等号
                '=' => {
                    chars.next();
                    position += 1;
                    tokens.push(Token::new(AstroTokenType::Equals, start..position));
                    continue;
                }
                // 处理引号
                '"' | '\'' => {
                    let quote_char = c;
                    chars.next();
                    position += 1;
                    tokens.push(Token::new(AstroTokenType::Quote(quote_char), start..position));
                    continue;
                }
                // 处理数字
                c if c.is_digit(10) => {
                    let mut num = String::new();
                    while let Some(&c) = chars.peek() {
                        if c.is_digit(10) || c == '.' {
                            num.push(c);
                            chars.next();
                            position += 1;
                        }
                        else {
                            break;
                        }
                    }
                    tokens.push(Token::new(AstroTokenType::Number, start..position));
                    continue;
                }
                // 处理标识符
                c if c.is_alphabetic() || c == '_' => {
                    let mut id = String::new();
                    while let Some(&c) = chars.peek() {
                        if c.is_alphanumeric() || c == '_' || c == '-' || c == ':' {
                            id.push(c);
                            chars.next();
                            position += 1;
                        }
                        else {
                            break;
                        }
                    }
                    tokens.push(Token::new(AstroTokenType::Identifier, start..position));
                    continue;
                }
                // 处理逗号
                ',' => {
                    chars.next();
                    position += 1;
                    tokens.push(Token::new(AstroTokenType::Comma, start..position));
                    continue;
                }
                // 处理点
                '.' => {
                    chars.next();
                    position += 1;
                    tokens.push(Token::new(AstroTokenType::Dot, start..position));
                    continue;
                }
                // 处理左括号
                '(' => {
                    chars.next();
                    position += 1;
                    tokens.push(Token::new(AstroTokenType::LeftParen, start..position));
                    continue;
                }
                // 处理右括号
                ')' => {
                    chars.next();
                    position += 1;
                    tokens.push(Token::new(AstroTokenType::RightParen,