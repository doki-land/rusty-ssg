# 修复 Cargo Check 错误 - 产品需求文档

## Overview
- **Summary**: 在保持现有项目结构的情况下，修复 rusty-ssg 项目中所有编译器的 cargo check 错误，确保项目能够正常编译。
- **Purpose**: 解决当前项目中的编译错误，提高代码质量，确保所有编译器能够正常构建。
- **Target Users**: 项目开发团队、维护者和贡献者。

## Goals
- 修复 nargo-document 库中的编译错误
- 修复 jekyll 编译器中的编译错误
- 确保所有编译器能够通过 cargo check
- 保持项目结构不变

## Non-Goals (Out of Scope)
- 不修改项目的目录结构
- 不改变编译器的核心功能
- 不引入新的依赖库
- 不修改现有的 API 接口

## Background & Context
- 项目已经统一了所有编译器的目录结构
- 但是 cargo check 仍然存在一些错误
- 主要错误来自 nargo-document 库和 jekyll 编译器
- 这些错误影响了项目的正常构建

## Functional Requirements
- **FR-1**: 修复 nargo-document 库中的编译错误
- **FR-2**: 修复 jekyll 编译器中的编译错误
- **FR-3**: 确保所有编译器能够通过 cargo check
- **FR-4**: 保持项目结构不变

## Non-Functional Requirements
- **NFR-1**: 保持代码风格一致性
- **NFR-2**: 确保所有 public 项都有文档注释
- **NFR-3**: 不引入新的依赖库
- **NFR-4**: 不修改现有的 API 接口

## Constraints
- **Technical**: 使用 Rust 语言，保持现有项目结构
- **Business**: 不影响现有编译器的功能和性能
- **Dependencies**: 依赖 nargo 和 oaks 库

## Assumptions
- 项目结构已经统一，不需要修改
- 所有编译器的核心功能正常
- 错误主要是语法和依赖问题，不是功能问题

## Acceptance Criteria

### AC-1: 修复 nargo-document 库中的编译错误
- **Given**: nargo-document 库存在编译错误
- **When**: 修复这些错误
- **Then**: nargo-document 库能够通过 cargo check
- **Verification**: `programmatic`

### AC-2: 修复 jekyll 编译器中的编译错误
- **Given**: jekyll 编译器存在编译错误
- **When**: 修复这些错误
- **Then**: jekyll 编译器能够通过 cargo check
- **Verification**: `programmatic`

### AC-3: 所有编译器能够通过 cargo check
- **Given**: 所有编译器项目
- **When**: 运行 cargo check
- **Then**: 所有编译器都能够通过 cargo check，没有错误
- **Verification**: `programmatic`

### AC-4: 保持项目结构不变
- **Given**: 现有项目结构
- **When**: 修复编译错误
- **Then**: 项目结构保持不变，没有新增或删除目录
- **Verification**: `human-judgment`

## Open Questions
- [ ] 修复后是否需要运行完整的测试套件？
- [ ] 是否需要更新依赖版本？