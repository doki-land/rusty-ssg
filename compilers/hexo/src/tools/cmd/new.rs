//! 新建文章命令实现

use super::super::NewArgs;
use crate::types::Result;
use chrono::Local;
use std::{fs, path::PathBuf};

/// 新建文章命令
pub struct NewCommand;

impl NewCommand {
    /// 执行新建文章命令
    pub async fn execute(args: NewArgs) -> Result<()> {
        // 确定文章路径
        let mut target_path = if args.draft {
            PathBuf::from("source/_drafts")
        }
        else if args.publish {
            PathBuf::from("source/_posts")
        }
        else {
            PathBuf::from("source/_posts")
        };

        // 创建目录
        fs::create_dir_all(&target_path)?;

        // 生成文件名
        let file_name = if let Some(path) = args.path {
            path
        }
        else {
            let slug = args.title.to_lowercase().replace(" ", "-");
            let date = Local::now().format("%Y-%m-%d").to_string();
            PathBuf::from(format!("{}-{}.md", date, slug))
        };

        target_path.push(file_name);

        // 生成文章内容
        let content = format!(
            r#"---
title: {}
date: {}
categories:
tags:
---

"#,
            args.title,
            Local::now().format("%Y-%m-%d %H:%M:%S")
        );

        // 写入文件
        fs::write(&target_path, content)?;

        println!("Created post at {:?}", target_path);
        Ok(())
    }
}
