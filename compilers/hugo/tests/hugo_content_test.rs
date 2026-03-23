//! Hugo 内容模块测试
//! 测试 Hugo 内容处理相关的功能

use hugo::types::document::{HugoContentIndex, HugoContentLoader, HugoFrontMatter, HugoPage};
use std::path::PathBuf;

#[test]
fn test_generate_summary_with_more_separator() {
    let content = "This is the summary<!--more-->This is the rest of the content";
    let summary = HugoContentLoader::generate_summary(content);
    assert_eq!(summary, Some("This is the summary".to_string()));
}

#[test]
fn test_generate_summary_without_more_separator() {
    let content = "This is a test content with multiple words to test the automatic summary generation. ".repeat(10);
    let summary = HugoContentLoader::generate_summary(&content);
    assert!(summary.is_some());
    let summary_str = summary.unwrap();
    let words: Vec<&str> = summary_str.split_whitespace().collect();
    assert!(words.len() <= 70);
}

#[test]
fn test_generate_summary_empty_content() {
    let content = "";
    let summary = HugoContentLoader::generate_summary(content);
    assert_eq!(summary, None);
}

#[test]
fn test_generate_summary_short_content() {
    let content = "Short content";
    let summary = HugoContentLoader::generate_summary(content);
    assert_eq!(summary, Some("Short content".to_string()));
}

#[test]
fn test_generate_table_of_contents() {
    let content = r#"# Heading 1

Some content

## Heading 2

More content

### Heading 3

Even more content

# Another Heading 1

Final content"#;

    let toc = HugoContentLoader::generate_table_of_contents(content);
    assert!(toc.is_some());

    let toc = toc.unwrap();
    assert_eq!(toc.items.len(), 2);
    assert_eq!(toc.items[0].text, "Heading 1");
    assert_eq!(toc.items[0].level, 1);
    assert_eq!(toc.items[0].children.len(), 2);
    assert_eq!(toc.items[0].children[0].text, "Heading 2");
    assert_eq!(toc.items[0].children[0].level, 2);
    assert_eq!(toc.items[0].children[0].children[0].text, "Heading 3");
    assert_eq!(toc.items[0].children[0].children[0].level, 3);
    assert_eq!(toc.items[1].text, "Another Heading 1");
    assert_eq!(toc.items[1].level, 1);
}

#[test]
fn test_generate_table_of_contents_empty() {
    let content = "No headings here";
    let toc = HugoContentLoader::generate_table_of_contents(content);
    assert_eq!(toc, None);
}

#[test]
fn test_generate_table_of_contents_html() {
    let content = r#"# Heading 1

## Heading 2"#;

    let toc = HugoContentLoader::generate_table_of_contents(content);
    assert!(toc.is_some());

    let toc = toc.unwrap();
    assert!(toc.html.contains("<nav id=\"TableOfContents\">"));
    assert!(toc.html.contains("<a href=\"#heading-1\">Heading 1</a>"));
    assert!(toc.html.contains("<a href=\"#heading-2\">Heading 2</a>"));
}

#[test]
fn test_generate_permalink() {
    let path = PathBuf::from("posts/2024-01-01-my-post.md");
    let frontmatter = HugoFrontMatter::new();

    let permalink = HugoContentLoader::generate_permalink(&path, &frontmatter);
    assert_eq!(permalink, "/posts/my-post/");
}

#[test]
fn test_generate_permalink_with_slug() {
    let path = PathBuf::from("posts/2024-01-01-my-post.md");
    let mut frontmatter = HugoFrontMatter::new();
    frontmatter.slug = Some("custom-slug".to_string());

    let permalink = HugoContentLoader::generate_permalink(&path, &frontmatter);
    assert_eq!(permalink, "/posts/custom-slug/");
}

#[test]
fn test_generate_permalink_index() {
    let path = PathBuf::from("posts/_index.md");
    let frontmatter = HugoFrontMatter::new();

    let permalink = HugoContentLoader::generate_permalink(&path, &frontmatter);
    assert_eq!(permalink, "/posts/");
}

#[test]
fn test_generate_permalink_root() {
    let path = PathBuf::from("index.md");
    let frontmatter = HugoFrontMatter::new();

    let permalink = HugoContentLoader::generate_permalink(&path, &frontmatter);
    assert_eq!(permalink, "/");
}

#[test]
fn test_sort_by_date() {
    let mut index = HugoContentIndex::new();

    // 创建测试页面
    let mut page1 = HugoPage::new(PathBuf::from("page1.md"), PathBuf::from("page1.md"));
    page1.frontmatter.date = Some("2024-01-01".to_string());

    let mut page2 = HugoPage::new(PathBuf::from("page2.md"), PathBuf::from("page2.md"));
    page2.frontmatter.date = Some("2024-02-01".to_string());

    let mut page3 = HugoPage::new(PathBuf::from("page3.md"), PathBuf::from("page3.md"));
    page3.frontmatter.date = Some("2023-12-01".to_string());

    index.add_page(page1);
    index.add_page(page2);
    index.add_page(page3);

    let sorted = index.sort_by_date();
    assert_eq!(sorted.len(), 3);
    assert_eq!(sorted[0].frontmatter.date, Some("2024-02-01".to_string()));
    assert_eq!(sorted[1].frontmatter.date, Some("2024-01-01".to_string()));
    assert_eq!(sorted[2].frontmatter.date, Some("2023-12-01".to_string()));
}

#[test]
fn test_sort_by_weight() {
    let mut index = HugoContentIndex::new();

    // 创建测试页面
    let mut page1 = HugoPage::new(PathBuf::from("page1.md"), PathBuf::from("page1.md"));
    page1.frontmatter.weight = Some(3);

    let mut page2 = HugoPage::new(PathBuf::from("page2.md"), PathBuf::from("page2.md"));
    page2.frontmatter.weight = Some(1);

    let mut page3 = HugoPage::new(PathBuf::from("page3.md"), PathBuf::from("page3.md"));
    page3.frontmatter.weight = Some(2);

    index.add_page(page1);
    index.add_page(page2);
    index.add_page(page3);

    let sorted = index.sort_by_weight();
    assert_eq!(sorted.len(), 3);
    assert_eq!(sorted[0].frontmatter.weight, Some(1));
    assert_eq!(sorted[1].frontmatter.weight, Some(2));
    assert_eq!(sorted[2].frontmatter.weight, Some(3));
}

#[test]
fn test_group_by_section() {
    let mut index = HugoContentIndex::new();

    // 创建测试页面
    let mut page1 = HugoPage::new(PathBuf::from("posts/page1.md"), PathBuf::from("posts/page1.md"));
    page1.section = Some("posts".to_string());

    let mut page2 = HugoPage::new(PathBuf::from("posts/page2.md"), PathBuf::from("posts/page2.md"));
    page2.section = Some("posts".to_string());

    let mut page3 = HugoPage::new(PathBuf::from("about.md"), PathBuf::from("about.md"));
    page3.section = Some("about".to_string());

    index.add_page(page1);
    index.add_page(page2);
    index.add_page(page3);

    let groups = index.group_by_section();
    assert_eq!(groups.len(), 2);
    assert_eq!(groups.get("posts").unwrap().len(), 2);
    assert_eq!(groups.get("about").unwrap().len(), 1);
}

#[test]
fn test_group_by_tag() {
    let mut index = HugoContentIndex::new();

    // 创建测试页面
    let mut page1 = HugoPage::new(PathBuf::from("page1.md"), PathBuf::from("page1.md"));
    page1.frontmatter.tags = Some(vec!["rust".to_string(), "programming".to_string()]);

    let mut page2 = HugoPage::new(PathBuf::from("page2.md"), PathBuf::from("page2.md"));
    page2.frontmatter.tags = Some(vec!["rust".to_string(), "web".to_string()]);

    let mut page3 = HugoPage::new(PathBuf::from("page3.md"), PathBuf::from("page3.md"));
    page3.frontmatter.tags = Some(vec!["programming".to_string()]);

    index.add_page(page1);
    index.add_page(page2);
    index.add_page(page3);

    let groups = index.group_by_tag();
    assert_eq!(groups.len(), 3);
    assert_eq!(groups.get("rust").unwrap().len(), 2);
    assert_eq!(groups.get("programming").unwrap().len(), 2);
    assert_eq!(groups.get("web").unwrap().len(), 1);
}
