import { describe, it, expect, beforeAll, afterAll } from "vitest";
import { existsSync, readdirSync, rmSync, mkdirSync, copyFileSync } from "fs";
import { execSync } from "child_process";
import { join } from "path";

// 构建配置
const rootDir = join(__dirname, "..");
const publicOfficialDir = join(rootDir, "public-official");
const publicRustyDir = join(rootDir, "public-rusty");

describe("构建脚本测试", () => {
    // 测试前清理目录
    beforeAll(() => {
        cleanupDirectories();
    });

    // 测试后清理目录
    afterAll(() => {
        cleanupDirectories();
    });

    // 清理目录
    function cleanupDirectories() {
        if (existsSync(publicOfficialDir)) {
            rmSync(publicOfficialDir, { recursive: true, force: true });
        }

        if (existsSync(publicRustyDir)) {
            rmSync(publicRustyDir, { recursive: true, force: true });
        }
    }

    // 获取 Hugo 二进制文件路径
    function getBinary(isOfficial: boolean): string {
        return isOfficial
            ? "hugo"
            : join(
                  "..",
                  "..",
                  "target",
                  "debug",
                  process.platform === "win32" ? "hugo.exe" : "hugo",
              );
    }

    // 运行官方 Hugo 构建
    function runOfficialHugoBuild() {
        const binary = getBinary(true);
        console.log("Using global official hugo");
        execSync(`${binary} build -d public-official`, { cwd: rootDir, stdio: "inherit" });
    }

    // 运行 rusty-ssg Hugo 构建
    function runRustySsgBuild() {
        const binary = getBinary(false);
        console.log(`Using local rusty-ssg hugo: ${binary}`);
        execSync(`${binary} build -d public-rusty`, { cwd: rootDir, stdio: "inherit" });
    }

    // 比较两个目录
    function compareDirectories() {
        console.log("Comparing public-official and public-rusty directories:");

        function getAllFiles(dir: string): string[] {
            const files: string[] = [];
            const entries = readdirSync(dir, { withFileTypes: true });

            for (const entry of entries) {
                const fullPath = join(dir, entry.name);
                if (entry.isDirectory()) {
                    files.push(...getAllFiles(fullPath));
                } else {
                    files.push(fullPath.replace(publicOfficialDir, ""));
                }
            }

            return files;
        }

        const officialFiles = getAllFiles(publicOfficialDir);
        const missingFiles: string[] = [];

        for (const file of officialFiles) {
            const publicFilePath = join(publicRustyDir, file);
            if (!existsSync(publicFilePath)) {
                missingFiles.push(file);
            }
        }

        if (missingFiles.length > 0) {
            console.log("Files in public-official but not in public-rusty:");
            missingFiles.forEach((file) => console.log(`- ${file}`));
        } else {
            console.log("All files in public-official are present in public-rusty");
        }

        return missingFiles;
    }

    it("构建结果比较测试（先 rusty 后官方）", () => {
        // 先清空两个目录
        cleanupDirectories();
        // 先执行 rusty-ssg 构建
        runRustySsgBuild();
        // 再执行官方 hugo 构建
        runOfficialHugoBuild();
        // 执行比较命令
        const missingFiles = compareDirectories();
        // 验证所有文件都存在
        expect(missingFiles.length).toBe(0);
    });
});