#!/usr/bin/env node

import { existsSync, readdirSync, statSync } from "fs";
import { join, relative } from "path";
import { execSync } from "child_process";

// 检查脚本文件是否存在
const buildRustyPath = join("scripts", "build-rusty.mjs");
const buildOfficialPath = join("scripts", "build-official.mjs");

console.log("Checking script files:");
console.log("build-rusty.mjs exists:", existsSync(buildRustyPath));
console.log("build-official.mjs exists:", existsSync(buildOfficialPath));

// 执行构建脚本
console.log("\nExecuting build scripts...");
try {
    console.log("Running build-official.mjs...");
    execSync(`node ${buildOfficialPath}`, { stdio: "inherit" });
    console.log("\nRunning build-rusty.mjs...");
    execSync(`node ${buildRustyPath}`, { stdio: "inherit" });
} catch (error) {
    console.error("Build failed:", error.message);
    process.exit(1);
}

// 比较两个目录的文件结构
console.log("\nComparing public-official and public-rusty directories:");
const publicOfficialDir = "public-official";
const publicDir = "public-rusty";

function getAllFiles(dir) {
    const files = [];
    const entries = readdirSync(dir, { withFileTypes: true });
    for (const entry of entries) {
        const fullPath = join(dir, entry.name);
        if (entry.isDirectory()) {
            files.push(...getAllFiles(fullPath));
        } else {
            files.push(relative(publicOfficialDir, fullPath));
        }
    }
    return files;
}

const officialFiles = getAllFiles(publicOfficialDir);
const missingFiles = [];

for (const file of officialFiles) {
    const publicFilePath = join(publicDir, file);
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
