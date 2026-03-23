# Hugo - Rust Reimplementation

## 概述

Hugo 是一个速度极快的静态站点生成器，现在使用 Rust 重新实现，以获得更好的性能和可靠性。它旨在帮助您轻松构建美丽、现代的网站，结合了速度和灵活性。

### 🎯 核心特性

- 🚀 **快速构建**：在几秒钟内编译您的站点，而不是几分钟
- 🎨 **强大的模板系统**：使用 Hugo 灵活的模板系统
- 📦 **易于部署**：生成可在任何地方工作的静态文件
- 🔧 **可扩展**：通过插件和短代码进行自定义
- 🛠 **开发者友好**：优秀的工具和开发体验
- 📝 **Markdown 支持**：轻松使用 Markdown 编写内容
- 🌍 **跨平台**：适用于 Windows、macOS 和 Linux
- 📱 **100% 兼容**：使用静态功能时完全兼容
- 🔄 **增量构建**：智能缓存系统，仅重建修改的文件
- 📱 **响应式设计**：内置对响应式布局的支持
- 🔌 **插件生态**：丰富的插件支持，扩展功能
- 📚 **强大的分类系统**：支持标签、分类和自定义分类
- 🌐 **国际化支持**：内置多语言支持

## 安装

### 从 Crates.io 安装

```bash
cargo install hugo
```

### 从源代码安装

```bash
# 克隆仓库
git clone https://github.com/doki-land/rusty-ssg.git

# 构建和安装
cd rusty-ssg/compilers/hugo
cargo install --path .
```

## 基本用法

### 创建新站点

```bash
hugo init my-site
cd my-site
```

### 创建新内容文件

```bash
hugo new content/posts/my-first-post.md
```

### 本地开发

```bash
hugo server
```

这将启动一个带有热重载的本地开发服务器，默认端口为 1313，因此您可以实时查看更改。

### 生产构建

```bash
hugo build
```

这将在 `public` 目录中生成优化的静态文件，ready for deployment.

### 构建命令选项

```bash
# 增量构建（更快）
hugo build --incremental

# 详细输出
hugo build --verbose

# 构建草稿内容
hugo build --buildDrafts

# 指定输出目录
hugo build --destination ./build
```

## 项目结构

一个典型的 Hugo 项目结构如下：

```
my-site/
├── content/           # 内容文件
│   ├── posts/         # 博客文章
│   ├── pages/         # 静态页面
│   └── _index.md      # 首页内容
├── layouts/           # 模板文件
│   ├── _default/      # 默认模板
│   ├── partials/      # 可重用模板片段
│   ├── shortcodes/    # 短代码
│   └── index.html     # 首页模板
├── static/            # 静态资源
├── themes/            # 主题
├── data/              # 数据文件
├── i18n/              # 国际化文件
├── config.toml        # 配置文件
└── README.md          # 项目说明
```

## 配置

### 基本配置

以下是一个 `config.toml` 文件的示例：

```toml
# 基础配置
baseURL = "https://example.com"
title = "My Hugo Site"
languageCode = "en-us"
defaultContentLanguage = "en"
theme = "default"

# 目录配置
publishDir = "public"
contentDir = "content"
staticDir = "static"
layoutsDir = "layouts"
dataDir = "data"
i18nDir = "i18n"

# 构建配置
buildFuture = true
buildExpired = true
buildDrafts = false
uglyURLs = false
canonifyURLs = false
relativeURLs = false

# 分类配置
disableKinds = ["taxonomy", "taxonomyTerm"]
disableRSS = false
disableTaxonomies = false
disableTerms = false
disableSitemap = false
disableRobotsTXT = false

# 分类定义
taxonomies = [
  { name = "tags", singular = "tag" },
  { name = "categories", singular = "category" }
]

# 菜单配置
[menu]
  [[menu.main]]
    name = "Home"
    url = "/"
    weight = 1
  
  [[menu.main]]
    name = "About"
    url = "/about/"
    weight = 2
  
  [[menu.main]]
    name = "Blog"
    url = "/blog/"
    weight = 3

# 参数配置
[params]
description = "A site built with Rusty Hugo"
author = "Your Name"
disqusShortname = "your-disqus-shortname"

# 服务配置
[server]
port = 1313
interface = "localhost"
disableLiveReload = false

# 标记配置
[markup]
  [markup.goldmark]
    [markup.goldmark.renderer]
      hardWraps = false
      unsafe = false
    [markup.goldmark.extensions]
      table = true
      taskList = true
      strikethrough = true
      footnote = true
      definitionList = true
      linkify = true
      typographer = true
  
  [markup.highlight]
    style = "github-dark"
    lineNos = true
    tabWidth = 4

# 输出格式配置
[outputFormats]
  [outputFormats.RSS]
    baseName = "feed"
    isPlainText = false
    isHTML = false

# 媒体类型配置
[mediaTypes]
  [mediaTypes."text/netlify"]
    suffixes = ["netlify"]
```

### 配置格式

Hugo 支持多种配置格式：

- **TOML**：`config.toml`（推荐）
- **YAML**：`config.yaml` 或 `config.yml`
- **JSON**：`config.json`

### 配置选项

| 选项 | 描述 | 默认值 |
|------|------|--------|
| `baseURL` | 站点基础 URL | `""` |
| `title` | 站点标题 | `""` |
| `languageCode` | 语言代码 | `""` |
| `defaultContentLanguage` | 默认语言 | `"en"` |
| `theme` | 主题名称 | `"default"` |
| `publishDir` | 输出目录 | `"public"` |
| `contentDir` | 内容目录 | `"content"` |
| `staticDir` | 静态文件目录 | `"static"` |
| `layoutsDir` | 布局目录 | `"layouts"` |
| `dataDir` | 数据目录 | `"data"` |
| `i18nDir` | 国际化文件目录 | `"i18n"` |
| `buildFuture` | 构建未来日期的内容 | `false` |
| `buildExpired` | 构建过期内容 | `false` |
| `buildDrafts` | 构建草稿内容 | `false` |
| `uglyURLs` | 是否启用简洁 URL | `false` |
| `canonifyURLs` | 是否规范化 URL | `false` |
| `relativeURLs` | 是否使用相对 URL | `false` |
| `disableKinds` | 禁用的内容类型 | `[]` |
| `disableRSS` | 是否禁用 RSS | `false` |
| `disableTaxonomies` | 是否禁用分类 | `false` |
| `disableTerms` | 是否禁用分类项列表 | `false` |
| `disableSitemap` | 是否禁用 sitemap | `false` |
| `disableRobotsTXT` | 是否禁用 robots.txt | `false` |
| `taxonomies` | 分类配置 | `[{name = "tags"}, {name = "categories"}]` |
| `menu` | 导航菜单配置 | `{}` |
| `params` | 参数配置 | `{}` |
| `server.port` | 开发服务器端口 | `1313` |
| `server.interface` | 开发服务器接口 | `"localhost"` |
| `server.disableLiveReload` | 是否禁用实时重载 | `false` |

## 内容管理

### 内容文件

Hugo 使用 Markdown 文件作为内容源，支持 front matter 来定义元数据。

```markdown
---
title: "My First Post"
date: 2024-01-01
draft: false
tags:
  - rust
  - hugo
categories:
  - tutorials
---

# My First Post

This is the content of my first post.

## Using Shortcodes

Hugo shortcodes make it easy to add complex content:

{{< figure src="image.jpg" alt="Description" >}}

{{< highlight rust >}}
fn main() {
    println!("Hello, Hugo!");
}
{{< /highlight >}}

## Why Use Hugo?

- It's blazingly fast
- It has a powerful template system
- It supports shortcodes for reusable content
- It's 100% compatible with static features

Happy coding! 🎉
```

### Front Matter

Front matter 可以使用 YAML、TOML 或 JSON 格式：

**YAML**（推荐）：
```yaml
---
title: "My Post"
date: 2024-01-01
---
```

**TOML**：
```toml
+++
title = "My Post"
date = 2024-01-01
+++
```

**JSON**：
```json
{
  "title": "My Post",
  "date": "2024-01-01"
}
```

### 内容组织

Hugo 使用目录结构来组织内容：

- **Single files**：单个 Markdown 文件，如 `about.md`
- **Sections**：包含 `_index.md` 的目录，如 `posts/_index.md`
- **Pages**：目录中的 Markdown 文件，如 `posts/my-post.md`
- **Bundles**：包含多个文件的内容包，如 `posts/my-post/index.md`

## 模板系统

Hugo 使用 Go 模板语言，提供强大的模板功能。

### 基本模板

```html
<!-- layouts/_default/baseof.html -->
<!DOCTYPE html>
<html>
<head>
    <title>{{ .Site.Title }}</title>
    <link rel="stylesheet" href="{{ "css/style.css" | relURL }}">
</head>
<body>
    {{ partial "header.html" . }}
    <main>
        {{ block "main" . }}{{ end }}
    </main>
    {{ partial "footer.html" . }}
</body>
</html>
```

### 列表模板

```html
<!-- layouts/_default/list.html -->
{{ define "main" }}
    <h1>{{ .Title }}</h1>
    {{ .Content }}
    <ul>
        {{ range .Pages }}
            <li>
                <a href="{{ .RelPermalink }}">{{ .Title }}</a>
                <p>{{ .Summary }}</p>
                <p>{{ .Date.Format "2006-01-02" }}</p>
            </li>
        {{ end }}
    </ul>
{{ end }}
```

### 单页模板

```html
<!-- layouts/_default/single.html -->
{{ define "main" }}
    <h1>{{ .Title }}</h1>
    <p>{{ .Date.Format "2006-01-02" }}</p>
    {{ .Content }}
    {{ if .Params.tags }}
        <div>
            <strong>Tags:</strong>
            {{ range .Params.tags }}
                <a href="{{ "tags/" | relURL }}{{ . | urlize }}">{{ . }}</a>
            {{ end }}
        </div>
    {{ end }}
{{ end }}
```

### 部分模板

```html
<!-- layouts/partials/header.html -->
<header>
    <h1><a href="{{ "/" | relURL }}">{{ .Site.Title }}</a></h1>
    <nav>
        {{ range .Site.Menus.main }}
            <a href="{{ .URL }}">{{ .Name }}</a>
        {{ end }}
    </nav>
</header>
```

### 模板函数

Hugo 提供了丰富的模板函数：

- **字符串函数**：`title`、`lower`、`upper`、`trim` 等
- **日期函数**：`format`、`now`、`time` 等
- **数学函数**：`add`、`sub`、`mul`、`div` 等
- **集合函数**：`len`、`first`、`last`、`sort` 等
- **URL 函数**：`relURL`、`absURL`、`urlize` 等
- **条件函数**：`eq`、`ne`、`lt`、`gt` 等

## 短代码系统

Hugo 短代码是可重用的内容组件，可以在 Markdown 文件中使用。

### 内置短代码

- **figure**：插入图片
- **highlight**：代码高亮
- **gist**：嵌入 GitHub Gist
- **youtube**：嵌入 YouTube 视频
- **vimeo**：嵌入 Vimeo 视频
- **twitter**：嵌入 Twitter 推文
- **instagram**：嵌入 Instagram 帖子
- **audio**：嵌入音频文件
- **video**：嵌入视频文件

### 使用短代码

```markdown
{{< figure src="image.jpg" alt="Description" caption="Image caption" >}}

{{< highlight rust >}}
fn main() {
    println!("Hello, Hugo!");
}
{{< /highlight >}}

{{< youtube dQw4w9WgXcQ >}}
```

### 自定义短代码

创建自定义短代码：

```html
<!-- layouts/shortcodes/alert.html -->
<div class="alert alert-{{ .Get "type" | default "info" }}">
    {{ .Get "title" | default "Alert" }}
    {{ .Inner }}
</div>
```

使用自定义短代码：

```markdown
{{< alert type="warning" title="Warning" >}}
This is a warning message.
{{< /alert >}}
```

## 主题系统

Hugo 支持主题，您可以选择内置主题或创建自己的主题。

### 内置主题

- 🎨 **default**：干净、现代的设计
- 🌙 **dark**：深色模式主题
- 📦 **minimal**：极简主义设计
- 📝 **blog**：博客专注的主题
- 📚 **docs**：文档专注的主题
- 🛍️ **ecommerce**：电子商务主题
- 📱 **portfolio**：作品集主题

### 使用主题

在 `config.toml` 中配置主题：

```toml
theme = "default"
```

### 主题结构

一个典型的 Hugo 主题结构如下：

```
theme-name/
├── layouts/           # 模板文件
│   ├── _default/      # 默认模板
│   ├── partials/      # 可重用模板片段
│   ├── shortcodes/    # 短代码
│   └── index.html     # 首页模板
├── static/            # 静态资源
│   ├── css/           # 样式文件
│   ├── js/            # JavaScript 文件
│   └── images/        # 图片
├── data/              # 主题数据
├── i18n/              # 主题国际化
└── theme.toml         # 主题配置
```

### 主题继承

Hugo 支持主题继承，允许您基于现有主题创建自定义主题：

```toml
# theme.toml
name = "My Custom Theme"
description = "A custom theme based on the default theme"
author = "Your Name"
license = "MIT"

[theme]
parent = "default"
```

## 插件系统

Hugo 支持通过插件扩展功能，使用 IPC 模式与插件通信。

### 内置插件

- 📊 **katex**：渲染数学公式
- 🎨 **prism**：代码块语法高亮
- 📈 **mermaid**：渲染图表和流程图
- 🔍 **google-analytics**：添加 Google Analytics 跟踪
- 🗺️ **sitemap**：生成 sitemap.xml
- 📱 **pwa**：添加 PWA 支持
- 🎯 **tailwind**：集成 Tailwind CSS
- 🔧 **webpack**：集成 Webpack

### 使用插件

在 `config.toml` 中配置插件：

```toml
[plugins]
enabled = ["katex", "prism", "sitemap"]

[plugin.katex]
enabled = true

[plugin.prism]
enabled = true
theme = "github-dark"

[plugin.sitemap]
enabled = true
```

## 部署

Hugo 生成可在任何地方部署的静态文件。

### Netlify

```toml
# netlify.toml
[build]
  command = "hugo build"
  publish = "public"

[build.environment]
  RUST_VERSION = "stable"

[[redirects]]
  from = "/*"
  to = "/index.html"
  status = 200
```

### Vercel

```json
// vercel.json
{
  "buildCommand": "hugo build",
  "outputDirectory": "public",
  "env": {
    "RUST_VERSION": "stable"
  },
  "rewrites": [
    {
      "source": "/(.*)",
      "destination": "/index.html"
    }
  ]
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
      
      - name: Install Hugo
        run: cargo install hugo
      
      - name: Build site
        run: hugo build
      
      - name: Deploy to GitHub Pages
        uses: peaceiris/actions-gh-pages@v3
        with:
          github_token: ${{ secrets.GITHUB_TOKEN }}
          publish_dir: ./public
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
      - run: cargo install hugo
      - run: hugo build
      - name: Deploy to Cloudflare Pages
        uses: cloudflare/pages-action@v1
        with:
          apiToken: ${{ secrets.CLOUDFLARE_API_TOKEN }}
          accountId: ${{ secrets.CLOUDFLARE_ACCOUNT_ID }}
          projectName: my-site
          directory: ./public
```

## 高级功能

### 内容分类

Hugo 支持通过 taxonomies 对内容进行分类：

- **Tags**：标签
- **Categories**：分类
- **自定义分类**：如作者、系列等

### 分页

Hugo 提供内置的分页功能：

```html
<!-- layouts/_default/list.html -->
{{ define "main" }}
    <h1>{{ .Title }}</h1>
    {{ .Content }}
    <ul>
        {{ range .Paginator.Pages }}
            <li>
                <a href="{{ .RelPermalink }}">{{ .Title }}</a>
                <p>{{ .Summary }}</p>
            </li>
        {{ end }}
    </ul>
    {{ template "pagination.html" . }}
{{ end }}

<!-- layouts/partials/pagination.html -->
{{ if or .Paginator.HasPrev .Paginator.HasNext }}
    <nav>
        {{ if .Paginator.HasPrev }}
            <a href="{{ .Paginator.Prev.URL }}">Previous</a>
        {{ end }}
        {{ range .Paginator.Pagers }}
            <a href="{{ .URL }}"{{ if eq . .Paginator }} class="active"{{ end }}>{{ .PageNumber }}</a>
        {{ end }}
        {{ if .Paginator.HasNext }}
            <a href="{{ .Paginator.Next.URL }}">Next</a>
        {{ end }}
    </nav>
{{ end }}
```

### 数据文件

Hugo 支持使用数据文件来存储结构化数据：

```yaml
# data/team.yaml
- name: "John Doe"
  position: "Developer"
  email: "john@example.com"
- name: "Jane Smith"
  position: "Designer"
  email: "jane@example.com"
```

在模板中使用数据：

```html
<!-- layouts/partials/team.html -->
<ul>
    {{ range .Site.Data.team }}
        <li>
            <h3>{{ .name }}</h3>
            <p>{{ .position }}</p>
            <a href="mailto:{{ .email }}">{{ .email }}</a>
        </li>
    {{ end }}
</ul>
```

### 国际化

Hugo 支持国际化（i18n）：

```toml
# config.toml
defaultContentLanguage = "en"

[languages]
[languages.en]
weight = 1
languageName = "English"
[languages.zh]
weight = 2
languageName = "中文"
```

创建翻译文件：

```toml
# i18n/en.toml
[greeting]
other = "Hello!"

[about]
other = "About"
```

```toml
# i18n/zh.toml
[greeting]
other = "你好！"

[about]
other = "关于"
```

在模板中使用翻译：

```html
<!-- layouts/partials/header.html -->
<header>
    <h1>{{ i18n "greeting" }}</h1>
    <nav>
        <a href="{{ "/" | relURL }}">{{ i18n "home" }}</a>
        <a href="{{ "/about/" | relURL }}">{{ i18n "about" }}</a>
    </nav>
</header>
```

### 短代码高级用法

Hugo 短代码支持参数和嵌套：

```html
<!-- layouts/shortcodes/callout.html -->
<div class="callout callout-{{ .Get "type" | default "info" }}">
    {{ with .Get "title" }}
        <h4>{{ . }}</h4>
    {{ end }}
    {{ .Inner }}
</div>
```

使用嵌套短代码：

```markdown
{{< callout type="success" title="Success!" >}}
This is a success message with a {{< highlight rust >}}code example{{< /highlight >}}.
{{< /callout >}}
```

## 性能优化

### 缓存

Hugo 使用缓存来提高构建速度：

- **内容缓存**：缓存解析后的内容
- **模板缓存**：缓存编译后的模板
- **资源缓存**：缓存处理后的资源

### 并行处理

Hugo 使用并行处理来提高构建速度，特别是对于大型站点。

### 资源优化

Hugo 提供内置的资源优化功能：

- **CSS 压缩**：压缩 CSS 文件
- **JavaScript 压缩**：压缩 JavaScript 文件
- **图像优化**：优化图像大小和质量
- **资源指纹**：为静态资源添加内容哈希

### 构建优化

- **增量构建**：使用 `hugo build --incremental`
- **并行构建**：使用 `hugo build --parallel`
- **减少内容文件大小**：优化 Markdown 文件
- **减少模板复杂度**：优化模板逻辑

## 开发工具

### VS Code 扩展

推荐使用以下 VS Code 扩展：

- **Hugo Language and Syntax Support**：提供语法高亮和智能提示
- **Prettier**：代码格式化
- **ESLint**：代码质量检查
- **Hugo Helper**：提供 Hugo 特定的功能

### 调试

使用 `--verbose` 标志启用详细输出：

```bash
hugo server --verbose
```

使用 `--print-template-debug` 查看模板调试信息：

```bash
hugo server --print-template-debug
```

## 常见问题

### 1. Hugo 与其他静态站点生成器的区别是什么？

Hugo 的主要区别在于其极快的构建速度和强大的模板系统，使其成为大型站点的理想选择。

### 2. 如何处理动态内容？

Hugo 主要针对静态内容优化，但您可以使用客户端 JavaScript 或外部服务来处理动态内容。

### 3. 如何优化构建速度？

- 使用增量构建：`hugo build --incremental`
- 优化模板和内容
- 减少插件数量
- 使用适当的缓存策略
- 增加并行度：`hugo build --parallel`

### 4. 如何添加自定义字体？

```html
<!-- layouts/partials/head.html -->
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
- 使用 `hugo check` 检查站点配置

### 开发服务器问题

- 确保端口未被占用
- 检查防火墙设置
- 清除浏览器缓存
- 重启开发服务器
- 使用 `hugo server --noHTTPCache` 禁用 HTTP 缓存

### 模板错误

- 检查模板语法是否正确
- 确保所有变量都已定义
- 使用 `hugo server --print-template-debug` 查看模板调试信息
- 检查模板路径是否正确

### 内容问题

- 检查 front matter 格式是否正确
- 确保内容文件路径符合 Hugo 的命名约定
- 检查 Markdown 语法是否正确

## 示例项目

### 博客示例

```
blog-example/
├── content/
│   ├── posts/
│   │   ├── my-first-post.md
│   │   └── another-post.md
│   └── _index.md
├── layouts/
│   ├── _default/
│   │   ├── baseof.html
│   │   ├── list.html
│   │   └── single.html
│   ├── partials/
│   │   ├── header.html
│   │   └── footer.html
│   ├── shortcodes/
│   │   └── alert.html
│   └── index.html
├── static/
│   ├── css/
│   │   └── style.css
│   └── images/
├── config.toml
└── README.md
```

### 文档站点示例

```
docs-site/
├── content/
│   ├── guide/
│   │   ├── getting-started.md
│   │   └── advanced.md
│   └── _index.md
├── layouts/
│   ├── _default/
│   │   ├── baseof.html
│   │   ├── list.html
│   │   └── single.html
│   ├── partials/
│   │   ├── header.html
│   │   ├── sidebar.html
│   │   └── footer.html
│   └── index.html
├── static/
│   ├── css/
│   │   └── docs.css
│   └── assets/
├── config.toml
└── README.md
```

## 贡献指南

我们欢迎对 Hugo 的贡献！🤝

### 报告问题

如果您发现错误或有功能请求，请 [打开一个 issue](https://github.com/doki-land/rusty-ssg/issues)。

### 提交拉取请求

1. Fork 仓库
2. 创建一个新分支
3. 进行更改
4. 运行测试：`cargo test`
5. 提交拉取请求

### 代码风格

请遵循 Rust 风格指南并使用 `cargo fmt` 格式化代码。

## 致谢

Hugo 受到原始 Hugo 项目的启发，并受益于 Rust 生态系统，包括 nargo 和 oak 库。

## 许可证

Hugo 在 AGPL-3.0 许可证下发布。有关更多信息，请参阅 [LICENSE](../../../license.md)。

---

使用 Hugo 构建愉快！🚀