# Jekyll - Rust Reimplementation

## 概述

Jekyll 是一个基于 Ruby 的静态站点生成器，现在使用 Rust 重新实现，以获得更好的性能和可靠性。它使用 Liquid 模板系统，支持 Markdown 内容，是 GitHub Pages 的默认静态站点生成器。

### 🎯 核心特性

- 🚀 **快速构建**：在几秒钟内编译您的站点，而不是几分钟
- 🎨 **Liquid 模板系统**：使用灵活的 Liquid 模板语言
- 📦 **易于部署**：生成可在任何地方工作的静态文件
- 🔧 **可扩展**：通过插件和主题进行自定义
- 🛠 **开发者友好**：优秀的工具和开发体验
- 📝 **Markdown 支持**：轻松使用 Markdown 编写内容
- 🌍 **跨平台**：适用于 Windows、macOS 和 Linux
- 📱 **100% 兼容**：使用静态功能时完全兼容

## 安装

### 从 Crates.io 安装

```bash
cargo install jekyll
```

### 从源代码安装

```bash
# 克隆仓库
git clone https://github.com/doki-land/rusty-ssg.git

# 构建和安装
cd rusty-ssg/compilers/jekyll
cargo install --path .
```

## 基本用法

### 创建新站点

```bash
jekyll new my-site
cd my-site
```

### 本地开发

```bash
jekyll serve
```

这将启动一个带有热重载的本地开发服务器，因此您可以实时查看更改。

### 生产构建

```bash
jekyll build
```

这将在 `_site` 目录中生成优化的静态文件，ready for deployment.

## 项目结构

一个典型的 Jekyll 项目结构如下：

```
my-site/
├── _posts/            # 博客文章
├── _layouts/          # 布局模板
├── _includes/         # 包含文件
├── _sass/             # Sass 文件
├── _data/             # 数据文件
├── assets/            # 静态资源
├── _config.yml        # 配置文件
└── index.md           # 首页
```

## 配置

### 基本配置

Here's an example `_config.yml` file:

```yaml
# _config.yml

# Site settings
title: My Jekyll Site
description: A site built with Rusty Jekyll
author: Your Name
baseurl: "" # the subpath of your site, e.g. /blog
url: "https://example.com" # the base hostname & protocol for your site

# Build settings
markdown: kramdown
theme: minima
gems:
  - jekyll-feed
  - jekyll-sitemap

# Exclude from processing.
exclude:
  - Gemfile
  - Gemfile.lock
  - node_modules
  - vendor/bundle/
  - vendor/cache/
  - vendor/gems/
  - vendor/ruby/
```

### 配置选项

| 选项 | 描述 | 默认值 |
|------|------|--------|
| `title` | 站点标题 | `""` |
| `description` | 站点描述 | `""` |
| `author` | 站点作者 | `""` |
| `baseurl` | 站点子路径 | `""` |
| `url` | 站点 URL | `""` |
| `markdown` | Markdown 解析器 | `"kramdown"` |
| `theme` | 主题名称 | `"minima"` |

## 内容管理

### 博客文章

```markdown
---
layout: post
title: "My First Post"
date: 2024-01-01 10:00:00 +0000
categories: [tutorials, rust]
tags: [jekyll, static-site-generator]
---

# My First Post

Welcome to Jekyll! This is your first post.

## What is Jekyll?

Jekyll is a simple, blog-aware, static site generator that uses Markdown and Liquid templates.

## Why Use Jekyll?

- It's simple and straightforward
- It's blog-aware
- It's the default static site generator for GitHub Pages
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
layout: page
title: "About"
date: 2024-01-01 10:00:00 +0000
---

# About Me

Hello! I'm using Jekyll to build this blog.

## My Background

I'm a web developer passionate about static site generators and modern web technologies.

## Contact

Feel free to reach out if you have any questions! 📧
```

## 模板系统

Jekyll 使用 Liquid 模板系统：

### 布局模板

```html
<!-- _layouts/default.html -->
<!DOCTYPE html>
<html>
<head>
  <meta charset="UTF-8">
  <title>{{ page.title | default: site.title }}</title>
  <link rel="stylesheet" href="{{ "/assets/main.css" | relative_url }}">
</head>
<body>
  {% include header.html %}
  <main>
    {{ content }}
  </main>
  {% include footer.html %}
</body>
</html>
```

### 包含文件

```html
<!-- _includes/header.html -->
<header>
  <h1><a href="{{ "/" | relative_url }}">{{ site.title }}</a></h1>
  <nav>
    <a href="{{ "/" | relative_url }}">Home</a>
    <a href="{{ "/about" | relative_url }}">About</a>
    <a href="{{ "/blog" | relative_url }}">Blog</a>
  </nav>
</header>
```

### 首页模板

```html
<!-- index.html -->
---
layout: default
title: Home
---

<h1>Welcome to {{ site.title }}</h1>
<p>{{ site.description }}</p>

<h2>Recent Posts</h2>
<ul>
  {% for post in site.posts limit: 5 %}
    <li>
      <a href="{{ post.url | relative_url }}">{{ post.title }}</a>
      <p>{{ post.date | date: "%B %d, %Y" }}</p>
    </li>
  {% endfor %}
</ul>
```

## 主题系统

Jekyll 支持主题，您可以选择内置主题或创建自己的主题。

### 内置主题

- 🎨 **minima**：默认主题
- 🌙 **minima-dark**：深色模式主题
- 📦 **jekyll-theme-primer**：GitHub 风格主题
- 📝 **jekyll-theme-slate**：Slate 主题

### 使用主题

在 `_config.yml` 中配置主题：

```yaml
# _config.yml
theme: minima
```

## 插件系统

Jekyll 支持通过插件扩展功能。

### 内置插件

- 📊 **jekyll-katex**：渲染数学公式
- 🎨 **jekyll-syntax-highlighter**：代码块语法高亮
- 📈 **jekyll-mermaid**：渲染图表和流程图
- 🔍 **jekyll-google-analytics**：添加 Google Analytics 跟踪
- 🗺️ **jekyll-sitemap**：生成 sitemap.xml
- 📱 **jekyll-feed**：生成 RSS 订阅

### 使用插件

在 `_config.yml` 中配置插件：

```yaml
# _config.yml
gems:
  - jekyll-feed
  - jekyll-sitemap
  - jekyll-katex
  - jekyll-syntax-highlighter
```

## 部署

Jekyll 生成可在任何地方部署的静态文件。

### Netlify

```toml
# netlify.toml
[build]
  command = "cargo install jekyll && jekyll build"
  publish = "_site"

[build.environment]
  RUST_VERSION = "stable"
```

### Vercel

```json
// vercel.json
{
  "buildCommand": "cargo install jekyll && jekyll build",
  "outputDirectory": "_site",
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
      
      - name: Install Jekyll
        run: cargo install jekyll
      
      - name: Build site
        run: jekyll build
      
      - name: Deploy to GitHub Pages
        uses: peaceiris/actions-gh-pages@v3
        with:
          github_token: ${{ secrets.GITHUB_TOKEN }}
          publish_dir: ./_site
```

## 高级功能

### 数据文件

Jekyll 支持使用数据文件存储结构化数据：

```yaml
# _data/navigation.yml
- name: Home
  url: /
- name: About
  url: /about/
- name: Blog
  url: /blog/
```

在模板中使用数据：

```html
<!-- _includes/header.html -->
<header>
  <h1><a href="{{ "/" | relative_url }}">{{ site.title }}</a></h1>
  <nav>
    {% for item in site.data.navigation %}
      <a href="{{ item.url | relative_url }}">{{ item.name }}</a>
    {% endfor %}
  </nav>
</header>
```

### 集合

Jekyll 支持通过集合组织内容：

```yaml
# _config.yml
collections:
  docs:
    output: true
    permalink: /:collection/:path/
```

### 前置数据

Jekyll 支持在 Markdown 文件顶部使用前置数据（front matter）：

```markdown
---
layout: post
title: "My First Post"
date: 2024-01-01 10:00:00 +0000
categories: [tutorials, rust]
tags: [jekyll, static-site-generator]
author: "Your Name"
image: "featured-image.jpg"
---
```

## 性能优化

### 增量构建

Jekyll 支持增量构建，只重新构建修改过的文件：

```bash
jekyll build --incremental
```

### 缓存

Jekyll 使用缓存来提高构建速度：
- **内容缓存**：缓存解析后的内容
- **模板缓存**：缓存编译后的模板

### 资源优化

- **压缩**：确保所有静态资源（HTML、CSS、JavaScript）都已压缩
- **图像优化**：优化图像大小和质量
- **代码分割**：使用代码分割减少初始加载时间

## 开发工具

### VS Code 扩展

推荐使用以下 VS Code 扩展：

- **Jekyll**：提供语法高亮和智能提示
- **Liquid**：Liquid 模板语法支持
- **Prettier**：代码格式化
- **ESLint**：代码质量检查

### 调试

使用 `--verbose` 标志启用详细输出：

```bash
jekyll build --verbose
```

## 常见问题

### 1. Jekyll 与其他静态站点生成器的区别是什么？

Jekyll 的主要区别在于其简单性和与 GitHub Pages 的集成，使其成为 GitHub Pages 用户的理想选择。

### 2. 如何处理动态内容？

Jekyll 主要针对静态内容优化，但您可以使用客户端 JavaScript 或外部服务来处理动态内容。

### 3. 如何优化构建速度？

- 使用增量构建
- 优化模板和内容
- 减少插件数量
- 使用适当的缓存策略

## 示例项目

### 博客示例

```
blog-example/
├── _posts/
│   ├── 2024-01-01-my-first-post.md
│   └── 2024-01-02-second-post.md
├── _layouts/
│   ├── default.html
│   └── post.html
├── _includes/
│   ├── header.html
│   └── footer.html
├── assets/
│   └── main.css
├── _config.yml
└── index.md
```

## 贡献指南

我们欢迎对 Jekyll 的贡献！🤝

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

Jekyll 受到原始 Jekyll 项目的启发，并受益于 Rust 生态系统，包括 nargo 和 oak 库。

## 许可证

Jekyll 在 AGPL-3.0 许可证下发布。有关更多信息，请参阅 [LICENSE](../../../license.md)。

---

使用 Jekyll 构建愉快！🚀