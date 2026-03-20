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
        let result = template.to_string();
        let mut new_result = String::new();
        let mut i = 0;
        let len = result.len();
        
        while i < len {
            // 查找循环开始标签
            let loop_start = "{% for ";
            if result[i..].starts_with(loop_start) {
                let start = i;
                let loop_start_len = loop_start.len();
                
                // 查找循环开始标签的结束
                let end_tag = "%}";
                if let Some(end_idx) = result[start + loop_start_len..].find(end_tag) {
                    let loop_end = start + loop_start_len + end_idx + end_tag.len();
                    let loop_expr = result[start + loop_start_len..start + loop_start_len + end_idx].trim();
                    
                    // 解析循环表达式
                    if let Some(in_pos) = loop_expr.find(" in ") {
                        let var_name = loop_expr[..in_pos].trim();
                        let array_name = loop_expr[in_pos + 4..].trim();
                        
                        // 查找循环结束标签
                        let endfor_tag = "{% endfor %}";
                        if let Some(endfor_idx) = result[loop_end..].find(endfor_tag) {
                            let endfor_end = loop_end + endfor_idx + endfor_tag.len();
                            let block_content = &result[loop_end..loop_end + endfor_idx];
                            
                            // 处理循环
                            if let Some(array) = context.get(array_name) {
                                if let Some(array) = array.as_array() {
                                    for item in array {
                                        let mut item_context = context.clone();
                                        item_context.insert(var_name.to_string(), item.clone());
                                        
                                        // 处理变量插值
                                        let processed_item = self.process_variables(block_content, &item_context);
                                        new_result.push_str(&processed_item);
                                    }
                                }
                            }
                            
                            i = endfor_end;
                            continue;
                        }
                    }
                }
            }
            
            // 处理条件语句
            let if_start = "{% if ";
            if result[i..].starts_with(if_start) {
                let start = i;
                let if_start_len = if_start.len();
                
                // 查找条件开始标签的结束
                let end_tag = "%}";
                if let Some(end_idx) = result[start + if_start_len..].find(end_tag) {
                    let if_end = start + if_start_len + end_idx + end_tag.len();
                    let condition = result[start + if_start_len..start + if_start_len + end_idx].trim();
                    
                    // 查找条件结束标签
                    let endif_tag = "{% endif %}";
                    if let Some(endif_idx) = result[if_end..].find(endif_tag) {
                        let endif_end = if_end + endif_idx + endif_tag.len();
                        let block_content = &result[if_end..if_end + endif_idx];
                        
                        // 处理条件
                        if context.contains_key(condition) {
                            // 处理变量插值
                            let processed_content = self.process_variables(block_content, context);
                            new_result.push_str(&processed_content);
                        }
                        
                        i = endif_end;
                        continue;
                    }
                }
            }
            
            // 处理变量插值
            if result[i..].starts_with("{{{") {
                let start = i;
                if let Some(end_idx) = result[start..].find("}}}") {
                    let end = start + end_idx + 3;
                    let var_name = result[start + 3..start + end_idx].trim();
                    
                    if let Some(value) = context.get(var_name) {
                        let value_str = self.value_to_string(value);
                        new_result.push_str(&value_str);
                        i = end;
                        continue;
                    }
                }
            } else if result[i..].starts_with("{{") {
                let start = i;
                if let Some(end_idx) = result[start..].find("}}") {
                    let end = start + end_idx + 2;
                    let var_name = result[start + 2..start + end_idx].trim();
                    
                    if let Some(value) = context.get(var_name) {
                        let value_str = self.value_to_string(value);
                        let escaped = self.escape_html(&value_str);
                        new_result.push_str(&escaped);
                        i = end;
                        continue;
                    }
                }
            }
            
            // 复制当前字符
            if i < len {
                new_result.push(result.chars().nth(i).unwrap());
                i += 1;
            } else {
                break;
            }
        }
        
        // 清理剩余的 %}
        new_result = new_result.replace(" %}", "");
        new_result = new_result.replace("%}", "");
        
        new_result
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
                    let var_name = result[start + 3..end - 3].trim();
                    if let Some(value) = context.get(var_name) {
                        let value_str = self.value_to_string(value);
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
                    let var_name = result[start + 2..end - 2].trim();
                    if let Some(value) = context.get(var_name) {
                        let value_str = self.value_to_string(value);
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

    /// 将 serde_json::Value 转换为字符串，处理字符串值的引号
    fn value_to_string(&self, value: &serde_json::Value) -> String {
        match value {
            serde_json::Value::String(s) => s.clone(),
            _ => value.to_string(),
        }
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
                    let condition = result[start + 6..condition_end - 3].trim();
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
                        let block_content = result[block_start..endif_pos].to_string();
                        
                        // 简单的条件评估（仅支持变量存在性检查）
                        let condition_met = context.contains_key(condition);
                        
                        if condition_met {
                            // 保留条件块内容
                            result.replace_range(start..block_end, &block_content);
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
        
        // 清理剩余的 %}
        result = result.replace(" %}", "");
        
        result
    }

    /// 处理循环语句
    fn process_loops(&self, content: &str, context: &Context) -> String {
        let template = content.to_string();
        let mut new_result = String::new();
        let mut i = 0;
        
        while i < template.len() {
            // 查找循环开始标签
            let loop_start = "{% for ";
            if template[i..].starts_with(loop_start) {
                let start = i;
                let loop_start_len = loop_start.len();
                
                // 查找循环开始标签的结束
                if let Some(end_idx) = template[start + loop_start_len..].find("%}") {
                    let loop_end = start + loop_start_len + end_idx + 2; // +2 for "%}"
                    let loop_expr = &template[start + loop_start_len..start + loop_start_len + end_idx].trim();
                    
                    // 解析循环表达式：变量名 in 数组名
                    if let Some(in_pos) = loop_expr.find(" in ") {
                        let var_name = loop_expr[..in_pos].trim();
                        let array_name = loop_expr[in_pos + 4..].trim();
                        
                        // 寻找对应的 {% endfor %}
                        let end_tag = "{% endfor %}";
                        if let Some(endfor_idx) = template[loop_end..].find(end_tag) {
                            let endfor_start = loop_end + endfor_idx;
                            let endfor_end = endfor_start + end_tag.len();
                            let block_content = &template[loop_end..endfor_start];
                            
                            // 处理循环
                            if let Some(array) = context.get(array_name) {
                                if let Some(array) = array.as_array() {
                                    for item in array {
                                        let mut item_context = context.clone();
                                        item_context.insert(var_name.to_string(), item.clone());
                                        
                                        // 递归处理循环体内的变量
                                        let processed_item = self.process_variables(block_content, &item_context);
                                        new_result.push_str(&processed_item);
                                    }
                                }
                            }
                            
                            i = endfor_end;
                            continue;
                        }
                    }
                }
            }
            
            // 复制当前字符
            if i < template.len() {
                new_result.push(template.chars().nth(i).unwrap());
                i += 1;
            } else {
                break;
            }
        }
        
        new_result
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
