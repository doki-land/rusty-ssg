# Rusty SSG 编译器重复造轮子问题报告

## 1. 问题概述

通过对 rusty-ssg 下的 9 个编译器（astro、eleventy、gatsby、hexo、hugo、jekyll、mkdocs、vitepress、vuepress）的分析，发现存在大量重复造轮子的问题，导致代码冗余、维护成本高、一致性差等问题。

## 2. 具体重复问题

### 2.1 HTML 渲染器重复实现

**问题**：每个编译器都实现了自己的 HTML 渲染器，功能相似但代码重复。

**影响**：
- 代码冗余，维护成本高
- 渲染逻辑不一致，可能导致输出差异
- 新功能需要在多个地方实现

**涉及编译器**：
- astro：`src/compiler/renderer/html_renderer.rs`
- eleventy：`src/compiler/renderer/html_renderer.rs`
- gatsby：`src/compiler/renderer/html_renderer.rs`
- hugo：`src/compiler/renderer/html_renderer.rs`
- jekyll：`src/compiler/renderer/html_renderer.rs`
- mkdocs：`src/compiler/renderer/html_renderer.rs`
- vitepress：`src/compiler/renderer/html_renderer.rs`
- vuepress：`src/compiler/renderer/html_renderer.rs`

### 2.2 命令行工具重复实现

**问题**：每个编译器都实现了类似的命令行工具，包括 build、dev、init、check 等命令。

**影响**：
- 命令行接口不一致
- 代码重复，维护成本高
- 新命令需要在多个地方实现

**涉及编译器**：
- astro：`src/tools/cmd/`
- eleventy：`src/tools/cmd/`
-