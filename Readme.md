# VuTeX

高性能、可扩展的现代化文档生成器。

## 特性

- ⚡ **高性能** - 基于 Rust 编译时架构，提供极致的文档处理性能
- 🔌 **可扩展** - 灵活的插件系统，轻松集成 KaTeX、Mermaid 等高级功能
- 🌐 **多语言支持** - 统一 URL 路径下实现多语言切换，无需维护不同路由
- 🎨 **主题定制** - 完整的主题系统，支持自定义组件和样式
- 💚 **高性能原生实现** - 基于 Rust 全链路编译器，提供极致性能
- 🏗️ **编译时与运行时分离** - 清晰的架构设计，编译时负责解析转换，运行时专注渲染交互

## 快速开始

### 环境要求

- Node.js 18+
- pnpm (推荐)

### 创建新项目

```bash
# 使用 pnpm 创建新项目
pnpm create vutex@latest my-docs

# 进入项目目录
cd my-docs

# 安装依赖
pnpm install

# 启动开发服务器
pnpm dev
```

访问 http://localhost:5173 查看文档站点！

## 项目结构

```
my-vutex-docs/
├── docs/
│   ├── .vutex/
│   │   ├── components/       # 自定义 Vue 组件
│   │   ├── theme/            # 自定义主题
│   │   ├── public/           # 静态资源（图片、字体等）
│   │   └── vutex.config.ts   # 配置文件
│   ├── guide/                # 指南文档
│   ├── api/                  # API 文档
│   └── index.md              # 首页
├── package.json
├── pnpm-lock.yaml
└── tsconfig.json
```

**重要**：静态资源必须放在 `docs/.vutex/public/` 目录下！

## 配置

编辑 `docs/.vutex/vutex.config.ts`：

```typescript
import { defineConfig } from '@vutex/core'

export default defineConfig({
  title: '我的文档',
  description: '这是一个使用 VuTeX 构建的文档站点',
  lang: 'zh-CN',
  themeConfig: {
    nav: [
      { text: '首页', link: '/' },
      { text: '指南', link: '/guide/' }
    ]
  }
})
```

更多配置选项请参考 [配置参考](./documentation/zh-hans/reference/config.md)。

## 插件

VuTeX 支持丰富的插件系统：

```bash
# 安装插件
pnpm add -D @vutex/plugin-katex @vutex/plugin-mermaid
```

在配置中使用：

```typescript
import { katexPlugin } from '@vutex/plugin-katex'
import { mermaidPlugin } from '@vutex/plugin-mermaid'

export default defineConfig({
  plugins: [
    katexPlugin(),
    mermaidPlugin()
  ]
})
```

## 可用脚本

```bash
# 启动开发服务器
pnpm dev

# 构建生产版本
pnpm build

# 预览构建结果
pnpm preview
```

## AI 助手技能

本项目包含 Claude Skills 技能包，帮助你更高效地使用 VuTeX！

### 安装技能

#### 方法一：在 Trae IDE 中使用（推荐）

如果你正在使用 Trae IDE，技能已经准备就绪！

1. 技能文件夹位置：
   ```
   .skills/vutex-documentation/
   ```

2. Trae IDE 会自动识别 `.skills` 文件夹中的技能

3. 在对话中直接询问 VuTeX 相关问题即可

#### 方法二：在 Claude Desktop 中安装

1. 打开 Claude Desktop
2. 点击设置或技能管理界面
3. 选择"添加技能"或"导入技能"
4. 选择本项目的技能文件夹：
   ```
   .skills/vutex-documentation/
   ```
5. 确认导入

### 技能文件说明

- **SKILL.md** - 技能描述和元数据
- **instructions.md** - 完整的使用指南
- **examples.md** - 实用示例集合
- **INSTALL.md** - 详细的安装说明

### 使用技能

安装成功后，你可以这样询问：

```
"帮我创建一个 VuTeX 项目"
```

```
"如何配置 VuTeX 的侧边栏？"
```

```
"给我一个 VuTeX 首页的示例"
```

## 文档

查看完整文档：[./documentation/zh-hans/](./documentation/zh-hans/)

- [快速开始](./documentation/zh-hans/guide/quick-start.md)
- [配置参考](./documentation/zh-hans/reference/config.md)
- [插件指南](./documentation/zh-hans/plugins/readme.md)
- [主题定制](./documentation/zh-hans/themes/readme.md)

## 开发

### 开发指南

本项目使用 pnpm 作为包管理器：

```bash
# 安装依赖
pnpm install

# 格式化代码
pnpm fmt
```

## 许可证

MIT
