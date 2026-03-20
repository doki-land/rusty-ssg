//! VuTeX 编译器基准测试
//! 测试编译器在不同场景下的性能表现

use criterion::{Criterion, criterion_group, criterion_main};
use std::collections::HashMap;
use vutex::{VutexCompiler, compile_batch, compile_single};

/// 简单测试文档
const SIMPLE_DOC: &str = r#"
# 简单测试文档

这是一个简单的 Markdown 文档，用于测试基本编译性能。

## 第一节

这是第一节的内容。包含一些列表：

- 项目一
- 项目二
- 项目三

## 第二节

这是第二节的内容。包含一些代码：

```rust
fn main() {
    println!("Hello, VuTeX!");
}
```
"#;

/// KaTeX 测试文档
const KATEX_DOC: &str = include_str!("../../benchmarks/test-docs/katex-demo.md");

/// 综合测试文档
const COMBINED_DOC: &str = include_str!("../../benchmarks/test-docs/combined-demo.md");

/// 测试单个文档编译
fn bench_compile_single(c: &mut Criterion) {
    let mut group = c.benchmark_group("compile_single");

    group.bench_function("simple_document", |b| {
        b.iter(|| {
            compile_single(SIMPLE_DOC, "/test/simple.md").unwrap();
        });
    });

    group.bench_function("katex_document", |b| {
        b.iter(|| {
            compile_single(KATEX_DOC, "/test/katex.md").unwrap();
        });
    });

    group.bench_function("combined_document", |b| {
        b.iter(|| {
            compile_single(COMBINED_DOC, "/test/combined.md").unwrap();
        });
    });

    group.finish();
}

/// 测试编译器对象的复用
fn bench_compiler_reuse(c: &mut Criterion) {
    let mut group = c.benchmark_group("compiler_reuse");

    group.bench_function("new_compiler_each_time", |b| {
        b.iter(|| {
            let mut compiler = VutexCompiler::new();
            compiler.compile_document(SIMPLE_DOC, "/test/simple.md").unwrap();
        });
    });

    group.bench_function("reuse_compiler", |b| {
        let mut compiler = VutexCompiler::new();
        b.iter(|| {
            compiler.clear_cache();
            compiler.compile_document(SIMPLE_DOC, "/test/simple.md").unwrap();
        });
    });

    group.bench_function("with_cache", |b| {
        let mut compiler = VutexCompiler::new();
        compiler.compile_document(SIMPLE_DOC, "/test/simple.md").unwrap();
        b.iter(|| {
            compiler.compile_document(SIMPLE_DOC, "/test/simple.md").unwrap();
        });
    });

    group.finish();
}

/// 测试批量编译
fn bench_compile_batch(c: &mut Criterion) {
    let mut group = c.benchmark_group("compile_batch");

    let mut single_doc = HashMap::new();
    single_doc.insert("/test/simple.md".to_string(), SIMPLE_DOC.to_string());

    group.bench_function("batch_single", |b| {
        b.iter(|| {
            compile_batch(&single_doc);
        });
    });

    let mut multiple_docs = HashMap::new();
    multiple_docs.insert("/test/simple.md".to_string(), SIMPLE_DOC.to_string());
    multiple_docs.insert("/test/katex.md".to_string(), KATEX_DOC.to_string());
    multiple_docs.insert("/test/combined.md".to_string(), COMBINED_DOC.to_string());

    group.bench_function("batch_multiple", |b| {
        b.iter(|| {
            compile_batch(&multiple_docs);
        });
    });

    group.finish();
}

criterion_group!(
    name = benches;
    config = Criterion::default().sample_size(100);
    targets = bench_compile_single, bench_compiler_reuse, bench_compile_batch
);

criterion_main!(benches);
