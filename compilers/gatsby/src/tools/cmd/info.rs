#![warn(missing_docs)]

//! Info 命令实现
//! 显示项目信息

use std::{env, fs, path::Path};

/// Info 命令
pub struct InfoCommand;

impl InfoCommand {
    /// 执行 info 命令
    pub async fn execute() {
        println!("📊 Gatsby Project Info");
        println!("====================");

        // 显示 Gatsby 版本
        println!("Gatsby version: {}", env!("CARGO_PKG_VERSION"));

        // 显示 Node.js 版本（如果可用）
        if let Ok(output) = std::process::Command::new("node").arg("-v").output() {
            if output.status.success() {
                let version = String::from_utf8_lossy(&output.stdout).trim().to_string();
                println!("Node.js version: {}", version);
            }
        }

        // 显示项目目录
        println!("Project directory: {}", env::current_dir().unwrap().display());

        // 检查是否存在配置文件
        let config_files =
            ["gatsby-config.js", "gatsby-config.json", "gatsby-config.yaml", "gatsby-config.yml", "gatsby-config.toml"];

        println!("\nConfiguration files:");
        let mut found = false;
        for config_file in &config_files {
            if Path::new(config_file).exists() {
                println!("✅ {}", config_file);
                found = true;
            }
        }
        if !found {
            println!("❌ No Gatsby config file found");
        }

        // 检查 src 目录
        println!("\nProject structure:");
        if Path::new("src").exists() {
            println!("✅ src/ directory exists");
            if Path::new("src/pages").exists() {
                println!("✅ src/pages/ directory exists");
            }
            else {
                println!("⚠️  src/pages/ directory missing");
            }
        }
        else {
            println!("❌ src/ directory missing");
        }

        println!("\n🎉 Info command completed");
    }
}
