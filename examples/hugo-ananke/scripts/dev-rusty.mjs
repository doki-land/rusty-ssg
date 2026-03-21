#!/usr/bin/env node

import { execSync } from "child_process";
import { join } from "path";

// 使用本地构建的 hugo
const hugoPath = join("..", "..", "target", "debug", "hugo.exe");

console.log("Using local rusty-ssg hugo:", hugoPath);

try {
    // 启动开发服务器
    execSync(`${hugoPath} server`, { stdio: "inherit" });
} catch (error) {
    console.error("Development server failed with rusty-ssg hugo:", error.message);
    process.exit(1);
}
