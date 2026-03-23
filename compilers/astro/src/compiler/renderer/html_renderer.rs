//! HTML 渲染器

use crate::compiler::{ComponentRegistry, parser::ast::{AstNode, InterpolationType, DirectiveType}};
use std::collections::HashMap;

/// 模板上下文，用于存储变量和数据
pub type Context = HashMap<String, serde_json::Value>;

/// HTML 渲染器
pub struct HtmlRenderer {
    /// 渲染配置
    config: HashMap<String, String>,
    /// 组件注册表
    component_registry: ComponentRegistry,
}

impl HtmlRenderer {
    /// 创建新的 HTML 渲染器
    pub fn new() -> Self {
        Self { config: HashMap::new(), component_registry: ComponentRegistry::new() }
    }

    /// 设置组件注册表
    pub fn set_component_registry(&mut self, registry: ComponentRegistry) {
        self.component_registry = registry;
    }

    /// 获取组件注册表
    pub fn component_registry(&self) -> &ComponentRegistry {
        &self.component_registry
    }

    /// 渲染 Markdown 内容为 HTML
    pub fn render(&self, content: &str) -> String {
        // 执行插件预处理
        let preprocessed_content = self.execute_plugins(content);

        // 简单的 Markdown 渲染实现
        let mut html_output = preprocessed_content;

        // 执行插件后处理
        self.execute_plugins(&html_output)
    }

    /// 渲染 MDX 内容为 HTML
    ///
    /// # 参数
    /// - `content`: MDX 内容
    ///
    /// # 返回值
    /// 渲染后的 HTML 内容
    pub fn render_mdx(&self, content: &str) -> String {
        // 简单的 MDX 处理实现
        // 1. 提取并处理组件
        // 2. 将剩余的 Markdown 内容转换为 HTML
        // 3. 合并处理结果

        let processed_content = self.process_mdx_components(content);
        self.render(&processed_content)
    }

    /// 处理 MDX 中的组件
    fn process_mdx_components(&self, content: &str) -> String {
        // 简单的组件处理实现
        // 这里我们假设组件格式为 <ComponentName prop1="value1" prop2={value2} />
        // 或者 <ComponentName prop1="value1" prop2={value2}>内容</ComponentName>

        let mut result = content.to_string();

        // 处理框架特定的语法
        // 1. 处理 React 组件语法
        result = self.process_react_syntax(&result);

        // 2. 处理 Vue 组件语法
        result = self.process_vue_syntax(&result);

        // 3. 处理 Svelte 组件语法
        result = self.process_svelte_syntax(&result);

        result
    }

    /// 处理 React 组件语法
    fn process_react_syntax(&self, content: &str) -> String {
        // 处理 JSX 语法
        let mut result = content.to_string();

        // 这里可以添加更复杂的 JSX 语法处理
        // 例如：
        // 1. 处理 JSX 表达式
        // 2. 处理 JSX 属性
        // 3. 处理 JSX 子元素

        result
    }

    /// 处理 Vue 组件语法
    fn process_vue_syntax(&self, content: &str) -> String {
        // 处理 Vue 模板语法
        let mut result = content.to_string();

        // 这里可以添加更复杂的 Vue 语法处理
        // 例如：
        // 1. 处理 Vue 指令
        // 2. 处理 Vue 插值
        // 3. 处理 Vue 组件

        result
    }

    /// 处理 Svelte 组件语法
    fn process_svelte_syntax(&self, content: &str) -> String {
        // 处理 Svelte 模板语法
        let mut result = content.to_string();

        // 这里可以添加更复杂的 Svelte 语法处理
        // 例如：
        // 1. 处理 Svelte 指令
        // 2. 处理 Svelte 插值
        // 3. 处理 Svelte 组件

        result
    }

    /// 执行插件
    fn execute_plugins(&self, content: &str) -> String {
        // 插件系统暂时未实现，直接返回原始内容
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
        // 执行插件预处理
        let preprocessed_template = self.execute_plugins(template);
        
        // 使用解析器解析模板
        let mut parser = crate::compiler::parser::Parser::new(Box::leak(preprocessed_template.into_boxed_str()));
        let ast = parser.parse();
        
        // 渲染 AST
        let rendered = self.render_ast(&ast, context);
        
        // 执行插件后处理
        self.execute_plugins(&rendered)
    }

    /// 渲染抽象语法树
    fn render_ast(&self, nodes: &[AstNode], context: &Context) -> String {
        let mut result = String::new();
        
        for node in nodes {
            match node {
                AstNode::Text(text) => result.push_str(text),
                AstNode::Interpolation { expression, interpolation_type } => {
                    let value = self.evaluate_expression(expression, context);
                    let value_str = self.value_to_string(&value);
                    match interpolation_type {
                        InterpolationType::Escaped => {
                            let escaped = self.escape_html(&value_str);
                            result.push_str(&escaped);
                        }
                        InterpolationType::Unescaped => {
                            result.push_str(&value_str);
                        }
                    }
                }
                AstNode::Directive { directive_type, arguments, content } => {
                    match directive_type {
                        DirectiveType::For => {
                            if let Some(content) = content {
                                if arguments.len() >= 3 {
                                    let var_name = &arguments[0];
                                    let array_name = &arguments[2];
                                    
                                    if let Some(array) = context.get(array_name) {
                                        if let Some(array) = array.as_array() {
                                            for item in array {
                                                let mut item_context = context.clone();
                                                item_context.insert(var_name.to_string(), item.clone());
                                                
                                                let processed_content = self.render_ast(content, &item_context);
                                                result.push_str(&processed_content);
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        DirectiveType::If => {
                            if let Some(content) = content {
                                if let Some(condition) = arguments.first() {
                                    let condition_met = self.evaluate_condition(condition, context);
                                    if condition_met {
                                        let processed_content = self.render_ast(content, context);
                                        result.push_str(&processed_content);
                                    }
                                }
                            }
                        }
                        DirectiveType::Else => {
                            if let Some(content) = content {
                                let processed_content = self.render_ast(content, context);
                                result.push_str(&processed_content);
                            }
                        }
                        DirectiveType::Elif => {
                            if let Some(content) = content {
                                if let Some(condition) = arguments.first() {
                                    let condition_met = self.evaluate_condition(condition, context);
                                    if condition_met {
                                        let processed_content = self.render_ast(content, context);
                                        result.push_str(&processed_content);
                                    }
                                }
                            }
                        }
                        DirectiveType::Import => {
                            // 处理导入指令
                        }
                        DirectiveType::Layout => {
                            // 处理布局指令
                            if let Some(content) = content {
                                let processed_content = self.render_ast(content, context);
                                result.push_str(&processed_content);
                            }
                        }
                        _ => {
                            // 其他指令
                        }
                    }
                }
                AstNode::Component { name, attributes, content, self_closing } => {
                    // 提取组件 props
                    let props = self.extract_component_props(attributes, context);
                    
                    // 渲染组件
                    if let Some(component) = self.component_registry.get(name) {
                        let component_html = component.render(&props);
                        result.push_str(&component_html);
                    } else {
                        // 如果组件不存在，渲染为普通标签
                        result.push_str(&format!("<{}", name));
                        for (attr_name, attr_value) in attributes {
                            result.push_str(&format!(" {}=\"{}\"", attr_name, self.escape_html(attr_value)));
                        }
                        if *self_closing {
                            result.push_str(" />");
                        } else {
                            result.push_str(">");
                            if let Some(content) = content {
                                let processed_content = self.render_ast(content, context);
                                result.push_str(&processed_content);
                            }
                            result.push_str(&format!("</{}}", name));
                        }
                    }
                }
                AstNode::Layout { name, attributes, content } => {
                    // 处理布局组件
                    let props = self.extract_component_props(attributes, context);
                    let processed_content = self.render_ast(content, context);
                    
                    if let Some(component) = self.component_registry.get(name) {
                        let mut layout_props = props;
                        layout_props.insert("children".to_string(), serde_json::Value::String(processed_content));
                        let layout_html = component.render(&layout_props);
                        result.push_str(&layout_html);
                    } else {
                        // 如果布局组件不存在，直接渲染内容
                        result.push_str(&processed_content);
                    }
                }
                AstNode::Slot { name, content } => {
                    // 处理插槽
                    let processed_content = self.render_ast(content, context);
                    result.push_str(&processed_content);
                }
            }
        }
        
        result
    }

    /// 评估表达式
    fn evaluate_expression(&self, expression: &str, context: &Context) -> serde_json::Value {
        // 简单的表达式评估实现
        // 支持变量访问和基本的属性访问
        let expression = expression.trim();
        
        // 处理属性访问，如 user.name
        if expression.contains('.') {
            let parts: Vec<&str> = expression.split('.').collect();
            if let Some(first_part) = parts.first() {
                if let Some(value) = context.get(*first_part) {
                    let mut current_value = value;
                    for part in parts.iter().skip(1) {
                        match current_value {
                            serde_json::Value::Object(obj) => {
                                if let Some(next_value) = obj.get(*part) {
                                    current_value = next_value;
                                } else {
                                    return serde_json::Value::Null;
                                }
                            }
                            _ => return serde_json::Value::Null,
                        }
                    }
                    return current_value.clone();
                }
            }
        } else if let Some(value) = context.get(expression) {
            return value.clone();
        }
        
        // 尝试解析为字面量
        if let Ok(num) = expression.parse::<i64>() {
            return serde_json::Value::Number(serde_json::Number::from(num));
        } else if let Ok(num) = expression.parse::<f64>() {
            return serde_json::Value::Number(serde_json::Number::from_f64(num).unwrap());
        } else if expression == "true" {
            return serde_json::Value::Bool(true);
        } else if expression == "false" {
            return serde_json::Value::Bool(false);
        } else if expression == "null" {
            return serde_json::Value::Null;
        }
        
        serde_json::Value::Null
    }

    /// 评估条件表达式
    fn evaluate_condition(&self, condition: &str, context: &Context) -> bool {
        // 简单的条件评估实现
        let condition = condition.trim();
        
        // 处理逻辑运算符
        if condition.contains(" && ") {
            let parts: Vec<&str> = condition.split(" && ").collect();
            for part in parts {
                if !self.evaluate_condition(part, context) {
                    return false;
                }
            }
            return true;
        } else if condition.contains(" || ") {
            let parts: Vec<&str> = condition.split(" || ").collect();
            for part in parts {
                if self.evaluate_condition(part, context) {
                    return true;
                }
            }
            return false;
        } else if condition.starts_with("!") {
            let inner_condition = &condition[1..].trim();
            return !self.evaluate_condition(inner_condition, context);
        } else if condition.contains(" == ") {
            let parts: Vec<&str> = condition.split(" == ").collect();
            if parts.len() == 2 {
                let left = self.evaluate_expression(parts[0], context);
                let right = self.evaluate_expression(parts[1], context);
                return left == right;
            }
        } else if condition.contains(" != ") {
            let parts: Vec<&str> = condition.split(" != ").collect();
            if parts.len() == 2 {
                let left = self.evaluate_expression(parts[0], context);
                let right = self.evaluate_expression(parts[1], context);
                return left != right;
            }
        } else if condition.contains(" > ") {
            let parts: Vec<&str> = condition.split(" > ").collect();
            if parts.len() == 2 {
                let left = self.evaluate_expression(parts[0], context);
                let right = self.evaluate_expression(parts[1], context);
                if let (serde_json::Value::Number(left_num), serde_json::Value::Number(right_num)) = (left, right) {
                    return left_num.as_f64().unwrap() > right_num.as_f64().unwrap();
                }
            }
        } else if condition.contains(" < ") {
            let parts: Vec<&str> = condition.split(" < ").collect();
            if parts.len() == 2 {
                let left = self.evaluate_expression(parts[0], context);
                let right = self.evaluate_expression(parts[1], context);
                if let (serde_json::Value::Number(left_num), serde_json::Value::Number(right_num)) = (left, right) {
                    return left_num.as_f64().unwrap() < right_num.as_f64().unwrap();
                }
            }
        } else if condition.contains(" >= ") {
            let parts: Vec<&str> = condition.split(" >= ").collect();
            if parts.len() == 2 {
                let left = self.evaluate_expression(parts[0], context);
                let right = self.evaluate_expression(parts[1], context);
                if let (serde_json::Value::Number(left_num), serde_json::Value::Number(right_num)) = (left, right) {
                    return left_num.as_f64().unwrap() >= right_num.as_f64().unwrap();
                }
            }
        } else if condition.contains(" <= ") {
            let parts: Vec<&str> = condition.split(" <= ").collect();
            if parts.len() == 2 {
                let left = self.evaluate_expression(parts[0], context);
                let right = self.evaluate_expression(parts[1], context);
                if let (serde_json::Value::Number(left_num), serde_json::Value::Number(right_num)) = (left, right) {
                    return left_num.as_f64().unwrap() <= right_num.as_f64().unwrap();
                }
            }
        }
        
        // 检查变量是否存在
        self.evaluate_expression(condition, context) != serde_json::Value::Null
    }

    /// 解析组件标签，提取组件名称
    fn parse_component_tag(&self, tag: &str) -> Option<String> {
        // 简单的组件标签解析
        // 假设组件标签格式为 <ComponentName prop1="value1" prop2={value2} />
        let tag = tag.trim();
        if !tag.starts_with("<") {
            return None;
        }

        // 提取标签名称
        let tag_name_start = 1;
        let tag_name_end = tag[tag_name_start..].find(|c: char| c.is_whitespace() || c == '>').unwrap_or(tag.len() - 1);

        let component_name = &tag[tag_name_start..tag_name_start + tag_name_end];

        // 检查是否为组件（首字母大写）
        if component_name.chars().next().unwrap_or('a').is_uppercase() { Some(component_name.to_string()) } else { None }
    }

    /// 提取组件 props
    fn extract_component_props(&self, attributes: &HashMap<String, String>, context: &Context) -> Context {
        let mut props = Context::new();
        
        for (name, value) in attributes {
            let processed_value = self.process_prop_value(value, context);
            props.insert(name.to_string(), processed_value);
        }
        
        props
    }

    /// 处理组件属性值
    fn process_prop_value(&self, value: &str, context: &Context) -> serde_json::Value {
        let value = value.trim();

        // 处理字符串值
        if (value.starts_with('"') && value.ends_with('"')) || (value.starts_with('\'') && value.ends_with('\'')) {
            let str_value = &value[1..value.len() - 1];
            serde_json::Value::String(str_value.to_string())
        }
        // 处理表达式值
        else if value.starts_with('{') && value.ends_with('}') {
            let expr = &value[1..value.len() - 1].trim();
            self.evaluate_expression(expr, context)
        }
        // 处理数字值
        else if let Ok(num) = value.parse::<i64>() {
            serde_json::Value::Number(serde_json::Number::from(num))
        }
        else if let Ok(num) = value.parse::<f64>() {
            serde_json::Value::Number(serde_json::Number::from_f64(num).unwrap())
        }
        // 处理布尔值
        else if value == "true" {
            serde_json::Value::Bool(true)
        }
        else if value == "false" {
            serde_json::Value::Bool(false)
        }
        // 处理 null
        else if value == "null" {
            serde_json::Value::Null
        }
        // 默认处理为字符串
        else {
            serde_json::Value::String(value.to_string())
        }
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
                        }
                        else if result[endif_pos..].starts_with("{% endif %}") {
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
                        }
                        else {
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
            }
            else {
                break;
            }
        }

        new_result
    }

    /// HTML 转义
    fn escape_html(&self, content: &str) -> String {
        content.replace("&", "&amp;").replace("<", "&lt;").replace(">", "&gt;").replace("\"", "&quot;").replace("'", "&#39;")
    }
}

impl Default for HtmlRenderer {
    fn default() -> Self {
        Self::new()
    }
}
