#!/usr/bin/env node

import { execSync } from "child_process";
import { join } from "path";

// 使用本地构建的 hugo
const hugoPath = join("..", "..", "target", "debug", "hugo.exe");

console.log("Using local rusty-ssg hugo:", hugoPath);

try {
    // 构建站点到 public-rusty 目录
    execSync(`${hugoPath} build -d public-rusty`, { stdio: "inherit" });
    console.log("\nBuild completed successfully with rusty-ssg hugo!");
} catch (error) {
    console.error("Build failed with rusty-ssg hugo:", error.message);
    process.exit(1);
}
