//! 组件系统模块

use crate::compiler::renderer::html_renderer::Context;
use hashbrown::HashMap;
use std::{borrow::Cow, fs::File, io::Read, path::Path, sync::RwLock};

/// 前端框架类型
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Framework {
    /// Astro 组件
    Astro,
    /// React 组件
    React,
    /// Vue 组件
    Vue,
    /// Svelte 组件
    Svelte,
    /// Solid 组件
    Solid,
    /// Preact 组件
    Preact,
    /// Lit 组件
    Lit,
    /// 其他框架组件
    Other(String),
}

/// 组件定义
#[derive(Clone)]
pub struct Component {
    /// 组件名称
    name: Cow<'static, str>,
    /// 组件模板
    template: Cow<'static, str>,
    /// 组件脚本
    script: Option<Cow<'static, str>>,
    /// 组件样式
    style: Option<Cow<'static, str>>,
    /// 组件所属框架
    framework: Framework,
}

impl Component {
    /// 创建新组件
    pub fn new(name: &str, template: &str, framework: Framework) -> Self {
        Self {
            name: Cow::Owned(name.to_string()),
            template: Cow::Owned(template.to_string()),
            script: None,
            style: None,
            framework,
        }
    }

    /// 创建 Astro 组件
    pub fn new_astro(name: &str, template: &str) -> Self {
        Self::new(name, template, Framework::Astro)
    }

    /// 创建 React 组件
    pub fn new_react(name: &str, template: &str) -> Self {
        Self::new(name, template, Framework::React)
    }

    /// 创建 Vue 组件
    pub fn new_vue(name: &str, template: &str) -> Self {
        Self::new(name, template, Framework::Vue)
    }

    /// 创建 Svelte 组件
    pub fn new_svelte(name: &str, template: &str) -> Self {
        Self::new(name, template, Framework::Svelte)
    }

    /// 创建 Solid 组件
    pub fn new_solid(name: &str, template: &str) -> Self {
        Self::new(name, template, Framework::Solid)
    }

    /// 创建 Preact 组件
    pub fn new_preact(name: &str, template: &str) -> Self {
        Self::new(name, template, Framework::Preact)
    }

    /// 创建 Lit 组件
    pub fn new_lit(name: &str, template: &str) -> Self {
        Self::new(name, template, Framework::Lit)
    }

    /// 创建其他框架组件
    pub fn new_other(name: &str, template: &str, framework_name: &str) -> Self {
        Self::new(name, template, Framework::Other(framework_name.to_string()))
    }

    /// 设置组件脚本
    pub fn set_script(&mut self, script: &str) {
        self.script = Some(Cow::Owned(script.to_string()));
    }

    /// 设置组件样式
    pub fn set_style(&mut self, style: &str) {
        self.style = Some(Cow::Owned(style.to_string()));
    }

    /// 获取组件名称
    pub fn name(&self) -> &str {
        &self.name
    }

    /// 获取组件所属框架
    pub fn framework(&self) -> &Framework {
        &self.framework
    }

    /// 渲染组件
    pub fn render(&self, props: &Context) -> String {
        // 根据框架类型选择不同的渲染方式
        match &self.framework {
            Framework::Astro => self.render_astro(props),
            Framework::React => self.render_react(props),
            Framework::Vue => self.render_vue(props),
            Framework::Svelte => self.render_svelte(props),
            Framework::Solid => self.render_solid(props),
            Framework::Preact => self.render_preact(props),
            Framework::Lit => self.render_lit(props),
            Framework::Other(_) => self.render_other(props),
        }
    }

    /// 渲染 Astro 组件
    fn render_astro(&self, props: &Context) -> String {
        // 使用 HTML 渲染器处理组件模板
        let renderer = crate::compiler::renderer::html_renderer::HtmlRenderer::new();
        renderer.render_astro(&self.template, props)
    }

    /// 渲染 React 组件
    fn render_react(&self, props: &Context) -> String {
        // 生成 React 组件的 HTML 包装
        let props_json = serde_json::to_string(props).unwrap();
        format!(r#"<div data-react-component="{}" data-props="{}"></div>"#, self.name, Self::escape_html(&props_json))
    }

    /// 渲染 Vue 组件
    fn render_vue(&self, props: &Context) -> String {
        // 生成 Vue 组件的 HTML 包装
        let props_json = serde_json::to_string(props).unwrap();
        format!(r#"<div data-vue-component="{}" data-props="{}"></div>"#, self.name, Self::escape_html(&props_json))
    }

    /// 渲染 Svelte 组件
    fn render_svelte(&self, props: &Context) -> String {
        // 生成 Svelte 组件的 HTML 包装
        let props_json = serde_json::to_string(props).unwrap();
        format!(r#"<div data-svelte-component="{}" data-props="{}"></div>"#, self.name, Self::escape_html(&props_json))
    }

    /// 渲染 Solid 组件
    fn render_solid(&self, props: &Context) -> String {
        // 生成 Solid 组件的 HTML 包装
        let props_json = serde_json::to_string(props).unwrap();
        format!(r#"<div data-solid-component="{}" data-props="{}"></div>"#, self.name, Self::escape_html(&props_json))
    }

    /// 渲染 Preact 组件
    fn render_preact(&self, props: &Context) -> String {
        // 生成 Preact 组件的 HTML 包装
        let props_json = serde_json::to_string(props).unwrap();
        format!(r#"<div data-preact-component="{}" data-props="{}"></div>"#, self.name, Self::escape_html(&props_json))
    }

    /// 渲染 Lit 组件
    fn render_lit(&self, props: &Context) -> String {
        // 生成 Lit 组件的 HTML 包装
        let props_json = serde_json::to_string(props).unwrap();
        format!(r#"<div data-lit-component="{}" data-props="{}"></div>"#, self.name, Self::escape_html(&props_json))
    }

    /// 渲染其他框架组件
    fn render_other(&self, props: &Context) -> String {
        // 生成其他框架组件的 HTML 包装
        let props_json = serde_json::to_string(props).unwrap();
        format!(r#"<div data-component="{}" data-props="{}"></div>"#, self.name, Self::escape_html(&props_json))
    }

    /// 将 serde_json::Value 转换为字符串
    fn value_to_string(value: &serde_json::Value) -> String {
        match value {
            serde_json::Value::String(s) => s.clone(),
            _ => value.to_string(),
        }
    }

    /// HTML 转义
    fn escape_html(content: &str) -> String {
        content.replace("&", "&amp;").replace("<", "&lt;").replace(">", "&gt;").replace("\"", "&quot;").replace("'", "&#39;")
    }
}

/// 组件注册表
pub struct ComponentRegistry {
    /// 组件映射
    components: RwLock<HashMap<String, Component>>,
}

impl ComponentRegistry {
    /// 创建新的组件注册表
    pub fn new() -> Self {
        Self { components: RwLock::new(HashMap::new()) }
    }

    /// 注册组件
    pub fn register(&self, component: Component) {
        let mut components = self.components.write().unwrap();
        components.insert(component.name().to_string(), component);
    }

    /// 从文件路径导入组件
    ///
    /// # 参数
    /// - `file_path`: 组件文件路径
    ///
    /// # 返回值
    /// 导入的组件名称
    pub fn import_component(&self, file_path: &Path) -> Result<String, String> {
        // 读取文件内容
        let content = std::fs::read_to_string(file_path).map_err(|e| format!("Failed to read file: {}", e))?;

        // 简单实现：创建一个 Astro 组件
        let file_name = file_path.file_name().unwrap_or_default().to_string_lossy();
        let component_name = file_name.split('.').next().unwrap_or(&file_name).to_string();
        let component = Component::new_astro(&component_name, &content);

        // 注册组件
        self.register(component);

        Ok(component_name)
    }

    /// 从文件路径批量导入组件
    ///
    /// # 参数
    /// - `file_paths`: 组件文件路径列表
    ///
    /// # 返回值
    /// 导入的组件名称列表
    pub fn import_components(&self, file_paths: &[&Path]) -> Result<Vec<String>, String> {
        let mut imported_components = Vec::new();

        for file_path in file_paths {
            let component_name = self.import_component(file_path)?;
            imported_components.push(component_name);
        }

        Ok(imported_components)
    }

    /// 获取组件
    pub fn get(&self, name: &str) -> Option<Component> {
        let components = self.components.read().unwrap();
        components.get(name).cloned()
    }

    /// 检查组件是否存在
    pub fn exists(&self, name: &str) -> bool {
        let components = self.components.read().unwrap();
        components.contains_key(name)
    }

    /// 获取所有组件的迭代器
    pub fn iter(&self) -> Vec<(String, Component)> {
        let components = self.components.read().unwrap();
        components.iter().map(|(k, v)| (k.clone(), v.clone())).collect()
    }
}

impl Default for ComponentRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// 组件解析器
pub struct ComponentParser {
    /// 组件注册表
    registry: ComponentRegistry,
}

impl ComponentParser {
    /// 创建新的组件解析器
    pub fn new() -> Self {
        Self { registry: ComponentRegistry::new() }
    }

    /// 解析组件文件
    pub fn parse_component(&self, content: &str) -> Result<Component, String> {
        // 暂时返回一个简单的组件，不使用 oaks 库的解析功能
        Ok(Component::new_astro("TestComponent", content))
    }

    /// 从文件路径解析并注册组件
    pub fn parse_and_register_from_path(&self, file_path: &Path) -> Result<String, String> {
        self.registry.import_component(file_path)
    }

    /// 注册组件
    pub fn register_component(&self, component: Component) {
        self.registry.register(component);
    }

    /// 获取组件注册表
    pub fn registry(&self) -> &ComponentRegistry {
        &self.registry
    }
}

impl Default for ComponentParser {
    fn default() -> Self {
        Self::new()
    }
}
