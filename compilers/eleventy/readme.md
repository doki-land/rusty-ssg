# Rusty Eleventy Compiler

一个用Rust编写的Eleventy静态站点生成器兼容实现。

## 功能特性

- 100% 兼容原始Eleventy静态站点生成器
- 支持多种模板引擎：Liquid、Handlebars、Markdown
- 完整的命令行接口
- 配置文件解析系统
- 数据系统（全局数据、模板数据、frontmatter）
- 插件系统
- 构建系统
- 开发服务器与热重载

## 安装

```bash
# 从源代码构建
cd e:\灵之镜有限公司\rusty-ssg\compilers\eleventy
cargo build --release

# 或者使用cargo install（未来）
cargo install --path .
```

## 使用方法

### 基本命令

```bash
# 构建站点
cargo run -- build

# 启动开发服务器
cargo run -- serve

# 测试数据系统
cargo run -- test

# 显示帮助信息
cargo run -- help
```

### 配置文件

支持JSON格式的配置文件：

```json
{
  "input": ".",
  "output": "_site",
  "templateFormats": ["liquid", "hbs", "md"],
  "dataDir": "_data",
  "includes": "_includes",
  "layouts": "_layouts",
  "passthroughCopy": ["assets"],
  "watch": true,
  "port": 8080,
  "host": "localhost"
}
```

## 示例

### 基本示例

```markdown
---
title: 示例页面
date: 2024-01-01
tags: [示例, 文档]
---

# {{ title }}

发布日期：{{ date }}

## 标签

{% for tag in tags %}
- {{ tag }}
{% endfor %}

## 内容

这是一个示例页面，展示了如何使用Rust Eleventy编译器。
```

### 数据系统示例

在`_data`目录中创建`site.json`：

```json
{
  "name": "我的网站",
  "description": "这是一个使用Rust Eleventy构建的网站",
  "author": "作者姓名"
}
```

然后在模板中使用：

```liquid
# {{ site.name }}

{{ site.description }}

作者：{{ site.author }}
```

## 模板引擎

### Liquid

```liquid
{% assign name = "World" %}
Hello, {{ name }}!

{% if true %}
  这是一个条件语句
{% endif %}

{% for item in [1, 2, 3] %}
  项目：{{ item }}
{% endfor %}
```

### Handlebars

```handlebars
{{! 这是一个注释 }}
Hello, {{ name }}!

{{#if true}}
  这是一个条件语句
{{/if}}

{{#each items}}
  项目：{{ this }}
{{/each}}
```

### Markdown

```markdown
# 标题

## 子标题

- 列表项1
- 列表项2

**粗体**和*斜体*文本。

[链接](https://example.com)

![图片](image.jpg)

```

## 插件系统

Rust Eleventy支持插件系统，可通过实现`Plugin` trait来创建插件：

```rust
use eleventy::plugin::Plugin;

struct MyPlugin;

impl Plugin for MyPlugin {
    fn name(&self) -> &str {
        "my-plugin"
    }
    
    fn init(&self) {
        println!("初始化我的插件");
    }
}

// 注册插件
let mut eleventy = Eleventy::new();
eleventy.add_plugin(Box::new(MyPlugin));
```

## 开发

### 项目结构

```
eleventy/
├── Cargo.toml          # 项目依赖
├── src/
│   ├── lib.rs          # 库入口
│   ├── bin/
│   │   └── eleventy.rs # 命令行入口
│   ├── types/          # 类型定义
│   ├── config/         # 配置系统
│   ├── data/           # 数据系统
│   ├── compiler/       # 编译器
│   ├── plugin/         # 插件系统
│   ├── build/          # 构建系统
│   ├── server/         # 开发服务器
│   ├── tools/          # 工具函数
│   ├── plugin_host/    # 插件宿主
│   └── session/        # 会话管理
├── test-data.md        # 测试数据文件
├── test-config.json    # 测试配置文件
└── README.md           # 项目文档
```

### 构建和测试

```bash
# 构建项目
cargo build

# 运行测试
cargo test

# 构建发布版本
cargo build --release
```

## 贡献

欢迎贡献代码！请遵循以下步骤：

1. Fork 仓库
2. 创建特性分支
3. 提交更改
4. 推送分支
5. 创建 Pull Request

## 许可证

MIT 许可证
