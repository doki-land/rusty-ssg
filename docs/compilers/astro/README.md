# Astro - Rust Reimplementation

## 概述

Astro 是一个速度极快的静态站点生成器，现在使用 Rust 重新实现，以获得更好的性能和可靠性。它旨在帮助您轻松构建美丽、现代的网站，结合了静态站点生成的最佳特性和现代框架的强大功能。

### 🎯 核心特性

- 🚀 **快速构建**：在几秒钟内编译您的站点，而不是几分钟
- 🎨 **现代模板**：使用 Astro 独特的基于组件的方法和 .astro 文件
- 📦 **易于部署**：生成可在任何地方工作的静态文件
- 🔧 **可扩展**：通过插件和集成进行自定义
- 🛠 **开发者友好**：优秀的工具和开发体验
- 🌐 **框架无关**：使用 React、Vue、Svelte 或纯 HTML
- 🌍 **跨平台**：适用于 Windows、macOS 和 Linux
- 📱 **100% 兼容**：使用静态功能时完全兼容

## 安装

### 从 Crates.io 安装

```bash
cargo install astro
```

### 从源代码安装

```bash
# 克隆仓库
git clone https://github.com/doki-land/rusty-ssg.git

# 构建和安装
cd rusty-ssg/compilers/astro
cargo install --path .
```

## 基本用法

### 创建新站点

```bash
astro init my-site
cd my-site
```

### 本地开发

```bash
astro dev
```

这将启动一个带有热重载的本地开发服务器，默认端口为 4321，因此您可以实时查看更改。

### 生产构建

```bash
astro build
```

这将在 `dist` 目录中生成优化的静态文件， ready for deployment.

## 项目结构

一个典型的 Astro 项目结构如下：

```
my-site/
├── src/
│   ├── components/      # Astro 组件
│   ├── layouts/         # 布局组件
│   ├── pages/           # 页面文件
│   └── styles/          # 样式文件
├── public/              # 静态资源
├── astro.config.mjs     # Astro 配置文件
├── package.json         # 项目依赖
└── README.md            # 项目说明
```

## 配置

### 基本配置

Here's an example `astro.config.mjs` file:

```javascript
// astro.config.mjs
import { defineConfig } from 'astro';

export default defineConfig({
  // 站点设置
  site: 'https://example.com',
  base: '/',
  
  // 构建设置
  output: 'static',
  outDir: './dist',
  
  // 集成
  integrations: [
    // 添加集成
  ],
  
  // Markdown 设置
  markdown: {
    shikiConfig: {
      theme: 'github-dark',
    },
  },
  
  // 服务器设置
  server: {
    port: 3000,
    host: true,
  },
});
```

### 配置选项

| 选项 | 描述 | 默认值 |
|------|------|--------|
| `site` | 站点最终部署的链接 | `undefined` |
| `base` | 部署到的基本路径 | `/` |
| `trailing_slash` | 末尾斜杠设置 | `ignore` |
| `output` | 构建输出类型 (`static` 或 `server`) | `static` |
| `root` | 项目根目录 | `undefined` |
| `src_dir` | 源代码目录 | `./src` |
| `public_dir` | 静态资源目录 | `./public` |
| `out_dir` | 输出目录 | `dist` |
| `cache_dir` | 缓存目录 | `./node_modules/.astro` |
| `compress_html` | 是否压缩HTML | `true` |
| `scoped_style_strategy` | 样式作用范围策略 | `attribute` |
| `build.format` | 构建格式 | `directory` |
| `build.client` | 客户端输出目录 | `./client` |
| `build.server` | 服务器输出目录 | `./server` |
| `build.assets` | 资源目录 | `_astro` |
| `build.concurrency` | 并行构建页面数 | `1` |
| `server.port` | 开发服务器端口 | `4321` |
| `server.host` | 开发服务器主机 | `undefined` |
| `markdown.shiki_config` | Shiki配置 | `{}` |
| `markdown.gfm` | 是否使用GitHub-flavored Markdown | `true` |
| `markdown.smartypants` | 是否使用SmartyPants formatter | `true` |

## Astro 组件

### 基本组件

Here's an example of an Astro component:

```astro
---
// src/components/Header.astro
const { title } = Astro.props;
---

<header>
  <h1>{title}</h1>
  <nav>
    <a href="/">Home</a>
    <a href="/about">About</a>
    <a href="/blog">Blog</a>
  </nav>
</header>

<style>
  header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 1rem;
    background-color: #f0f0f0;
  }
  
  h1 {
    margin: 0;
  }
  
  nav a {
    margin-left: 1rem;
    text-decoration: none;
    color: #333;
  }
</style>
```

### 布局组件

```astro
---
// src/layouts/BaseLayout.astro
const { title, children } = Astro.props;
---

<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>{title}</title>
  <link rel="stylesheet" href="/styles/main.css">
</head>
<body>
  <Header title="My Site" />
  <main>
    {children}
  </main>
  <Footer />
</body>
</html>

<script>
  // 客户端 JavaScript (可选)
  console.log('Hello from Astro!');
</script>

<style>
  /* 全局样式 */
  body {
    font-family: Arial, sans-serif;
    margin: 0;
    padding: 0;
  }
  
  main {
    padding: 2rem;
  }
</style>
```

### 页面组件

```astro
---
// src/pages/index.astro
import BaseLayout from '../layouts/BaseLayout.astro';
import Header from '../components/Header.astro';
import Footer from '../components/Footer.astro';

export const title = 'Home Page';
---

<BaseLayout title={title}>
  <section>
    <h2>Welcome to My Site</h2>
    <p>This is the home page of my Astro site.</p>
    <p>Astro is a fast, modern static site generator that lets you use your favorite JavaScript frameworks while generating fully static HTML.</p>
  </section>
</BaseLayout>
```

## Markdown 支持

### 基本 Markdown

```markdown
---
title: "Getting Started with Astro"
date: 2024-01-01
author: "Your Name"
categories: ["tutorial", "getting-started"]
tags: ["astro", "static-site-generator"]
---

# Getting Started with Astro

Welcome to Astro! This is your first blog post.

## What is Astro?

Astro is a fast, modern static site generator that lets you use your favorite JavaScript frameworks while generating fully static HTML.

## Why Use Astro?

- It's blazingly fast
- It supports multiple frameworks (React, Vue, Svelte)
- It has a unique component-based approach
- It's 100% compatible with static features

## Next Steps

1. Create more content
2. Customize your components
3. Add integrations
4. Deploy your site

Happy coding! 🎉
```

### 高级 Markdown 功能

Astro 支持以下 Markdown 功能：

- **Front Matter**：在文件顶部使用 YAML、TOML 或 JSON 定义元数据
- **代码高亮**：使用 Shiki 进行语法高亮
- **数学公式**：使用 KaTeX 渲染数学公式
- **图表**：使用 Mermaid 渲染图表
- **组件嵌入**：在 Markdown 中嵌入 Astro 组件

## 插件系统

Astro 支持通过插件扩展功能，使用 IPC 模式与插件通信。

### 内置插件

- 📊 **@astrojs/katex**：渲染数学公式
- 🎨 **@astrojs/prism**：代码块语法高亮
- 📈 **@astrojs/mermaid**：渲染图表和流程图
- 🔍 **@astrojs/google-analytics**：添加 Google Analytics 跟踪
- 🗺️ **@astrojs/sitemap**：生成 sitemap.xml
- 📱 **@astrojs/pwa**：添加 PWA 支持

### 使用插件

在 `astro.config.mjs` 中配置插件：

```javascript
// astro.config.mjs
import { defineConfig } from 'astro';
import katex from '@astrojs/katex';
import sitemap from '@astrojs/sitemap';

export default defineConfig({
  integrations: [
    katex(),
    sitemap(),
  ],
});
```

## 主题系统

Astro 支持主题，您可以选择内置主题或创建自己的主题。

### 内置主题

- 🎨 **default**：干净、现代的设计
- 🌙 **dark**：深色模式主题
- 📦 **minimal**：极简主义设计
- 📝 **blog**：博客专注的主题

### 使用主题

在 `astro.config.mjs` 中配置主题：

```javascript
// astro.config.mjs
import { defineConfig } from 'astro';
import myTheme from 'my-astro-theme';

export default defineConfig({
  theme: myTheme,
});
```

## 部署

Astro 生成可在任何地方部署的静态文件。

### Netlify

```toml
# netlify.toml
[build]
  command = "astro build"
  publish = "dist"
```

### Vercel

```json
// vercel.json
{
  "buildCommand": "astro build",
  "outputDirectory": "dist"
}
```

### GitHub Pages

```yaml
# .github/workflows/deploy.yml
name: Deploy
on: [push]
jobs:
  deploy:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - run: cargo install astro
      - run: astro build
      - uses: peaceiris/actions-gh-pages@v3
        with:
          github_token: ${{ secrets.GITHUB_TOKEN }}
          publish_dir: ./dist
```

### Cloudflare Pages

```yaml
# .github/workflows/deploy.yml
name: Deploy to Cloudflare Pages
on: [push]
jobs:
  deploy:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - run: cargo install astro
      - run: astro build
      - name: Deploy to Cloudflare Pages
        uses: cloudflare/pages-action@v1
        with:
          apiToken: ${{ secrets.CLOUDFLARE_API_TOKEN }}
          accountId: ${{ secrets.CLOUDFLARE_ACCOUNT_ID }}
          projectName: my-site
          directory: ./dist
```

## 高级功能

### 框架集成

Astro 支持集成多种前端框架：

- **React**：使用 `@astrojs/react` 集成
- **Vue**：使用 `@astrojs/vue` 集成
- **Svelte**：使用 `@astrojs/svelte` 集成
- **Solid**：使用 `@astrojs/solid-js` 集成

### 示例：集成 React

```javascript
// astro.config.mjs
import { defineConfig } from 'astro';
import react from '@astrojs/react';

export default defineConfig({
  integrations: [react()],
});
```

```astro
---
// src/components/ReactComponent.astro
import React from 'react';
import ReactCounter from '../components/ReactCounter.jsx';
---

<div>
  <ReactCounter client:load />
</div>
```

```jsx
// src/components/ReactCounter.jsx
import React, { useState } from 'react';

export default function ReactCounter() {
  const [count, setCount] = useState(0);
  
  return (
    <div>
      <p>Count: {count}</p>
      <button onClick={() => setCount(count + 1)}>Increment</button>
    </div>
  );
}
```

### 数据获取

Astro 支持在组件中获取数据：

```astro
---
// src/pages/blog/index.astro
import BaseLayout from '../../layouts/BaseLayout.astro';

// 从 API 获取数据
const response = await fetch('https://api.example.com/posts');
const posts = await response.json();
---

<BaseLayout title="Blog">
  <h1>Blog Posts</h1>
  <ul>
    {posts.map(post => (
      <li key={post.id}>
        <a href={`/blog/${post.slug}`}>{post.title}</a>
        <p>{post.excerpt}</p>
      </li>
    ))}
  </ul>
</BaseLayout>
```

### 动态路由

Astro 支持动态路由：

```astro
---
// src/pages/blog/[slug].astro
import BaseLayout from '../../layouts/BaseLayout.astro';

// 获取路由参数
const { slug } = Astro.params;

// 从 API 获取文章数据
const response = await fetch(`https://api.example.com/posts/${slug}`);
const post = await response.json();
---

<BaseLayout title={post.title}>
  <h1>{post.title}</h1>
  <p>{post.date}</p>
  <div set:html={post.content}></div>
</BaseLayout>
```

## 性能优化

### 代码分割

Astro 自动进行代码分割，只加载页面所需的 JavaScript。

### 图像优化

Astro 提供内置的图像优化功能：

```astro
---
import { Image } from 'astro:assets';
import myImage from '../images/photo.jpg';
---

<Image
  src={myImage}
  alt="Description"
  width={300}
  height={200}
  loading="lazy"
/>
```

### 预加载

Astro 支持预加载关键资源：

```astro
---
// src/pages/index.astro
---

<head>
  <link rel="preload" href="/styles/main.css" as="style" />
  <link rel="preload" href="/fonts/roboto.woff2" as="font" type="font/woff2" crossorigin />
</head>
```

## 开发工具

### VS Code 扩展

推荐使用以下 VS Code 扩展：

- **Astro**：提供语法高亮和智能提示
- **Prettier**：代码格式化
- **ESLint**：代码质量检查

### 调试

在 `astro.config.mjs` 中启用调试模式：

```javascript
// astro.config.mjs
import { defineConfig } from 'astro';

export default defineConfig({
  devToolbar: {
    enabled: true,
  },
});
```

## 常见问题

### 1. Astro 与其他静态站点生成器的区别是什么？

Astro 的主要区别在于其独特的组件架构和 " islands " 概念，允许您在静态 HTML 中选择性地激活交互式组件。

### 2. 如何处理动态内容？

Astro 主要针对静态内容优化，但您可以使用客户端 JavaScript 或集成服务器端渲染来处理动态内容。

### 3. 如何优化构建速度？

- 使用增量构建
- 优化图像和静态资源
- 减少插件数量
- 使用适当的缓存策略

### 4. 如何添加自定义字体？

```astro
---
---

<head>
  <link rel="preconnect" href="https://fonts.googleapis.com" />
  <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin />
  <link href="https://fonts.googleapis.com/css2?family=Roboto:wght@400;700&display=swap" rel="stylesheet" />
  <style>
    body {
      font-family: 'Roboto', sans-serif;
    }
  </style>
</head>
```

## 故障排除

### 构建失败

- 检查配置文件中的语法错误
- 确保所有依赖项都已正确安装
- 检查文件路径是否正确
- 查看详细的错误信息以确定问题所在

### 开发服务器问题

- 确保端口未被占用
- 检查防火墙设置
- 清除浏览器缓存
- 重启开发服务器

### 插件问题

- 确保插件版本与 Astro 版本兼容
- 检查插件配置是否正确
- 查看插件文档以了解正确的用法

## 示例项目

### 博客示例

```
blog-example/
├── src/
│   ├── components/
│   │   ├── Header.astro
│   │   ├── Footer.astro
│   │   └── PostCard.astro
│   ├── layouts/
│   │   └── BaseLayout.astro
│   ├── pages/
│   │   ├── index.astro
│   │   ├── about.astro
│   │   └── blog/
│   │       ├── index.astro
│   │       └── [slug].astro
│   └── styles/
│       └── main.css
├── public/
│   └── images/
├── astro.config.mjs
└── package.json
```

### 文档站点示例

```
docs-site/
├── src/
│   ├── components/
│   │   ├── Sidebar.astro
│   │   ├── Navbar.astro
│   │   └── Footer.astro
│   ├── layouts/
│   │   └── DocsLayout.astro
│   ├── pages/
│   │   ├── index.astro
│   │   └── guide/
│   │       ├── getting-started.astro
│   │       └── advanced.astro
│   └── styles/
│       └── docs.css
├── public/
│   └── assets/
├── astro.config.mjs
└── package.json
```

## 贡献指南

我们欢迎对 Astro 的贡献！🤝

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

Astro 受到原始 Astro 项目的启发，并受益于 Rust 生态系统，包括 nargo 和 oak 库。

## 许可证

Astro 在 AGPL-3.0 许可证下发布。有关更多信息，请参阅 [LICENSE](../../../license.md)。

---

使用 Astro 构建愉快！🚀