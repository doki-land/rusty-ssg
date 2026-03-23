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

这将启动一个带有热重载的本地开发服务器，因此您可以实时查看更改。

### 生产构建

```bash
hugo build
```

这将在 `public` 目录中生成优化的静态文件，ready for deployment.

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
├── config.toml        # 配置文件
└── README.md          # 项目说明
```

## 配置

### 基本配置

Here's an example `config.toml` file:

```toml
[site]
title = "My Hugo Site"
description = "A site built with Rusty Hugo"
author = "Your Name"
base_url = "https://example.com"

[build]
output_dir = "public"

[params]
theme = "default"
disqus_shortname = "your-disqus-shortname"

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
```

### 配置格式

Hugo 支持多种配置格式：

- **TOML**：`config.toml`（推荐）
- **YAML**：`config.yaml` 或 `config.yml`
- **JSON**：`config.json`

### 配置选项

| 选项 | 描述 | 默认值 |
|------|------|--------|
| `site.title` | 站点标题 | `""` |
| `site.description` | 站点描述 | `""` |
| `site.author` | 站点作者 | `""` |
| `site.base_url` | 站点基础 URL | `""` |
| `build.output_dir` | 输出目录 | `"public"` |
| `params.theme` | 主题名称 | `"default"` |
| `params.disqus_shortname` | Disqus 短名称 | `""` |
| `menu` | 导航菜单配置 | `{}` |

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

## 短代码系统

Hugo 短代码是可重用的内容组件，可以在 Markdown 文件中使用。

### 内置短代码

- **figure**：插入图片
- **highlight**：代码高亮
- **gist**：嵌入 GitHub Gist
- **youtube**：嵌入 YouTube 视频
- **vimeo**：嵌入 Vimeo 视频
- **twitter**：嵌入 Twitter 推文

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
<div class="alert alert-{{ .Get "type" |