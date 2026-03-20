//! Astro 语法 token 定义

use std::fmt;

/// Astro 语法的 token 类型
#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    /// 文本内容
    Text(String),
    /// 变量插值开始: {{或 {{{
    InterpolationStart(InterpolationType),
    /// 变量插值结束: }}或 }}}
    InterpolationEnd(InterpolationType),
    /// 指令开始: {%
    DirectiveStart,
    /// 指令结束: %}
    DirectiveEnd,
    /// 组件标签开始: <
    TagStart,
    /// 组件标签结束: >
    TagEnd,
    /// 组件标签闭合: />
    TagClose,
    /// 组件标签结束: </
    TagEndStart,
    /// 等号
    Equals,
    /// 引号
    Quote(char),
    /// 标识符
    Identifier(String),
    /// 数字
    Number(String),
    /// 字符串
    String(String),
    /// 逗号
    Comma,
    /// 点
    Dot,
    /// 左括号
    LeftParen,
    /// 右括号
    RightParen,
    /// 左中括号
    LeftBracket,
    /// 右中括号
    RightBracket,
    /// 左大括号
    LeftBrace,
    /// 右大括号
    RightBrace,
    /// 加号
    Plus,
    /// 减号
    Minus,
    /// 乘号
    Multiply,
    /// 除号
    Divide,
    /// 模运算
    Modulo,
    /// 等于
    Equal,
    /// 不等于
    NotEqual,
    /// 大于
    GreaterThan,
    /// 小于
    LessThan,
    /// 大于等于
    GreaterThanOrEqual,
    /// 小于等于
    LessThanOrEqual,
    /// 逻辑与
    And,
    /// 逻辑或
    Or,
    /// 逻辑非
    Not,
    /// 冒号
    Colon,
    /// 分号
    Semicolon,
    /// 注释
    Comment(String),
    /// 空白字符
    Whitespace(String),
    /// 结束标记
    Eof,
}

/// 插值类型
#[derive(Debug, PartialEq, Clone)]
pub enum InterpolationType {
    /// 转义插值: {{}}
    Escaped,
    /// 非转义插值: {{{}}}
    Unescaped,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Text(text) => write!(f, "Text({:?})", text),
            Token::InterpolationStart(ty) => write!(f, "InterpolationStart({:?})", ty),
            Token::InterpolationEnd(ty) => write!(f, "InterpolationEnd({:?})", ty),
            Token::DirectiveStart => write!(f, "DirectiveStart"),
            Token::DirectiveEnd => write!(f, "DirectiveEnd"),
            Token::TagStart => write!(f, "TagStart"),
            Token::TagEnd => write!(f, "TagEnd"),
            Token::TagClose => write!(f, "TagClose"),
            Token::TagEndStart => write!(f, "TagEndStart"),
            Token::Equals => write!(f, "Equals"),
            Token::Quote(c) => write!(f, "Quote({})", c),
            Token::Identifier(id) => write!(f, "Identifier({})", id),
            Token::Number(num) => write!(f, "Number({})", num),
            Token::String(s) => write!(f, "String({:?})", s),
            Token::Comma => write!(f, "Comma"),
            Token::Dot => write!(f, "Dot"),
            Token::LeftParen => write!(f, "LeftParen"),
            Token::RightParen => write!(f, "RightParen"),
            Token::LeftBracket => write!(f, "LeftBracket"),
            Token::RightBracket => write!(f, "RightBracket"),
            Token::LeftBrace => write!(f, "LeftBrace"),
            Token::RightBrace => write!(f, "RightBrace"),
            Token::Plus => write!(f, "Plus"),
            Token::Minus => write!(f, "Minus"),
            Token::Multiply => write!(f, "Multiply"),
            Token::Divide => write!(f, "Divide"),
            Token::Modulo => write!(f, "Modulo"),
            Token::Equal => write!(f, "Equal"),
            Token::NotEqual => write!(f, "NotEqual"),
            Token::GreaterThan => write!(f, "GreaterThan"),
            Token::LessThan => write!(f, "LessThan"),
            Token::GreaterThanOrEqual => write!(f, "GreaterThanOrEqual"),
            Token::LessThanOrEqual => write!(f, "LessThanOrEqual"),
            Token::And => write!(f, "And"),
            Token::Or => write!(f, "Or"),
            Token::Not => write!(f, "Not"),
            Token::Colon => write!(f, "Colon"),
            Token::Semicolon => write!(f, "Semicolon"),
            Token::Comment(comment) => write!(f, "Comment({:?})", comment),
            Token::Whitespace(ws) => write!(f, "Whitespace({:?})", ws),
            Token::Eof => write!(f, "Eof"),
        }
    }
}