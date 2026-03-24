//! HTML 渲染器

use crate::compiler::parser::ast::{AstNode, DirectiveType, InterpolationType};
use std::collections::HashMap;

/// 模板上下文，用于存储变量和数据
pub type Context = HashMap<String, serde_json::Value>;

/// 组件状态管理
pub struct ComponentState {
    /// 组件状态数据
    data: std::cell::RefCell<HashMap<String, serde_json::Value>>,
    /// 组件事件处理函数
    event_handlers: std::cell::RefCell<HashMap<String, Box<dyn Fn(&ComponentState, serde_json::Value)>>>,
}

impl ComponentState {
    /// 创建新的组件状态
    pub fn new() -> Self {
        Self {
            data: std::cell::RefCell::new(HashMap::new()),
            event_handlers: std::cell::RefCell::new(HashMap::new()),
        }
    }

    /// 获取状态值
    pub fn get(&self, key: &str) -> Option<serde_json::Value> {
        self.data.borrow().get(key).cloned()
    }

    /// 设置状态值
    pub fn set(&self, key: &str, value: serde_json::Value) {
        self.data.borrow_mut().insert(key.to_string(), value);
    }

    /// 注册事件处理函数
    pub fn on(&self, event: &str, handler: impl Fn(&ComponentState, serde_json::Value) + 'static) {
        self.event_handlers.borrow_mut().insert(event.to_string(), Box::new(handler));
    }

    /// 触发事件
    pub fn emit(&self, event: &str, data: serde_json::Value) {
        if let Some(handler) = self.event_handlers.borrow().get(event) {
            let handler = handler.clone();
            handler(self, data);
        }
    }

    /// 转换为上下文
    pub fn to_context(&self) -> Context {
        self.data.borrow().clone()
    }
}

/// HTML 渲染器
pub struct HtmlRenderer {
    /// 渲染配置
    config: HashMap<String, String>,
    /// 组件注册表
    component_registry: Option<crate::compiler::ComponentRegistry>,
}

impl HtmlRenderer {
    /// 创建新的 HTML 渲染器
    pub fn new() -> Self {
        Self { 
            config: HashMap::new(),
            component_registry: None
        }
    }

    /// 设置组件注册表
    pub fn set_component_registry(&mut self, registry: crate::compiler::ComponentRegistry) {
        self.component_registry = Some(registry);
    }

    /// 获取组件注册表
    pub fn component_registry(&self) -> Option<&crate::compiler::ComponentRegistry> {
        self.component_registry.as_ref()
    }

    /// 渲染 Astro 模板内容
    ///
    /// # 参数
    /// - `template`: Astro 模板内容
    /// - `context`: 模板上下文，包含变量和数据
    ///
    /// # 返回值
    /// 渲染后的 HTML 内容
    pub fn render_astro(&mut self, template: &str, context: &Context) -> String {
        // 使用解析器解析模板
        let mut parser = crate::compiler::parser::Parser::new(template);
        let ast = parser.parse();

        // 渲染 AST
        self.render_ast(&ast, context)
    }

    /// 渲染抽象语法树
    fn render_ast(&mut self, nodes: &[AstNode], context: &Context) -> String {
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
                    // 渲染为普通标签
                    result.push_str(&format!("<{}", name));
                    for (attr_name, attr_value) in attributes {
                        result.push_str(&format!(" {}=\"{}\"", attr_name, self.escape_html(attr_value)));
                    }
                    if *self_closing {
                        result.push_str(" />");
                    }
                    else {
                        result.push_str("> ");
                        if let Some(content) = content {
                            let processed_content = self.render_ast(content, context);
                            result.push_str(&processed_content);
                        }
                        result.push_str(&format!("</{}>", name));
                    }
                }
                AstNode::Layout { name, attributes, content } => {
                    // 处理布局组件
                    let processed_content = self.render_ast(content, context);
                    result.push_str(&processed_content);
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
                                }
                                else {
                                    return serde_json::Value::Null;
                                }
                            }
                            _ => return serde_json::Value::Null,
                        }
                    }
                    return current_value.clone();
                }
            }
        }
        else if let Some(value) = context.get(expression) {
            return value.clone();
        }

        // 尝试解析为字面量
        if let Ok(num) = expression.parse::<i64>() {
            return serde_json::Value::Number(serde_json::Number::from(num));
        }
        else if let Ok(num) = expression.parse::<f64>() {
            return serde_json::Value::Number(serde_json::Number::from_f64(num).unwrap());
        }
        else if expression == "true" {
            return serde_json::Value::Bool(true);
        }
        else if expression == "false" {
            return serde_json::Value::Bool(false);
        }
        else if expression == "null" {
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
        }
        else if condition.contains(" || ") {
            let parts: Vec<&str> = condition.split(" || ").collect();
            for part in parts {
                if self.evaluate_condition(part, context) {
                    return true;
                }
            }
            return false;
        }
        else if condition.starts_with("!") {
            let inner_condition = &condition[1..].trim();
            return !self.evaluate_condition(inner_condition, context);
        }
        else if condition.contains(" == ") {
            let parts: Vec<&str> = condition.split(" == ").collect();
            if parts.len() == 2 {
                let left = self.evaluate_expression(parts[0], context);
                let right = self.evaluate_expression(parts[1], context);
                return left == right;
            }
        }
        else if condition.contains(" != ") {
            let parts: Vec<&str> = condition.split(" != ").collect();
            if parts.len() == 2 {
                let left = self.evaluate_expression(parts[0], context);
                let right = self.evaluate_expression(parts[1], context);
                return left != right;
            }
        }
        else if condition.contains(" > ") {
            let parts: Vec<&str> = condition.split(" > ").collect();
            if parts.len() == 2 {
                let left = self.evaluate_expression(parts[0], context);
                let right = self.evaluate_expression(parts[1], context);
                if let (serde_json::Value::Number(left_num), serde_json::Value::Number(right_num)) = (left, right) {
                    return left_num.as_f64().unwrap() > right_num.as_f64().unwrap();
                }
            }
        }
        else if condition.contains(" < ") {
            let parts: Vec<&str> = condition.split(" < ").collect();
            if parts.len() == 2 {
                let left = self.evaluate_expression(parts[0], context);
                let right = self.evaluate_expression(parts[1], context);
                if let (serde_json::Value::Number(left_num), serde_json::Value::Number(right_num)) = (left, right) {
                    return left_num.as_f64().unwrap() < right_num.as_f64().unwrap();
                }
            }
        }
        else if condition.contains(" >= ") {
            let parts: Vec<&str> = condition.split(" >= ").collect();
            if parts.len() == 2 {
                let left = self.evaluate_expression(parts[0], context);
                let right = self.evaluate_expression(parts[1], context);
                if let (serde_json::Value::Number(left_num), serde_json::Value::Number(right_num)) = (left, right) {
                    return left_num.as_f64().unwrap() >= right_num.as_f64().unwrap();
                }
            }
        }
        else if condition.contains(" <= ") {
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

    /// 将 serde_json::Value 转换为字符串
    fn value_to_string(&self, value: &serde_json::Value) -> String {
        match value {
            serde_json::Value::String(s) => s.clone(),
            _ => value.to_string(),
        }
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
