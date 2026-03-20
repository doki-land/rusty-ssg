//! Hugo 模板函数库
//! 提供 Hugo 兼容的模板函数实现

use handlebars::{Context, Handlebars, Helper, HelperResult, Output, RenderContext, RenderError};
use serde_json::Value;

/// 注册所有 Hugo 模板函数到 Handlebars 实例
///
/// # Arguments
///
/// * `handlebars` - Handlebars 模板引擎实例
pub fn register_hugo_functions(handlebars: &mut Handlebars) {
    handlebars.register_helper("upper", Box::new(upper));
    handlebars.register_helper("lower", Box::new(lower));
    handlebars.register_helper("markdownify", Box::new(markdownify));
    handlebars.register_helper("dateFormat", Box::new(date_format));
    handlebars.register_helper("jsonify", Box::new(jsonify));
    handlebars.register_helper("absurl", Box::new(absurl));
    handlebars.register_helper("relurl", Box::new(relurl));
    handlebars.register_helper("humanize", Box::new(humanize));
    handlebars.register_helper("truncate", Box::new(truncate));
    handlebars.register_helper("default", Box::new(default_helper));
}

/// 字符串转大写
///
/// # Examples
///
/// ```hbs
/// {{upper "hello world"}}
/// ```
fn upper(h: &Helper, _: &Handlebars, _: &Context, _: &mut RenderContext, out: &mut dyn Output) -> HelperResult {
    let value = h.param(0).and_then(|v| v.value().as_str()).unwrap_or("");
    out.write(value.to_uppercase().as_ref())?;
    Ok(())
}

/// 字符串转小写
///
/// # Examples
///
/// ```hbs
/// {{lower "HELLO WORLD"}}
/// ```
fn lower(h: &Helper, _: &Handlebars, _: &Context, _: &mut RenderContext, out: &mut dyn Output) -> HelperResult {
    let value = h.param(0).and_then(|v| v.value().as_str()).unwrap_or("");
    out.write(value.to_lowercase().as_ref())?;
    Ok(())
}

/// Markdown 渲染（简化实现）
///
/// # Examples
///
/// ```hbs
/// {{markdownify "**bold text**"}
/// ```
fn markdownify(h: &Helper, _: &Handlebars, _: &Context, _: &mut RenderContext, out: &mut dyn Output) -> HelperResult {
    let value = h.param(0).and_then(|v| v.value().as_str()).unwrap_or("");
    out.write(value)?;
    Ok(())
}

/// 日期格式化
///
/// # Examples
///
/// ```hbs
/// {{dateFormat "2023-01-01" "January 2, 2006"}
/// ```
fn date_format(h: &Helper, _: &Handlebars, _: &Context, _: &mut RenderContext, out: &mut dyn Output) -> HelperResult {
    let date = h.param(0).and_then(|v| v.value().as_str()).unwrap_or("");
    out.write(date)?;
    Ok(())
}

/// JSON 序列化
///
/// # Examples
///
/// ```hbs
/// {{jsonify .}}
/// ```
fn jsonify(h: &Helper, _: &Handlebars, _: &Context, _: &mut RenderContext, out: &mut dyn Output) -> HelperResult {
    let value = h.param(0).map(|v| v.value()).unwrap_or(&Value::Null);
    out.write(serde_json::to_string(value).unwrap_or_default().as_ref())?;
    Ok(())
}

/// 构建绝对 URL
///
/// # Examples
///
/// ```hbs
/// {{absurl "/about"}}
/// ```
fn absurl(h: &Helper, _: &Handlebars, _: &Context, _: &mut RenderContext, out: &mut dyn Output) -> HelperResult {
    let path = h.param(0).and_then(|v| v.value().as_str()).unwrap_or("");
    out.write(path)?;
    Ok(())
}

/// 构建相对 URL
///
/// # Examples
///
/// ```hbs
/// {{relurl "/about"}}
/// ```
fn relurl(h: &Helper, _: &Handlebars, _: &Context, _: &mut RenderContext, out: &mut dyn Output) -> HelperResult {
    let path = h.param(0).and_then(|v| v.value().as_str()).unwrap_or("");
    out.write(path)?;
    Ok(())
}

/// 人性化显示（简化实现）
///
/// # Examples
///
/// ```hbs
/// {{humanize 1000}}
/// ```
fn humanize(h: &Helper, _: &Handlebars, _: &Context, _: &mut RenderContext, out: &mut dyn Output) -> HelperResult {
    let value = h.param(0).and_then(|v| v.value().as_i64()).unwrap_or(0);
    out.write(value.to_string().as_ref())?;
    Ok(())
}

/// 字符串截断
///
/// # Examples
///
/// ```hbs
/// {{truncate "Long text" 5}}
/// ```
fn truncate(h: &Helper, _: &Handlebars, _: &Context, _: &mut RenderContext, out: &mut dyn Output) -> HelperResult {
    let value = h.param(0).and_then(|v| v.value().as_str()).unwrap_or("");
    let length = h.param(1).and_then(|v| v.value().as_u64()).unwrap_or(50) as usize;
    let truncated = if value.len() > length {
        let mut result = value[0..length].to_string();
        result.push_str("...");
        result
    } else {
        value.to_string()
    };
    out.write(truncated.as_ref())?;
    Ok(())
}

/// 默认值
///
/// # Examples
///
/// ```hbs
/// {{default .Title "Untitled"}}
/// ```
fn default_helper(h: &Helper, _: &Handlebars, _: &Context, _: &mut RenderContext, out: &mut dyn Output) -> HelperResult {
    let value = h.param(0).map(|v| v.value()).unwrap_or(&Value::Null);
    let default_value = h.param(1).map(|v| v.value()).unwrap_or(&Value::Null);
    
    let result = if value.is_null() || (value.is_string() && value.as_str().unwrap_or("").is_empty()) {
        default_value
    } else {
        value
    };
    
    let result_str = result.as_str().unwrap_or("");
    out.write(result_str)?;
    Ok(())
}
