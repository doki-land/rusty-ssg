# VuePress - Rust Reimplementation

## 概述

VuePress 是一个基于 Vue 的静态站点生成器，现在使用 Rust 重新实现，以获得更好的性能和可靠性。它旨在帮助您轻松构建漂亮、现代的网站，结合了 Vue 的灵活性和静态站点生成的优势。

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
cargo install vuepress
```

### 从源代码安装

```bash
# 克隆仓库
git clone https://github.com/doki-land/rusty-ssg.git

# 构建和安装
cd rusty-ssg/compilers/vuepress
cargo install --path .
```

## 基本用法

### 创建新站点

```bash
vuepress init my-site
cd my-site
```

### 本地开发

```bash
vuepress dev
```

这将启动一个带有热重载的本地开发服务器，因此您可以实时查看更改。

### 生产构建

```bash
vuepress build
```

这将在 `.vuepress/dist` 目录中生成优化的静态文件，ready for deployment.

## 项目结构

一个典型的 VuePress 项目结构如下：

```
my-site/
├── .vuepress/         # 配置和主题
│   ├── config.js      # VuePress 配置文件
│   └── theme/         # 自定义主题
├── guide/             # 指南内容
│   ├── getting-started.md
│   └── advanced.md
├── index.md           # 首页
└── README.md          # 项目说明
```

## 配置

### 基本配置

Here's an example `.vuepress/config.js` file:

```javascript
// .vuepress/config.js
module.exports = {
  // 站点设置
  title: 'My Site',
  description: 'A site built with VuePress',
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
          title: 'Guide',
          children: [
            { title: 'Getting Started', path: '/guide/getting-started' },
            { title: 'Advanced', path: '/guide/advanced' },
          ],
        },
      ],
    },
  },
  
  // Markdown 设置
  markdown: {
    lineNumbers: true,
  },
  
  // 构建设置
  dest: '.vuepress/dist',
};
```

### 配置选项

| 选项 | 描述 | 默认值 |
|------|------|--------|
| `title` | 站点标题 | `""` |
| `description` | 站点描述 | `""` |
| `base` | 站点的基础路径 | `/` |
| `themeConfig.nav` | 导航菜单配置 | `[]` |
| `themeConfig.sidebar` | 侧边栏配置 | `{}` |
| `markdown.lineNumbers` | 是否显示行号 | `false` |
| `dest` | 输出目录 | `.vuepress/dist` |

## Markdown 支持

### 基本 Markdown

```markdown
---
title: "Getting Started with VuePress"
date: 2024-01-01
---

# Getting Started with VuePress

Welcome to VuePress! This is your first documentation page.

## What is VuePress?

VuePress is a static site generator built on top of Vue.

## Why Use VuePress?

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

VuePress 支持以下 Markdown 功能：

- **Front Matter**：在文件顶部使用 YAML 定义元数据
- **代码高亮**：使用 Prism 进行语法高亮
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

VuePress 支持主题，您可以选择内置主题或创建自己的主题。

### 内置主题

- 🎨 **default**：干净、现代的文档设计
- 🌙 **dark**：深色模式主题
- 📦 **minimal**：极简主义设计

### 自定义主题

创建自定义主题：

```javascript
// .vuepress/theme/index.js
module.exports = {
  extend: '@vuepress/theme-default',
  layouts: {
    // 自定义布局
  },
  plugins: [
    // 主题插件
  ],
};
```

## 插件系统

VuePress 支持通过插件扩展功能，使用 IPC 模式与插件通信。

### 内置插件

- 📊 **@vuepress/plugin-katex**：渲染数学公式
- 🎨 **@vuepress/plugin-prismjs**：代码块语法高亮
- 📈 **@vuepress/plugin-medium-zoom**：图片缩放
- 🔍 **@vuepress/plugin-search**：内置搜索功能
- 🗺️ **@vuepress/plugin-sitemap**：生成 sitemap.xml

### 使用插件

在 `.vuepress/config.js` 中配置插件：

```javascript
// .vuepress/config.js
module.exports = {
  plugins: [
    '@vuepress/plugin-katex',
    '@vuepress/plugin-prismjs',
    ['@vuepress/plugin-search', {
      searchMaxSuggestions: 10
    }]
  ],
};
```

## 部署

VuePress 生成可在任何地方部署的静态文件。

### Netlify

```toml
# netlify.toml
[build]
  command = "vuepress build"
  publish = ".vuepress/dist"
```

### Vercel

```json
// vercel.json
{
  "buildCommand": "vuepress build",
  "outputDirectory": ".vuepress/dist"
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
      - run: cargo install vuepress
      - run: vuepress build
      - uses: peaceiris/actions-gh-pages@v3
        with:
          github_token: ${{ secrets.GITHUB_TOKEN }}
          publish_dir: ./.vuepress/dist
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
      - run: cargo install vuepress
      - run: vuepress build
      - name: Deploy to Cloudflare Pages
        uses: cloudflare/pages-action@v1
        with:
          apiToken: ${{ secrets.CLOUDFLARE_API_TOKEN }}
          accountId: ${{ secrets.CLOUDFLARE_ACCOUNT_ID }}
          projectName: my-site
          directory: ./.vuepress/dist
```

## 高级功能

### Vue 组件

在 Markdown 中使用 Vue 组件：

```vue
<!-- .vuepress/components/HelloWorld.vue -->
<template>
  <div class="hello">
    <h1>{{ msg }}</h1>
  </div>
</template>

<script>
export default {
  props: {
    msg: {
      type: String,
      default: 'Hello World!'
    }
  }
};
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

<HelloWorld msg="Hello from VuePress!" />
```

### 代码示例

VuePress 提供了强大的代码示例功能：

```markdown
```js
console.log('Hello, VuePress!');
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

VuePress 支持多语言文档：

```
my-site/
├── en/
│   ├── index.md
│   └── guide/
├── zh/
│   ├── index.md
│   └── guide/
└── .vuepress/
    └── config.js
```

配置：

```javascript
// .vuepress/config.js
module.exports = {
  locales: {
    '/': {
      lang: 'en-US',
      title: 'My Site',
      description: 'English documentation'
    },
    '/zh/': {
      lang: 'zh-CN',
      title: '我的站点',
      description: '中文文档'
    }
  },
};
```

## 性能优化

### 代码分割

VuePress 自动进行代码分割，只加载页面所需的 JavaScript。

### 图像优化

VuePress 提供内置的图像优化功能：

```markdown
![Description](/image.jpg)
```

### 预加载

VuePress 支持预加载关键资源：

```javascript
// .vuepress/config.js
module.exports = {
  head: [
    ['link', { rel: 'preload', href: '/fonts/roboto.woff2', as: 'font', type: 'font/woff2', crossorigin: '' }]
  ],
};
```

## 开发工具

### VS Code 扩展

推荐使用以下 VS Code 扩展：

- **Vue**：提供 Vue 语法高亮和智能提示
- **Prettier**：代码格式化
- **ESLint**：代码质量检查

### 调试

在 `.vuepress/config.js` 中启用调试模式：

```javascript
// .vuepress/config.js
module.exports = {
  debug: true,
};
```

## 常见问题

### 1. VuePress 与其他静态站点生成器的区别是什么？

VuePress 的主要区别在于其基于 Vue，提供了更灵活的组件系统和更好的开发体验。

### 2. 如何处理动态内容？

VuePress 主要针对静态内容优化，但您可以使用客户端 JavaScript 或集成服务器端渲染来处理动态内容。

### 3. 如何优化构建速度？

- 使用增量构建
- 优化图像和静态资源
- 减少插件数量
- 使用适当的缓存策略

### 4. 如何添加自定义字体？

```javascript
// .vuepress/config.js
module.exports = {
  head: [
    ['link', { rel: 'preconnect', href: 'https://fonts.googleapis.com' }],
    ['link', { rel: 'preconnect', href: 'https://fonts.gstatic.com', crossorigin: '' }],
    ['link', { href: 'https://fonts.googleapis.com/css2?family=Roboto:wght@400;700&display=swap', rel: 'stylesheet' }]
  ],
};
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

- 确保插件版本与 VuePress 版本兼容
- 检查插件配置是否正确
- 查看插件文档以了解正确的用法

## 示例项目

### 文档站点示例

```
docs-site/
├── .vuepress/
│   ├── config.js
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

我们欢迎对 VuePress 的贡献！🤝

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

VuePress 受到原始 VuePress 项目的启发，并受益于 Rust 生态系统，包括 nargo 和 oak 库。

## 许可证

VuePress 在 AGPL-3.0 许可证下发布。有关更多信息，请参阅 [LICENSE](../../../license.md)。

---

使用 VuePress 构建愉快！🚀