//! HTML 渲染器

use std::collections::HashMap;

/// 模板上下文，用于存储变量和数据
pub type Context = HashMap<String, serde_json::Value>;

/// HTML 渲染器
pub struct HtmlRenderer {
    /// 渲染配置
    config: HashMap<String, String>,
}

impl HtmlRenderer {
    /// 创建新的 HTML 渲染器
    pub fn new() -> Self {
        Self { config: HashMap::new() }
    }

    /// 渲染 Markdown 内容为 HTML
    pub fn render(&self, content: &str) -> String {
        // 简单的渲染实现
        content.to_string()
    }

    /// 渲染 Astro 模板内容
    /// 
    /// # 参数
    /// - `template`: Astro 模板内容
    /// - `context`: 模板上下文，包含变量和数据
    /// 
    /// # 返回值
    /// 渲染后的 HTML 内容
    pub fn render_astro(&self, template: &str, context: &Context) -> String {
        let mut result = template.to_string();
        
        // 处理变量插值: {{{变量名}}} 或 {{变量名}}
        result = self.process_variables(&result, context);
        
        // 处理条件语句: {% if 条件 %}{% endif %}
        result = self.process_conditions(&result, context);
        
        // 处理循环语句: {% for 变量 in 数组 %}{% endfor %}
        result = self.process_loops(&result, context);
        
        result
    }

    /// 处理变量插值
    fn process_variables(&self, content: &str, context: &Context) -> String {
        let mut result = content.to_string();
        
        // 匹配 {{{变量名}}} 格式（不转义）
        let mut i = 0;
        while i < result.len() {
            if result[i..].starts_with("{{{") {
                let start = i;
                let end = result[i..].find("}}}").map(|pos| i + pos + 3);
                
                if let Some(end) = end {
                    let var_name = &result[start + 3..end - 3].trim();
                    if let Some(value) = context.get(var_name) {
                        let value_str = value.to_string();
                        result.replace_range(start..end, &value_str);
                        i = start + value_str.len();
                        continue;
                    }
                }
            }
            
            // 匹配 {{变量名}} 格式（转义）
            if result[i..].starts_with("{{") && !result[i..].starts_with("{{{") {
                let start = i;
                let end = result[i..].find("}}").map(|pos| i + pos + 2);
                
                if let Some(end) = end {
                    let var_name = &result[start + 2..end - 2].trim();
                    if let Some(value) = context.get(var_name) {
                        let value_str = value.to_string();
                        // 简单的 HTML 转义
                        let escaped = self.escape_html(&value_str);
                        result.replace_range(start..end, &escaped);
                        i = start + escaped.len();
                        continue;
                    }
                }
            }
            
            i += 1;
        }
        
        result
    }

    /// 处理条件语句
    fn process_conditions(&self, content: &str, context: &Context) -> String {
        let mut result = content.to_string();
        
        // 匹配 {% if 条件 %}{% endif %}
        let mut i = 0;
        while i < result.len() {
            if result[i..].starts_with("{% if ") {
                let start = i;
                let condition_end = result[i..].find(" %}").map(|pos| i + pos + 3);
                
                if let Some(condition_end) = condition_end {
                    let condition = &result[start + 6..condition_end - 3].trim();
                    let block_start = condition_end;
                    
                    // 寻找对应的 {% endif %}
                    let mut endif_pos = block_start;
                    let mut depth = 1;
                    while endif_pos < result.len() {
                        if result[endif_pos..].starts_with("{% if ") {
                            depth += 1;
                        } else if result[endif_pos..].starts_with("{% endif %}") {
                            depth -= 1;
                            if depth == 0 {
                                break;
                            }
                        }
                        endif_pos += 1;
                    }
                    
                    if depth == 0 && result[endif_pos..].starts_with("{% endif %}") {
                        let block_end = endif_pos + 8; // {% endif %} 的长度
                        let block_content = &result[block_start..endif_pos];
                        
                        // 简单的条件评估（仅支持变量存在性检查）
                        let condition_met = context.contains_key(condition);
                        
                        if condition_met {
                            // 保留条件块内容
                            result.replace_range(start..block_end, block_content);
                            i = start + block_content.len();
                        } else {
                            // 移除条件块
                            result.replace_range(start..block_end, "");
                            i = start;
                        }
                        continue;
                    }
                }
            }
            
            i += 1;
        }
        
        result
    }

    /// 处理循环语句
    fn process_loops(&self, content: &str, context: &Context) -> String {
        let mut result = content.to_string();
        
        // 匹配 {% for 变量 in 数组 %}{% endfor %}
        let mut i = 0;
        while i < result.len() {
            if result[i..].starts_with("{% for ") {
                let start = i;
                let loop_end = result[i..].find(" %}").map(|pos| i + pos + 3);
                
                if let Some(loop_end) = loop_end {
                    let loop_expr = &result[start + 7..loop_end - 3].trim();
                    let block_start = loop_end;
                    
                    // 解析循环表达式：变量名 in 数组名
                    if let Some(in_pos) = loop_expr.find(" in ") {
                        let var_name = loop_expr[..in_pos].trim();
                        let array_name = loop_expr[in_pos + 4..].trim();
                        
                        // 寻找对应的 {% endfor %}
                        let mut endfor_pos = block_start;
                        let mut depth = 1;
                        while endfor_pos < result.len() {
                            if result[endfor_pos..].starts_with("{% for ") {
                                depth += 1;
                            } else if result[endfor_pos..].starts_with("{% endfor %}") {
                                depth -= 1;
                                if depth == 0 {
                                    break;
                                }
                            }
                            endfor_pos += 1;
                        }
                        
                        if depth == 0 && result[endfor_pos..].starts_with("{% endfor %}") {
                            let block_end = endfor_pos + 9; // {% endfor %} 的长度
                            let block_content = &result[block_start..endfor_pos];
                            
                            // 处理循环
                            if let Some(array) = context.get(array_name) {
                                if let Some(array) = array.as_array() {
                                    let mut loop_result = String::new();
                                    
                                    for item in array {
                                        let mut item_context = context.clone();
                                        item_context.insert(var_name.to_string(), item.clone());
                                        
                                        // 递归处理循环体内的变量
                                        let processed_item = self.process_variables(block_content, &item_context);
                                        loop_result.push_str(&processed_item);
                                    }
                                    
                                    result.replace_range(start..block_end, &loop_result);
                                    i = start + loop_result.len();
                                } else {
                                    // 不是数组，移除循环块
                                    result.replace_range(start..block_end, "");
                                    i = start;
                                }
                            } else {
                                // 数组不存在，移除循环块
                                result.replace_range(start..block_end, "");
                                i = start;
                            }
                            continue;
                        }
                    }
                }
            }
            
            i += 1;
        }
        
        result
    }

    /// HTML 转义
    fn escape_html(&self, content: &str) -> String {
        content
            .replace("&", "&amp;")
            .replace("<", "&lt;")
            .replace(">", "&gt;")
            .replace("\"", "&quot;")
            .replace("'", "&#39;")
    }
}

impl Default for HtmlRenderer {
    fn default() -> Self {
        Self::new()
    }
}
