# 统一编译器项目结构规范 - 产品需求文档

## Overview
- **Summary**: 对rusty-ssg项目中所有模板引擎编译器（astro、eleventy、gatsby、hexo、hugo、jekyll、mkdocs、vitepress、vuepress）进行项目结构统一化改造，确保所有编译器遵循相同的目录结构和代码组织规范。
- **Purpose**: 解决当前各编译器项目结构混乱、各自为战的问题，提高代码可维护性和可扩展性，便于团队协作和代码复用。
- **Target Users**: 项目开发团队、维护者和贡献者。

## Goals
- 统一所有编译器的项目结构，遵循readme.md中定义的规范
- 确保bin目录为RIIR版本的唯一入口，移除src/bin中的重复实现
- 统一使用nargo-template中的抽象层，复用nargo提供的组件
- 所有解析器统一使用oaks库，避免引入重型非自研库
- 保持代码风格和文档注释的一致性

## Non-Goals (Out of Scope)
- 不修改编译器的核心功能逻辑
- 不重新实现现有的模板引擎特性
- 不引入新的依赖库
- 不改变编译器的外部接口和命令行参数

## Background & Context
- 当前各编译器项目结构混乱，目录组织不一致
- 存在bin目录和src/bin目录重复实现的问题
- 缺乏统一的抽象层，代码复用率低
- 部分编译器可能使用了不符合规范的第三方库

## Functional Requirements
- **FR-1**: 统一所有编译器的目录结构，遵循readme.md中定义的规范
- **FR-2**: 确保每个编译器只有bin目录下的主入口文件，移除src/bin中的重复实现
- **FR-3**: 所有编译器使用nargo-template中的抽象层
- **FR-4**: 所有解析器统一使用oaks库
- **FR-5**: 移除不符合规范的第三方依赖库

## Non-Functional Requirements
- **NFR-1**: 保持代码风格一致性，遵循Rust官方代码风格
- **NFR-2**: 所有public结构体、枚举、方法、字段都需要文档注释
- **NFR-3**: 使用pnpm workspace管理依赖
- **NFR-4**: 保持版本号的一致性

## Constraints
- **Technical**: 使用Rust语言，遵循readme.md中的架构规范
- **Business**: 不影响现有编译器的功能和性能
- **Dependencies**: 依赖nargo和oaks库，禁止使用其他重型非自研库

## Assumptions
- nargo-template提供了必要的抽象接口
- oaks库能够满足所有解析器的需求
- 统一结构不会破坏现有编译器的功能

## Acceptance Criteria

### AC-1: 统一目录结构
- **Given**: 所有编译器项目
- **When**: 按照readme.md规范进行结构调整
- **Then**: 所有编译器具有相同的目录结构，包括compiler、config、watcher、data、errors、plugin、tools、types等模块
- **Verification**: `human-judgment`

### AC-2: 移除重复入口
- **Given**: 存在src/bin重复实现的编译器
- **When**: 检查并移除src/bin目录
- **Then**: 每个编译器只有bin目录下的主入口文件
- **Verification**: `programmatic`

### AC-3: 使用统一抽象层
- **Given**: 所有编译器
- **When**: 引入nargo-template依赖
- **Then**: 所有编译器使用nargo-template提供的抽象接口
- **Verification**: `programmatic`

### AC-4: 统一使用oaks解析器
- **Given**: 所有编译器的解析器
- **When**: 检查解析器实现
- **Then**: 所有解析器使用oaks库，没有使用其他解析库
- **Verification**: `programmatic`

### AC-5: 移除不符合规范的依赖
- **Given**: 所有编译器的Cargo.toml文件
- **When**: 检查依赖项
- **Then**: 移除pulldown-cmark、handlebars、liquid、askama、yaml-rust、serde_yaml、toml等重型非自研库
- **Verification**: `programmatic`

## Open Questions
- [ ] nargo-template是否提供了所有必要的抽象接口？
- [ ] oaks库是否支持所有编译器的解析需求？
- [ ] 统一结构后是否需要更新测试用例？