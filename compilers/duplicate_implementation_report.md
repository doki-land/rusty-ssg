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
- gatsby：`src/tools/cmd/`
- hexo：`src/tools/cmd/`
- hugo：`src/tools/cmd/`
- jekyll：`src/tools/cmd/`
- mkdocs：`src/tools/cmd/`
- vitepress：`src/tools/cmd/`
- vuepress：`src/tools/cmd/`

### 2.3 配置处理模块重复实现

**问题**：每个编译器都实现了自己的配置处理模块，处理类似的配置文件。

**影响**：
- 配置处理逻辑不一致
- 代码重复，维护成本高
- 配置格式不统一

**涉及编译器**：
- astro：`src/config/`
- eleventy：`src/config/`
- gatsby：`src/config/`
- hexo：`src/config/`
- hugo：`src/config/`
- jekyll：`src/config/`
- mkdocs：`src/config/`
- vitepress：`src/config/`
- vuepress：`src/config/`

### 2.4 插件系统重复实现

**问题**：每个编译器都实现了自己的插件系统，功能相似但代码重复。

**影响**：
- 插件接口不一致
- 代码重复，维护成本高
- 插件开发体验差

**涉及编译器**：
- astro：`src/plugin/`
- eleventy：`src/plugin/`
- gatsby：`src/plugin/`
- hexo：`src/plugin/`
- hugo：`src/plugin/`
- jekyll：`src/plugin/`
- mkdocs：`src/plugin/`
- vitepress：`src/plugin/`
- vuepress：`src/plugin/`

### 2.5 错误处理重复实现

**问题**：每个编译器都实现了自己的错误处理机制，定义了类似的错误类型。

**影响**：
- 错误处理逻辑不一致
- 代码重复，维护成本高
- 错误信息格式不统一

**涉及编译器**：
- astro：`src/errors/`
- eleventy：`src/errors/`
- gatsby：`src/errors/`
- hexo：`src/errors/`
- hugo：`src/errors/`
- jekyll：`src/errors/`
- mkdocs：`src/errors/`
- vitepress：`src/errors/`
- vuepress：`src/errors/`

### 2.6 类型定义重复实现

**问题**：多个编译器定义了类似的类型，如 CompileResult、Config 等。

**影响**：
- 类型定义不一致
- 代码重复，维护成本高
- 类型转换复杂

**涉及编译器**：
- gatsby：`src/types/`
- hugo：`src/types/`
- mkdocs：`src/types/`
- vitepress：`src/types/`
- vuepress：`src/types/`

### 2.7 站点生成器重复实现

**问题**：多个编译器实现了类似的站点生成器功能。

**影响**：
- 生成逻辑不一致
- 代码重复，维护成本高
- 功能差异导致用户体验不一致

**涉及编译器**：
- gatsby：`src/tools/site_generator.rs`
- hugo：`src/tools/site_generator.rs`
- jekyll：`src/tools/site_generator.rs`
- mkdocs：`src/tools/site_generator.rs`
- vitepress：`src/tools/site_generator.rs`
- vuepress：`src/tools/site_generator.rs`

### 2.8 CompileResult 结构体重复实现

**问题**：多个编译器实现了几乎相同的 CompileResult 结构体。

**影响**：
- 代码重复，维护成本高
- 序列化/反序列化逻辑不一致
- 类型定义冗余

**涉及编译器**：
- gatsby：`src/lib.rs`
- hugo：`src/lib.rs`
- mkdocs：`src/lib.rs`
- vitepress：`src/lib.rs`
- vuepress：`src/lib.rs`

## 3. 根本原因分析

### 3.1 缺乏统一抽象

虽然架构规范中提到了统一抽象，但实际实现中每个编译器都独立实现了核心功能，没有充分利用 nargo 和 oaks 提供的组件。

### 3.2 架构设计问题

编译器之间缺乏共享的核心库，导致每个编译器都需要从头实现基本功能。

### 3.3 依赖管理不当

虽然规范要求使用 pnpm workspace 管理依赖，但实际实现中没有充分利用共享依赖。

## 4. 解决方案建议

### 4.1 建立共享核心库

创建一个共享的核心库，包含以下功能：
- 统一的 HTML 渲染器
- 统一的命令行工具框架
- 统一的配置处理模块
- 统一的插件系统
- 统一的错误处理机制
- 统一的类型定义
- 统一的站点生成器

### 4.2 利用 nargo 和 oaks

充分利用 nargo 和 oaks 提供的组件，避免重复实现：
- 使用 nargo-document 处理文档分析
- 使用 nargo-template 处理模板渲染
- 使用 oaks 提供的解析器

### 4.3 重构现有代码

逐步重构现有编译器，将重复代码迁移到共享核心库：
1. 提取重复功能到共享库
2. 修改编译器使用共享库
3. 移除重复实现

### 4.4 建立统一的架构规范

建立更严格的架构规范，确保所有编译器遵循相同的设计模式和代码组织方式。

## 5. 结论

通过分析发现，rusty-ssg 下的编译器存在大量重复造轮子的问题，这不仅增加了代码维护成本，也导致了功能不一致和用户体验差异。通过建立共享核心库、充分利用现有组件、重构现有代码和建立统一的架构规范，可以显著减少代码冗余，提高代码质量和维护效率。