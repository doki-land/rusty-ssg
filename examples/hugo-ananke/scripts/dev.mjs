#!/usr/bin/env node

import { execSync } from "child_process";

// 使用全局的 hugo
const hugoCommand = "hugo";

console.log("Using global official hugo");

try {
    // 启动开发服务器
    execSync(`${hugoCommand} server`, { stdio: "inherit" });
} catch (error) {
    console.error("Development server failed with official hugo:", error.message);
    console.error("Please make sure hugo is installed and available in PATH");
    process.exit(1);
}
