# Eleventy - Rust Reimplementation

## 概述

Eleventy 是一个灵活的静态站点生成器，现在使用 Rust 重新实现，以获得更好的性能和可靠性。它采用内容优先的方法，支持多种模板引擎，使您能够使用自己喜欢的工具构建静态站点。

### 🎯 核心特性

- 🚀 **快速构建**：在几秒钟内编译您的站点，而不是几分钟
- 🎨 **多模板引擎支持**：支持 Nunjucks、Markdown、HTML、Liquid、Handlebars、Mustache、EJS 等
- 📦 **易于部署**：生成可在任何地方工作的静态文件
- 🔧 **可扩展**：通过插件和配置进行自定义
- 🛠 **开发者友好**：优秀的工具和开发体验
- 📝 **内容优先**：专注于内容创作
- 🌍 **跨平台**：适用于 Windows、macOS 和 Linux
- 📱 **100% 兼容**：使用静态功能时完全兼容

## 安装

### 从 Crates.io 安装

```bash
cargo install eleventy
```

### 从源代码安装

```bash
# 克隆仓库
git clone https://github.com/doki-land/rusty-ssg.git

# 构建和安装
cd rusty-ssg/compilers/eleventy
cargo install --path .
```

## 基本用法

### 创建新站点

```bash
eleventy init my-site
cd my-site
```

### 本地开发

```bash
eleventy --serve
```

这将启动一个带有热重载的本地开发服务器，因此您可以实时查看更改。

### 生产构建

```bash
eleventy build
```

这将在 `_site` 目录中生成优化的静态文件，ready for deployment.

## 项目结构

一个典型的 Eleventy 项目结构如下：

```
my-site/
├── src/                # 源文件
│   ├── _data/          # 数据文件
│   ├── _includes/      # 包含文件和模板
│   ├── _layouts/       # 布局模板
│   ├── assets/         # 静态资源
│   └── index.md        # 首页
├── .eleventy.js        # 配置文件
└── README.md           # 项目说明
```

## 配置

### 基本配置

Here's an example `.eleventy.js` file:

```javascript
// .eleventy.js
module.exports = function(eleventyConfig) {
  // 复制静态资源
  eleventyConfig.addPassthroughCopy('src/assets');
  
  // 添加自定义过滤器
  eleventyConfig.addFilter('uppercase', function(value) {
    return value.toUpperCase();
  });
  
  // 添加自定义短代码
  eleventyConfig.addShortcode('year', function() {
    return new Date().getFullYear();
  });
  
  return {
    input_dir: 'src',
    output_dir: '_site',
    template_dir: '_includes',
    data_dir: '_data',
    plugins: [
      {
        name: '@11ty/eleventy-plugin-syntaxhighlight',
        options: {}
      },
      {
        name: '@11ty/eleventy-plugin-katex',
        options: {}
      }
    ],
    global_data: {
      site: {
        title: 'My Eleventy Site',
        description: 'A site built with Rusty Eleventy'
      }
    }
  };
};
```

### 配置选项

| 选项 | 描述 | 默认值 |
|------|------|--------|
| `input_dir` | 输入目录 | `"."` |
| `output_dir` | 输出目录 | `"_site"` |
| `template_dir` | 模板目录 | `"_includes"` |
| `data_dir` | 数据目录 | `"_data"` |
| `plugins` | 插件配置 | `[]` |
| `global_data` | 全局数据 | `{}` |
| `filters` | 过滤器配置 | `{}` |
| `collections` | 集合配置 | `{}` |

## 模板系统

### Nunjucks 模板

```html
<!-- src/_layouts/base.njk -->
<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>{{ title }}</title>
  <link rel="stylesheet" href="/assets/css/style.css">
</head>
<body>
  {% include "header.njk" %}
  <main>
    {{ content | safe }}
  </main>
  {% include "footer.njk" %}
</body>
</html>
```

### Markdown 文件

```markdown
---
title: "Getting Started with Eleventy"
date: 2024-01-01
author: "Your Name"
tags: ["eleventy", "static-site-generator"]
layout: "base.njk"
---

# Getting Started with Eleventy

Welcome to Eleventy! This is your first post.

## What is Eleventy?

Eleventy is a flexible static site generator that lets you use your favorite template engines.

## Why Use Eleventy?

- It's flexible and customizable
- It supports multiple template engines
- It's content-first
- It's 100% compatible with static features

## Next Steps

1. Create more content
2. Customize your templates
3. Add plugins
4. Deploy your site

Happy coding! 🎉
```

## 数据文件

Eleventy 支持多种数据文件格式：

### JSON 数据文件

```json
// src/_data/site.json
{
  "title": "My Eleventy Site",
  "description": "A site built with Rusty Eleventy",
  "author": "Your Name",
  "baseUrl": "https://example.com"
}
```

### JavaScript 数据文件

```javascript
// src/_data/navigation.js
module.exports = [
  { "name": "Home", "url": "/" },
  { "name": "About", "url": "/about/" },
  { "name": "Blog", "url": "/blog/" }
];
```

### YAML 数据文件

```yaml
# src/_data/team.yml
- name: "John Doe"
  position: "Developer"
  email: "john@example.com"
- name: "Jane Smith"
  position: "Designer"
  email: "jane@example.com"
```

## 插件系统

Eleventy 支持通过插件扩展功能。

### 内置插件

- 📊 **@11ty/eleventy-plugin-katex**：渲染数学公式
- 🎨 **@11ty/eleventy-plugin-syntaxhighlight**：代码块语法高亮
- 📈 **@11ty/eleventy-plugin-mermaid**：渲染图表和流程图
- 🔍 **@11ty/eleventy-plugin-google-analytics**：添加 Google Analytics 跟踪
- 🗺️ **@11ty/eleventy-plugin-sitemap**：生成 sitemap.xml

### 使用插件

在 `.eleventy.js` 中配置插件：

```javascript
// .eleventy.js
const syntaxHighlight = require('@11ty/eleventy-plugin-syntaxhighlight');
const katex = require('@11ty/eleventy-plugin-katex');

module.exports = function(eleventyConfig) {
  eleventyConfig.addPlugin(syntaxHighlight);
  eleventyConfig.addPlugin(katex);
  
  return {
    // 配置选项
  };
};
```

## 部署

Eleventy 生成可在任何地方部署的静态文件。

### Netlify

```toml
# netlify.toml
[build]
  command = "cargo install eleventy && eleventy build"
  publish = "_site"

[build.environment]
  RUST_VERSION = "stable"
```

### Vercel

```json
// vercel.json
{
  "buildCommand": "cargo install eleventy && eleventy build",
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
      
      - name: Install Eleventy
        run: cargo install eleventy
      
      - name: Build site
        run: eleventy build
      
      - name: Deploy to GitHub Pages
        uses: peaceiris/actions-gh-pages@v3
        with:
          github_token: ${{ secrets.GITHUB_TOKEN }}
          publish_dir: ./_site
```

## 高级功能

### 分页

Eleventy 提供内置的分页功能：

```html
<!-- src/blog/index.njk -->
---
layout: base.njk
title: Blog
pagination:
  data: collections.posts
  size: 5
  alias: posts
---

<h1>Blog</h1>

<ul>
  {% for post in posts %}
    <li>
      <a href="{{ post.url }}">{{ post.data.title }}</a>
      <p>{{ post.data.date | date('YYYY-MM-DD') }}</p>
      <p>{{ post.data.description }}</p>
    </li>
  {% endfor %}
</ul>

{% if pagination.href.previous %}
  <a href="{{ pagination.href.previous }}">Previous</a>
{% endif %}

{% if pagination.href.next %}
  <a href="{{ pagination.href.next }}">Next</a>
{% endif %}
```

### 集合

Eleventy 支持通过集合组织内容：

```javascript
// .eleventy.js
module.exports = function(eleventyConfig) {
  // 创建一个博客文章集合
  eleventyConfig.addCollection('posts', function(collectionApi) {
    return collectionApi.getFilteredByGlob('src/posts/**/*.md').reverse();
  });
  
  return {
    // 配置选项
  };
};
```

### 短路代码

Eleventy 支持短路代码（shortcodes）：

```javascript
// .eleventy.js
module.exports = function(eleventyConfig) {
  // 添加自定义短路代码
  eleventyConfig.addShortcode('figure', function(src, alt, caption) {
    return `<figure>
      <img src="${src}" alt="${alt}">
      ${caption ? `<figcaption>${caption}</figcaption>` : ''}
    </figure>`;
  });
  
  return {
    // 配置选项
  };
};
```

使用短路代码：

```markdown
{% figure "image.jpg" "Description" "Image caption" %}
```

## 性能优化

### 增量构建

Eleventy 支持增量构建，只重新构建修改过的文件：

```bash
eleventy build --incremental
```

### 缓存

Eleventy 使用缓存来提高构建速度：

- **内容缓存**：缓存解析后的内容
- **模板缓存**：缓存编译后的模板
- **数据缓存**：缓存处理后的数据

### 资源优化

- **压缩**：确保所有静态资源（HTML、CSS、JavaScript）都已压缩
- **图像优化**：优化图像大小和质量
- **代码分割**：使用代码分割减少初始加载时间

## 开发工具

### VS Code 扩展

推荐使用以下 VS Code 扩展：

- **Eleventy**：提供语法高亮和智能提示
- **Prettier**：代码格式化
- **ESLint**：代码质量检查

### 调试

使用 `--verbose` 标志启用详细输出：

```bash
eleventy --verbose
```

## 常见问题

### 1. Eleventy 与其他静态站点生成器的区别是什么？

Eleventy 的主要区别在于其灵活性和对多种模板引擎的支持，使其成为内容优先项目的理想选择。

### 2. 如何处理动态内容？

Eleventy 主要针对静态内容优化，但您可以使用客户端 JavaScript 或外部服务来处理动态内容。

### 3. 如何优化构建速度？

- 使用增量构建
- 优化模板和内容
- 减少插件数量
- 使用适当的缓存策略

## 示例项目

### 博客示例

```
blog-example/
├── src/
│   ├── _data/
│   │   ├── site.json
│   │   └── navigation.js
│   ├── _includes/
│   │   ├── header.njk
│   │   └── footer.njk
│   ├── _layouts/
│   │   └── base.njk
│   ├── assets/
│   │   ├── css/
│   │   │   └── style.css
│   │   └── images/
│   ├── posts/
│   │   ├── first-post.md
│   │   └── second-post.md
│   ├── blog.njk
│   └── index.md
├── .eleventy.js
└── README.md
```

### 文档站点示例

```
docs-site/
├── src/
│   ├── _data/
│   │   ├── site.json
│   │   └── navigation.js
│   ├── _includes/
│   │   ├── header.njk
│   │   ├── sidebar.njk
│   │   └── footer.njk
│   ├── _layouts/
│   │   └── base.njk
│   ├── assets/
│   │   ├── css/
│   │   │   └── docs.css
│   │   └── js/
│   ├── guide/
│   │   ├── getting-started.md
│   │   └── advanced.md
│   ├── index.md
│   └── guide.njk
├── .eleventy.js
└── README.md
```

## 贡献指南

我们欢迎对 Eleventy 的贡献！🤝

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

Eleventy 受到原始 Eleventy 项目的启发，并受益于 Rust 生态系统，包括 nargo 和 oak 库。

## 许可证

Eleventy 在 AGPL-3.0 许可证下发布。有关更多信息，请参阅 [LICENSE](../../../license.md)。

---

使用 Eleventy 构建愉快！🚀