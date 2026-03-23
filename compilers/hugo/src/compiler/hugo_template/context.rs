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

/// 菜单项
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct MenuItem {
    /// 菜单项名称
    pub name: Option<String>,
    /// 菜单项 URL
    pub url: Option<String>,
    /// 菜单项权重
    pub weight: Option<i32>,
    /// 菜单项父级
    pub parent: Option<String>,
}

/// 分类项
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct TaxonomyItem {
    /// 分类名称
    pub name: String,
    /// 分类计数
    pub count: i32,
    /// 分类链接
    pub permalink: String,
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
    /// 菜单配置
    pub menus: HashMap<String, Vec<MenuItem>>,
    /// 分类配置
    pub taxonomies: HashMap<String, Vec<TaxonomyItem>>,
    /// 数据文件
    pub data: serde_json::Value,
    /// 所有页面
    pub pages: Vec<HugoPage>,
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

    /// 添加菜单项
    ///
    /// # Arguments
    ///
    /// * `menu_name` - 菜单名称
    /// * `item` - 菜单项
    pub fn with_menu_item(mut self, menu_name: String, item: MenuItem) -> Self {
        self.menus.entry(menu_name).or_default().push(item);
        self
    }

    /// 添加分类项
    ///
    /// # Arguments
    ///
    /// * `taxonomy_name` - 分类名称
    /// * `item` - 分类项
    pub fn with_taxonomy_item(mut self, taxonomy_name: String, item: TaxonomyItem) -> Self {
        self.taxonomies.entry(taxonomy_name).or_default().push(item);
        self
    }

    /// 设置数据
    ///
    /// # Arguments
    ///
    /// * `data` - 数据值
    pub fn with_data(mut self, data: serde_json::Value) -> Self {
        self.data = data;
        self
    }

    /// 添加页面
    ///
    /// # Arguments
    ///
    /// * `page` - 页面
    pub fn with_page(mut self, page: HugoPage) -> Self {
        self.pages.push(page);
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

/// 文件信息
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct FileInfo {
    /// 文件路径
    pub path: Option<String>,
    /// 文件名称
    pub name: Option<String>,
    /// 文件扩展名
    pub extension: Option<String>,
    /// 文件大小
    pub size: Option<u64>,
    /// 文件修改时间
    pub modified: Option<String>,
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
    /// 页面关键词
    pub keywords: Vec<String>,
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
    /// 发布日期
    pub publish_date: Option<String>,
    /// 过期日期
    pub expiry_date: Option<String>,
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
    /// 页面文件信息
    pub file: FileInfo,
    /// 子页面
    pub pages: Vec<HugoPage>,
    /// 父页面
    pub parent: Option<Box<HugoPage>>,
    /// 下一个页面
    pub next: Option<Box<HugoPage>>,
    /// 上一个页面
    pub prev: Option<Box<HugoPage>>,
    /// 页面别名
    pub aliases: Vec<String>,
    /// 页面菜单
    pub menus: HashMap<String, Vec<MenuItem>>,
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

    /// 添加关键词
    ///
    /// # Arguments
    ///
    /// * `keyword` - 关键词
    pub fn add_keyword(mut self, keyword: String) -> Self {
        self.keywords.push(keyword);
        self
    }

    /// 设置文件信息
    ///
    /// # Arguments
    ///
    /// * `file` - 文件信息
    pub fn with_file(mut self, file: FileInfo) -> Self {
        self.file = file;
        self
    }

    /// 添加子页面
    ///
    /// # Arguments
    ///
    /// * `page` - 子页面
    pub fn add_page(mut self, page: HugoPage) -> Self {
        self.pages.push(page);
        self
    }

    /// 设置父页面
    ///
    /// # Arguments
    ///
    /// * `parent` - 父页面
    pub fn with_parent(mut self, parent: HugoPage) -> Self {
        self.parent = Some(Box::new(parent));
        self
    }

    /// 设置下一个页面
    ///
    /// # Arguments
    ///
    /// * `next` - 下一个页面
    pub fn with_next(mut self, next: HugoPage) -> Self {
        self.next = Some(Box::new(next));
        self
    }

    /// 设置上一个页面
    ///
    /// # Arguments
    ///
    /// * `prev` - 上一个页面
    pub fn with_prev(mut self, prev: HugoPage) -> Self {
        self.prev = Some(Box::new(prev));
        self
    }

    /// 设置发布日期
    ///
    /// # Arguments
    ///
    /// * `publish_date` - 发布日期字符串
    pub fn with_publish_date(mut self, publish_date: String) -> Self {
        self.publish_date = Some(publish_date);
        self
    }

    /// 设置过期日期
    ///
    /// # Arguments
    ///
    /// * `expiry_date` - 过期日期字符串
    pub fn with_expiry_date(mut self, expiry_date: String) -> Self {
        self.expiry_date = Some(expiry_date);
        self
    }

    /// 添加别名
    ///
    /// # Arguments
    ///
    /// * `alias` - 页面别名
    pub fn add_alias(mut self, alias: String) -> Self {
        self.aliases.push(alias);
        self
    }

    /// 添加菜单项
    ///
    /// # Arguments
    ///
    /// * `menu_name` - 菜单名称
    /// * `item` - 菜单项
    pub fn add_menu_item(mut self, menu_name: String, item: MenuItem) -> Self {
        self.menus.entry(menu_name).or_default().push(item);
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
    /// 环境信息
    pub env: EnvironmentInfo,
}

/// 环境信息
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct EnvironmentInfo {
    /// 是否为开发环境
    pub is_dev: bool,
    /// 是否为生产环境
    pub is_production: bool,
    /// 构建日期
    pub build_date: String,
    /// Hugo 版本
    pub hugo_version: String,
}

impl EnvironmentInfo {
    /// 创建新的环境信息
    pub fn new() -> Self {
        Self {
            is_dev: true,
            is_production: false,
            build_date: chrono::Utc::now().to_rfc3339(),
            hugo_version: "0.135.0".to_string(),
        }
    }
}

impl HugoTemplateContext {
    /// 创建新的模板上下文
    ///
    /// # Arguments
    ///
    /// * `site` - 站点信息
    /// * `page` - 页面信息
    pub fn new(site: HugoSite, page: HugoPage) -> Self {
        Self { site, page, env: EnvironmentInfo::new() }
    }
}
