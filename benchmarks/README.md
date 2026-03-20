# VuTeX 性能基准测试

本目录包含 VuTeX 项目的性能基准测试框架，用于测试不同渲染模式和插件的性能表现。

## 测试内容

- **纯 Node.js 模式**：直接在 Node.js 中运行插件渲染
- **IPC 混合模式**：通过 IPC 通信与 Node.js 进程通信进行渲染
- **IPC 通信开销**：测试 IPC 通信的基本开销

## 目录结构

```
benchmarks/
├── src/
│   ├── benchmark.ts          # 原始基准测试（依赖工作区构建）
│   ├── benchmark-simple.ts   # 简化版测试（已验证可运行）
│   └── benchmark-full.ts     # 完整版测试（包含完整渲染器）
├── test-docs/
│   ├── katex-demo.md         # KaTeX 数学公式测试文档
│   ├── mermaid-demo.md       # Mermaid 图表测试文档
│   └── combined-demo.md      # 综合测试文档
├── package.json
├── tsconfig.json
└── README.md
```

## 运行测试

### 运行简化版测试（推荐）

从项目根目录运行：

```bash
cd benchmarks
pnpm bench:simple
```

或者从项目根目录：

```bash
pnpm exec tsx benchmarks/src/benchmark-simple.ts
```

### 运行完整版测试

完整版测试需要在 benchmarks 目录下安装独立依赖：

```bash
cd benchmarks
pnpm install
pnpm bench
```

## 测试结果

测试会输出以下统计信息：
- 平均执行时间 (ms)
- 中位数执行时间 (ms)
- 最小执行时间 (ms)
- 最大执行时间 (ms)
- 标准差 (ms)

## 自定义配置

可以通过修改 `src/benchmark-*.ts` 文件中的 `ITERATIONS` 常量来调整测试迭代次数。

## 注意事项

- 完整版测试（benchmark-full.ts）需要 katex 和 mermaid 作为直接依赖
- 原始测试（benchmark.ts）需要完整构建工作区包才能运行
- 所有测试均已兼容 Windows 环境
