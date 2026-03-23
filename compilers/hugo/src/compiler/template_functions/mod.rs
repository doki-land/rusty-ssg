//! Hugo 模板函数模块
//! 提供与 Hugo 兼容的模板函数实现

pub mod collection;
pub mod condition;
pub mod content;
pub mod date;
pub mod math;
pub mod page;
pub mod partial;
pub mod string;
pub mod url;

pub use collection::CollectionFunctions;
pub use condition::ConditionFunctions;
pub use content::ContentFunctions;
pub use date::DateFunctions;
pub use math::MathFunctions;
pub use page::PageFunctions;
pub use partial::PartialFunctions;
pub use string::StringFunctions;
pub use url::UrlFunctions;

use serde_json::Value;
use std::collections::HashMap;

/// 模板函数注册表
pub struct TemplateFunctions {
    /// 函数映射表
    functions: HashMap<String, Box<dyn for<'a> Fn(&'a [Value]) -> Result<Value, String> + Send + Sync>>,
}

impl TemplateFunctions {
    /// 创建新的函数注册表
    pub fn new() -> Self {
        let mut registry = Self { functions: HashMap::new() };

        registry.register_all();
        registry
    }

    /// 注册所有函数
    fn register_all(&mut self) {
        self.register_url_functions();
        self.register_string_functions();
        self.register_collection_functions();
        self.register_date_functions();
        self.register_math_functions();
        self.register_condition_functions();
        self.register_content_functions();
        self.register_page_functions();
        self.register_partial_functions();
    }

    /// 注册 URL 处理函数
    fn register_url_functions(&mut self) {
        self.register("relURL", |args| UrlFunctions::new().rel_url(args));
        self.register("absURL", |args| UrlFunctions::new().abs_url(args));
        self.register("urlize", |args| UrlFunctions::new().urlize(args));
        self.register("absLangURL", |args| UrlFunctions::new().abs_lang_url(args));
        self.register("relLangURL", |args| UrlFunctions::new().rel_lang_url(args));
    }

    /// 注册字符串处理函数
    fn register_string_functions(&mut self) {
        self.register("lower", |args| StringFunctions::new().lower(args));
        self.register("upper", |args| StringFunctions::new().upper(args));
        self.register("title", |args| StringFunctions::new().title(args));
        self.register("slug", |args| StringFunctions::new().slug(args));
        self.register("truncate", |args| StringFunctions::new().truncate(args));
        self.register("replace", |args| StringFunctions::new().replace(args));
        self.register("replaceRE", |args| StringFunctions::new().replace_re(args));
        self.register("substr", |args| StringFunctions::new().substr(args));
        self.register("split", |args| StringFunctions::new().split(args));
        self.register("trim", |args| StringFunctions::new().trim(args));
        self.register("trimSuffix", |args| StringFunctions::new().trim_suffix(args));
        self.register("trimPrefix", |args| StringFunctions::new().trim_prefix(args));
    }

    /// 注册集合处理函数
    fn register_collection_functions(&mut self) {
        self.register("slice", |args| CollectionFunctions::new().slice(args));
        self.register("dict", |args| CollectionFunctions::new().dict(args));
        self.register("first", |args| CollectionFunctions::new().first(args));
        self.register("last", |args| CollectionFunctions::new().last(args));
        self.register("after", |args| CollectionFunctions::new().after(args));
        self.register("before", |args| CollectionFunctions::new().before(args));
        self.register("append", |args| CollectionFunctions::new().append(args));
        self.register("prepend", |args| CollectionFunctions::new().prepend(args));
        self.register("shuffle", |args| CollectionFunctions::new().shuffle(args));
        self.register("seq", |args| CollectionFunctions::new().seq(args));
        self.register("union", |args| CollectionFunctions::new().union(args));
        self.register("intersection", |args| CollectionFunctions::new().intersection(args));
        self.register("difference", |args| CollectionFunctions::new().difference(args));
        self.register("apply", |args| CollectionFunctions::new().apply(args));
    }

    /// 注册日期处理函数
    fn register_date_functions(&mut self) {
        self.register("now", |args| DateFunctions::new().now(args));
        self.register("dateFormat", |args| DateFunctions::new().date_format(args));
        self.register("time", |args| DateFunctions::new().time(args));
        self.register("addDate", |args| DateFunctions::new().add_date(args));
    }

    /// 注册数学函数
    fn register_math_functions(&mut self) {
        self.register("add", |args| MathFunctions::new().add(args));
        self.register("sub", |args| MathFunctions::new().sub(args));
        self.register("mul", |args| MathFunctions::new().mul(args));
        self.register("div", |args| MathFunctions::new().div(args));
        self.register("mod", |args| MathFunctions::new().mod_(args));
        self.register("math.Max", |args| MathFunctions::new().max(args));
        self.register("math.Min", |args| MathFunctions::new().min(args));
        self.register("math.Abs", |args| MathFunctions::new().abs(args));
    }

    /// 注册条件函数
    fn register_condition_functions(&mut self) {
        self.register("cond", |args| ConditionFunctions::new().cond(args));
        self.register("default", |args| ConditionFunctions::new().default(args));
        self.register("isset", |args| ConditionFunctions::new().isset(args));
        self.register("empty", |args| ConditionFunctions::new().empty(args));
    }

    /// 注册内容处理函数
    fn register_content_functions(&mut self) {
        self.register("markdownify", |args| ContentFunctions::new().markdownify(args));
        self.register("plainify", |args| ContentFunctions::new().plainify(args));
        self.register("highlight", |args| ContentFunctions::new().highlight(args));
        self.register("emojify", |args| ContentFunctions::new().emojify(args));
    }

    /// 注册页面处理函数
    fn register_page_functions(&mut self) {
        self.register("ref", |args| PageFunctions::new().ref_(args));
        self.register("relref", |args| PageFunctions::new().relref(args));
        self.register("getPage", |args| PageFunctions::new().get_page(args));
        self.register("pages", |args| PageFunctions::new().pages(args));
        self.register("site", |args| PageFunctions::new().site(args));
    }

    /// 注册 Partial 模板函数
    fn register_partial_functions(&mut self) {
        self.register("partial", |args| PartialFunctions::new().partial(args));
        self.register("partialCached", |args| PartialFunctions::new().partial_cached(args));
    }

    /// 注册单个函数
    pub fn register<F>(&mut self, name: &str, func: F)
    where
        F: for<'a> Fn(&'a [Value]) -> Result<Value, String> + Send + Sync + 'static,
    {
        self.functions.insert(name.to_string(), Box::new(func));
    }

    /// 调用函数
    pub fn call(&self, name: &str, args: &[Value]) -> Result<Value, String> {
        let func = self.functions.get(name).ok_or_else(|| format!("Function not found: {}", name))?;

        func(args)
    }

    /// 获取所有已注册的函数名
    pub fn list_functions(&self) -> Vec<&String> {
        self.functions.keys().collect()
    }
}

impl Default for TemplateFunctions {
    fn default() -> Self {
        Self::new()
    }
}
