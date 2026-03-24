# Gatsby 静态站点生成器 (Rust 实现)

这是一个用 Rust 语言实现的 Gatsby 静态站点生成器，旨在提供与原版 Gatsby 兼容的功能，同时利用 Rust 的性能优势。

## 项目特点

- ✅ 完全用 Rust 实现，性能优异
- ✅ 与原版 Gatsby 配置文件兼容
- ✅ 支持 Markdown 文档编译
- ✅ 内置 GraphQL 服务
- ✅ 插件系统支持
- ✅ 静态站点生成，包括 404 页面、sitemap 和 robots.txt
- ✅ 并行编译和生成，提高性能

## 安装

### 从源码构建

1. 确保你已经安装了 Rust 和 Cargo
2. 克隆仓库：

```bash
git clone https://github.com/doki-land/rusty-ssg.git
cd rusty-ssg
```

3. 构建 Gatsby 编译器：

```bash
cargo build --package gatsby
```

4. 安装到系统：

```bash
cargo install --path compilers/gatsby
```

## 基本使用

### 初始化项目

```bash
gatsby init my-gatsby-site
cd my-gatsby-site
```

### 配置文件

Gatsby 支持多种格式的配置文件：
- `gatsby-config.js`
- `gatsby-config.json`
- `gatsby-config.yaml`
- `gatsby-config.yml`
- `gatsby-config.toml`

示例配置 (`gatsby-config.json`)：

```json
{
  "siteMetadata": {
    "title": "My Gatsby Site",
    "description": "A static site generated with Gatsby",
    "author": "Your Name",
    "siteUrl": "https://example.com"
  },
  "plugins": [],
  "pathPrefix": "/blog"
}
```

### 编写内容

在 `src/pages` 目录中创建 Markdown 文件：

```markdown
---
title: "Hello World"
description: "My first Gatsby post"
date: "2024-01-01"
author: "Your Name"
---

# Hello World

Welcome to my Gatsby site!
```

### 构建站点

```bash
gatsby build
```

构建结果将生成在 `public` 目录中。

### 开发服务器

```bash
gatsby develop
```

这将启动一个本地开发服务器，你可以在浏览器中访问 `http://localhost:8000` 查看站点。

## 命令行工具

### `gatsby init [name]`

初始化一个新的 Gatsby 项目。

### `gatsby build`

构建静态站点。

### `gatsby develop`

启动开发服务器。

### `gatsby clean`

清除缓存和构建文件。

### `gatsby check`

检查项目配置和依赖。

### `gatsby version`

显示 Gatsby 版本信息。

## GraphQL 支持

Gatsby 内置了 GraphQL 服务，你可以使用 GraphQL 查询来获取数据。在开发模式下，你可以通过 `http://localhost:8000/___graphql` 访问 GraphQL  playground。

### 示例查询

```graphql
query {
  site {
    siteMetadata {
      title
      description
      author
      siteUrl
    }
  }
  allMarkdownRemark {
    edges {
      node {
        id
        frontmatter {
          title
          date
          author
        }
        html
      }
    }
  }
}
```

## 插件系统

Gatsby 支持插件系统，你可以通过配置文件添加插件：

```json
{
  "plugins": [
    "gatsby-plugin-sharp",
    "gatsby-transformer-remark"
  ]
}
```

### 插件生命周期钩子

- `onPreBootstrap`
- `onBootstrap`
- `onPreBuild`
- `onBuild`
- `onPostBuild`
- `onPreDeleteCache`
- `onPostDeleteCache`
- `onPreExtractQueries`
- `onPostExtractQueries`
- `onPreInit`
- `onPostInit`
- `onPreRenderHTML`
- `onPostRenderHTML`
- `onRouteUpdate`
- `onRouteUpdateDelayed`
- `onServiceWorkerUpdateFound`

## 与原版 Gatsby 的兼容性

### 兼容特性

- ✅ 配置文件格式（JSON、YAML、TOML、JavaScript）
- ✅ Markdown 文档处理
- ✅ GraphQL 查询
- ✅ 静态站点生成
- ✅ 命令行接口
- ✅ 插件系统

### 已知差异

- 部分高级插件可能不兼容
- 某些 Gatsby 特有的 API 可能尚未实现
- 性能优化策略不同（Rust 实现可能更快）

## 性能优化

1. **并行编译**：使用 Rayon 库并行处理文档编译
2. **缓存机制**：避免重复编译未修改的文件
3. **并行文件写入**：使用多线程加速站点生成
4. **内存管理**：优化内存使用，减少内存分配

## 目录结构

```
my-gatsby-site/
├── gatsby-config.json    # 配置文件
├── src/
│   ├── pages/            # 页面文件
│   │   ├── index.md
│   │   └── blog/
│   │       └── first-post.md
│   └── components/       # 组件（预留）
├── public/              # 构建输出目录
└── node_modules/        # 依赖（预留）
```

## 示例

### 基本站点

```bash
gatsby init my-site
cd my-site
echo "---
title: 'Home'
---

# Welcome to my site
" > src/pages/index.md
gatsby build
```

### 博客站点

```bash
gatsby init my-blog
cd my-blog
mkdir -p src/pages/blog

echo "---
title: 'My First Post'
date: '2024-01-01'
---

# Hello World

This is my first blog post.
" > src/pages/blog/first-post.md

echo "---
title: 'About'
---

# About Me

I'm a blogger.
" > src/pages/about.md

gatsby build
```

## 扩展 Gatsby 编译器

### 自定义插件开发

Gatsby 编译器支持自定义插件，你可以通过实现 `Plugin` trait 来创建自己的插件：

```rust
use gatsby::{Plugin, PluginContext, PluginMeta, Page};

struct MyPlugin;

impl Plugin for MyPlugin {
    fn meta(&self) -> PluginMeta {
        PluginMeta {
            name: "my-plugin".to_string(),
            version: "1.0.0".to_string(),
            description: "My custom plugin".to_string(),
            author: "Your Name".to_string(),
            license: "MIT".to_string(),
            repository: None,
        }
    }

    fn on_pre_build(&self, ctx: &PluginContext) -> Result<(), gatsby::PluginError> {
        // 构建前的处理逻辑
        Ok(())
    }

    // 实现其他生命周期钩子...
}
```

### 扩展解析器

你可以通过实现 `Parser` trait 来创建自定义解析器：

```rust
use gatsby::{Parser, Document};

struct MyCustomParser;

impl Parser for MyCustomParser {
    fn parse(&self, content: &str, path: &str) -> Result<Document, String> {
        // 自定义解析逻辑
        // ...
        Ok(document)
    }
}
```

### 扩展渲染器

你可以创建自定义渲染器来处理不同的内容格式：

```rust
use gatsby::compiler::renderer::HtmlRenderer;

struct MyCustomRenderer;

impl HtmlRenderer for MyCustomRenderer {
    fn render(&self, content: &str) -> String {
        // 自定义渲染逻辑
        // ...
        rendered_html
    }
}
```

### 使用 GraphQL 服务

你可以直接使用 GraphQL 服务来查询和修改数据：

```rust
use gatsby::{GraphQLService, GraphQLRequest};

let mut service = GraphQLService::new();

// 添加节点
let node = Node::new(
    NodeId::generate(),
    NodeType::new("MarkdownRemark"),
    "content-digest"
);
service.add_node(node).unwrap();

// 执行查询
let request = GraphQLRequest {
    query: "query { site { siteMetadata { title } } }".to_string(),
    operation_name: None,
    variables: None,
};

let response = service.execute_query(request);
```

### 自定义站点生成器

你可以扩展 `StaticSiteGenerator` 来自定义站点生成逻辑：

```rust
use gatsby::{StaticSiteGenerator, GatsbyConfig};

let config = GatsbyConfig::new();
let mut generator = StaticSiteGenerator::new(config).unwrap();

// 自定义生成逻辑
// ...

// 生成站点
generator.generate(&documents, &output_dir).unwrap();
```

## API 参考

### 核心类型

- `GatsbyCompiler` - 主编译器，负责编译文档
- `MarkdownParser` - Markdown 解析器
- `HtmlRenderer` - HTML 渲染器
- `StaticSiteGenerator` - 静态站点生成器
- `GraphQLService` - GraphQL 服务
- `PluginRegistry` - 插件注册表

### 主要函数

- `compile_single()` - 编译单个文档
- `compile_batch()` - 批量编译文档

### 配置类型

- `GatsbyConfig` - 主配置结构
- `SiteMetadata` - 站点元数据
- `PluginConfig` - 插件配置
- `DevelopConfig` - 开发服务器配置

## 已知问题和限制

- 部分 Gatsby 插件可能不兼容
- 某些高级功能可能尚未实现
- 错误处理和用户体验仍在改进中

## 贡献

欢迎贡献代码、报告问题或提出建议！请查看项目的 GitHub 仓库了解如何贡献。

## 许可证

本项目采用 AGPL-3.0 许可证。
