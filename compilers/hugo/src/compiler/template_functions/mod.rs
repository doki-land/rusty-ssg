//! Hugo 模板函数模块
//! 提供与 Hugo 兼容的模板函数实现

pub mod url;
pub mod string;
pub mod collection;
pub mod date;
pub mod math;
pub mod condition;
pub mod content;
pub mod page;
pub mod partial;

pub use url::UrlFunctions;
pub use string::StringFunctions;
pub use collection::CollectionFunctions;
pub use date::DateFunctions;
pub use math::MathFunctions;
pub use condition::ConditionFunctions;
pub use content::ContentFunctions;
pub use page::PageFunctions;
pub use partial::PartialFunctions;

use serde_json::Value;
use std::collections::HashMap;

/// 模板函数注册表
pub struct TemplateFunctions {
    /// 函数映射表
    functions: HashMap<String, Box<dyn Fn(&[Value]) -> Result<Value, String> + Send + Sync>>,
}

impl TemplateFunctions {
    /// 创建新的函数注册表
    pub fn new() -> Self {
        let mut registry = Self {
            functions: HashMap::new(),
        };
        
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
        let funcs = UrlFunctions::new();
        
        self.register("relURL", Box::new(move |args| funcs.rel_url(args)));
        self.register("absURL", Box::new(move |args| funcs.abs_url(args)));
        self.register("urlize", Box::new(move |args| funcs.urlize(args)));
        self.register("absLangURL", Box::new(move |args| funcs.abs_lang_url(args)));
        self.register("relLangURL", Box::new(move |args| funcs.rel_lang_url(args)));
    }
    
    /// 注册字符串处理函数
    fn register_string_functions(&mut self) {
        let funcs = StringFunctions::new();
        
        self.register("lower", Box::new(move |args| funcs.lower(args)));
        self.register("upper", Box::new(move |args| funcs.upper(args)));
        self.register("title", Box::new(move |args| funcs.title(args)));
        self.register("slug", Box::new(move |args| funcs.slug(args)));
        self.register("truncate", Box::new(move |args| funcs.truncate(args)));
        self.register("replace", Box::new(move |args| funcs.replace(args)));
        self.register("replaceRE", Box::new(move |args| funcs.replace_re(args)));
        self.register("substr", Box::new(move |args| funcs.substr(args)));
        self.register("split", Box::new(move |args| funcs.split(args)));
        self.register("trim", Box::new(move |args| funcs.trim(args)));
        self.register("trimSuffix", Box::new(move |args| funcs.trim_suffix(args)));
        self.register("trimPrefix", Box::new(move |args| funcs.trim_prefix(args)));
    }
    
    /// 注册集合处理函数
    fn register_collection_functions(&mut self) {
        let funcs = CollectionFunctions::new();
        
        self.register("slice", Box::new(move |args| funcs.slice(args)));
        self.register("dict", Box::new(move |args| funcs.dict(args)));
        self.register("first", Box::new(move |args| funcs.first(args)));
        self.register("last", Box::new(move |args| funcs.last(args)));
        self.register("after", Box::new(move |args| funcs.after(args)));
        self.register("before", Box::new(move |args| funcs.before(args)));
        self.register("append", Box::new(move |args| funcs.append(args)));
        self.register("prepend", Box::new(move |args| funcs.prepend(args)));
        self.register("shuffle", Box::new(move |args| funcs.shuffle(args)));
        self.register("seq", Box::new(move |args| funcs.seq(args)));
        self.register("union", Box::new(move |args| funcs.union(args)));
        self.register("intersection", Box::new(move |args| funcs.intersection(args)));
        self.register("difference", Box::new(move |args| funcs.difference(args)));
        self.register("apply", Box::new(move |args| funcs.apply(args)));
    }
    
    /// 注册日期处理函数
    fn register_date_functions(&mut self) {
        let funcs = DateFunctions::new();
        
        self.register("now", Box::new(move |args| funcs.now(args)));
        self.register("dateFormat", Box::new(move |args| funcs.date_format(args)));
        self.register("time", Box::new(move |args| funcs.time(args)));
        self.register("addDate", Box::new(move |args| funcs.add_date(args)));
    }
    
    /// 注册数学函数
    fn register_math_functions(&mut self) {
        let funcs = MathFunctions::new();
        
        self.register("add", Box::new(move |args| funcs.add(args)));
        self.register("sub", Box::new(move |args| funcs.sub(args)));
        self.register("mul", Box::new(move |args| funcs.mul(args)));
        self.register("div", Box::new(move |args| funcs.div(args)));
        self.register("mod", Box::new(move |args| funcs.mod_(args)));
        self.register("math.Max", Box::new(move |args| funcs.max(args)));
        self.register("math.Min", Box::new(move |args| funcs.min(args)));
        self.register("math.Abs", Box::new(move |args| funcs.abs(args)));
    }
    
    /// 注册条件函数
    fn register_condition_functions(&mut self) {
        let funcs = ConditionFunctions::new();
        
        self.register("cond", Box::new(move |args| funcs.cond(args)));
        self.register("default", Box::new(move |args| funcs.default(args)));
        self.register("isset", Box::new(move |args| funcs.isset(args)));
        self.register("empty", Box::new(move |args| funcs.empty(args)));
    }
    
    /// 注册内容处理函数
    fn register_content_functions(&mut self) {
        let funcs = ContentFunctions::new();
        
        self.register("markdownify", Box::new(move |args| funcs.markdownify(args)));
        self.register("plainify", Box::new(move |args| funcs.plainify(args)));
        self.register("highlight", Box::new(move |args| funcs.highlight(args)));
        self.register("emojify", Box::new(move |args| funcs.emojify(args)));
    }
    
    /// 注册页面处理函数
    fn register_page_functions(&mut self) {
        let funcs = PageFunctions::new();
        
        self.register("ref", Box::new(move |args| funcs.ref_(args)));
        self.register("relref", Box::new(move |args| funcs.relref(args)));
        self.register("getPage", Box::new(move |args| funcs.get_page(args)));
        self.register("pages", Box::new(move |args| funcs.pages(args)));
        self.register("site", Box::new(move |args| funcs.site(args)));
    }
    
    /// 注册 Partial 模板函数
    fn register_partial_functions(&mut self) {
        let funcs = PartialFunctions::new();
        
        self.register("partial", Box::new(move |args| funcs.partial(args)));
        self.register("partialCached", Box::new(move |args| funcs.partial_cached(args)));
    }
    
    /// 注册单个函数
    pub fn register<F>(&mut self, name: &str, func: F)
    where
        F: Fn(&[Value]) -> Result<Value, String> + Send + Sync + 'static,
    {
        self.functions.insert(name.to_string(), Box::new(func));
    }
    
    /// 调用函数
    pub fn call(&self, name: &str, args: &[Value]) -> Result<Value, String> {
        self.functions
            .get(name)
            .ok_or_else(|| format!("Function not found: {}", name))?
            .call(args)
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
