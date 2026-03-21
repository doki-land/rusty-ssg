#!/usr/bin/env node

import { execSync } from "child_process";

// 使用全局的 hugo
const hugoCommand = "hugo";

console.log("Using global official hugo");

try {
    // 构建站点到 public-official 目录
    execSync(`${hugoCommand} build -d public-official`, { stdio: "inherit" });
    console.log("\nBuild completed successfully with official hugo!");
} catch (error) {
    console.error("Build failed with official hugo:", error.message);
    console.error("Please make sure hugo is installed and available in PATH");
    process.exit(1);
}
