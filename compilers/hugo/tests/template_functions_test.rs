//! 模板函数测试
//! 测试 Hugo 模板函数的功能

use hugo::compiler::template_functions::collection::CollectionFunctions;
use serde_json::json;

#[test]
fn test_slice() {
    let funcs = CollectionFunctions::new();

    assert_eq!(funcs.slice(&[json!(1), json!(2), json!(3)]).unwrap(), json!([1, 2, 3]));
}

#[test]
fn test_dict() {
    let funcs = CollectionFunctions::new();

    let result = funcs.dict(&[json!("key1"), json!("value1"), json!("key2"), json!("value2")]).unwrap();

    assert_eq!(result["key1"], json!("value1"));
    assert_eq!(result["key2"], json!("value2"));
}

#[test]
fn test_first() {
    let funcs = CollectionFunctions::new();

    assert_eq!(funcs.first(&[json!(2), json!([1, 2, 3, 4, 5])]).unwrap(), json!([1, 2]));
}

#[test]
fn test_last() {
    let funcs = CollectionFunctions::new();

    assert_eq!(funcs.last(&[json!(2), json!([1, 2, 3, 4, 5])]).unwrap(), json!([4, 5]));
}

#[test]
fn test_seq() {
    let funcs = CollectionFunctions::new();

    assert_eq!(funcs.seq(&[json!(5)]).unwrap(), json!([1, 2, 3, 4, 5]));

    assert_eq!(funcs.seq(&[json!(10), json!(5), json!(2)]).unwrap(), json!([5, 7, 9]));
}

#[test]
fn test_union() {
    let funcs = CollectionFunctions::new();

    assert_eq!(funcs.union(&[json!([1, 2, 3]), json!([3, 4, 5])]).unwrap(), json!([1, 2, 3, 4, 5]));
}

#[test]
fn test_intersection() {
    let funcs = CollectionFunctions::new();

    assert_eq!(funcs.intersection(&[json!([1, 2, 3]), json!([3, 4, 5])]).unwrap(), json!([3]));
}

#[test]
fn test_difference() {
    let funcs = CollectionFunctions::new();

    assert_eq!(funcs.difference(&[json!([1, 2, 3, 4]), json!([3, 4, 5])]).unwrap(), json!([1, 2]));
}

// 条件函数测试
use hugo::compiler::template_functions::condition::ConditionFunctions;

#[test]
fn test_cond() {
    let funcs = ConditionFunctions::new();

    assert_eq!(funcs.cond(&[json!(true), json!("yes"), json!("no")]).unwrap(), json!("yes"));
    assert_eq!(funcs.cond(&[json!(false), json!("yes"), json!("no")]).unwrap(), json!("no"));
}

#[test]
fn test_default() {
    let funcs = ConditionFunctions::new();

    assert_eq!(funcs.default(&[json!(null), json!("default")]).unwrap(), json!("default"));
    assert_eq!(funcs.default(&[json!("value"), json!("default")]).unwrap(), json!("value"));
}

#[test]
fn test_isset() {
    let funcs = ConditionFunctions::new();

    assert_eq!(funcs.isset(&[json!("value")]).unwrap(), json!(true));
    assert_eq!(funcs.isset(&[json!(null)]).unwrap(), json!(false));
}

#[test]
fn test_empty() {
    let funcs = ConditionFunctions::new();

    assert_eq!(funcs.empty(&[json!(null)]).unwrap(), json!(true));
    assert_eq!(funcs.empty(&[json!("")]).unwrap(), json!(true));
    assert_eq!(funcs.empty(&[json!([])]).unwrap(), json!(true));
    assert_eq!(funcs.empty(&[json!({})]).unwrap(), json!(true));
    assert_eq!(funcs.empty(&[json!(0)]).unwrap(), json!(true));
    assert_eq!(funcs.empty(&[json!(false)]).unwrap(), json!(true));
    assert_eq!(funcs.empty(&[json!("value")]).unwrap(), json!(false));
}

// 内容函数测试
use hugo::compiler::template_functions::content::ContentFunctions;

#[test]
fn test_markdownify() {
    let funcs = ContentFunctions::new();
    let result = funcs.markdownify(&[json!("# Hello")]).unwrap();
    assert!(result.is_string());
}

#[test]
fn test_plainify() {
    let funcs = ContentFunctions::new();
    assert_eq!(funcs.plainify(&[json!("<p>Hello <b>World</b></p>")]).unwrap(), json!("Hello World"));
}

#[test]
fn test_highlight() {
    let funcs = ContentFunctions::new();
    let result = funcs.highlight(&[json!("let x = 1;")]).unwrap();
    assert!(result.is_string());
}

#[test]
fn test_emojify() {
    let funcs = ContentFunctions::new();
    assert_eq!(funcs.emojify(&[json!("Hello :smile:")]).unwrap(), json!("Hello 😊"));
}

// 日期函数测试
use hugo::compiler::template_functions::date::DateFunctions;

#[test]
fn test_now() {
    let funcs = DateFunctions::new();
    let result = funcs.now(&[]).unwrap();
    assert!(result.is_string());
}

#[test]
fn test_date_format() {
    let funcs = DateFunctions::new();

    assert_eq!(funcs.date_format(&[json!("%Y-%m-%d"), json!("2024-01-15")]).unwrap(), json!("2024-01-15"));
}

#[test]
fn test_time() {
    let funcs = DateFunctions::new();
    let result = funcs.time(&[json!("2024-01-15")]).unwrap();
    assert!(result.is_string());
}

#[test]
fn test_add_date() {
    let funcs = DateFunctions::new();
    let result = funcs.add_date(&[json!("2024-01-15"), json!(1), json!(2), json!(3)]).unwrap();
    assert!(result.is_string());
}

// 数学函数测试
use hugo::compiler::template_functions::math::MathFunctions;

#[test]
fn test_add() {
    let funcs = MathFunctions::new();
    assert_eq!(funcs.add(&[json!(1), json!(2)]).unwrap(), json!(3));
}

#[test]
fn test_sub() {
    let funcs = MathFunctions::new();
    assert_eq!(funcs.sub(&[json!(5), json!(2)]).unwrap(), json!(3));
}

#[test]
fn test_mul() {
    let funcs = MathFunctions::new();
    assert_eq!(funcs.mul(&[json!(2), json!(3)]).unwrap(), json!(6));
}

#[test]
fn test_div() {
    let funcs = MathFunctions::new();
    assert_eq!(funcs.div(&[json!(6), json!(2)]).unwrap(), json!(3));
}

#[test]
fn test_mod() {
    let funcs = MathFunctions::new();
    assert_eq!(funcs.mod_(&[json!(7), json!(3)]).unwrap(), json!(1));
}

#[test]
fn test_max() {
    let funcs = MathFunctions::new();
    assert_eq!(funcs.max(&[json!(1), json!(5), json!(3)]).unwrap(), json!(5));
}

#[test]
fn test_min() {
    let funcs = MathFunctions::new();
    assert_eq!(funcs.min(&[json!(1), json!(5), json!(3)]).unwrap(), json!(1));
}

#[test]
fn test_abs() {
    let funcs = MathFunctions::new();
    assert_eq!(funcs.abs(&[json!(-5)]).unwrap(), json!(5));
    assert_eq!(funcs.abs(&[json!(5)]).unwrap(), json!(5));
}

// 模板函数注册表测试
use hugo::compiler::template_functions::TemplateFunctions;

#[test]
fn test_string_functions() {
    let funcs = TemplateFunctions::new();

    assert_eq!(funcs.call("lower", &[json!("HELLO")]).unwrap(), json!("hello"));

    assert_eq!(funcs.call("upper", &[json!("hello")]).unwrap(), json!("HELLO"));

    assert_eq!(funcs.call("title", &[json!("hello world")]).unwrap(), json!("Hello World"));
}

#[test]
fn test_url_functions() {
    let funcs = TemplateFunctions::new();

    assert_eq!(funcs.call("urlize", &[json!("Hello World!")]).unwrap(), json!("hello-world"));
}

#[test]
fn test_collection_functions() {
    let funcs = TemplateFunctions::new();

    let arr = json!([1, 2, 3, 4, 5]);
    assert_eq!(funcs.call("first", &[json!(2), arr.clone()]).unwrap(), json!([1, 2]));

    assert_eq!(funcs.call("last", &[json!(2), arr]).unwrap(), json!([4, 5]));
}

// 页面函数测试
use hugo::compiler::template_functions::page::PageFunctions;

#[test]
fn test_ref() {
    let funcs = PageFunctions::new();
    assert_eq!(funcs.ref_(&[json!("about")]).unwrap(), json!("/about"));
}

#[test]
fn test_relref() {
    let funcs = PageFunctions::new();
    assert_eq!(funcs.relref(&[json!("about")]).unwrap(), json!("/about"));
}

#[test]
fn test_get_page() {
    let funcs = PageFunctions::new();
    let result = funcs.get_page(&[json!("about")]).unwrap();
    assert!(result.is_object());
}

#[test]
fn test_pages() {
    let funcs = PageFunctions::new();
    let result = funcs.pages(&[]).unwrap();
    assert!(result.is_array());
}

#[test]
fn test_site() {
    let funcs = PageFunctions::new();
    let result = funcs.site(&[]).unwrap();
    assert!(result.is_object());
}

// Partial 函数测试
use hugo::compiler::template_functions::partial::PartialFunctions;

#[test]
fn test_partial() {
    let funcs = PartialFunctions::new();

    let result = funcs.partial(&[json!("header.html")]).unwrap();
    assert!(result.as_str().unwrap().contains("header.html"));
}

#[test]
fn test_partial_cached() {
    let funcs = PartialFunctions::new();

    // 第一次调用，应该生成新内容
    let result1 = funcs.partial_cached(&[json!("footer.html")]).unwrap();
    assert!(result1.as_str().unwrap().contains("footer.html"));
    assert_eq!(funcs.cache_size(), 1);

    // 第二次调用，应该返回缓存的内容
    let result2 = funcs.partial_cached(&[json!("footer.html")]).unwrap();
    assert_eq!(result1, result2);
    assert_eq!(funcs.cache_size(), 1);

    // 清除缓存
    funcs.clear_cache();
    assert_eq!(funcs.cache_size(), 0);
}

#[test]
fn test_partial_cached_with_key() {
    let funcs = PartialFunctions::new();

    // 使用不同的缓存键
    let result1 = funcs.partial_cached(&[json!("sidebar.html"), json!("key1")]).unwrap();
    let result2 = funcs.partial_cached(&[json!("sidebar.html"), json!("key2")]).unwrap();

    assert!(result1.as_str().unwrap().contains("sidebar.html"));
    assert!(result2.as_str().unwrap().contains("sidebar.html"));
    assert_eq!(funcs.cache_size(), 2);
}

// 字符串函数测试
use hugo::compiler::template_functions::string::StringFunctions;

#[test]
fn test_lower() {
    let funcs = StringFunctions::new();
    assert_eq!(funcs.lower(&[json!("HELLO")]).unwrap(), json!("hello"));
}

#[test]
fn test_upper() {
    let funcs = StringFunctions::new();
    assert_eq!(funcs.upper(&[json!("hello")]).unwrap(), json!("HELLO"));
}

#[test]
fn test_title() {
    let funcs = StringFunctions::new();
    assert_eq!(funcs.title(&[json!("hello world")]).unwrap(), json!("Hello World"));
}

#[test]
fn test_truncate() {
    let funcs = StringFunctions::new();

    assert_eq!(funcs.truncate(&[json!(5), json!("Hello World")]).unwrap(), json!("Hello..."));

    assert_eq!(funcs.truncate(&[json!(5), json!("Hello World"), json!("..")]).unwrap(), json!("Hello.."));
}

#[test]
fn test_replace() {
    let funcs = StringFunctions::new();

    assert_eq!(funcs.replace(&[json!("Hello World"), json!("World"), json!("Hugo")]).unwrap(), json!("Hello Hugo"));
}

#[test]
fn test_trim_suffix() {
    let funcs = StringFunctions::new();

    assert_eq!(funcs.trim_suffix(&[json!("Hello World"), json!("World")]).unwrap(), json!("Hello "));

    assert_eq!(funcs.trim_suffix(&[json!("Hello"), json!("World")]).unwrap(), json!("Hello"));
}

#[test]
fn test_trim_prefix() {
    let funcs = StringFunctions::new();

    assert_eq!(funcs.trim_prefix(&[json!("Hello World"), json!("Hello")]).unwrap(), json!(" World"));

    assert_eq!(funcs.trim_prefix(&[json!("Hello"), json!("World")]).unwrap(), json!("Hello"));
}

#[test]
fn test_replace_re() {
    let funcs = StringFunctions::new();

    assert_eq!(
        funcs.replace_re(&[json!(r"\d+"), json!("[NUMBER]"), json!("There are 42 apples")]).unwrap(),
        json!("There are [NUMBER] apples")
    );
}

// URL 函数测试
use hugo::compiler::template_functions::url::UrlFunctions;

#[test]
fn test_rel_url() {
    let funcs = UrlFunctions::new();

    assert_eq!(funcs.rel_url(&[json!("path/to/file")]).unwrap(), json!("/path/to/file"));

    assert_eq!(funcs.rel_url(&[json!("/already/absolute")]).unwrap(), json!("/already/absolute"));
}

#[test]
fn test_abs_url() {
    let funcs = UrlFunctions::new().with_base_url("https://example.com".to_string());

    assert_eq!(funcs.abs_url(&[json!("path/to/file")]).unwrap(), json!("https://example.com/path/to/file"));

    assert_eq!(funcs.abs_url(&[json!("/absolute/path")]).unwrap(), json!("https://example.com/absolute/path"));
}

#[test]
fn test_urlize() {
    let funcs = UrlFunctions::new();

    assert_eq!(funcs.urlize(&[json!("Hello World")]).unwrap(), json!("hello-world"));

    assert_eq!(funcs.urlize(&[json!("This is a Test!")]).unwrap(), json!("this-is-a-test"));

    assert_eq!(funcs.urlize(&[json!("Multiple   Spaces")]).unwrap(), json!("multiple-spaces"));
}

#[test]
fn test_abs_lang_url() {
    let funcs = UrlFunctions::new().with_base_url("https://example.com".to_string()).with_current_lang("en".to_string());

    assert_eq!(funcs.abs_lang_url(&[json!("path/to/file")]).unwrap(), json!("https://example.com/en/path/to/file"));

    assert_eq!(funcs.abs_lang_url(&[json!("/absolute/path")]).unwrap(), json!("https://example.com/en/absolute/path"));
}

#[test]
fn test_rel_lang_url() {
    let funcs = UrlFunctions::new().with_current_lang("en".to_string());

    assert_eq!(funcs.rel_lang_url(&[json!("path/to/file")]).unwrap(), json!("/en/path/to/file"));

    assert_eq!(funcs.rel_lang_url(&[json!("/absolute/path")]).unwrap(), json!("/en/absolute/path"));
}
