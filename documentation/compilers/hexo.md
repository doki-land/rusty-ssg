# Hexo - Rust Reimplementation

## 概述

Hexo 是一个博客专注的静态站点生成器，现在使用 Rust 重新实现，以获得更好的性能和可靠性。它使用 Markdown 编写内容，支持主题系统和插件扩展，使您能够轻松构建漂亮的博客网站。

### 🎯 核心特性

- 🚀 **快速构建**：在几秒钟内编译您的站点，而不是几分钟
- 🎨 **博客专注**：专为博客设计的功能和结构
- 📦 **易于部署**：生成可在任何地方工作的静态文件
- 🔧 **可扩展**：通过插件和主题进行自定义
- 🛠 **开发者友好**：优秀的工具和开发体验
- 📝 **Markdown 支持**：轻松使用 Markdown 编写内容
- 🌍 **跨平台**：适用于 Windows、macOS 和 Linux
- 📱 **100% 兼容**：使用静态功能时完全兼容

## 安装

### 从 Crates.io 安装

```bash
cargo install hexo
```

### 从源代码安装

```bash
# 克隆仓库
git clone https://github.com/doki-land/rusty-ssg.git

# 构建和安装
cd rusty-ssg/compilers/hexo
cargo install --path .
```

## 基本用法

### 创建新站点

```bash
hexo init my-site
cd my-site
```

### 创建新文章

```bash
hexo new "My First Post"
```

### 本地开发

```bash
hexo server
```

这将启动一个带有热重载的本地开发服务器，因此您可以实时查看更改。

### 生产构建

```bash
hexo generate
```

这将在 `public` 目录中生成优化的静态文件，ready for deployment.

## 项目结构

一个典型的 Hexo 项目结构如下：

```
my-site/
├── source/             # 源文件
│   ├── _posts/         # 博客文章
│   ├── _drafts/        # 草稿
│   └── about/          # 页面
├── themes/             # 主题
├── _config.yml         # 配置文件
└── package.json        # 项目依赖
```

## 配置

### 基本配置

Here's an example `_config.yml` file:

```yaml
# _config.yml

# Site
title: My Hexo Site
description: A site built with Rusty Hexo
author: Your Name
language: en
timezone:

# URL
url: https://example.com
root: /
permalink: :year/:month/:day/:title/
permalink_defaults:

# Directory
source_dir: source
theme_dir: themes
public_dir: public

# Writing
new_post_name: :title.md  # File name of new posts
default_layout: post
auto_spacing: false
titlecase: false
external_link: true
filename_case: 0
render_drafts: false
post_asset_folder: false
relative_link: false
future: true
highlight:
  enable: true
  line_number: true
  auto_detect: false
  tab_replace:

# Category & Tag
default_category: uncategorized
category_map:
tag_map:

# Date / Time format
date_format: YYYY-MM-DD
time_format: HH:mm:ss

# Pagination
per_page: 10
pagination_dir: page

# Extensions
theme: landscape
plugins:
  - hexo-generator-feed
  - hexo-generator-sitemap
```

### 配置选项

| 选项 | 描述 | 默认值 |
|------|------|--------|
| `title` | 站点标题 | `""` |
| `description` | 站点描述 | `""` |
| `author` | 站点作者 | `""` |
| `language` | 站点语言 | `"en"` |
| `url` | 站点 URL | `""` |
| `root` | 站点根路径 | `"/"` |
| `permalink` | 永久链接格式 | `":year/:month/:day/:title/"` |
| `theme` | 主题名称 | `"landscape"` |
| `per_page` | 每页文章数 | `10` |

## 内容管理

### 博客文章

```markdown
---
title: "My First Post"
date: 2024-01-01 10:00:00
tags:
  - rust
  - hexo
categories:
  - tutorials
---

# My First Post

Welcome to Hexo! This is your first post.

## What is Hexo?

Hexo is a fast, blog-focused static site generator that uses Markdown.

## Why Use Hexo?

- It's blazingly fast
- It's blog-focused
- It has a rich theme ecosystem
- It's 100% compatible with static features

## Next Steps

1. Create more content
2. Customize your theme
3. Add plugins
4. Deploy your site

Happy coding! 🎉
```

### 页面

```markdown
---
title: "About"
date: 2024-01-01 10:00:00
---

# About Me

Hello! I'm using Hexo to build this blog.

## My Background

I'm a web developer passionate about static site generators and modern web technologies.

## Contact

Feel free to reach out if you have any questions! 📧
```

## 主题系统

Hexo 支持主题，您可以选择内置主题或创建自己的主题。

### 内置主题

- 🎨 **landscape**：默认主题
- 🌙 **dark**：深色模式主题
- 📦 **minimal**：极简主义主题
- 📝 **blog**：博客专注的主题

### 使用主题

在 `_config.yml` 中配置主题：

```yaml
# _config.yml
theme: landscape
```

### 主题结构

一个典型的 Hexo 主题结构如下：

```
theme-name/
├── layout/            # 模板文件
│   ├── _partial/      # 部分模板
│   ├── _widget/       # 小部件
│   ├── archive.ejs    # 归档页面
│   ├── index.ejs      # 首页
│   └── post.ejs       # 文章页面
├── source/            # 静态资源
│   ├── css/           # 样式文件
│   ├── js/            # JavaScript 文件
│   └── images/        # 图片
└── _config.yml        # 主题配置
```

## 插件系统

Hexo 支持通过插件扩展功能。

### 内置插件

- 📊 **hexo-renderer-katex**：渲染数学公式
- 🎨 **hexo-prism-plugin**：代码块语法高亮
- 📈 **hexo-filter-mermaid**：渲染图表和流程图
- 🔍 **hexo-google-analytics**：添加 Google Analytics 跟踪
- 🗺️ **hexo-generator-sitemap**：生成 sitemap.xml
- 📱 **hexo-generator-feed**：生成 RSS 订阅

### 使用插件

在 `_config.yml` 中配置插件：

```yaml
# _config.yml
plugins:
  - hexo-generator-feed
  - hexo-generator-sitemap
  - hexo-renderer-katex
  - hexo-prism-plugin
```

## 部署

Hexo 生成可在任何地方部署的静态文件。

### Netlify

```toml
# netlify.toml
[build]
  command = "cargo install hexo && hexo generate"
  publish = "public"

[build.environment]
  RUST_VERSION = "stable"
```

### Vercel

```json
// vercel.json
{
  "buildCommand": "cargo install hexo && hexo generate",
  "outputDirectory": "public",
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
      
      - name: Install Hexo
        run: cargo install hexo
      
      - name: Build site
        run: hexo generate
      
      - name: Deploy to GitHub Pages
        uses: peaceiris/actions-gh-pages@v3
        with:
          github_token: ${{ secrets.GITHUB_TOKEN }}
          publish_dir: ./public
```

## 高级功能

### 命令行工具

Hexo 提供了丰富的命令行工具：

- **hexo init**：初始化新站点
- **hexo new**：创建新文章
- **hexo generate**：生成静态文件
- **hexo server**：启动本地服务器
- **hexo deploy**：部署站点
- **hexo clean**：清理缓存

### 标签和分类

Hexo 支持通过标签和分类组织内容：

```markdown
---
title: "My First Post"
tags:
  - rust
  - hexo
categories:
  - tutorials
---
```

### 草稿

Hexo 支持草稿功能：

```bash
# 创建草稿
hexo new draft "My Draft"

# 发布草稿
hexo publish draft "My Draft"
```

### 自定义配置

您可以在 `_config.yml` 中自定义各种设置：

```yaml
# _config.yml

# 自定义永久链接
permalink: :category/:title/

# 自定义分页
per_page: 5

# 自定义日期格式
date_format: YYYY/MM/DD
```

## 性能优化

### 增量构建

Hexo 支持增量构建，只重新构建修改过的文件：

```bash
hexo generate --watch
```

### 缓存

Hexo 使用缓存来提高构建速度：
- **内容缓存**：缓存解析后的内容
- **模板缓存**：缓存编译后的模板

### 资源优化

- **压缩**：确保所有静态资源（HTML、CSS、JavaScript）都已压缩
- **图像优化**：优化图像大小和质量
- **代码分割**：使用代码分割减少初始加载时间

## 开发工具

### VS Code 扩展

推荐使用以下 VS Code 扩展：

- **Hexo**：提供语法高亮和智能提示
- **Prettier**：代码格式化
- **ESLint**：代码质量检查

### 调试

使用 `--debug` 标志启用详细输出：

```bash
hexo generate --debug
```

## 常见问题

### 1. Hexo 与其他静态站点生成器的区别是什么？

Hexo 的主要区别在于其博客专注的设计和丰富的主题生态系统，使其成为构建博客的理想选择。

### 2. 如何处理动态内容？

Hexo 主要针对静态内容优化，但您可以使用客户端 JavaScript 或外部服务来处理动态内容。

### 3. 如何优化构建速度？

- 使用增量构建
- 优化模板和内容
- 减少插件数量
- 使用适当的缓存策略

## 示例项目

### 博客示例

```
blog-example/
├── source/
│   ├── _posts/
│   │   ├── first-post.md
│   │   └── second-post.md
│   ├── _drafts/
│   │   └── my-draft.md
│   └── about/
│       └── index.md
├── themes/
│   └── landscape/
├── _config.yml
└── package.json
```

## 贡献指南

我们欢迎对 Hexo 的贡献！🤝

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

Hexo 受到原始 Hexo 项目的启发，并受益于 Rust 生态系统，包括 nargo 和 oak 库。

## 许可证

Hexo 在 AGPL-3.0 许可证下发布。有关更多信息，请参阅 [LICENSE](../../../license.md)。

---

使用 Hexo 构建愉快！🚀