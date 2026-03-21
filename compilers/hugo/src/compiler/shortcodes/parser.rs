//! 短代码解析器
//!
//! 负责解析 Hugo 风格的短代码语法。

use crate::compiler::shortcodes::types::{Shortcode, ShortcodeError, ShortcodeParams, ShortcodeResult, ShortcodeType};
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    /// 匹配短代码开始标签的正则表达式
    static ref SHORTCODE_START_RE: Regex = Regex::new(
        r"\{\{(?P<type>[<%])\s*(?P<name>\w+)(?P<params>[^>%]*?)(?P<self_close>\s*/)?\s*(?P<end>[>%])\}\}"
    ).unwrap();

    /// 匹配短代码结束标签的正则表达式
    static ref SHORTCODE_END_RE: Regex = Regex::new(
        r"\{\{(?P<type>[<%])\s*/\s*(?P<name>\w+)\s*(?P<end>[>%])\}\}"
    ).unwrap();

    /// 匹配参数的正则表达式
    static ref PARAM_RE: Regex = Regex::new(
        r#"(?P<key>\w+)\s*=\s*(?:"(?P<value>[^"]*)"|'(?P<value2>[^']*)'|(?P<value3>[^\s]+))|(?P<pos>"(?P<pos_val>[^"]*)"|'(?P<pos_val2>[^']*)'|[^\s]+)"#
    ).unwrap();
}

/// 短代码解析器
pub struct ShortcodeParser;

impl ShortcodeParser {
    /// 创建新的短代码解析器
    pub fn new() -> Self {
        Self
    }

    /// 解析整个文本中的所有短代码
    ///
    /// # Arguments
    ///
    /// * `text` - 要解析的文本
    ///
    /// # Returns
    ///
    /// 解析结果包含处理后的文本和可能的错误
    pub fn parse_text(&self, text: &str) -> ShortcodeResult<Vec<TextFragment>> {
        let mut fragments = Vec::new();
        let mut remaining = text;

        while !remaining.is_empty() {
            if let Some(captures) = SHORTCODE_START_RE.captures(remaining) {
                let start = captures.get(0).unwrap().start();
                let end = captures.get(0).unwrap().end();

                if start > 0 {
                    fragments.push(TextFragment::Text(remaining[..start].to_string()));
                }

                let shortcode_type = match &captures["type"] {
                    "<" => ShortcodeType::Raw,
                    "%" => ShortcodeType::Markdown,
                    _ => unreachable!(),
                };

                let name = captures["name"].to_string();
                let params_str = captures.name("params").map(|m| m.as_str()).unwrap_or("");
                let self_close = captures.name("self_close").is_some();

                let params = Self::parse_params(params_str)?;

                if self_close {
                    fragments.push(TextFragment::Shortcode(Shortcode { name, shortcode_type, params, inner: None }));
                    remaining = &remaining[end..];
                }
                else {
                    let start_type = &captures["type"];
                    let remaining_text = &remaining[end..];
                    
                    let mut found_end = None;
                    for end_capture in SHORTCODE_END_RE.captures_iter(remaining_text) {
                        let end_type = &end_capture["type"];
                        let end_name = &end_capture["name"];
                        
                        if end_type == start_type && end_name == name {
                            let match_start = end_capture.get(0).unwrap().start();
                            let match_end = end_capture.get(0).unwrap().end();
                            found_end = Some((match_start, match_end));
                            break;
                        }
                    }
                    
                    if let Some((end_idx, end_match_len)) = found_end {
                        let inner = remaining[end..end + end_idx].to_string();
                        fragments.push(TextFragment::Shortcode(Shortcode { name, shortcode_type, params, inner: Some(inner) }));
                        remaining = &remaining[end + end_idx + end_match_len..];
                    }
                    else {
                        fragments.push(TextFragment::Shortcode(Shortcode { name, shortcode_type, params, inner: None }));
                        remaining = &remaining[end..];
                    }
                }
            }
            else {
                fragments.push(TextFragment::Text(remaining.to_string()));
                break;
            }
        }

        Ok(fragments)
    }

    /// 解析参数字符串
    fn parse_params(params_str: &str) -> ShortcodeResult<ShortcodeParams> {
        let mut params = ShortcodeParams::new();

        for captures in PARAM_RE.captures_iter(params_str) {
            if let Some(key) = captures.name("key") {
                let value = captures
                    .name("value")
                    .or_else(|| captures.name("value2"))
                    .or_else(|| captures.name("value3"))
                    .map(|m| m.as_str().to_string())
                    .unwrap_or_default();
                params.add_named(key.as_str().to_string(), value);
            }
            else if let Some(pos_val) =
                captures.name("pos_val").or_else(|| captures.name("pos_val2")).or_else(|| captures.name("pos"))
            {
                params.add_positional(pos_val.as_str().to_string());
            }
        }

        Ok(params)
    }
}

impl Default for ShortcodeParser {
    fn default() -> Self {
        Self::new()
    }
}

/// 文本片段，可能是普通文本或短代码
#[derive(Debug, Clone)]
pub enum TextFragment {
    /// 普通文本
    Text(String),
    /// 短代码
    Shortcode(Shortcode),
}
