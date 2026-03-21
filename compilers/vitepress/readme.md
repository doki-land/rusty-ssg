# VitePress - Rust Reimplementation

## Overview

VitePress is a fast, modern static site generator for documentation, now reimplemented in Rust for even better performance and reliability. It's designed to help you build beautiful, Vue-powered documentation sites with ease, using Markdown.

### 🎯 Key Features
- 🚀 **Fast Builds**: Compile your site in seconds, not minutes
- 🎨 **Vue-Powered**: Leverage Vue components in your documentation
- 📦 **Easy Deployment**: Generate static files that work anywhere
- 🔧 **Extensible**: Customize with plugins and themes
- 🛠 **Developer Friendly**: Great tooling and developer experience
- 📝 **Markdown Support**: Write content in Markdown with ease
- 🌍 **Cross-Platform**: Works on Windows, macOS, and Linux
- 📱 **100% Compatible**: Full compatibility when using static features

## Installation

### From Crates.io

```bash
cargo install vitepress
```

### From Source

```bash
# Clone the repository
git clone https://github.com/doki-land/rusty-ssg.git

# Build and install
cd rusty-ssg/compilers/vitepress
git checkout dev
cargo install --path .
```

## Usage

### Initialize a New Project

```bash
vitepress init my-docs
cd my-docs
```

### Develop Locally

```bash
vitepress dev
```

This will start a local development server with hot reloading, so you can see your changes in real-time.

### Build for Production

```bash
vitepress build
```

This will generate optimized static files in the `.vitepress/dist` directory, ready for deployment.

## Architecture

VitePress follows a modular architecture designed for performance and extensibility, leveraging external libraries for enhanced functionality:

```mermaid
flowchart TD
    A[CLI] --> B[Config Loader]
    B --> C[Content Scanner]
    C --> D[Parser]
    D --> E[Vue Renderer]
    E --> F[Output Generator]
    G[Plugins] --> E
    H[Themes] --> E
    I[nargo] --> E
    J[oak] --> D
    K[IPC] --> G
```

### Core Components

- **CLI**: Command-line interface for interacting with the compiler
- **Config Loader**: Reads and parses VitePress configuration files (TypeScript/JavaScript)
- **Content Scanner**: Discovers and processes content files
- **Parser**: Converts Markdown to HTML (uses oak)
- **Vue Renderer**: Renders content using Vue components
- **Output Generator**: Writes final static files
- **Plugins**: Extend functionality with custom plugins (uses IPC mode)
- **Themes**: Provide reusable templates and styles
- **nargo**: External library with analysis engines and bundlers
- **oak**: External library for parsing
- **IPC**: Inter-process communication for plugin system

## Configuration

Here's an example `vitepress.config.ts` file:

```typescript
// vitepress.config.ts
export default {
  title: 'My Documentation',
  description: 'A comprehensive guide to my project',
  themeConfig: {
    nav: [
      { text: 'Home', link: '/' },
      { text: 'Guide', link: '/guide/' },
      { text: 'API', link: '/api/' }
    ],
    sidebar: {
      '/guide/': [
        { text: 'Introduction', link: '/guide/' },
        { text: 'Getting Started', link: '/guide/getting-started/' }
      ],
      '/api/': [
        { text: 'Overview', link: '/api/' },
        { text: 'Reference', link: '/api/reference/' }
      ]
    }
  }
};
```

## Examples

### Example Documentation Page

Here's an example of a documentation page in VitePress:

```markdown
---
title: Getting Started
description: Learn how to get started with our project
---

# Getting Started

Welcome to our project! This guide will help you get started quickly.

## Installation

To install our project, run:

```bash
npm install
```

## Usage

Once installed, you can use it as follows:

```javascript
const myProject = require('my-project');
myProject.initialize();
```

## Vue Components in Markdown

VitePress allows you to use Vue components directly in your Markdown files:

```vue
<template>
  <div class="welcome">
    <h2>Welcome to VitePress!</h2>
    <p>This is a Vue component embedded in Markdown.</p>
  </div>
</template>

<style scoped>
.welcome {
  padding: 2rem;
  background-color: #f0f0f0;
  border-radius: 8px;
}
</style>
```

## Why Use VitePress?

- It's blazingly fast
- It uses Vue for powerful component-based documentation
- It has a clean, modern default theme
- It's 100% compatible with static features

Happy documenting! 🎉
```

### Example Theme Configuration

Here's an example of a more complex theme configuration:

```typescript
// vitepress.config.ts
export default {
  title: 'My Documentation',
  description: 'A comprehensive guide to my project',
  themeConfig: {
    logo: '/logo.svg',
    nav: [
      { text: 'Home', link: '/' },
      { text: 'Guide', link: '/guide/' },
      { text: 'API', link: '/api/' },
      { text: 'GitHub', link: 'https://github.com/rusty-ssg/vitepress' }
    ],
    sidebar: {
      '/guide/': [
        { 
          text: 'Getting Started',
          children: [
            { text: 'Introduction', link: '/guide/' },
            { text: 'Installation', link: '/guide/installation/' },
            { text: 'Basic Usage', link: '/guide/basic-usage/' }
          ]
        },
        { 
          text: 'Advanced Topics',
          children: [
            { text: 'Customization', link: '/guide/customization/' },
            { text: 'Plugins', link: '/guide/plugins/' }
          ]
        }
      ],
      '/api/': [
        { text: 'Overview', link: '/api/' },
        { text: 'Reference', link: '/api/reference/' }
      ]
    },
    socialLinks: [
      { icon: 'github', link: 'https://github.com/rusty-ssg/vitepress' },
      { icon: 'twitter', link: 'https://twitter.com/rusty_ssg' }
    ]
  }
};
```

## Compatibility Note

⚠️ **Important**: VitePress provides 100% compatibility only when using static features. Dynamic features may have limited support or require additional configuration.

## Plugins

VitePress supports a wide range of plugins to extend functionality (using IPC mode):

- 🔍 **search**: Built-in search functionality
- 📊 **katex**: Render mathematical formulas
- 🎨 **prism**: Syntax highlighting for code blocks
- 📈 **mermaid**: Render diagrams and flowcharts
- 🗺️ **sitemap**: Generate sitemap.xml

## Themes

VitePress comes with a beautiful default theme that's highly customizable. You can also create your own themes:

- 🎨 **default**: Modern, responsive theme (default)
- 🌙 **dark**: Dark mode theme
- 📦 **minimal**: Minimalist design
- 📝 **docs**: Documentation-focused theme

## Deployment

VitePress generates static files that can be deployed anywhere:

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

## Contribution Guidelines

We welcome contributions to VitePress! 🤝

### Reporting Issues

If you find a bug or have a feature request, please [open an issue](https://github.com/rusty-ssg/vitepress/issues).

### Pull Requests

1. Fork the repository
2. Create a new branch
3. Make your changes
4. Run tests
5. Submit a pull request

### Code Style

Please follow the Rust style guide and use `cargo fmt` to format your code.

## Acknowledgements

VitePress is inspired by the original VitePress project and benefits from the Rust ecosystem, including the nargo and oak libraries.

## License

VitePress is licensed under the terms specified in the LICENSE file. See [LICENSE](https://github.com/doki-land/rusty-ssg/blob/dev/License.md) for more information.

---

Happy documenting with VitePress! 🚀