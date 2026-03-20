# Gatsby 编译器 100% 兼容性 - Product Requirement Document

## Overview
- **Summary**: 推进 Gatsby 编译器项目，实现与官方 Gatsby 100% 功能兼容性，包括配置解析、Markdown/MDX 文档处理、GraphQL 数据层、插件系统、主题系统和完整的站点生成能力。
- **Purpose**: 提供一个纯 Rust 实现的、高性能的 Gatsby 替代品，能够无缝处理现有的 Gatsby 项目，无需任何修改。
- **Target Users**: 使用 Gatsby 构建现代化网站的开发者和团队。

## Goals
- 100% 兼容 Gatsby 的配置系统（支持 gatsby-config.js）
- 完整实现 Gatsby 的 GraphQL 数据层和查询功能
- 支持 Markdown 和 MDX 文档解析和渲染
- 完整的插件系统架构，支持官方 Gatsby 插件
- 支持主题系统和默认主题
- 提供完整的 CLI 命令（new, develop, build, check）
- 所有代码遵循 Rust 最佳实践，有完整文档注释

## Non-Goals (Out of Scope)
- 不实现 Gatsby 的 React 组件运行时支持（仅静态生成）
- 不实现自定义 JavaScript 主题的完整运行时
- 不支持 Gatsby Cloud 功能

## Background & Context
- 项目已有基本框架结构和类型定义
- 参考了 Hugo 和 VuePress 等其他编译器的实现模式
- 项目使用 Rust，依赖工作区 crate 如 oak-markdown、nargo 等
- 需要与现有 rusty-ssg 项目架构保持一致

## Functional Requirements
- **FR-1**: 完整的配置解析系统，支持 Gatsby 所有配置项
- **FR-2**: GraphQL 数据层实现，包括 schema 定义和查询执行
- **FR-3**: Markdown 和 MDX 文档解析和渲染，支持所有 Gatsby Markdown 特性
- **FR-4**: 插件系统架构，支持插件注册和生命周期钩子
- **FR-5**: 默认主题完整实现
- **FR-6**: 完整的 CLI 工具链（new, develop, build, check）
- **FR-7**: 静态站点生成器，支持多语言和复杂路由

## Non-Functional Requirements
- **NFR-1**: 所有测试 100% 通过
- **NFR-2**: 编译性能保持 3-5x 于官方 Gatsby
- **NFR-3**: 代码遵循 Rust 最佳实践，有完整文档注释
- **NFR-4**: 内存使用高效，比官方 Gatsby 减少 60-70%

## Constraints
- **Technical**: 必须使用 Rust，使用现有工作区依赖
- **Business**: 保持与现有 rusty-ssg 项目架构一致
- **Dependencies**: oak-markdown, nargo-types, nargo-document 等工作区 crate

## Assumptions
- 工作区依赖 crate 功能完整且稳定
- 现有测试框架可以覆盖新增功能
- Gatsby 官方文档是功能规格的权威来源

## Acceptance Criteria

### AC-1: 配置系统完整兼容
- **Given**: 一个标准的 Gatsby 配置文件 gatsby-config.js
- **When**: 使用 Gatsby 编译器加载该配置
- **Then**: 所有配置项被正确解析和验证
- **Verification**: `programmatic`
- **Notes**: 支持 siteMetadata、plugins、pathPrefix 等所有配置

### AC-2: GraphQL 数据层实现
- **Given**: 一个包含 GraphQL 查询的 Gatsby 项目
- **When**: 编译器执行 GraphQL 查询
- **Then**: 查询被正确解析并返回预期结果
- **Verification**: `programmatic`

### AC-3: Markdown/MDX 渲染完整
- **Given**: 包含所有 Gatsby Markdown/MDX 特性的文档
- **When**: 编译器渲染该文档
- **Then**: 渲染结果与官方 Gatsby 一致
- **Verification**: `programmatic`

### AC-4: 插件系统可用
- **Given**: 一个配置了 Gatsby 插件的项目
- **When**: 编译器加载和执行插件
- **Then**: 插件按预期工作，正确处理数据
- **Verification**: `programmatic`

### AC-5: CLI 命令完整可用
- **Given**: 一个 Gatsby 项目
- **When**: 执行 gatsby new/develop/build/check 命令
- **Then**: 命令按预期工作，生成正确结果
- **Verification**: `programmatic`

### AC-6: 静态站点生成
- **Given**: 一个完整的 Gatsby 项目
- **When**: 执行 gatsby build
- **Then**: 生成完整的静态站点，所有页面正确输出
- **Verification**: `programmatic`

## Open Questions
- [ ] 是否需要支持 Gatsby v5 的所有最新功能？
- [ ] 插件系统是否需要支持完整的 JavaScript 插件运行时？
