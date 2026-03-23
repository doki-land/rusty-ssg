# 部署指南

本指南将帮助您将使用 Rusty SSG 构建的静态站点部署到各种平台。

## 部署准备

在部署之前，请确保您已经：

1. 构建了生产版本的站点：
   ```bash
   # 对于 Astro
   astro build
   
   # 对于 Hugo
   hugo build
   
   # 对于其他编译器
   <compiler-name> build
   ```

2. 验证构建输出是否正确：
   - 检查输出目录（通常是 `dist` 或 `public`）
   - 确保所有静态资源都已正确生成
   - 测试站点的基本功能

## 部署平台

### Netlify

Netlify 是一个流行的静态站点托管平台，提供持续部署、CDN 和其他功能。

#### 配置步骤

1. **连接仓库**：
   - 登录 Netlify 账户
   - 点击 "Add new site" > "Import an existing project"
   - 选择您的 Git 仓库

2. **配置构建设置**：
   - **Build command**：`cargo install <compiler-name> && <compiler-name> build`
   - **Publish directory**：根据编译器的输出目录（如 `dist` 或 `public`）
   - **Environment variables**：如果需要，可以添加环境变量

3. **部署**：
   - 点击 "Deploy site"
   - Netlify 将自动构建和部署您的站点

#### 示例 `netlify.toml`

```toml
# netlify.toml
[build]
  command = "cargo install astro && astro build"
  publish = "dist"

[build.environment]
  RUST_VERSION = "stable"

[[redirects]]
  from = "/*"
  to = "/index.html"
  status = 200
```

### Vercel

Vercel 是另一个流行的静态站点托管平台，特别适合前端项目。

#### 配置步骤

1. **连接仓库**：
   - 登录 Vercel 账户
   - 点击 "New Project"
   - 选择您的 Git 仓库

2. **配置构建设置**：
   - **Build Command**：`cargo install <compiler-name> && <compiler-name> build`
   - **Output Directory**：根据编译器的输出目录（如 `dist` 或 `public`）
   - **Environment Variables**：如果需要，可以添加环境变量

3. **部署**：
   - 点击 "Deploy"
   - Vercel 将自动构建和部署您的站点

#### 示例 `vercel.json`

```json
// vercel.json
{
  "buildCommand": "cargo install astro && astro build",
  "outputDirectory": "dist",
  "env": {
    "RUST_VERSION": "stable"
  },
  "rewrites": [
    {
      "source": "/(.*)",
      "destination": "/index.html"
    }
  ]
}
```

### GitHub Pages

GitHub Pages 是一个免费的静态站点托管服务，直接集成到 GitHub 中。

#### 配置步骤

1. **创建 GitHub Actions 工作流**：
   - 在您的仓库中创建 `.github/workflows/deploy.yml` 文件

2. **配置工作流**：
   ```yaml
   # .github/workflows/deploy.yml
   name: Deploy to GitHub Pages
   
   on:
     push:
       branches: [ main ]
   
   jobs:
     deploy:
       runs-on: ubuntu-latest
       steps:
         - uses: actions/checkout@v3
         
         - name: Set up Rust
           uses: actions-rs/toolchain@v1
           with:
             toolchain: stable
             override: true
         
         - name: Install compiler
           run: cargo install --git https://github.com/doki-land/rusty-ssg.git <compiler-name>
         
         - name: Build site
           run: <compiler-name> build
         
         - name: Deploy to GitHub Pages
           uses: peaceiris/actions-gh-pages@v3
           with:
             github_token: ${{ secrets.GITHUB_TOKEN }}
             publish_dir: ./dist  # 根据编译器的输出目录调整
   ```

3. **启用 GitHub Pages**：
   - 进入仓库设置
   - 导航到 "Pages" 部分
   - 在 "Source" 下拉菜单中选择 "GitHub Actions"

4. **部署**：
   - 推送更改到 `main` 分支
   - GitHub Actions 将自动构建和部署您的站点

### GitLab Pages

GitLab Pages 是 GitLab 提供的静态站点托管服务。

#### 配置步骤

1. **创建 `.gitlab-ci.yml` 文件**：
   ```yaml
   # .gitlab-ci.yml
   pages:
     image: rust:latest
     script:
       - cargo install --git https://github.com/doki-land/rusty-ssg.git <compiler-name>
       - <compiler-name> build
       - mv <output-dir> public  # 根据编译器的输出目录调整
     artifacts:
       paths:
         - public
     only:
       - main
   ```

2. **部署**：
   - 推送更改到 `main` 分支
   - GitLab CI 将自动构建和部署您的站点

3. **访问站点**：
   - 站点将在 `https://<username>.gitlab.io/<project-name>` 可用

### Cloudflare Pages

Cloudflare Pages 是 Cloudflare 提供的静态站点托管服务，具有全球 CDN 和其他性能特性。

#### 配置步骤

1. **连接仓库**：
   - 登录 Cloudflare 账户
   - 导航到 "Pages"
   - 点击 "Create a project"
   - 选择您的 Git 仓库

2. **配置构建设置**：
   - **Production branch**：`main`
   - **Build command**：`cargo install <compiler-name> && <compiler-name> build`
   - **Build output directory**：根据编译器的输出目录（如 `dist` 或 `public`）
   - **Environment variables**：添加 `RUST_VERSION=stable`

3. **部署**：
   - 点击 "Save and Deploy"
   - Cloudflare Pages 将自动构建和部署您的站点

### AWS S3 + CloudFront

对于需要更多控制和可扩展性的项目，可以使用 AWS S3 存储静态文件，配合 CloudFront CDN 提供全球分发。

#### 配置步骤

1. **创建 S3 存储桶**：
   - 登录 AWS 控制台
   - 导航到 S3
   - 创建一个新的存储桶，名称与您的域名匹配
   - 启用 "Static website hosting"
   - 设置索引文档为 `index.html`
   - 设置错误文档为 `404.html`

2. **配置存储桶权限**：
   - 确保存储桶策略允许公共读取访问

3. **创建 CloudFront 分发**：
   - 导航到 CloudFront
   - 创建一个新的分发
   - 选择您的 S3 存储桶作为源
   - 配置其他设置（如 HTTPS、缓存行为等）

4. **部署脚本**：
   ```bash
   # deploy.sh
   #!/bin/bash
   
   # 构建站点
   <compiler-name> build
   
   # 同步到 S3
   aws s3 sync <output-dir> s3://<bucket-name> --delete
   
   #  invalidate CloudFront cache
   aws cloudfront create-invalidation --distribution-id <distribution-id> --paths "/*"
   ```

5. **运行部署**：
   - 执行 `./deploy.sh` 脚本

### 传统 Web 服务器

如果您有自己的 Web 服务器，可以手动部署静态文件。

#### 配置步骤

1. **构建站点**：
   ```bash
   <compiler-name> build
   ```

2. **上传文件**：
   - 使用 FTP、SFTP 或其他文件传输工具将构建输出目录（如 `dist` 或 `public`）上传到您的 Web 服务器

3. **配置 Web 服务器**：
   - 对于 Nginx：
     ```nginx
     server {
       listen 80;
       server_name example.com;
       
       root /var/www/example.com;
       index index.html;
       
       location / {
         try_files $uri $uri/ /index.html;
       }
     }
     ```
   
   - 对于 Apache：
     ```apache
     <VirtualHost *:80>
       ServerName example.com
       DocumentRoot /var/www/example.com
       
       <Directory /var/www/example.com>
         Options Indexes FollowSymLinks
         AllowOverride All
         Require all granted
       </Directory>
       
       ErrorDocument 404 /404.html
     </VirtualHost>
     ```

## 部署最佳实践

### 环境变量

在部署过程中，您可能需要使用环境变量来配置构建过程：

- **RUST_VERSION**：指定 Rust 版本（建议使用 `stable`）
- **BUILD_ENV**：指定构建环境（如 `production` 或 `staging`）
- **API_KEYS**：如果您的站点需要 API 密钥，确保使用安全的方式管理

### 缓存策略

为了提高站点性能，建议配置适当的缓存策略：

- **静态资源**：设置较长的缓存时间（如 1 年）
- **HTML 文件**：设置较短的缓存时间或禁用缓存
- **使用内容哈希**：对于静态资源，使用内容哈希作为文件名，以便在内容更改时自动更新缓存

### 安全措施

- **HTTPS**：确保站点使用 HTTPS
- **CSP (Content Security Policy)**：配置适当的 CSP 以防止 XSS 攻击
- **XSS 防护**：确保站点代码不包含 XSS 漏洞
- **依赖安全**：定期更新依赖项以修复安全漏洞

### 性能优化

- **压缩**：确保所有静态资源（HTML、CSS、JavaScript）都已压缩
- **图像优化**：优化图像大小和质量
- **代码分割**：使用代码分割减少初始加载时间
- **预加载**：预加载关键资源
- **CDN**：使用 CDN 分发静态资源

### 监控和分析

- **Google Analytics**：添加 Google Analytics 或其他分析工具来跟踪站点访问
- **错误监控**：设置错误监控以捕获生产环境中的错误
- **性能监控**：使用 Lighthouse 或其他工具定期测试站点性能

## 常见部署问题

### 构建失败

- **Rust 版本问题**：确保使用正确的 Rust 版本
- **依赖项问题**：确保所有依赖项都已正确安装
- **配置错误**：检查构建命令和输出目录配置
- **内存不足**：对于大型站点，可能需要增加构建环境的内存

### 部署后问题

- **404 错误**：确保服务器配置正确处理路由
- **资源加载失败**：检查资源路径是否正确
- **样式问题**：确保 CSS 文件已正确加载
- **JavaScript 错误**：检查浏览器控制台是否有 JavaScript 错误

### 性能问题

- **加载时间长**：优化图像、压缩资源、使用 CDN
- **首次内容绘制慢**：优化关键渲染路径
- **交互响应慢**：优化 JavaScript 执行

## 自动化部署

为了简化部署过程，建议设置自动化部署流程：

### GitHub Actions

```yaml
# .github/workflows/deploy.yml
name: Deploy
on:
  push:
    branches: [ main ]
  pull_request:
    branches: [ main ]
jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - name: Set up Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - name: Build
        run: |
          cargo install <compiler-name>
          <compiler-name> build
      - name: Test
        run: |
          # 运行测试（如果有）
      - name: Deploy
        if: github.event_name == 'push' && github.ref == 'refs/heads/main'
        run: |
          # 部署命令
```

### GitLab CI

```yaml
# .gitlab-ci.yml
stages:
  - build
  - test
  - deploy

build:
  stage: build
  image: rust:latest
  script:
    - cargo install <compiler-name>
    - <compiler-name> build
  artifacts:
    paths:
      - <output-dir>

test:
  stage: test
  image: rust:latest
  script:
    - # 运行测试（如果有）
  dependencies:
    - build

deploy:
  stage: deploy
  image: rust:latest
  script:
    - # 部署命令
  dependencies:
    - build
  only:
    - main
```

## 部署检查清单

在部署站点之前，请确保：

- [ ] 站点在本地构建成功
- [ ] 所有链接都已验证
- [ ] 图像和其他静态资源已优化
- [ ] 站点在不同设备和浏览器上测试通过
- [ ] 分析和监控工具已集成
- [ ] 安全设置已配置
- [ ] 缓存策略已优化
- [ ] 部署配置已正确设置

## 结论

通过本指南，您应该能够将使用 Rusty SSG 构建的静态站点部署到各种平台。选择最适合您需求的部署平台，并遵循最佳实践以确保站点性能和安全性。

如果您遇到任何部署问题，请参考各平台的官方文档或寻求社区支持。

---

祝您部署愉快！🚀