//! 初始化命令实现

use super::super::InitArgs;
use crate::types::Result;
use std::{fs, path::PathBuf};

/// 初始化命令
pub struct InitCommand;

impl InitCommand {
    /// 执行初始化命令
    pub async fn execute(args: InitArgs) -> Result<()> {
        // 确定目标目录
        let target_dir =
            args.dir.unwrap_or_else(|| args.name.map(|name| PathBuf::from(name)).unwrap_or_else(|| PathBuf::from(".")));

        // 创建目录结构
        fs::create_dir_all(&target_dir)?;
        fs::create_dir_all(target_dir.join("source"))?;
        fs::create_dir_all(target_dir.join("source/_posts"))?;
        fs::create_dir_all(target_dir.join("themes"))?;

        // 创建配置文件
        let config_content = r#"# Hexo Configuration
## Docs: https://hexo.io/docs/configuration.html
## Source: https://github.com/hexojs/hexo/

# Site
site:
  title: My Blog
  subtitle: Hello World
  description: This is my blog
  author: Your Name
  language: en
  timezone: UTC

# Theme
theme:
  name: landscape
  config: {}

# Deployment
deploy:
  type_: git
  config:
    repo: https://github.com/yourname/yourblog.git
    branch: master
"#;

        fs::write(target_dir.join("_config.toml"), config_content)?;

        // 创建.gitignore文件
        let gitignore_content = r#"# Dependencies
node_modules/

# Hexo generated files
public/
db.json

# Editor directories and files
.vscode/
.idea/
*.swp
*.swo
*~
"#;

        fs::write(target_dir.join(".gitignore"), gitignore_content)?;

        // 创建package.json文件
        let package_json_content = r#"{
  "name": "my-blog",
  "version": "0.0.0",
  "private": true,
  "scripts": {
    "build": "hexo generate",
    "clean": "hexo clean",
    "deploy": "hexo deploy",
    "server": "hexo server"
  },
  "hexo": {
    "version": "4.2.1"
  }
}
"#;

        fs::write(target_dir.join("package.json"), package_json_content)?;

        // 创建示例文章
        let sample_post_content = r#"---
title: Hello World
date: 2024-01-01 00:00:00
categories:
  - Test
tags:
  - Hexo
  - Blog
---

Welcome to [Hexo](https://hexo.io/)! This is your very first post. Check [documentation](https://hexo.io/docs/) for more info. If you get any problems when using Hexo, you can find the answer in [troubleshooting](https://hexo.io/docs/troubleshooting.html) or you can ask me on [GitHub](https://github.com/hexojs/hexo/issues).

## Quick Start

### Create a new post

``` bash
$ hexo new "My New Post"
```

More info: [Writing](https://hexo.io/docs/writing.html)

### Run server

``` bash
$ hexo server
```

More info: [Server](https://hexo.io/docs/server.html)

### Generate static files

``` bash
$ hexo generate
```

More info: [Generating](https://hexo.io/docs/generating.html)

### Deploy to remote sites

``` bash
$ hexo deploy
```

More info: [Deployment](https://hexo.io/docs/deployment.html)
"#;

        fs::write(target_dir.join("source/_posts/hello-world.md"), sample_post_content)?;

        println!("Blog initialized successfully in {:?}", target_dir);
        Ok(())
    }
}
