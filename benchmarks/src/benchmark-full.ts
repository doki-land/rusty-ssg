import * as fs from "fs";
import * as path from "path";
import { fileURLToPath } from "url";
import { spawn, ChildProcess } from "child_process";
import katex from "katex";

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

const TEST_DOCS_DIR = path.join(__dirname, "../test-docs");
const ITERATIONS = 5;

interface BenchmarkResult {
    name: string;
    mean: number;
    min: number;
    max: number;
    median: number;
    stdDev: number;
    iterations: number;
}

interface PluginContext {
    content: string;
    frontmatter: Record<string, unknown>;
    path: string;
}

interface KatexPluginConfig {
    enableInline: boolean;
    enableBlock: boolean;
    inlineDelimiters: [string, string];
    blockDelimiters: [string, string];
    katexOptions: Record<string, unknown>;
}

const defaultKatexPluginConfig: KatexPluginConfig = {
    enableInline: true,
    enableBlock: true,
    inlineDelimiters: ["$", "$"],
    blockDelimiters: ["$$", "$$"],
    katexOptions: {},
};

interface MermaidPluginConfig {
    enable: boolean;
    startDelimiter: string;
    endDelimiter: string;
    mermaidConfig: Record<string, unknown>;
}

const defaultMermaidPluginConfig: MermaidPluginConfig = {
    enable: true,
    startDelimiter: "```mermaid",
    endDelimiter: "```",
    mermaidConfig: {},
};

class KatexRenderer {
    private config: Required<KatexPluginConfig>;

    constructor(config?: KatexPluginConfig) {
        this.config = {
            ...defaultKatexPluginConfig,
            ...config,
        };
    }

    renderInline(latex: string): string {
        return katex.renderToString(latex, {
            ...this.config.katexOptions,
            displayMode: false,
        });
    }

    renderBlock(latex: string): string {
        return katex.renderToString(latex, {
            ...this.config.katexOptions,
            displayMode: true,
        });
    }

    private escapeRegExp(str: string): string {
        return str.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
    }

    renderContent(content: string): string {
        let result = content;

        if (this.config.enableBlock) {
            const [open, close] = this.config.blockDelimiters;
            const blockRegex = new RegExp(
                `${this.escapeRegExp(open)}([\\s\\S]*?)${this.escapeRegExp(close)}`,
                "g",
            );
            result = result.replace(blockRegex, (_, latex) => {
                return this.renderBlock(latex.trim());
            });
        }

        if (this.config.enableInline) {
            const [open, close] = this.config.inlineDelimiters;
            const inlineRegex = new RegExp(
                `${this.escapeRegExp(open)}([^${this.escapeRegExp(close)}]+?)${this.escapeRegExp(close)}`,
                "g",
            );
            result = result.replace(inlineRegex, (_, latex) => {
                return this.renderInline(latex.trim());
            });
        }

        return result;
    }
}

class MermaidRenderer {
    private config: Required<MermaidPluginConfig>;
    private initialized = false;
    private diagramCounter = 0;
    private mermaidModule: unknown | null = null;

    constructor(config?: MermaidPluginConfig) {
        this.config = {
            ...defaultMermaidPluginConfig,
            ...config,
        };
    }

    private async loadMermaid(): Promise<void> {
        if (!this.mermaidModule) {
            this.mermaidModule = await import("mermaid");
        }
    }

    private async initializeMermaid(): Promise<void> {
        if (!this.initialized) {
            await this.loadMermaid();
            const mermaid = this.mermaidModule as {
                initialize: (config: Record<string, unknown>) => void;
            };
            mermaid.initialize(this.config.mermaidConfig);
            this.initialized = true;
        }
    }

    async renderDiagram(code: string): Promise<string> {
        await this.initializeMermaid();
        const diagramId = `mermaid-diagram-${this.diagramCounter++}`;

        try {
            const mermaid = this.mermaidModule as {
                render: (id: string, code: string) => Promise<{ svg: string }>;
            };
            const { svg } = await mermaid.render(diagramId, code.trim());
            return svg;
        } catch (error) {
            console.error("Mermaid 渲染错误:", error);
            return `<div class="mermaid-error">图表渲染失败: ${(error as Error).message}</div>`;
        }
    }

    private escapeRegExp(str: string): string {
        return str.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
    }

    async renderContent(content: string): Promise<string> {
        if (!this.config.enable) {
            return content;
        }

        let result = content;
        const { startDelimiter, endDelimiter } = this.config;
        const regex = new RegExp(
            `${this.escapeRegExp(startDelimiter)}\\s*([\\s\\S]*?)${this.escapeRegExp(endDelimiter)}`,
            "g",
        );

        const matches = [...content.matchAll(regex)];
        for (const match of matches.reverse()) {
            const code = match[1];
            const svg = await this.renderDiagram(code);
            const startIndex = match.index as number;
            const endIndex = startIndex + match[0].length;
            result = result.slice(0, startIndex) + svg + result.slice(endIndex);
        }

        return result;
    }
}

class Benchmarker {
    private results: BenchmarkResult[] = [];

    async measure(name: string, fn: () => Promise<void>): Promise<void> {
        const times: number[] = [];

        console.log(`\n=== 正在测试: ${name} ===`);
        console.log(`执行 ${ITERATIONS} 次迭代...`);

        for (let i = 0; i < ITERATIONS; i++) {
            const start = performance.now();
            await fn();
            const end = performance.now();
            const duration = end - start;
            times.push(duration);
            console.log(`  迭代 ${i + 1}: ${duration.toFixed(2)}ms`);
        }

        const result = this.calculateStats(name, times);
        this.results.push(result);
        this.printResult(result);
    }

    private calculateStats(name: string, times: number[]): BenchmarkResult {
        const sorted = [...times].sort((a, b) => a - b);
        const sum = times.reduce((a, b) => a + b, 0);
        const mean = sum / times.length;
        const min = sorted[0];
        const max = sorted[sorted.length - 1];
        const median =
            sorted.length % 2 === 0
                ? (sorted[sorted.length / 2 - 1] + sorted[sorted.length / 2]) / 2
                : sorted[Math.floor(sorted.length / 2)];
        const variance = times.reduce((acc, t) => acc + Math.pow(t - mean, 2), 0) / times.length;
        const stdDev = Math.sqrt(variance);

        return {
            name,
            mean,
            min,
            max,
            median,
            stdDev,
            iterations: times.length,
        };
    }

    private printResult(result: BenchmarkResult): void {
        console.log(`\n--- ${result.name} 结果 ---`);
        console.log(`  平均: ${result.mean.toFixed(2)}ms`);
        console.log(`  中位数: ${result.median.toFixed(2)}ms`);
        console.log(`  最小: ${result.min.toFixed(2)}ms`);
        console.log(`  最大: ${result.max.toFixed(2)}ms`);
        console.log(`  标准差: ${result.stdDev.toFixed(2)}ms`);
    }

    printReport(): void {
        console.log("\n" + "=".repeat(60));
        console.log("             性能基准测试报告");
        console.log("=".repeat(60));

        console.log(
            "\n".padEnd(24, " ") +
                "平均(ms)".padEnd(12) +
                "中位数(ms)".padEnd(14) +
                "最小(ms)".padEnd(12) +
                "最大(ms)".padEnd(12) +
                "标准差(ms)",
        );
        console.log("-".repeat(90));

        for (const result of this.results) {
            console.log(
                result.name.padEnd(24) +
                    result.mean.toFixed(2).padStart(10) +
                    "ms".padEnd(2) +
                    result.median.toFixed(2).padStart(12) +
                    "ms".padEnd(2) +
                    result.min.toFixed(2).padStart(10) +
                    "ms".padEnd(2) +
                    result.max.toFixed(2).padStart(10) +
                    "ms".padEnd(2) +
                    result.stdDev.toFixed(2).padStart(10) +
                    "ms",
            );
        }

        console.log("\n" + "=".repeat(60));
    }
}

class IPCTester {
    private server: ChildProcess | null = null;

    async startServer(): Promise<void> {
        const serverPath = path.join(__dirname, "../../runtimes/vutex-ipc-server/src/cli.ts");
        const isWindows = process.platform === "win32";

        if (isWindows) {
            this.server = spawn("cmd", ["/c", "npx", "tsx", serverPath], {
                stdio: ["pipe", "pipe", "inherit"],
                shell: true,
            });
        } else {
            this.server = spawn("npx", ["tsx", serverPath], {
                stdio: ["pipe", "pipe", "inherit"],
            });
        }

        await new Promise((resolve) => setTimeout(resolve, 1000));
    }

    async sendRequest(pluginName: string, hookName: string, context: PluginContext): Promise<any> {
        if (!this.server || !this.server.stdin || !this.server.stdout) {
            throw new Error("IPC server not started");
        }

        const request = {
            type: "Request",
            payload: {
                hook_name: hookName,
                plugin_name: pluginName,
                context,
            },
        };

        const responsePromise = new Promise<any>((resolve, reject) => {
            const timeout = setTimeout(() => reject(new Error("IPC timeout")), 5000);

            const onData = (data: Buffer) => {
                const lines = data
                    .toString()
                    .split("\n")
                    .filter((line) => line.trim());
                for (const line of lines) {
                    try {
                        const response = JSON.parse(line);
                        if (response.type === "Response") {
                            clearTimeout(timeout);
                            this.server?.stdout?.removeListener("data", onData);
                            resolve(response.payload);
                        }
                    } catch {}
                }
            };

            this.server?.stdout?.on("data", onData);
        });

        this.server.stdin.write(JSON.stringify(request) + "\n");
        return responsePromise;
    }

    async stopServer(): Promise<void> {
        if (this.server) {
            this.server.kill();
            this.server = null;
        }
    }
}

function loadTestDocs(): Record<string, string> {
    const docs: Record<string, string> = {};
    const files = fs.readdirSync(TEST_DOCS_DIR);

    for (const file of files) {
        if (file.endsWith(".md")) {
            const content = fs.readFileSync(path.join(TEST_DOCS_DIR, file), "utf-8");
            docs[file] = content;
        }
    }

    return docs;
}

async function main(): Promise<void> {
    console.log("=".repeat(60));
    console.log("  VuTeX 性能基准测试 (完整版)");
    console.log("=".repeat(60));
    console.log(`测试迭代次数: ${ITERATIONS}`);

    const docs = loadTestDocs();
    console.log(`加载了 ${Object.keys(docs).length} 个测试文档`);

    const katexRenderer = new KatexRenderer();
    const mermaidRenderer = new MermaidRenderer();

    const benchmarker = new Benchmarker();
    const ipcTester = new IPCTester();

    try {
        await benchmarker.measure("纯 Node.js - KaTeX 渲染", async () => {
            katexRenderer.renderContent(docs["katex-demo.md"]);
        });

        await benchmarker.measure("纯 Node.js - Mermaid 渲染", async () => {
            await mermaidRenderer.renderContent(docs["mermaid-demo.md"]);
        });

        await benchmarker.measure("纯 Node.js - 综合渲染", async () => {
            let content = katexRenderer.renderContent(docs["combined-demo.md"]);
            content = await mermaidRenderer.renderContent(content);
        });

        await ipcTester.startServer();
        await new Promise((resolve) => setTimeout(resolve, 500));

        await ipcTester.sendRequest("@vutex/plugin-katex", "setup", {
            content: "",
            frontmatter: {},
            path: "",
        });

        await ipcTester.sendRequest("@vutex/plugin-mermaid", "setup", {
            content: "",
            frontmatter: {},
            path: "",
        });

        await benchmarker.measure("IPC 通信开销 (空消息)", async () => {
            const context = {
                content: "",
                frontmatter: {},
                path: "/test/empty.md",
            };
            await ipcTester.sendRequest("@vutex/plugin-katex", "before_render", context);
        });

        await ipcTester.stopServer();

        benchmarker.printReport();
    } catch (error) {
        console.error("测试过程中发生错误:", error);
        await ipcTester.stopServer();
        process.exit(1);
    }
}

main().catch(console.error);
