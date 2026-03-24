# Gatsby - Rust Reimplementation

## 概述

Gatsby 是一个基于 React 的静态站点生成器，现在使用 Rust 重新实现，以获得更好的性能和可靠性。它结合了 React 的组件模型和 GraphQL 数据层，使您能够构建现代、高性能的静态网站。

### 🎯 核心特性

- 🚀 **快速构建**：在几秒钟内编译您的站点，而不是几分钟
- 🎨 **React 组件**：使用 React 组件构建 UI
- 📦 **GraphQL 数据层**：统一的数据获取和管理
- 🔧 **可扩展**：通过插件和主题进行自定义
- 🛠 **开发者友好**：优秀的工具和开发体验
- 📝 **Markdown 支持**：轻松使用 Markdown 编写内容
- 🌍 **跨平台**：适用于 Windows、macOS 和 Linux
- 📱 **100% 兼容**：使用静态功能时完全兼容

## 安装

### 从 Crates.io 安装

```bash
cargo install gatsby
```

### 从源代码安装

```bash
# 克隆仓库
git clone https://github.com/doki-land/rusty-ssg.git

# 构建和安装
cd rusty-ssg/compilers/gatsby
cargo install --path .
```

## 基本用法

### 创建新站点

```bash
gatsby new my-site
cd my-site
```

### 本地开发

```bash
gatsby develop
```

这将启动一个带有热重载的本地开发服务器，因此您可以实时查看更改。

### 生产构建

```bash
gatsby build
```

这将在 `public` 目录中生成优化的静态文件，ready for deployment.

## 项目结构

一个典型的 Gatsby 项目结构如下：

```
my-site/
├── src/                # 源代码
│   ├── components/     # React 组件
│   ├── pages/          # 页面组件
│   ├── templates/      # 模板组件
│   ├── styles/         # 样式文件
│   └── utils/          # 工具函数
├── static/             # 静态资源
├── gatsby-config.js    # 配置文件
├── gatsby-node.js      # 节点 API
├── gatsby-browser.js   # 浏览器 API
├── gatsby-ssr.js       # SSR API
└── package.json        # 项目依赖
```

## 配置

### 基本配置

Here's an example `gatsby-config.js` file:

```javascript
// gatsby-config.js
module.exports = {
  siteMetadata: {
    title: "My Gatsby Site",
    description: "A site built with Rusty Gatsby",
    author: "Your Name",
    siteUrl: "https://example.com",
    social: [
      { name: "Twitter", url: "https://twitter.com/yourusername" },
      { name: "GitHub", url: "https://github.com/yourusername" }
    ]
  },
  plugins: [
    "gatsby-plugin-react-helmet",
    "gatsby-plugin-sitemap",
    {
      resolve: "gatsby-source-filesystem",
      options: {
        name: "content",
        path: `${__dirname}/content`,
      },
    },
    "gatsby-transformer-remark",
    "gatsby-plugin-sharp",
    "gatsby-transformer-sharp",
  ],
};
```

### 配置选项

| 选项 | 描述 | 默认值 |
|------|------|--------|
| `siteMetadata` | 站点元数据 | `{}` |
| `pathPrefix` | 路径前缀 | `""` |
| `plugins` | 插件配置 | `[]` |
| `polyfill` | 是否启用 polyfill | `true` |
| `mapping` | 映射配置 | `{}` |
| `developMiddleware` | 开发中间件 | `[]` |
| `proxy` | 代理配置 | `[]` |
| `develop` | 开发服务器配置 | `{}` |
| `flags` | 构建标志配置 | `{}` |
| `graphqlTypegen` | GraphQL 类型生成配置 | `{}` |
| `adapter` | 适配器配置 | `{}` |
| `jsxRuntime` | JSX 运行时 | `"automatic"` |
| `trailingSlash` | 尾随斜杠配置 | `"always"` |
| `public` | 输出目录 | `"public"` |
| `cache` | 缓存目录 | `".cache"` |

## React 组件

### 页面组件

```jsx
// src/pages/index.js
import React from "react";
import { Link } from "gatsby";
import Layout from "../components/Layout";

export default function Home() {
  return (
    <Layout>
      <h1>Welcome to My Gatsby Site</h1>
      <p>This is the home page of my Gatsby site.</p>
      <p>
        Gatsby is a fast, modern static site generator that uses React and GraphQL.
      </p>
      <Link to="/about">About</Link>
    </Layout>
  );
}
```

### 布局组件

```jsx
// src/components/Layout.js
import React from "react";
import Header from "./Header";
import Footer from "./Footer";
import "../styles/main.css";

export default function Layout({ children }) {
  return (
    <div>
      <Header />
      <main>{children}</main>
      <Footer />
    </div>
  );
}
```

### 模板组件

```jsx
// src/templates/blog-post.js
import React from "react";
import { graphql, Link } from "gatsby";
import Layout from "../components/Layout";

export default function BlogPost({ data }) {
  const post = data.markdownRemark;
  return (
    <Layout>
      <h1>{post.frontmatter.title}</h1>
      <p>{post.frontmatter.date}</p>
      <div dangerouslySetInnerHTML={{ __html: post.html }} />
      <Link to="/blog">Back to Blog</Link>
    </Layout>
  );
}

export const query = graphql`
  query BlogPostBySlug($slug: String!) {
    markdownRemark(fields: { slug: { eq: $slug } }) {
      html
      frontmatter {
        title
        date(formatString: "YYYY-MM-DD")
      }
    }
  }
`;
```

## GraphQL 数据层

### 数据查询

Gatsby 使用 GraphQL 来查询数据：

```jsx
// src/pages/blog.js
import React from "react";
import { graphql, Link } from "gatsby";
import Layout from "../components/Layout";

export default function Blog({ data }) {
  const posts = data.allMarkdownRemark.edges;
  return (
    <Layout>
      <h1>Blog</h1>
      <ul>
        {posts.map(({ node }) => (
          <li key={node.id}>
            <Link to={node.fields.slug}>{node.frontmatter.title}</Link>
            <p>{node.frontmatter.date}</p>
            <p>{node.excerpt}</p>
          </li>
        ))}
      </ul>
    </Layout>
  );
}

export const query = graphql`
  query BlogQuery {
    allMarkdownRemark(sort: { fields: [frontmatter___date], order: DESC }) {
      edges {
        node {
          id
          excerpt
          fields {
            slug
          }
          frontmatter {
            title
            date(formatString: "YYYY-MM-DD")
          }
        }
      }
    }
  }
`;
```

### 数据转换

在 `gatsby-node.js` 中配置数据转换：

```javascript
// gatsby-node.js
const path = require("path");
const { createFilePath } = require("gatsby-source-filesystem");

exports.onCreateNode = ({ node, getNode, actions }) => {
  const { createNodeField } = actions;
  if (node.internal.type === "MarkdownRemark") {
    const slug = createFilePath({ node, getNode, basePath: "content" });
    createNodeField({
      node,
      name: "slug",
      value: `/blog${slug}`,
    });
  }
};

exports.createPages = async ({ graphql, actions }) => {
  const { createPage } = actions;
  const result = await graphql(`
    query {
      allMarkdownRemark {
        edges {
          node {
            fields {
              slug
            }
          }
        }
      }
    }
  `);
  result.data.allMarkdownRemark.edges.forEach(({ node }) => {
    createPage({
      path: node.fields.slug,
      component: path.resolve("./src/templates/blog-post.js"),
      context: {
        slug: node.fields.slug,
      },
    });
  });
};
```

## 插件系统

Gatsby 支持通过插件扩展功能。

### 内置插件

- 📊 **gatsby-plugin-katex**：渲染数学公式
- 🎨 **gatsby-remark-prismjs**：代码块语法高亮
- 📈 **gatsby-remark-mermaid**：渲染图表和流程图
- 🔍 **gatsby-plugin-google-analytics**：添加 Google Analytics 跟踪
- 🗺️ **gatsby-plugin-sitemap**：生成 sitemap.xml
- 📱 **gatsby-plugin-pwa**：添加 PWA 支持

### 使用插件

在 `gatsby-config.js` 中配置插件：

```javascript
// gatsby-config.js
module.exports = {
  plugins: [
    "gatsby-plugin-react-helmet",
    "gatsby-plugin-sitemap",
    {
      resolve: "gatsby-source-filesystem",
      options: {
        name: "content",
        path: `${__dirname}/content`,
      },
    },
    {
      resolve: "gatsby-transformer-remark",
      options: {
        plugins: [
          "gatsby-remark-prismjs",
          "gatsby-remark-katex",
          "gatsby-remark-mermaid",
        ],
      },
    },
    "gatsby-plugin-sharp",
    "gatsby-transformer-sharp",
  ],
};
```

## 主题系统

Gatsby 支持主题，您可以选择内置主题或创建自己的主题。

### 内置主题

- 🎨 **gatsby-theme-default**：默认主题
- 🌙 **gatsby-theme-dark**：深色模式主题
- 📦 **gatsby-theme-minimal**：极简主义主题
- 📝 **gatsby-theme-blog**：博客专注的主题
- 📚 **gatsby-theme-docs**：文档专注的主题

### 使用主题

在 `gatsby-config.js` 中配置主题：

```javascript
// gatsby-config.js
module.exports = {
  plugins: [
    {
      resolve: "gatsby-theme-blog",
      options: {
        basePath: "/blog",
      },
    },
  ],
};
```

## 部署

Gatsby 生成可在任何地方部署的静态文件。

### Netlify

```toml
# netlify.toml
[build]
  command = "cargo install gatsby && gatsby build"
  publish = "public"

[build.environment]
  RUST_VERSION = "stable"
```

### Vercel

```json
// vercel.json
{
  "buildCommand": "cargo install gatsby && gatsby build",
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
      
      - name: Install Gatsby
        run: cargo install gatsby
      
      - name: Build site
        run: gatsby build
      
      - name: Deploy to GitHub Pages
        uses: peaceiris/actions-gh-pages@v3
        with:
          github_token: ${{ secrets.GITHUB_TOKEN }}
          publish_dir: ./public
```

## 高级功能

### 图像优化

Gatsby 提供内置的图像优化功能：

```jsx
// src/components/Image.js
import React from "react";
import { useStaticQuery, graphql } from "gatsby";
import Img from "gatsby-image";

export default function Image() {
  const data = useStaticQuery(graphql`
    query {
      placeholderImage: file(relativePath: { eq: "image.jpg" }) {
        childImageSharp {
          fluid(maxWidth: 800) {
            ...GatsbyImageSharpFluid
          }
        }
      }
    }
  `);
  
  return <Img fluid={data.placeholderImage.childImageSharp.fluid} />;
}
```

### 页面创建

在 `gatsby-node.js` 中动态创建页面：

```javascript
// gatsby-node.js
exports.createPages = async ({ graphql, actions }) => {
  const { createPage } = actions;
  const result = await graphql(`
    query {
      allMarkdownRemark {
        edges {
          node {
            fields {
              slug
            }
            frontmatter {
              tags
            }
          }
        }
      }
    }
  `);
  
  // 创建博客文章页面
  result.data.allMarkdownRemark.edges.forEach(({ node }) => {
    createPage({
      path: node.fields.slug,
      component: path.resolve("./src/templates/blog-post.js"),
      context: {
        slug: node.fields.slug,
      },
    });
  });
  
  // 创建标签页面
  const tags = new Set();
  result.data.allMarkdownRemark.edges.forEach(({ node }) => {
    if (node.frontmatter.tags) {
      node.frontmatter.tags.forEach((tag) => tags.add(tag));
    }
  });
  
  tags.forEach((tag) => {
    createPage({
      path: `/tags/${tag}/`,
      component: path.resolve("./src/templates/tag-page.js"),
      context: {
        tag,
      },
    });
  });
};
```

### 浏览器 API

在 `gatsby-browser.js` 中使用浏览器 API：

```javascript
// gatsby-browser.js
exports.onClientEntry = () => {
  // 客户端入口点
  console.log("Gatsby browser entry");
};

exports.onRouteUpdate = ({ location }) => {
  // 路由更新时
  console.log("Route updated to", location.pathname);
};
```

### SSR API

在 `gatsby-ssr.js` 中使用 SSR API：

```javascript
// gatsby-ssr.js
exports.onRenderBody = ({ setHeadComponents }) => {
  // 设置头部组件
  setHeadComponents([
    <link
      key="google-fonts"
      href="https://fonts.googleapis.com/css2?family=Roboto:wght@400;700&display=swap"
      rel="stylesheet"
    />,
  ]);
};
```

## 性能优化

### 代码分割

Gatsby 自动进行代码分割，只加载页面所需的 JavaScript。

### 预加载

Gatsby 自动预加载关键资源，提高页面加载速度。

### 缓存

Gatsby 使用缓存来提高构建速度：
- **内容缓存**：缓存解析后的内容
- **模板缓存**：缓存编译后的模板
- **数据缓存**：缓存处理后的数据

### 图像优化

Gatsby 提供内置的图像优化功能：
- 自动生成多种尺寸的图像
- 支持 WebP 格式
- 延迟加载

## 开发工具

### VS Code 扩展

推荐使用以下 VS Code 扩展：

- **Gatsby**：提供语法高亮和智能提示
- **ESLint**：代码质量检查
- **Prettier**：代码格式化
- **GraphQL**：GraphQL 语法支持

### 调试

使用 `--verbose` 标志启用详细输出：

```bash
gatsby develop --verbose
```

## 常见问题

### 1. Gatsby 与其他静态站点生成器的区别是什么？

Gatsby 的主要区别在于其基于 React 和 GraphQL 的架构，使其成为构建现代、高性能静态网站的理想选择。

### 2. 如何处理动态内容？

Gatsby 主要针对静态内容优化，但您可以使用客户端 JavaScript 或外部服务来处理动态内容。

### 3. 如何优化构建速度？

- 使用增量构建
- 优化图像和静态资源
- 减少插件数量
- 使用适当的缓存策略

## 示例项目

### 博客示例

```
blog-example/
├── src/
│   ├── components/
│   │   ├── Layout.js
│   │   ├── Header.js
│   │   └── Footer.js
│   ├── pages/
│   │   ├── index.js
│   │   ├── about.js
│   │   └── blog.js
│   ├── templates/
│   │   └── blog-post.js
│   ├── styles/
│   │   └── main.css
│   └── utils/
│       └── helpers.js
├── content/
│   └── posts/
│       ├── first-post.md
│       └── second-post.md
├── static/
│   └── images/
├── gatsby-config.js
├── gatsby-node.js
├── gatsby-browser.js
├── gatsby-ssr.js
└── package.json
```

### 文档站点示例

```
docs-site/
├── src/
│   ├── components/
│   │   ├── Layout.js
│   │   ├── Header.js
│   │   ├── Sidebar.js
│   │   └── Footer.js
│   ├── pages/
│   │   ├── index.js
│   │   └── guide.js
│   ├── templates/
│   │   └── doc-page.js
│   ├── styles/
│   │   └── docs.css
│   └── utils/
│       └── helpers.js
├── content/
│   └── docs/
│       ├── getting-started.md
│       └── advanced.md
├── static/
│   └── images/
├── gatsby-config.js
├── gatsby-node.js
├── gatsby-browser.js
├── gatsby-ssr.js
└── package.json
```

## 贡献指南

我们欢迎对 Gatsby 的贡献！🤝

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

Gatsby 受到原始 Gatsby 项目的启发，并受益于 Rust 生态系统，包括 nargo 和 oak 库。

## 许可证

Gatsby 在 AGPL-3.0 许可证下发布。有关更多信息，请参阅 [LICENSE](../../../license.md)。

---

使用 Gatsby 构建愉快！🚀