//! 内置短代码实现
//!
//! 提供常见的 Hugo 风格内置短代码。

use crate::compiler::shortcodes::{
    registry::ShortcodeRegistry,
    types::{Shortcode, ShortcodeContext, ShortcodeError, ShortcodeResult},
};

/// 注册所有内置短代码
///
/// # Arguments
///
/// * `registry` - 短代码注册表
pub fn register_builtins(registry: &mut ShortcodeRegistry) {
    registry.register("highlight", handle_highlight);
    registry.register("figure", handle_figure);
    registry.register("gist", handle_gist);
    registry.register("twitter", handle_twitter);
    registry.register("youtube", handle_youtube);
    registry.register("vimeo", handle_vimeo);
}

/// 处理 highlight 短代码
///
/// 用于代码高亮显示
///
/// # Arguments
///
/// * `shortcode` - 短代码定义
/// * `context` - 短代码上下文
fn handle_highlight(shortcode: &Shortcode, _context: &ShortcodeContext) -> ShortcodeResult<String> {
    let language = shortcode
        .params
        .get("lang", 0)
        .ok_or_else(|| ShortcodeError::ParameterError { message: "Language parameter is required".to_string() })?;

    let linenos = shortcode.params.get_named("linenos").unwrap_or("false");
    let hl_lines = shortcode.params.get_named("hl_lines").unwrap_or("");

    let code = shortcode.inner.as_deref().unwrap_or("");

    let mut class = format!("language-{}", language);
    if linenos == "true" || linenos == "1" {
        class.push_str(" line-numbers");
    }

    let hl_attr = if !hl_lines.is_empty() { format!(" data-line=\"{}\"", hl_lines) } else { String::new() };

    Ok(format!("<pre><code class=\"{}\"{}>{}</code></pre>", class, hl_attr, escape_html(code)))
}

/// 处理 figure 短代码
///
/// 用于显示图片，支持标题和说明
///
/// # Arguments
///
/// * `shortcode` - 短代码定义
/// * `context` - 短代码上下文
fn handle_figure(shortcode: &Shortcode, _context: &ShortcodeContext) -> ShortcodeResult<String> {
    let src = shortcode
        .params
        .get("src", 0)
        .ok_or_else(|| ShortcodeError::ParameterError { message: "Src parameter is required".to_string() })?;

    let alt = shortcode.params.get("alt", 1).unwrap_or("");
    let title = shortcode.params.get_named("title").unwrap_or("");
    let caption = shortcode.params.get_named("caption").unwrap_or("");
    let link = shortcode.params.get_named("link").unwrap_or("");
    let target = shortcode.params.get_named("target").unwrap_or("_blank");
    let rel = shortcode.params.get_named("rel").unwrap_or("noopener noreferrer");
    let width = shortcode.params.get_named("width").unwrap_or("");
    let height = shortcode.params.get_named("height").unwrap_or("");

    let mut img_attrs = vec![format!("src=\"{}\"", escape_attr(src)), format!("alt=\"{}\"", escape_attr(alt))];

    if !title.is_empty() {
        img_attrs.push(format!("title=\"{}\"", escape_attr(title)));
    }
    if !width.is_empty() {
        img_attrs.push(format!("width=\"{}\"", escape_attr(width)));
    }
    if !height.is_empty() {
        img_attrs.push(format!("height=\"{}\"", escape_attr(height)));
    }

    let img_tag = format!("<img {} />", img_attrs.join(" "));

    let content = if !link.is_empty() {
        format!(
            "<a href=\"{}\" target=\"{}\" rel=\"{}\">{}</a>",
            escape_attr(link),
            escape_attr(target),
            escape_attr(rel),
            img_tag
        )
    }
    else {
        img_tag
    };

    let figcaption = if !caption.is_empty() {
        format!("<figcaption>{}</figcaption>", escape_html(caption))
    }
    else if !title.is_empty() {
        format!("<figcaption>{}</figcaption>", escape_html(title))
    }
    else {
        String::new()
    };

    Ok(format!("<figure>{}{}</figure>", content, figcaption))
}

/// 处理 gist 短代码
///
/// 用于嵌入 GitHub Gist
///
/// # Arguments
///
/// * `shortcode` - 短代码定义
/// * `context` - 短代码上下文
fn handle_gist(shortcode: &Shortcode, _context: &ShortcodeContext) -> ShortcodeResult<String> {
    let gist_id = shortcode
        .params
        .get("id", 0)
        .or_else(|| shortcode.params.get_positional(0))
        .ok_or_else(|| ShortcodeError::ParameterError { message: "Gist ID parameter is required".to_string() })?;

    let file = shortcode.params.get_named("file");

    let url = if let Some(f) = file {
        format!("https://gist.github.com/{}.js?file={}", gist_id, escape_attr(f))
    }
    else {
        format!("https://gist.github.com/{}.js", gist_id)
    };

    Ok(format!("<script src=\"{}\"></script>", url))
}

/// 处理 twitter 短代码
///
/// 用于嵌入 Twitter 推文
///
/// # Arguments
///
/// * `shortcode` - 短代码定义
/// * `context` - 短代码上下文
fn handle_twitter(shortcode: &Shortcode, _context: &ShortcodeContext) -> ShortcodeResult<String> {
    let tweet_id = shortcode
        .params
        .get("id", 0)
        .or_else(|| shortcode.params.get_positional(0))
        .ok_or_else(|| ShortcodeError::ParameterError { message: "Tweet ID parameter is required".to_string() })?;

    Ok(format!(
        "<blockquote class=\"twitter-tweet\"><a href=\"https://twitter.com/i/web/status/{}\"></a></blockquote><script async src=\"https://platform.twitter.com/widgets.js\" charset=\"utf-8\"></script>",
        tweet_id
    ))
}

/// 处理 youtube 短代码
///
/// 用于嵌入 YouTube 视频
///
/// # Arguments
///
/// * `shortcode` - 短代码定义
/// * `context` - 短代码上下文
fn handle_youtube(shortcode: &Shortcode, _context: &ShortcodeContext) -> ShortcodeResult<String> {
    let video_id = shortcode
        .params
        .get("id", 0)
        .or_else(|| shortcode.params.get_positional(0))
        .ok_or_else(|| ShortcodeError::ParameterError { message: "Video ID parameter is required".to_string() })?;

    let width = shortcode.params.get_named("width").unwrap_or("560");
    let height = shortcode.params.get_named("height").unwrap_or("315");
    let autoplay = shortcode.params.get_named("autoplay").unwrap_or("0");
    let controls = shortcode.params.get_named("controls").unwrap_or("1");
    let loop_val = shortcode.params.get_named("loop").unwrap_or("0");

    let mut params = vec![format!("autoplay={}", autoplay), format!("controls={}", controls), format!("loop={}", loop_val)];

    if loop_val == "1" {
        params.push(format!("playlist={}", video_id));
    }

    Ok(format!(
        "<div class=\"youtube-video\"><iframe width=\"{}\" height=\"{}\" src=\"https://www.youtube.com/embed/{}?{}\" frameborder=\"0\" allow=\"accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture\" allowfullscreen></iframe></div>",
        width,
        height,
        video_id,
        params.join("&amp;")
    ))
}

/// 处理 vimeo 短代码
///
/// 用于嵌入 Vimeo 视频
///
/// # Arguments
///
/// * `shortcode` - 短代码定义
/// * `context` - 短代码上下文
fn handle_vimeo(shortcode: &Shortcode, _context: &ShortcodeContext) -> ShortcodeResult<String> {
    let video_id = shortcode
        .params
        .get("id", 0)
        .or_else(|| shortcode.params.get_positional(0))
        .ok_or_else(|| ShortcodeError::ParameterError { message: "Video ID parameter is required".to_string() })?;

    let width = shortcode.params.get_named("width").unwrap_or("640");
    let height = shortcode.params.get_named("height").unwrap_or("360");
    let autoplay = shortcode.params.get_named("autoplay").unwrap_or("0");
    let loop_val = shortcode.params.get_named("loop").unwrap_or("0");
    let title = shortcode.params.get_named("title").unwrap_or("1");
    let byline = shortcode.params.get_named("byline").unwrap_or("1");
    let portrait = shortcode.params.get_named("portrait").unwrap_or("1");

    let params = vec![
        format!("autoplay={}", autoplay),
        format!("loop={}", loop_val),
        format!("title={}", title),
        format!("byline={}", byline),
        format!("portrait={}", portrait),
    ];

    Ok(format!(
        "<div class=\"vimeo-video\"><iframe src=\"https://player.vimeo.com/video/{}?{}\" width=\"{}\" height=\"{}\" frameborder=\"0\" allow=\"autoplay; fullscreen\" allowfullscreen></iframe></div>",
        video_id,
        params.join("&amp;"),
        width,
        height
    ))
}

/// HTML 转义函数
fn escape_html(s: &str) -> String {
    s.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;").replace('"', "&quot;").replace('\'', "&#39;")
}

/// 属性值转义函数
fn escape_attr(s: &str) -> String {
    s.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;").replace('"', "&quot;").replace('\'', "&#39;")
}
