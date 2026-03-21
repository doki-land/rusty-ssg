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
    registry.register("ref", handle_ref);
    registry.register("relref", handle_relref);
    registry.register("alert", handle_alert);
    registry.register("notice", handle_notice);
    registry.register("tip", handle_tip);
    registry.register("warning", handle_warning);
    registry.register("error", handle_error);
    registry.register("info", handle_info);
    registry.register("details", handle_details);
    registry.register("blockquote", handle_blockquote);
    registry.register("tabs", handle_tabs);
    registry.register("tab", handle_tab);
    registry.register("code", handle_code);
    registry.register("gist", handle_gist);
    registry.register("instagram", handle_instagram);
    registry.register("tiktok", handle_tiktok);
    registry.register("twitter", handle_twitter);
    registry.register("vimeo", handle_vimeo);
    registry.register("youtube", handle_youtube);
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

/// 处理 ref 短代码
///
/// 用于创建内部文档链接，生成绝对 URL
///
/// # Arguments
///
/// * `shortcode` - 短代码定义
/// * `context` - 短代码上下文
fn handle_ref(shortcode: &Shortcode, context: &ShortcodeContext) -> ShortcodeResult<String> {
    let path = shortcode
        .params
        .get("path", 0)
        .or_else(|| shortcode.params.get_positional(0))
        .ok_or_else(|| ShortcodeError::ParameterError { message: "Path parameter is required".to_string() })?;

    let anchor = shortcode.params.get_named("anchor").unwrap_or("");
    let text = shortcode.inner.as_deref().unwrap_or(path);

    let mut url = format!("/{}", path.trim_start_matches('/'));
    if !anchor.is_empty() {
        url.push_str(&format!("#{}", anchor));
    }

    Ok(format!("<a href=\"{}\">{}</a>", escape_attr(&url), escape_html(text)))
}

/// 处理 relref 短代码
///
/// 用于创建内部文档链接，生成相对 URL
///
/// # Arguments
///
/// * `shortcode` - 短代码定义
/// * `context` - 短代码上下文
fn handle_relref(shortcode: &Shortcode, context: &ShortcodeContext) -> ShortcodeResult<String> {
    let path = shortcode
        .params
        .get("path", 0)
        .or_else(|| shortcode.params.get_positional(0))
        .ok_or_else(|| ShortcodeError::ParameterError { message: "Path parameter is required".to_string() })?;

    let anchor = shortcode.params.get_named("anchor").unwrap_or("");
    let text = shortcode.inner.as_deref().unwrap_or(path);

    let mut url = path.to_string();
    if !anchor.is_empty() {
        url.push_str(&format!("#{}", anchor));
    }

    Ok(format!("<a href=\"{}\">{}</a>", escape_attr(&url), escape_html(text)))
}

/// 处理 alert 短代码
///
/// 用于显示警告信息
///
/// # Arguments
///
/// * `shortcode` - 短代码定义
/// * `context` - 短代码上下文
fn handle_alert(shortcode: &Shortcode, _context: &ShortcodeContext) -> ShortcodeResult<String> {
    let title = shortcode.params.get_named("title").unwrap_or("Alert");
    let content = shortcode.inner.as_deref().unwrap_or("");

    Ok(format!("<div class=\"alert alert-warning\"><strong>{}</strong>{}</div>", escape_html(title), content))
}

/// 处理 notice 短代码
///
/// 用于显示通知信息
///
/// # Arguments
///
/// * `shortcode` - 短代码定义
/// * `context` - 短代码上下文
fn handle_notice(shortcode: &Shortcode, _context: &ShortcodeContext) -> ShortcodeResult<String> {
    let title = shortcode.params.get_named("title").unwrap_or("Notice");
    let content = shortcode.inner.as_deref().unwrap_or("");

    Ok(format!("<div class=\"alert alert-info\"><strong>{}</strong>{}</div>", escape_html(title), content))
}

/// 处理 tip 短代码
///
/// 用于显示提示信息
///
/// # Arguments
///
/// * `shortcode` - 短代码定义
/// * `context` - 短代码上下文
fn handle_tip(shortcode: &Shortcode, _context: &ShortcodeContext) -> ShortcodeResult<String> {
    let title = shortcode.params.get_named("title").unwrap_or("Tip");
    let content = shortcode.inner.as_deref().unwrap_or("");

    Ok(format!("<div class=\"alert alert-success\"><strong>{}</strong>{}</div>", escape_html(title), content))
}

/// 处理 warning 短代码
///
/// 用于显示警告信息
///
/// # Arguments
///
/// * `shortcode` - 短代码定义
/// * `context` - 短代码上下文
fn handle_warning(shortcode: &Shortcode, _context: &ShortcodeContext) -> ShortcodeResult<String> {
    let title = shortcode.params.get_named("title").unwrap_or("Warning");
    let content = shortcode.inner.as_deref().unwrap_or("");

    Ok(format!("<div class=\"alert alert-warning\"><strong>{}</strong>{}</div>", escape_html(title), content))
}

/// 处理 error 短代码
///
/// 用于显示错误信息
///
/// # Arguments
///
/// * `shortcode` - 短代码定义
/// * `context` - 短代码上下文
fn handle_error(shortcode: &Shortcode, _context: &ShortcodeContext) -> ShortcodeResult<String> {
    let title = shortcode.params.get_named("title").unwrap_or("Error");
    let content = shortcode.inner.as_deref().unwrap_or("");

    Ok(format!("<div class=\"alert alert-danger\"><strong>{}</strong>{}</div>", escape_html(title), content))
}

/// 处理 info 短代码
///
/// 用于显示信息提示
///
/// # Arguments
///
/// * `shortcode` - 短代码定义
/// * `context` - 短代码上下文
fn handle_info(shortcode: &Shortcode, _context: &ShortcodeContext) -> ShortcodeResult<String> {
    let title = shortcode.params.get_named("title").unwrap_or("Info");
    let content = shortcode.inner.as_deref().unwrap_or("");

    Ok(format!("<div class=\"alert alert-info\"><strong>{}</strong>{}</div>", escape_html(title), content))
}

/// 处理 details 短代码
///
/// 用于创建可折叠的内容块
///
/// # Arguments
///
/// * `shortcode` - 短代码定义
/// * `context` - 短代码上下文
fn handle_details(shortcode: &Shortcode, _context: &ShortcodeContext) -> ShortcodeResult<String> {
    let summary = shortcode.params.get_named("summary").unwrap_or("Details");
    let open = shortcode.params.get_named("open").unwrap_or("false");
    let content = shortcode.inner.as_deref().unwrap_or("");

    let open_attr = if open == "true" { " open" } else { "" };

    Ok(format!("<details{}><summary>{}</summary>{}</details>", open_attr, escape_html(summary), content))
}

/// 处理 blockquote 短代码
///
/// 用于创建引用块
///
/// # Arguments
///
/// * `shortcode` - 短代码定义
/// * `context` - 短代码上下文
fn handle_blockquote(shortcode: &Shortcode, _context: &ShortcodeContext) -> ShortcodeResult<String> {
    let author = shortcode.params.get_named("author");
    let source = shortcode.params.get_named("source");
    let content = shortcode.inner.as_deref().unwrap_or("");

    let mut html = format!("<blockquote>{}</blockquote>", content);

    if author.is_some() || source.is_some() {
        let mut cite = String::new();
        if let Some(a) = author {
            cite.push_str(&format!("— {}", escape_html(a)));
        }
        if let Some(s) = source {
            if !cite.is_empty() {
                cite.push_str(", ");
            }
            cite.push_str(&format!("<cite>{}</cite>", escape_html(s)));
        }
        html = format!("<blockquote>{}<footer>{}</footer></blockquote>", content, cite);
    }

    Ok(html)
}

/// 处理 tabs 短代码
///
/// 用于创建标签页容器
///
/// # Arguments
///
/// * `shortcode` - 短代码定义
/// * `context` - 短代码上下文
fn handle_tabs(shortcode: &Shortcode, _context: &ShortcodeContext) -> ShortcodeResult<String> {
    let id = shortcode.params.get_named("id").unwrap_or("tabs");
    let content = shortcode.inner.as_deref().unwrap_or("");

    Ok(format!("<div class=\"tabs\" id=\"{}\">{}</div>", escape_attr(id), content))
}

/// 处理 tab 短代码
///
/// 用于创建单个标签页
///
/// # Arguments
///
/// * `shortcode` - 短代码定义
/// * `context` - 短代码上下文
fn handle_tab(shortcode: &Shortcode, _context: &ShortcodeContext) -> ShortcodeResult<String> {
    let name = shortcode
        .params
        .get("name", 0)
        .or_else(|| shortcode.params.get_positional(0))
        .ok_or_else(|| ShortcodeError::ParameterError { message: "Name parameter is required".to_string() })?;
    let active = shortcode.params.get_named("active").unwrap_or("false");
    let content = shortcode.inner.as_deref().unwrap_or("");

    let active_class = if active == "true" { " active" } else { "" };

    Ok(format!("<div class=\"tab{}\" data-tab-name=\"{}\">{}</div>", active_class, escape_attr(name), content))
}

/// 处理 code 短代码
///
/// 用于显示代码块
///
/// # Arguments
///
/// * `shortcode` - 短代码定义
/// * `context` - 短代码上下文
fn handle_code(shortcode: &Shortcode, _context: &ShortcodeContext) -> ShortcodeResult<String> {
    let language = shortcode.params.get("lang", 0).unwrap_or("");
    let linenos = shortcode.params.get_named("linenos").unwrap_or("false");
    let code = shortcode.inner.as_deref().unwrap_or("");

    let mut class = if !language.is_empty() { format!("language-{}", language) } else { String::new() };
    if linenos == "true" || linenos == "1" {
        if !class.is_empty() {
            class.push(' ');
        }
        class.push_str("line-numbers");
    }

    if class.is_empty() {
        Ok(format!("<pre><code>{}</code></pre>", escape_html(code)))
    }
    else {
        Ok(format!("<pre><code class=\"{}\">{}</code></pre>", class, escape_html(code)))
    }
}

/// 处理 instagram 短代码
///
/// 用于嵌入 Instagram 帖子
///
/// # Arguments
///
/// * `shortcode` - 短代码定义
/// * `context` - 短代码上下文
fn handle_instagram(shortcode: &Shortcode, _context: &ShortcodeContext) -> ShortcodeResult<String> {
    let post_id = shortcode
        .params
        .get("id", 0)
        .or_else(|| shortcode.params.get_positional(0))
        .ok_or_else(|| ShortcodeError::ParameterError { message: "Post ID parameter is required".to_string() })?;

    Ok(format!(
        "<blockquote class=\"instagram-media\" data-instgrm-permalink=\"https://www.instagram.com/p/{}/\" data-instgrm-version=\"13\"></blockquote><script async src=\"//www.instagram.com/embed.js\"></script>",
        post_id
    ))
}

/// 处理 tiktok 短代码
///
/// 用于嵌入 TikTok 视频
///
/// # Arguments
///
/// * `shortcode` - 短代码定义
/// * `context` - 短代码上下文
fn handle_tiktok(shortcode: &Shortcode, _context: &ShortcodeContext) -> ShortcodeResult<String> {
    let video_id = shortcode
        .params
        .get("id", 0)
        .or_else(|| shortcode.params.get_positional(0))
        .ok_or_else(|| ShortcodeError::ParameterError { message: "Video ID parameter is required".to_string() })?;

    Ok(format!(
        "<blockquote class=\"tiktok-embed\" cite=\"https://www.tiktok.com/@{}\" data-video-id=\"{}\"><section></section></blockquote><script async src=\"https://www.tiktok.com/embed.js\"></script>",
        video_id, video_id
    ))
}
