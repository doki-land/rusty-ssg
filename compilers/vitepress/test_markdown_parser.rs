use oak_core::{Builder, parser::session::ParseSession, source::SourceText};
use oak_markdown::{MarkdownBuilder, MarkdownLanguage};

fn main() {
    // 测试简单的无序列表
    let markdown = r#"
- 列表项 1
- 列表项 2
- 列表项 3
"#;

    let source_text = SourceText::new(markdown);
    let lang_config = MarkdownLanguage::default();
    let builder = MarkdownBuilder::new(&lang_config);
    let mut session = ParseSession::default();

    let output = builder.build(&source_text, &[], &mut session);

    match output.result {
        Ok(root) => {
            println!("解析成功！");
            println!("根节点包含 {} 个块", root.blocks.len());
            
            for (i, block) in root.blocks.iter().enumerate() {
                println!("块 {}: {:?}", i, block);
            }
        }
        Err(e) => {
            println!("解析失败: {}", e);
        }
    }
}
