//! Astro 语法词法分析器

use std::{iter::Peekable, str::Chars};

use super::tokens::{InterpolationType, Token};

/// 词法分析器
pub struct Lexer {
    /// 输入字符迭代器
    chars: Peekable<Chars<'static>>,
    /// 当前行号
    line: usize,
    /// 当前列号
    column: usize,
}

impl Lexer {
    /// 创建新的词法分析器
    pub fn new(input: &'static str) -> Self {
        Self { chars: input.chars().peekable(), line: 1, column: 1 }
    }

    /// 词法分析，生成下一个 token
    pub fn next_token(&mut self) -> Token {
        // 跳过空白字符
        self.skip_whitespace();

        // 检查是否到达文件末尾
        if let None = self.chars.peek() {
            return Token::Eof;
        }

        // 检查特殊字符序列
        let current_char = self.chars.peek().unwrap();

        match current_char {
            // 处理注释: {# #}
            '{' => {
                self.chars.next(); // 消费 '{' 字符
                // 检查是否是注释开始: {#
                if let Some(&'#') = self.chars.peek() {
                    self.chars.next(); // 消费 '#' 字符
                    
                    // 解析注释内容
                    let mut comment_content = String::new();
                    while let Some(&c) = self.chars.peek() {
                        if c == '#' && self.chars.nth(1) == Some('}') {
                            self.chars.next(); // 消费 '#' 字符
                            self.chars.next(); // 消费 '}' 字符
                            break;
                        }
                        comment_content.push(c);
                        self.chars.next();
                    }
                    
                    Token::Comment(comment_content)
                }
                // 检查是否是指令开始: {%
                else if let Some(&'%') = self.chars.peek() {
                    self.chars.next(); // 消费 '%' 字符
                    Token::DirectiveStart
                }
                else {
                    // 处理插值开始
                    let mut count = 1;
                    while self.chars.peek() == Some(&'{') {
                        self.chars.next();
                        count += 1;
                    }

                    match count {
                        2 => Token::InterpolationStart(InterpolationType::Escaped),
                        3 => Token::InterpolationStart(InterpolationType::Unescaped),
                        _ => {
                            // 不是有效的插值开始，返回普通文本
                            let mut text = String::new();
                            for _ in 0..count {
                                text.push('{');
                            }
                            Token::Text(text)
                        }
                    }
                }
            }
            // 处理插值结束
            '}' => {
                let mut count = 1; // 已经看到一个 '}'
                self.chars.next(); // 消费第一个 '}'
                
                while self.chars.peek() == Some(&'}') {
                    self.chars.next();
                    count += 1;
                }

                match count {
                    2 => Token::InterpolationEnd(InterpolationType::Escaped),
                    3 => Token::InterpolationEnd(InterpolationType::Unescaped),
                    _ => {
                        // 不是有效的插值结束，返回普通文本
                        let mut text = String::new();
                        for _ in 0..count {
                            text.push('}');
                        }
                        Token::Text(text)
                    }
                }
            }
            // 处理指令结束: %}
            '%' => {
                if let Some(&'}') = self.chars.peek() {
                    self.chars.next();
                    self.chars.next();
                    Token::DirectiveEnd
                }
                else {
                    self.chars.next();
                    Token::Text("%".to_string())
                }
            }
            // 处理标签开始: <
            '<' => {
                if let Some(&'/') = self.chars.peek() {
                    // 处理标签结束开始: </
                    self.chars.next();
                    self.chars.next();
                    Token::TagEndStart
                }
                else if let Some(&'!') = self.chars.peek() {
                    // 处理 HTML 注释: <!-- -->
                    self.chars.next(); // 消费 '!'
                    if let Some(&'-') = self.chars.peek() && let Some('-') = self.chars.nth(1) {
                        self.chars.next(); // 消费 '-'
                        self.chars.next(); // 消费 '-'
                        
                        let mut comment_content = String::new();
                        while let Some(&c) = self.chars.peek() {
                            if c == '-' && self.chars.nth(1) == Some('-') && self.chars.nth(2) == Some('>') {
                                self.chars.next(); // 消费 '-'
                                self.chars.next(); // 消费 '-'
                                self.chars.next(); // 消费 '>'
                                break;
                            }
                            comment_content.push(c);
                            self.chars.next();
                        }
                        
                        Token::Comment(comment_content)
                    }
                    else {
                        self.chars.next();
                        Token::Text("<!".to_string())
                    }
                }
                else {
                    self.chars.next();
                    Token::TagStart
                }
            }
            // 处理标签结束: >
            '>' => {
                self.chars.next();
                Token::TagEnd
            }
            // 处理标签闭合: />
            '/' => {
                if let Some(&'>') = self.chars.peek() {
                    self.chars.next();
                    self.chars.next();
                    Token::TagClose
                }
                else {
                    self.chars.next();
                    Token::Text("/".to_string())
                }
            }
            // 处理等号
            '=' => {
                self.chars.next();
                Token::Equals
            }
            // 处理引号和字符串
            '"' | '\'' => {
                let quote_char = *current_char;
                self.chars.next();
                
                // 解析字符串内容
                let mut string_content = String::new();
                while let Some(&c) = self.chars.peek() {
                    if c == quote_char {
                        self.chars.next(); // 消费引号
                        break;
                    }
                    string_content.push(c);
                    self.chars.next();
                }
                
                Token::String(string_content)
            }
            // 处理数字
            c if c.is_digit(10) => {
                let mut num = String::new();
                while let Some(&c) = self.chars.peek() {
                    if c.is_digit(10) || c == '.' {
                        num.push(c);
                        self.chars.next();
                    }
                    else {
                        break;
                    }
                }
                Token::Number(num)
            }
            // 处理标识符
            c if c.is_alphabetic() || *c == '_' => {
                let mut id = String::new();
                while let Some(&c) = self.chars.peek() {
                    if c.is_alphanumeric() || c == '_' || c == '-' || c == '.' || c == ':' {
                        id.push(c);
                        self.chars.next();
                    }
                    else {
                        break;
                    }
                }
                Token::Identifier(id)
            }
            // 处理逗号
            ',' => {
                self.chars.next();
                Token::Comma
            }
            // 处理点
            '.' => {
                self.chars.next();
                Token::Dot
            }
            // 处理左括号
            '(' => {
                self.chars.next();
                Token::LeftParen
            }
            // 处理右括号
            ')' => {
                self.chars.next();
                Token::RightParen
            }
            // 处理左中括号
            '[' => {
                self.chars.next();
                Token::LeftBracket
            }
            // 处理右中括号
            ']' => {
                self.chars.next();
                Token::RightBracket
            }

            // 处理加号
            '+' => {
                self.chars.next();
                Token::Plus
            }
            // 处理减号
            '-' => {
                self.chars.next();
                Token::Minus
            }
            // 处理乘号
            '*' => {
                self.chars.next();
                Token::Multiply
            }
            // 处理除号
            '/' => {
                self.chars.next();
                Token::Divide
            }
            // 处理模运算
            '%' => {
                self.chars.next();
                Token::Modulo
            }
            // 处理等于
            '=' => {
                if let Some(&'=') = self.chars.peek() {
                    self.chars.next();
                    self.chars.next();
                    Token::Equal
                }
                else {
                    self.chars.next();
                    Token::Equals
                }
            }
            // 处理不等于
            '!' => {
                if let Some(&'=') = self.chars.peek() {
                    self.chars.next();
                    self.chars.next();
                    Token::NotEqual
                }
                else {
                    self.chars.next();
                    Token::Not
                }
            }
            // 处理大于
            '>' => {
                if let Some(&'=') = self.chars.peek() {
                    self.chars.next();
                    self.chars.next();
                    Token::GreaterThanOrEqual
                }
                else {
                    self.chars.next();
                    Token::GreaterThan
                }
            }
            // 处理小于
            '<' => {
                if let Some(&'=') = self.chars.peek() {
                    self.chars.next();
                    self.chars.next();
                    Token::LessThanOrEqual
                }
                else {
                    self.chars.next();
                    Token::LessThan
                }
            }
            // 处理逻辑与
            '&' => {
                if let Some(&'&') = self.chars.peek() {
                    self.chars.next();
                    self.chars.next();
                    Token::And
                }
                else {
                    self.chars.next();
                    Token::Text("&".to_string())
                }
            }
            // 处理逻辑或
            '|' => {
                if let Some(&'|') = self.chars.peek() {
                    self.chars.next();
                    self.chars.next();
                    Token::Or
                }
                else {
                    self.chars.next();
                    Token::Text("|".to_string())
                }
            }
            // 处理冒号
            ':' => {
                self.chars.next();
                Token::Colon
            }
            // 处理分号
            ';' => {
                self.chars.next();
                Token::Semicolon
            }
            // 处理空白字符
            c if c.is_whitespace() => {
                let mut whitespace = String::new();
                while let Some(&c) = self.chars.peek() {
                    if c.is_whitespace() {
                        whitespace.push(c);
                        self.chars.next();
                    }
                    else {
                        break;
                    }
                }
                Token::Whitespace(whitespace)
            }
            // 处理其他字符，作为文本
            _ => {
                let mut text = String::new();
                while let Some(&c) = self.chars.peek() {
                    if c == '{' || c == '}' || c == '%' || c == '<' || c == '>' || c.is_whitespace() {
                        break;
                    }
                    text.push(c);
                    self.chars.next();
                }
                if !text.is_empty() {
                    Token::Text(text)
                } else {
                    // 消费当前字符并继续
                    self.chars.next();
                    self.next_token()
                }
            }
        }
    }

    /// 跳过空白字符
    fn skip_whitespace(&mut self) {
        while let Some(&c) = self.chars.peek() {
            if c.is_whitespace() {
                if c == '\n' {
                    self.line += 1;
                    self.column = 1;
                }
                else {
                    self.column += 1;
                }
                self.chars.next();
            }
            else {
                break;
            }
        }
    }
}
