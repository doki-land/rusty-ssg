#!/usr/bin/env node

/**
 * 跨平台测试脚本
 * 测试 Rusty Hexo 在不同平台上的功能
 */

import { execSync } from "child_process";
import { existsSync, rmSync, mkdirSync, writeFileSync } from "fs";
import { join } from "path";

// 测试目录
const TEST_DIR = join(process.cwd(), "test_cross_platform");

// 清理测试目录
if (existsSync(TEST_DIR)) {
    rmSync(TEST_DIR, { recursive: true, force: true });
}

mkdirSync(TEST_DIR, { recursive: true });

console.log("=== 跨平台测试开始 ===\n");

// 测试 1: 初始化博客
console.log("1. 测试初始化博客...");
try {
    execSync(`cargo run --bin hexo init ${TEST_DIR}`, { stdio: "inherit" });
    console.log("✓ 初始化博客成功\n");
} catch (error) {
    console.error("✗ 初始化博客失败:", error.message);
    process.exit(1);
}

// 测试 2: 创建新文章
console.log("2. 测试创建新文章...");
try {
    execSync('cargo run --bin hexo new "Test Post"', {
        stdio: "inherit",
        cwd: TEST_DIR,
    });
    console.log("✓ 创建新文章成功\n");
} catch (error) {
    console.error("✗ 创建新文章失败:", error.message);
    process.exit(1);
}

// 测试 3: 生成静态文件
console.log("3. 测试生成静态文件...");
try {
    execSync("cargo run --bin hexo generate", {
        stdio: "inherit",
        cwd: TEST_DIR,
    });
    console.log("✓ 生成静态文件成功\n");
} catch (error) {
    console.error("✗ 生成静态文件失败:", error.message);
    process.exit(1);
}

// 测试 4: 清理
console.log("4. 测试清理...");
try {
    execSync("cargo run --bin hexo clean", {
        stdio: "inherit",
        cwd: TEST_DIR,
    });
    console.log("✓ 清理成功\n");
} catch (error) {
    console.error("✗ 清理失败:", error.message);
    process.exit(1);
}

// 测试 5: 插件管理
console.log("5. 测试插件管理...");
try {
    execSync("cargo run --bin hexo plugin list", {
        stdio: "inherit",
        cwd: TEST_DIR,
    });
    console.log("✓ 插件管理成功\n");
} catch (error) {
    console.error("✗ 插件管理失败:", error.message);
    process.exit(1);
}

console.log("=== 跨平台测试完成 ===");
console.log("所有测试都通过了！");

// 清理测试目录
rmSync(TEST_DIR, { recursive: true, force: true });
