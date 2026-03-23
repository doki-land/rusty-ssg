# MkDocs - Rust Reimplementation

## 概述

MkDocs 是一个文档专注的静态站点生成器，现在使用 Rust 重新实现，以获得更好的性能和可靠性。它使用 Markdown 编写内容，支持主题系统和插件扩展，使您能够轻松构建漂亮的文档网站。

### 🎯 核心特性

- 🚀 **快速构建**：在几秒钟内编译您的站点，而不是几分钟
- 🎨 **文档专注**：专为文档设计的功能和结构
- 📦 **易于部署**：生成可在任何地方工作的静态文件
- 🔧 **可扩展**：通过插件和主题进行自定义
- 🛠 **开发者友好**：优秀的工具和开发体验
- 📝 **Markdown 支持**：轻松使用 Markdown 编写内容
- 🌍 **跨平台**：适用于 Windows、macOS 和 Linux
- 📱 **100% 兼容**：使用静态功能时完全兼容

## 安装

### 从 Crates.io 安装

```bash
cargo install mkdocs
```

### 从源代码安装

```bash
# 克隆仓库
git clone https://github.com/doki-land/rusty-ssg.git

# 构建和安装
cd rusty-ssg/compilers/mkdocs
cargo install --path .
```

## 基本用法

### 创建新站点

```bash
mkdocs new my-site
cd my-site
```

### 本地开发

```bash
mkdocs serve
```

这将启动一个带有热重载的本地开发服务器，因此您可以实时查看更改。

### 生产构建

```bash
mkdocs build
```

这将在 `site` 目录中生成优化的静态文件，ready for deployment.

## 项目结构

一个典型的 MkDocs 项目结构如下：

```
my-site/
├── docs/              # 文档文件
│   ├── index.md       # 首页
│   ├── getting-started.md  # 入门指南
│   └── advanced.md    # 高级主题
├── mkdocs.yml         # 配置文件
└── README.md          # 项目说明
```

## 配置

### 基本配置

Here's an example `mkdocs.yml` file:

```yaml
# mkdocs.yml
site_name: My MkDocs Site
site_description: A site built with Rusty MkDocs
site_url: https://example.com

nav:
  - Home: index.md
  - Getting Started: getting-started.md
  - Advanced: advanced.md

theme:
  name: material
  features:
    - navigation.tabs
    - navigation.sections
    - navigation.expand
    - navigation.top

plugins:
  - search
  - mkdocs-material

markdown_extensions:
  - admonition
  - codehilite
  - toc:
      permalink: true
  - pymdownx.arithmatex
  - pymdownx.superfences
```

### 配置选项

| 选项 | 描述 | 默认值 |
|------|------|--------|
| `site_name` | 站点名称 | `""` |
| `site_description` | 站点描述 | `""` |
| `site_url` | 站点 URL | `""` |
| `nav` | 导航配置 | `[]` |
| `theme` | 主题配置 | `{"name": "mkdocs"}` |
| `plugins` | 插件列表 | `["search"]` |
| `markdown_extensions` | Markdown 扩展 | `[]` |

## 内容管理

### 文档页面

```markdown
---
title: "Getting Started"
---

# Getting Started

Welcome to MkDocs! This is your first documentation page.

## What is MkDocs?

MkDocs is a fast, simple and downright gorgeous static site generator that's geared towards building project documentation.

## Why Use MkDocs?

- It's fast and simple
- It's documentation-focused
- It has beautiful themes
- It's 100% compatible with static features

## Next Steps

1. Create more documentation pages
2. Customize your theme
3. Add plugins
4. Deploy your site

Happy documenting! 🎉
```

### 首页

```markdown
---
title: "Home"
---

# Welcome to My MkDocs Site

This is the home page of my MkDocs documentation site.

## Features

- 🚀 Fast and simple
- 🎨 Beautiful themes
- 🔧 Extensible with plugins
- 📝 Easy to write content in Markdown

## Quick Start

1. Install MkDocs
2. Create a new project
3. Write documentation in Markdown
4. Build and deploy your site

## Navigation

- [Getting Started](getting-started.md)
- [Advanced](advanced.md)
```

## 主题系统

MkDocs 支持主题，您可以选择内置主题或创建自己的主题。

### 内置主题

- 🎨 **mkdocs**：默认主题
- 🎨 **readthedocs**：ReadTheDocs 风格主题
- 🎨 **material**：Material Design 风格主题
- 🎨 **mkdocs-bootstrap**：Bootstrap 风格主题

### 使用主题

在 `mkdocs.yml` 中配置主题：

```yaml
# mkdocs.yml
theme:
  name: material
  features:
    - navigation.tabs
    - navigation.sections
    - navigation.expand
    - navigation.top
  palette:
    primary: blue
    accent: indigo
```

## 插件系统

MkDocs 支持通过插件扩展功能。

### 内置插件

- 🔍 **search**：搜索功能
- 📊 **mkdocs-material**：Material Design 主题
- 📈 **mkdocs-git-revision-date**：显示 Git 修订日期
- 🗺️ **mkdocs-sitemap**：生成 sitemap.xml
- 📱 **mkdocs-redirects**：处理重定向

### 使用插件

在 `mkdocs.yml` 中配置插件：

```yaml
# mkdocs.yml
plugins:
  - search
  - mkdocs-material
  - git-revision-date
  - sitemap
```

## 部署

MkDocs 生成可在任何地方部署的静态文件。

### Netlify

```toml
# netlify.toml
[build]
  command = "cargo install mkdocs && mkdocs build"
  publish = "site"

[build.environment]
  RUST_VERSION = "stable"
```

### Vercel

```json
// vercel.json
{
  "buildCommand": "cargo install mkdocs && mkdocs build",
  "outputDirectory": "site",
  "env": {
    "RUST_VERSION": "stable"
  }
}
```

### GitHub Pages

```yaml
# .github/workflows/deploy.yml
name: Deploy to GitHub Pages

on:
  push:
    branches: [ main ]

jobs:
  deploy:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      
      - name: Set up Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          override: true
      
      - name: Install MkDocs
        run: cargo install mkdocs
      
      - name: Build site
        run: mkdocs build
      
      - name: Deploy to GitHub Pages
        uses: peaceiris/actions-gh-pages@v3
        with:
          github_token: ${{ secrets.GITHUB_TOKEN }}
          publish_dir: ./site
```

## 高级功能

### Markdown 扩展

MkDocs 支持多种 Markdown 扩展：

```yaml
# mkdocs.yml
markdown_extensions:
  - admonition
  - codehilite
  - toc:
      permalink: true
  - pymdownx.arithmatex
  - pymdownx.superfences
  - pymdownx.emoji
```

###  admonition 扩展

```markdown
!!! note
    This is a note admonition.

!!! warning
    This is a warning admonition.

!!! danger
    This is a danger admonition.
```

### 代码高亮

```markdown
```python
def hello():
    print("Hello, MkDocs!")
```
```

### 数学公式

```markdown
$$
E = mc^2
$$
```

## 性能优化

### 增量构建

MkDocs 支持增量构建，只重新构建修改过的文件：

```bash
mkdocs build --dirty
```

### 缓存

MkDocs 使用缓存来提高构建速度：
- **内容缓存**：缓存解析后的内容
- **模板缓存**：缓存编译后的模板

### 资源优化

- **压缩**：确保所有静态资源（HTML、CSS、JavaScript）都已压缩
- **图像优化**：优化图像大小和质量
- **代码分割**：使用代码分割减少初始加载时间

## 开发工具

### VS Code 扩展

推荐使用以下 VS Code 扩展：

- **MkDocs**：提供语法高亮和智能提示
- **Prettier**：代码格式化
- **ESLint**：代码质量检查

### 调试

使用 `--verbose` 标志启用详细输出：

```bash
mkdocs build --verbose
```

## 常见问题

### 1. MkDocs 与其他静态站点生成器的区别是什么？

MkDocs 的主要区别在于其文档专注的设计和漂亮的主题，使其成为构建项目文档的理想选择。

### 2. 如何处理动态内容？

MkDocs 主要针对静态内容优化，但您可以使用客户端 JavaScript 或外部服务来处理动态内容。

### 3. 如何优化构建速度？

- 使用增量构建
- 优化模板和内容
- 减少插件数量
- 使用适当的缓存策略

## 示例项目

### 文档站点示例

```
docs-site/
├── docs/
│   ├── index.md
│   ├── getting-started.md
│   ├── advanced.md
│   └── api.md
├── mkdocs.yml
└── README.md
```

## 贡献指南

我们欢迎对 MkDocs 的贡献！🤝

### 报告问题

如果您发现错误或有功能请求，请 [打开一个 issue](https://github.com/doki-land/rusty-ssg/issues)。

### 提交拉取请求

1. Fork 仓库
2. 创建一个新分支
3. 进行更改
4. 运行测试
5. 提交拉取请求

### 代码风格

请遵循 Rust 风格指南并使用 `cargo fmt` 格式化代码。

## 致谢

MkDocs 受到原始 MkDocs 项目的启发，并受益于 Rust 生态系统，包括 nargo 和 oak 库。

## 许可证

MkDocs 在 AGPL-3.0 许可证下发布。有关更多信息，请参阅 [LICENSE](../../../license.md)。

---

使用 MkDocs 构建愉快！🚀