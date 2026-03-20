//! 性能基准测试

use criterion::{Criterion, black_box, criterion_group, criterion_main};
use hexo::markdown::{parser::render_markdown, renderer::render_markdown_file};
use std::{fs::File, io::Write};
use tempfile::tempdir;

/// 测试 Markdown 渲染性能
fn bench_markdown_rendering(c: &mut Criterion) {
    // 创建测试内容
    let markdown_content = r#"# Test Post

This is a test post with **bold** and *italic* text.

## Subheading

- List item 1
- List item 2
- List item 3

```rust
fn main() {
    println!("Hello, world!");
}
```

> Blockquote

| Header 1 | Header 2 |
| -------- | -------- |
| Cell 1   | Cell 2   |
| Cell 3   | Cell 4   |
"#;

    // 测试字符串渲染
    c.bench_function("markdown_render_string", |b| {
        b.iter(|| {
            let result = render_markdown(black_box(markdown_content));
            black_box(result);
        });
    });

    // 测试文件渲染
    let dir = tempdir().unwrap();
    let test_file = dir.path().join("test.md");

    let mut file = File::create(&test_file).unwrap();
    file.write_all(markdown_content.as_bytes()).unwrap();

    c.bench_function("markdown_render_file", |b| {
        b.iter(|| {
            let result = render_markdown_file(black_box(&test_file)).unwrap();
            black_box(result);
        });
    });
}

criterion_group!(benches, bench_markdown_rendering);
criterion_main!(benches);
