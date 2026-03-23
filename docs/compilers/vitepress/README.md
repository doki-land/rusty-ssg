# VitePress - Rust Reimplementation

## 概述

VitePress 是一个基于 Vue 的文档站点生成器，现在使用 Rust 重新实现，以获得更好的性能和可靠性。它旨在帮助您轻松构建漂亮、现代的文档站点，结合了 Vite 的速度和 Vue 的灵活性。

### 🎯 核心特性

- 🚀 **快速构建**：在几秒钟内编译您的站点，而不是几分钟
- 🎨 **现代模板**：使用 Vue 组件和 Markdown
- 📦 **易于部署**：生成可在任何地方工作的静态文件
- 🔧 **可扩展**：通过插件和主题进行自定义
- 🛠 **开发者友好**：优秀的工具和开发体验
- 📝 **Markdown 支持**：高级 Markdown 解析和渲染
- 🌍 **跨平台**：适用于 Windows、macOS 和 Linux
- 📱 **100% 兼容**：使用静态功能时完全兼容

## 安装

### 从 Crates.io 安装

```bash
cargo install vitepress
```

### 从源代码安装

```bash
# 克隆仓库
git clone https://github.com/doki-land/rusty-ssg.git

# 构建和安装
cd rusty-ssg/compilers/vitepress
cargo install --path .
```

## 基本用法

### 创建新站点

```bash
vitepress init my-docs
cd my-docs
```

### 本地开发

```bash
vitepress dev
```

这将启动一个带有热重载的本地开发服务器，因此您可以实时查看更改。

### 生产构建

```bash
vitepress build
```

这将在 `.vitepress/dist` 目录中生成优化的静态文件，ready for deployment.

## 项目结构

一个典型的 VitePress 项目结构如下：

```
my-docs/
├── .vitepress/         # 配置和主题
│   ├── config.ts       # VitePress 配置文件
│   └── theme/          # 自定义主题
├── guide/              # 指南内容
│   ├── getting-started.md
│   └── advanced.md
├── index.md            # 首页
└── README.md           # 项目说明
```

## 配置

### 基本配置

Here's an example `.vitepress/config.ts` file:

```typescript
// .vitepress/config.ts
import { defineConfig } from 'vitepress';

export default defineConfig({
  // 站点设置
  title: 'My Docs',
  description: 'A documentation site built with VitePress',
  base: '/',
  
  // 主题设置
  themeConfig: {
    nav: [
      { text: 'Home', link: '/' },
      { text: 'Guide', link: '/guide/getting-started' },
      { text: 'API', link: '/api' },
    ],
    sidebar: {
      '/guide/': [
        {
          text: 'Guide',
          items: [
            { text: 'Getting Started', link: '/guide/getting-started' },
            { text: 'Advanced', link: '/guide/advanced' },
          ],
        },
      ],
    },
  },
  
  // Markdown 设置
  markdown: {
    theme: 'github-dark',
    lineNumbers: true,
  },
  
  // 构建设置
  build: {
    outDir: '.vitepress/dist',
  },
});
```

### 配置选项

| 选项 | 描述 | 默认值 |
|------|------|--------|
| `title` | 站点标题 | `""` |
| `description` | 站点描述 | `""` |
| `base` | 站点的基础路径 | `/` |
| `themeConfig.nav` | 导航菜单配置 | `[]` |
| `themeConfig.sidebar` | 侧边栏配置 | `{}` |
| `markdown.theme` | 代码高亮主题 | `"github-dark"` |
| `markdown.lineNumbers` | 是否显示行号 | `false` |
| `build.outDir` | 输出目录 | `.vitepress/dist` |

## Markdown 支持

### 基本 Markdown

```markdown
---
title: "Getting Started with VitePress"
date: 2024-01-01
---

# Getting Started with VitePress

Welcome to VitePress! This is your first documentation page.

## What is VitePress?

VitePress is a documentation site generator built on top of Vite and Vue.

## Why Use VitePress?

- It's blazingly fast
- It has a clean, modern design
- It supports Vue components in Markdown
- It's 100% compatible with static features

## Next Steps

1. Create more content
2. Customize your theme
3. Add plugins
4. Deploy your site

Happy coding! 🎉
```

### 高级 Markdown 功能

VitePress 支持以下 Markdown 功能：

- **Front Matter**：在文件顶部使用 YAML 定义元数据
- **代码高亮**：使用 Shiki 进行语法高亮
- **数学公式**：使用 KaTeX 渲染数学公式
- **图表**：使用 Mermaid 渲染图表
- **Vue 组件**：在 Markdown 中嵌入 Vue 组件
- **自定义容器**：使用自定义容器添加提示、警告等

### 自定义容器

```markdown
::: tip
This is a tip
:::

::: warning
This is a warning
:::

::: danger
This is a danger message
:::
```

## 主题系统

VitePress 支持主题，您可以选择内置主题或创建自己的主题。

### 内置主题

- 🎨 **default**：干净、现代的文档设计
- 🌙 **dark**：深色模式主题
- 📦 **minimal**：极简主义设计

### 自定义主题

创建自定义主题：

```typescript
// .vitepress/theme/index.ts
import { Theme } from 'vitepress';
import DefaultTheme from 'vitepress/theme';

export default {
  ...DefaultTheme,
  enhanceApp({ app, router, siteData }) {
    // 增强应用
  },
} as Theme;
```

## 插件系统

VitePress 支持通过插件扩展功能，使用 IPC 模式与插件通信。

### 内置插件

- 📊 **katex**：渲染数学公式
- 🎨 **prism**：代码块语法高亮
- 📈 **mermaid**：渲染图表和流程图
- 🔍 **search**：内置搜索功能

### 使用插件

在 `.vitepress/config.ts` 中配置插件：

```typescript
// .vitepress/config.ts
import { defineConfig } from 'vitepress';
import katex from 'vitepress-plugin-katex';
import mermaid from 'vitepress-plugin-mermaid';

export default defineConfig({
  markdown: {
    config: (md) => {
      md.use(katex);
      md.use(mermaid);
    },
  },
});
```

## 部署

VitePress 生成可在任何地方部署的静态文件。

### Netlify

```toml
# netlify.toml
[build]
  command = "vitepress build"
  publish = ".vitepress/dist"
```

### Vercel

```json
// vercel.json
{
  "buildCommand": "vitepress build",
  "outputDirectory": ".vitepress/dist"
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
      - run: cargo install vitepress
      - run: vitepress build
      - uses: peaceiris/actions-gh-pages@v3
        with:
          github_token: ${{ secrets.GITHUB_TOKEN }}
          publish_dir: ./.vitepress/dist
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
      - run: cargo install vitepress
      - run: vitepress build
      - name: Deploy to Cloudflare Pages
        uses: cloudflare/pages-action@v1
        with:
          apiToken: ${{ secrets.CLOUDFLARE_API_TOKEN }}
          accountId: ${{ secrets.CLOUDFLARE_ACCOUNT_ID }}
          projectName: my-docs
          directory: ./.vitepress/dist
```

## 高级功能

### Vue 组件

在 Markdown 中使用 Vue 组件：

```vue
<!-- .vitepress/theme/components/HelloWorld.vue -->
<template>
  <div class="hello">
    <h1>{{ msg }}</h1>
  </div>
</template>

<script setup>
defineProps({
  msg: {
    type: String,
    default: 'Hello World!'
  }
});
</script>

<style scoped>
.hello {
  text-align: center;
  padding: 2rem;
}
</style>
```

在 Markdown 中使用：

```markdown
# Using Vue Components

<HelloWorld msg="Hello from VitePress!" />
```

### 代码示例

VitePress 提供了强大的代码示例功能：

```markdown
```js
console.log('Hello, VitePress!');
```

```js{1,3-5}
const a = 1;
const b = 2;
const c = a + b;
console.log(c);
```

```js
// @filepath: example.js
console.log('This is a file example');
```
```

### 多语言支持

VitePress 支持多语言文档：

```
my-docs/
├── en/
│   ├── index.md
│   └── guide/
├── zh/
│   ├── index.md
│   └── guide/
└── .vitepress/
    └── config.ts
```

配置：

```typescript
// .vitepress/config.ts
import { defineConfig } from 'vitepress';

export default defineConfig({
  locales: {
    root: {
      label: 'English',
      lang: 'en',
    },
    zh: {
      label: '中文',
      lang: 'zh',
      link: '/zh/',
    },
  },
});
```

## 性能优化

### 代码分割

VitePress 自动进行代码分割，只加载页面所需的 JavaScript。

### 图像优化

VitePress 提供内置的图像优化功能：

```markdown
![Description](/image.jpg)
```

### 预加载

VitePress 支持预加载关键资源：

```typescript
// .vitepress/config.ts
import { defineConfig } from 'vitepress';

export default defineConfig({
  head: [
    ['link', { rel: 'preload', href: '/fonts/roboto.woff2', as: 'font', type: 'font/woff2', crossorigin: '' }]
  ],
});
```

## 开发工具

### VS Code 扩展

推荐使用以下 VS Code 扩展：

- **VitePress**：提供语法高亮和智能提示
- **Prettier**：代码格式化
- **ESLint**：代码质量检查

### 调试

在 `.vitepress/config.ts` 中启用调试模式：

```typescript
// .vitepress/config.ts
import { defineConfig } from 'vitepress';

export default defineConfig({
  debug: true,
});
```

## 常见问题

### 1. VitePress 与其他文档站点生成器的区别是什么？

VitePress 的主要区别在于其基于 Vite 和 Vue，提供了更快的开发体验和更现代的设计。

### 2. 如何处理动态内容？

VitePress 主要针对静态内容优化，但您可以使用客户端 JavaScript 或集成服务器端渲染来处理动态内容。

### 3. 如何优化构建速度？

- 使用增量构建
- 优化图像和静态资源
- 减少插件数量
- 使用适当的缓存策略

### 4. 如何添加自定义字体？

```typescript
// .vitepress/config.ts
import { defineConfig } from 'vitepress';

export default defineConfig({
  head: [
    ['link', { rel: 'preconnect', href: 'https://fonts.googleapis.com' }],
    ['link', { rel: 'preconnect', href: 'https://fonts.gstatic.com', crossorigin: '' }],
    ['link', { href: 'https://fonts.googleapis.com/css2?family=Roboto:wght@400;700&display=swap', rel: 'stylesheet' }]
  ],
});
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

- 确保插件版本与 VitePress 版本兼容
- 检查插件配置是否正确
- 查看插件文档以了解正确的用法

## 示例项目

### 文档站点示例

```
docs-site/
├── .vitepress/
│   ├── config.ts
│   └── theme/
├── guide/
│   ├── getting-started.md
│   └── advanced.md
├── api/
│   └── index.md
├── index.md
└── README.md
```

## 贡献指南

我们欢迎对 VitePress 的贡献！🤝

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

VitePress 受到原始 VitePress 项目的启发，并受益于 Rust 生态系统，包括 nargo 和 oak 库。

## 许可证

VitePress 在 AGPL-3.0 许可证下发布。有关更多信息，请参阅 [LICENSE](../../../license.md)。

---

使用 VitePress 构建愉快！🚀