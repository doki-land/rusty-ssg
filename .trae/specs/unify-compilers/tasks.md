# 统一编译器项目结构规范 - 实现计划

## [x] 任务1: 分析当前项目结构
- **Priority**: P0
- **Depends On**: None
- **Description**: 
  - 详细分析每个编译器的当前项目结构
  - 识别不符合规范的目录和文件
  - 记录需要调整的内容
- **Acceptance Criteria Addressed**: AC-1
- **Test Requirements**:
  - `human-judgement` TR-1.1: 所有编译器的结构分析报告
  - `human-judgement` TR-1.2: 不符合规范的目录和文件清单
- **Notes**: 重点关注bin目录和src/bin目录的重复实现问题

## [x] 任务2: 统一目录结构
- **Priority**: P0
- **Depends On**: 任务1
- **Description**: 
  - 按照readme.md规范调整所有编译器的目录结构
  - 确保每个编译器都有统一的目录布局
  - 移除或重命名不符合规范的目录
- **Acceptance Criteria Addressed**: AC-1
- **Test Requirements**:
  - `human-judgement` TR-2.1: 所有编译器目录结构符合规范
  - `programmatic` TR-2.2: 目录结构验证脚本执行通过
- **Notes**: 保持现有功能代码不变，只调整目录结构

## [x] 任务3: 移除重复入口文件
- **Priority**: P0
- **Depends On**: 任务2
- **Description**: 
  - 检查所有编译器是否存在src/bin目录
  - 移除src/bin目录中的重复实现
  - 确保只有bin目录下的主入口文件
- **Acceptance Criteria Addressed**: AC-2
- **Test Requirements**:
  - `programmatic` TR-3.1: 所有编译器只有bin目录下的主入口文件
  - `programmatic` TR-3.2: 编译测试通过
- **Notes**: 特别注意jekyll编译器的重复实现问题

## [x] 任务4: 集成nargo-template抽象层
- **Priority**: P1
- **Depends On**: 任务2
- **Description**: 
  - 在所有编译器中引入nargo-template依赖
  - 修改代码使用nargo-template提供的抽象接口
  - 确保代码能够正常编译和运行
- **Acceptance Criteria Addressed**: AC-3
- **Test Requirements**:
  - `programmatic` TR-4.1: 所有编译器成功引入nargo-template依赖
  - `programmatic` TR-4.2: 编译测试通过
- **Notes**: 需要先了解nargo-template提供的具体接口

## [x] 任务5: 统一使用oaks解析器
- **Priority**: P1
- **Depends On**: 任务2
- **Description**: 
  - 检查所有编译器的解析器实现
  - 确保所有解析器使用oaks库
  - 替换不符合规范的解析库
- **Acceptance Criteria Addressed**: AC-4
- **Test Requirements**:
  - `programmatic` TR-5.1: 所有解析器使用oaks库
  - `programmatic` TR-5.2: 解析功能测试通过
- **Notes**: 注意保持解析功能的一致性

## [x] 任务6: 移除不符合规范的依赖
- **Priority**: P1
- **Depends On**: 任务2
- **Description**: 
  - 检查所有编译器的Cargo.toml文件
  - 移除pulldown-cmark、handlebars、liquid、askama、yaml-rust、serde_yaml、toml等重型非自研库
  - 确保依赖符合规范
- **Acceptance Criteria Addressed**: AC-5
- **Test Requirements**:
  - `programmatic` TR-6.1: 所有编译器的依赖符合规范
  - `programmatic` TR-6.2: 编译测试通过
- **Notes**: 确保移除依赖后功能不受影响

## [x] 任务7: 代码风格和文档检查
- **Priority**: P2
- **Depends On**: 任务3, 任务4, 任务5, 任务6
- **Description**: 
  - 检查所有编译器的代码风格
  - 确保所有public结构体、枚举、方法、字段都有文档注释
  - 修复不符合规范的代码风格和文档
- **Acceptance Criteria Addressed**: NFR-1, NFR-2
- **Test Requirements**:
  - `human-judgement` TR-7.1: 代码风格符合Rust官方规范
  - `human-judgement` TR-7.2: 所有public项都有文档注释
- **Notes**: 使用rustfmt格式化代码

## [x] 任务8: 测试验证
- **Priority**: P1
- **Depends On**: 任务3, 任务4, 任务5, 任务6
- **Description**: 
  - 运行所有编译器的测试用例
  - 确保统一结构后功能正常
  - 修复测试中发现的问题
- **Acceptance Criteria Addressed**: 所有AC
- **Test Requirements**:
  - `programmatic` TR-8.1: 所有测试用例通过
  - `programmatic` TR-8.2: 编译和运行测试通过
- **Notes**: 确保所有功能都能正常工作