# 移除禁止使用的依赖 - 实现计划

## [x] Task 1: 确认 oaks/examples 中是否有对应的解析器实现
- **Priority**: P0
- **Depends On**: None
- **Description**: 
  - 检查 oaks/examples 目录中是否存在替代被禁止依赖的解析器实现
  - 确认是否需要为缺失的解析器创建实现
- **Acceptance Criteria Addressed**: N/A
- **Test Requirements**: 
  - `human-judgement` TR-1.1: 确认 oaks/examples 中存在所有需要的解析器实现
- **Notes**: 这是后续任务的前提，必须先确认解析器可用性

## [x] Task 2: 移除 vuepress 编译器中的 askama 依赖
- **Priority**: P0
- **Depends On**: Task 1
- **Description**: 
  - 从 vuepress/Cargo.toml 中移除 askama 依赖
  - 使用 oaks/examples 提供的解析器替代
  - 修改相关代码以使用新的解析器
- **Acceptance Criteria Addressed**: AC-1
- **Test Requirements**: 
  - `programmatic` TR-2.1: 运行 vuepress 编译器的测试，确保所有测试通过
  - `human-judgement` TR-2.2: 检查代码是否使用了 oaks/examples 提供的解析器
- **Notes**: 需要确保替换后的解析器功能与原 askama 库一致

## [x] Task 3: 移除 vitepress 编译器中的 toml 依赖
- **Priority**: P0
- **Depends On**: Task 1
- **Description**: 
  - 从 vitepress/Cargo.toml 中移除 toml 依赖
  - 使用 oaks/examples 提供的解析器替代
  - 修改相关代码以使用新的解析器
- **Acceptance Criteria Addressed**: AC-2
- **Test Requirements**: 
  - `programmatic` TR-3.1: 运行 vitepress 编译器的测试，确保所有测试通过
  - `human-judgement` TR-3.2: 检查代码是否使用了 oaks/examples 提供的解析器
- **Notes**: 需要确保替换后的解析器功能与原 toml 库一致

## [x] Task 4: 移除 mkdocs 编译器中的 serde_yaml 依赖
- **Priority**: P0
- **Depends On**: Task 1
- **Description**: 
  - 从 mkdocs/Cargo.toml 中移除 serde_yaml 依赖
  - 使用 oaks/examples 提供的解析器替代
  - 修改相关代码以使用新的解析器
- **Acceptance Criteria Addressed**: AC-3
- **Test Requirements**: 
  - `programmatic` TR-4.1: 运行 mkdocs 编译器的测试，确保所有测试通过
  - `human-judgement` TR-4.2: 检查代码是否使用了 oaks/examples 提供的解析器
- **Notes**: 需要确保替换后的解析器功能与原 serde_yaml 库一致

## [x] Task 5: 移除 hugo 编译器中的 handlebars 依赖
- **Priority**: P0
- **Depends On**: Task 1
- **Description**: 
  - 从 hugo/Cargo.toml 中移除 handlebars 依赖
  - 使用 oaks/examples 提供的解析器替代
  - 修改相关代码以使用新的解析器
- **Acceptance Criteria Addressed**: AC-4
- **Test Requirements**: 
  - `programmatic` TR-5.1: 运行 hugo 编译器的测试，确保所有测试通过
  - `human-judgement` TR-5.2: 检查代码是否使用了 oaks/examples 提供的解析器
- **Notes**: 需要确保替换后的解析器功能与原 handlebars 库一致

## [x] Task 6: 移除 gatsby 编译器中的 toml 依赖
- **Priority**: P0
- **Depends On**: Task 1
- **Description**: 
  - 从 gatsby/Cargo.toml 中移除 toml 依赖
  - 使用 oaks/examples 提供的解析器替代
  - 修改相关代码以使用新的解析器
- **Acceptance Criteria Addressed**: AC-5
- **Test Requirements**: 
  - `programmatic` TR-6.1: 运行 gatsby 编译器的测试，确保所有测试通过
  - `human-judgement` TR-6.2: 检查代码是否使用了 oaks/examples 提供的解析器
- **Notes**: 需要确保替换后的解析器功能与原 toml 库一致

## [/] Task 7: 运行所有编译器的测试
- **Priority**: P0
- **Depends On**: Task 2, Task 3, Task 4, Task 5, Task 6
- **Description**: 
  - 运行所有编译器的测试，确保所有测试通过
  - 验证移除依赖后功能是否正常
- **Acceptance Criteria Addressed**: AC-1, AC-2, AC-3, AC-4, AC-5
- **Test Requirements**: 
  - `programmatic` TR-7.1: 运行所有编译器的测试，确保所有测试通过
- **Notes**: 这是最终验证步骤，确保所有修改都能正常工作