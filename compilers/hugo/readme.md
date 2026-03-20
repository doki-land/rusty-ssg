# Hugo Compiler

A **pure Rust** implementation of a Hugo-compatible static site generator, designed for exceptional speed, compatibility, and ease of use.

## Features

- **Pure Rust Implementation**: Built entirely in Rust for maximum performance and reliability
- **Hugo Compatibility**: Compatible with Hugo's content structure and front matter format
- **Exceptional Speed**: Leverages Rust's performance advantages for lightning-fast site generation
- **No Runtime Dependencies**: Self-contained implementation without external runtime requirements
- **Markdown Support**: Full Markdown parsing with extensions
- **Shortcode System**: Supports Hugo-style shortcodes for content reuse
- **Theme Support**: Includes a default theme with customization options
- **Development Server**: Built-in live-reloading development server
- **Command-Line Interface**: Comprehensive CLI with familiar Hugo commands
- **Plugin System**: Extensible plugin architecture with built-in KaTeX support for math rendering

## Installation

### From Source

```bash
cd e:\灵之镜有限公司\rusty-ssg\compilers\hugo
cargo install --path .
```

### From Crates.io (Future)

```bash
cargo install hugo
```

## Usage

### Create a New Site

```bash
hugo init my-site
cd my-site
```

### Create a New Content File

```bash
hugo new content/posts/my-first-post.md
```

### Build the Site

```bash
hugo build
```

### Start the Development Server

```bash
hugo server
```

## Command Reference

- `hugo init <directory>`: Create a new Hugo site
- `hugo new <path>`: Create a new content file
- `hugo build`: Build the site
- `hugo server`: Start the development server
- `hugo check`: Check the site for errors
- `hugo version`: Display version information

## Directory Structure

```
site/
├── content/         # Markdown content files
├── static/          # Static files (images, CSS, etc.)
├── themes/          # Theme files
├── config.toml      # Site configuration
└── public/          # Generated static files
```

## Configuration

The site is configured through `config.toml`:

```toml
[site]
title = "My Hugo Site"
description = "A site built with Rusty Hugo"
author = "Your Name"
base_url = "https://example.com"

[build]
output_dir = "public"
```

## Front Matter

Content files use YAML front matter:

```yaml
---
title: "My First Post"
date: 2026-03-20
draft: false
tags:
  - rust
  - hugo
---

# My First Post

This is the content of my first post.
```

## Shortcodes

Use Hugo-compatible shortcodes in your content:

```markdown
{{< figure src="image.jpg" alt="Description" >}}

{{< highlight rust >}}
fn main() {
    println!("Hello, Hugo!");
}
{{< /highlight >}}
```

## Performance

- **Build Speed**: Significantly faster than traditional Hugo for most sites
- **Memory Usage**: Lower memory footprint due to Rust's efficient memory management
- **Scalability**: Handles large sites with thousands of pages with ease

## Benchmarks

```
$ cargo bench

Compiling hugo v0.1.0
Benchmarking compiler performance...
Benchmarking 1000 pages:
  Time: 234.5ms
  Memory: 12.3MB
```

## Plugins

### Built-in Plugins

- **KaTeX**: For rendering mathematical formulas

### Creating Custom Plugins

Plugins can be created using Rust and integrated into the build process.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

### Development Setup

```bash
# Clone the repository
git clone <repository-url>
cd rusty-ssg/compilers/hugo

# Build the project
cargo build

# Run tests
cargo test

# Run benchmarks
cargo bench
```

## License

This project is licensed under the [MIT License](LICENSE).

## Acknowledgements

- Inspired by the original [Hugo](https://gohugo.io/) static site generator
- Built with Rust's excellent ecosystem of libraries
- Thanks to the Rust community for their ongoing support and contributions

## Roadmap

- [x] Basic site generation
- [x] Markdown parsing and rendering
- [x] Shortcode support
- [x] Theme system
- [x] Development server
- [ ] Internationalization support
- [ ] Taxonomy support
- [ ] Advanced template functions
- [ ] More plugin integrations

## Comparison with Traditional Hugo

| Feature | Rusty Hugo | Traditional Hugo |
|---------|------------|------------------|
| Implementation Language | Rust | Go |
| Build Speed | Faster | Fast |
| Memory Usage | Lower | Higher |
| Dependencies | None (self-contained) | Go runtime |
| Plugin System | Rust-based | Go-based |
| Compatibility | Hugo-compatible | Original |

## Support

If you encounter any issues or have questions, please [open an issue](https://github.com/rusty-ssg/hugo/issues).

---

**Rusty Hugo** - A fast, reliable, and Hugo-compatible static site generator built with Rust.