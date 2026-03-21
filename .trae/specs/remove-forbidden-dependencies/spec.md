# 移除禁止使用的依赖 - 产品需求文档

## Overview
- **Summary**: 移除 `rusty-ssg/compilers` 目录中所有使用了被禁止第三方解析库的依赖，确保所有解析器都使用 `oaks/examples` 提供的解析器。
- **Purpose**: 遵守项目的解析器规范，避免使用被禁止的第三方库，确保代码库的一致性和可维护性。
- **Target Users**: 项目维护者和开发者。

## Goals
- 移除所有被禁止的第三方解析库依赖
- 确保所有编译器使用 `oaks/examples` 提供的解析器
- 保持代码功能完整性
- 确保所有测试通过

## Non-Goals (Out of Scope)
- 重构现有代码结构
- 添加新功能
- 修改其他类型的依赖

## Background & Context
根据 `compilers/readme.md` 文件中的解析器规范，所有解析器必须使用 `oaks/examples` 提供的解析器，禁止使用以下第三方解析库：
- pulldown-cmark
- handlebars
- liquid
- askama
- yaml-rust
- serde_yaml
- toml

## Functional Requirements
- **FR-1**: 移除 vuepress 编译器中的 `askama` 依赖
- **FR-2**: 移除 vitepress 编译器中的 `toml` 依赖
- **FR-3**: 移除 mkdocs 编译器中的 `serde_yaml` 依赖
- **FR-4**: 移除 hugo 编译器中的 `handlebars` 依赖
- **FR-5**: 移除 gatsby 编译器中的 `toml` 依赖

## Non-Functional Requirements
- **NFR-1**: 所有编译器的功能必须保持不变
- **NFR-2**: 所有测试必须通过
- **NFR-3**: 代码结构必须保持整洁

## Constraints
- **Technical**: 必须使用 `oaks/examples` 提供的解析器替代被移除的依赖
- **Dependencies**: 依赖于 `oaks` 项目中的解析器实现

## Assumptions
- `oaks/examples` 提供的解析器已经实现了所需的功能
- 移除这些依赖不会破坏现有功能

## Acceptance Criteria

### AC-1: 移除 vuepress 中的 askama 依赖
- **Given**: vuepress 编译器当前使用 askama 库
- **When**: 移除 askama 依赖并使用 oaks/examples 提供的解析器
- **Then**: vuepress 编译器功能正常，测试通过
- **Verification**: `programmatic`

### AC-2: 移除 vitepress 中的 toml 依赖
- **Given**: vitepress 编译器当前使用 toml 库
- **When**: 移除 toml 依赖并使用 oaks/examples 提供的解析器
- **Then**: vitepress 编译器功能正常，测试通过
- **Verification**: `programmatic`

### AC-3: 移除 mkdocs 中的 serde_yaml 依赖
- **Given**: mkdocs 编译器当前使用 serde_yaml 库
- **When**: 移除 serde_yaml 依赖并使用 oaks/examples 提供的解析器
- **Then**: mkdocs 编译器功能正常，测试通过
- **Verification**: `programmatic`

### AC-4: 移除 hugo 中的 handlebars 依赖
- **Given**: hugo 编译器当前使用 handlebars 库
- **When**: 移除 handlebars 依赖并使用 oaks/examples 提供的解析器
- **Then**: hugo 编译器功能正常，测试通过
- **Verification**: `programmatic`

### AC-5: 移除 gatsby 中的 toml 依赖
- **Given**: gatsby 编译器当前使用 toml 库
- **When**: 移除 toml 依赖并使用 oaks/examples 提供的解析器
- **Then**: gatsby 编译器功能正常，测试通过
- **Verification**: `programmatic`

## Open Questions
- [ ] 确认 `oaks/examples` 中是否有对应的解析器实现
- [ ] 确认移除这些依赖后是否需要修改相关代码