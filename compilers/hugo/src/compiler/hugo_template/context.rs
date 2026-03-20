//! Hugo 模板上下文模块
//! 定义 Hugo 模板中可用的变量和上下文结构

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 站点参数
///
/// 对应 Hugo 中的 .Site.Params
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SiteParams {
    /// 参数字典
    #[serde(flatten)]
    pub params: HashMap<String, String>,
}

impl SiteParams {
    /// 创建新的站点参数
    pub fn new() -> Self {
        Self::default()
    }

    /// 添加参数
    ///
    /// # Arguments
    ///
    /// * `key` - 参数键
    /// * `value` - 参数值
    pub fn with_param(mut self, key: String, value: String) -> Self {
        self.params.insert(key, value);
        self
    }

    /// 获取参数
    ///
    /// # Arguments
    ///
    /// * `key` - 参数键
    pub fn get(&self, key: &str) -> Option<&String> {
        self.params.get(key)
    }
}

/// 页面参数
///
/// 对应 Hugo 中的 .Page.Params
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct PageParams {
    /// 参数字典
    #[serde(flatten)]
    pub params: HashMap<String, String>,
}

impl PageParams {
    /// 创建新的页面参数
    pub fn new() -> Self {
        Self::default()
    }

    /// 添加参数
    ///
    /// # Arguments
    ///
    /// * `key` - 参数键
    /// * `value` - 参数值
    pub fn with_param(mut self, key: String, value: String) -> Self {
        self.params.insert(key, value);
        self
    }

    /// 获取参数
    ///
    /// # Arguments
    ///
    /// * `key` - 参数键
    pub fn get(&self, key: &str) -> Option<&String> {
        self.params.get(key)
    }
}

/// Hugo 站点信息
///
/// 对应 Hugo 中的 .Site 变量
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct HugoSite {
    /// 站点标题
    pub title: Option<String>,
    /// 基础 URL
    pub base_url: Option<String>,
    /// 语言代码
    pub language_code: Option<String>,
    /// 站点描述
    pub description: Option<String>,
    /// 站点作者
    pub author: Option<String>,
    /// 站点版权信息
    pub copyright: Option<String>,
    /// 站点参数
    pub params: SiteParams,
    /// 语言配置
    pub languages: HashMap<String, LanguageConfig>,
}

impl HugoSite {
    /// 创建新的站点配置
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置站点标题
    ///
    /// # Arguments
    ///
    /// * `title` - 站点标题
    pub fn with_title(mut self, title: String) -> Self {
        self.title = Some(title);
        self
    }

    /// 设置基础 URL
    ///
    /// # Arguments
    ///
    /// * `base_url` - 基础 URL
    pub fn with_base_url(mut self, base_url: String) -> Self {
        self.base_url = Some(base_url);
        self
    }

    /// 设置语言代码
    ///
    /// # Arguments
    ///
    /// * `language_code` - 语言代码
    pub fn with_language_code(mut self, language_code: String) -> Self {
        self.language_code = Some(language_code);
        self
    }

    /// 设置站点描述
    ///
    /// # Arguments
    ///
    /// * `description` - 站点描述
    pub fn with_description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    /// 设置站点作者
    ///
    /// # Arguments
    ///
    /// * `author` - 站点作者
    pub fn with_author(mut self, author: String) -> Self {
        self.author = Some(author);
        self
    }

    /// 设置版权信息
    ///
    /// # Arguments
    ///
    /// * `copyright` - 版权信息
    pub fn with_copyright(mut self, copyright: String) -> Self {
        self.copyright = Some(copyright);
        self
    }

    /// 设置站点参数
    ///
    /// # Arguments
    ///
    /// * `params` - 站点参数
    pub fn with_params(mut self, params: SiteParams) -> Self {
        self.params = params;
        self
    }

    /// 添加语言配置
    ///
    /// # Arguments
    ///
    /// * `code` - 语言代码
    /// * `config` - 语言配置
    pub fn with_language(mut self, code: String, config: LanguageConfig) -> Self {
        self.languages.insert(code, config);
        self
    }
}

/// 语言配置
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct LanguageConfig {
    /// 语言名称
    pub language_name: Option<String>,
    /// 语言标题
    pub title: Option<String>,
    /// 语言权重
    pub weight: Option<i32>,
    /// 是否为默认语言
    pub default: Option<bool>,
}

impl LanguageConfig {
    /// 创建新的语言配置
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置语言名称
    ///
    /// # Arguments
    ///
    /// * `language_name` - 语言名称
    pub fn with_language_name(mut self, language_name: String) -> Self {
        self.language_name = Some(language_name);
        self
    }

    /// 设置语言标题
    ///
    /// # Arguments
    ///
    /// * `title` - 语言标题
    pub fn with_title(mut self, title: String) -> Self {
        self.title = Some(title);
        self
    }

    /// 设置语言权重
    ///
    /// # Arguments
    ///
    /// * `weight` - 语言权重
    pub fn with_weight(mut self, weight: i32) -> Self {
        self.weight = Some(weight);
        self
    }

    /// 设置为默认语言
    pub fn as_default(mut self) -> Self {
        self.default = Some(true);
        self
    }
}

/// Hugo 页面信息
///
/// 对应 Hugo 中的 .Page 变量
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct HugoPage {
    /// 页面标题
    pub title: Option<String>,
    /// 页面描述
    pub description: Option<String>,
    /// 页面内容（已渲染的 HTML）
    pub content: Option<String>,
    /// 页面摘要
    pub summary: Option<String>,
    /// 页面 URL 路径
    pub permalink: Option<String>,
    /// 页面相对路径
    pub rel_permalink: Option<String>,
    /// 页面布局
    pub layout: Option<String>,
    /// 页面类型
    pub kind: Option<String>,
    /// 页面部分
    pub section: Option<String>,
    /// 页面类型
    pub type_: Option<String>,
    /// 页面日期
    pub date: Option<String>,
    /// 页面最后修改日期
    pub last_modified: Option<String>,
    /// 是否为草稿
    pub draft: Option<bool>,
    /// 页面权重
    pub weight: Option<i32>,
    /// 页面标签
    pub tags: Vec<String>,
    /// 页面分类
    pub categories: Vec<String>,
    /// 页面参数
    pub params: PageParams,
    /// 页面文件路径
    pub file_path: Option<String>,
}

impl HugoPage {
    /// 创建新的页面
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置页面标题
    ///
    /// # Arguments
    ///
    /// * `title` - 页面标题
    pub fn with_title(mut self, title: String) -> Self {
        self.title = Some(title);
        self
    }

    /// 设置页面描述
    ///
    /// # Arguments
    ///
    /// * `description` - 页面描述
    pub fn with_description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    /// 设置页面内容
    ///
    /// # Arguments
    ///
    /// * `content` - 页面内容（已渲染的 HTML）
    pub fn with_content(mut self, content: String) -> Self {
        self.content = Some(content);
        self
    }

    /// 设置页面摘要
    ///
    /// # Arguments
    ///
    /// * `summary` - 页面摘要
    pub fn with_summary(mut self, summary: String) -> Self {
        self.summary = Some(summary);
        self
    }

    /// 设置永久链接
    ///
    /// # Arguments
    ///
    /// * `permalink` - 永久链接
    pub fn with_permalink(mut self, permalink: String) -> Self {
        self.permalink = Some(permalink);
        self
    }

    /// 设置相对链接
    ///
    /// # Arguments
    ///
    /// * `rel_permalink` - 相对链接
    pub fn with_rel_permalink(mut self, rel_permalink: String) -> Self {
        self.rel_permalink = Some(rel_permalink);
        self
    }

    /// 设置页面布局
    ///
    /// # Arguments
    ///
    /// * `layout` - 页面布局
    pub fn with_layout(mut self, layout: String) -> Self {
        self.layout = Some(layout);
        self
    }

    /// 设置页面类型
    ///
    /// # Arguments
    ///
    /// * `kind` - 页面类型
    pub fn with_kind(mut self, kind: String) -> Self {
        self.kind = Some(kind);
        self
    }

    /// 设置页面部分
    ///
    /// # Arguments
    ///
    /// * `section` - 页面部分
    pub fn with_section(mut self, section: String) -> Self {
        self.section = Some(section);
        self
    }

    /// 设置页面类型
    ///
    /// # Arguments
    ///
    /// * `type_` - 页面类型
    pub fn with_type(mut self, type_: String) -> Self {
        self.type_ = Some(type_);
        self
    }

    /// 设置日期
    ///
    /// # Arguments
    ///
    /// * `date` - 日期字符串
    pub fn with_date(mut self, date: String) -> Self {
        self.date = Some(date);
        self
    }

    /// 设置最后修改日期
    ///
    /// # Arguments
    ///
    /// * `last_modified` - 最后修改日期字符串
    pub fn with_last_modified(mut self, last_modified: String) -> Self {
        self.last_modified = Some(last_modified);
        self
    }

    /// 设置为草稿
    ///
    /// # Arguments
    ///
    /// * `draft` - 是否为草稿
    pub fn with_draft(mut self, draft: bool) -> Self {
        self.draft = Some(draft);
        self
    }

    /// 设置页面权重
    ///
    /// # Arguments
    ///
    /// * `weight` - 页面权重
    pub fn with_weight(mut self, weight: i32) -> Self {
        self.weight = Some(weight);
        self
    }

    /// 添加标签
    ///
    /// # Arguments
    ///
    /// * `tag` - 标签
    pub fn add_tag(mut self, tag: String) -> Self {
        self.tags.push(tag);
        self
    }

    /// 添加分类
    ///
    /// # Arguments
    ///
    /// * `category` - 分类
    pub fn add_category(mut self, category: String) -> Self {
        self.categories.push(category);
        self
    }

    /// 设置页面参数
    ///
    /// # Arguments
    ///
    /// * `params` - 页面参数
    pub fn with_params(mut self, params: PageParams) -> Self {
        self.params = params;
        self
    }

    /// 设置文件路径
    ///
    /// # Arguments
    ///
    /// * `file_path` - 文件路径
    pub fn with_file_path(mut self, file_path: String) -> Self {
        self.file_path = Some(file_path);
        self
    }
}

/// Hugo 模板上下文
///
/// 包含站点和页面信息，是传递给模板的根上下文
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HugoTemplateContext {
    /// 站点信息
    pub site: HugoSite,
    /// 页面信息
    pub page: HugoPage,
}

impl HugoTemplateContext {
    /// 创建新的模板上下文
    ///
    /// # Arguments
    ///
    /// * `site` - 站点信息
    /// * `page` - 页面信息
    pub fn new(site: HugoSite, page: HugoPage) -> Self {
        Self { site, page }
    }
}
