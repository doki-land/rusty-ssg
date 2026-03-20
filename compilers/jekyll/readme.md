# Rusty Jekyll Compiler

A high-performance, pure Rust implementation of the Jekyll static site generator, designed for exceptional speed and full compatibility with Jekyll's core features.

## Key Features

- **Pure Rust Implementation**: Built entirely in Rust for maximum performance and reliability
- **Jekyll Compatibility**: Fully compatible with Jekyll's directory structure and functionality
- **No Runtime Dependencies**: Works without requiring Ruby or other runtime environments
- **Exceptional Speed**: Leverages Rust's performance characteristics for fast site generation
- **Markdown Processing**: Advanced Markdown parsing and rendering with support for extensions
- **Liquid Templating**: Full support for Jekyll's Liquid template system
- **Theme Support**: Compatible with Jekyll themes
- **Plugin System**: Extensible architecture for custom plugins
- **Command-Line Interface**: Familiar Jekyll-like CLI for ease of use

## Performance

Rusty Jekyll outperforms traditional Jekyll by a significant margin:

- **Up to 10x faster** build times for medium to large sites
- **Lower memory usage** compared to Ruby-based Jekyll
- **Parallel processing** of documents for improved performance

## Installation

```bash
# Install from source
cargo install --path .

# Or install from crates.io (when available)
cargo install jekyll
```

## Usage

### Create a New Site

```bash
jekyll init my-site
cd my-site
```

### Build Your Site

```bash
jekyll build
```

### Serve Your Site Locally

```bash
jekyll dev
```

### Check Your Site for Issues

```bash
jekyll check
```

## Command-Line Interface

Rusty Jekyll provides a familiar command-line interface that mirrors Jekyll's core commands:

- `jekyll init [PATH]` - Create a new Jekyll site
- `jekyll build [OPTIONS]` - Build your site
- `jekyll dev [OPTIONS]` - Serve your site locally with live reload
- `jekyll check [OPTIONS]` - Check your site for configuration issues

## Project Structure

```
compilers/jekyll/
├── src/
│   ├── bin/
│   │   └── jekyll.rs      # Command-line entry point
│   ├── compiler/           # Core compilation logic
│   ├── jekyll/             # Jekyll-specific functionality
│   ├── plugin/             # Plugin system
│   ├── tools/              # Command-line tools
│   └── types/              # Type definitions
├── tests/                  # Test suite
├── benches/                # Performance benchmarks
└── Cargo.toml              # Project configuration
```

## Configuration

Rusty Jekyll uses the same `_config.yml` file structure as traditional Jekyll, making it easy to migrate existing sites.

## Plugins

Rusty Jekyll supports a plugin system that allows you to extend functionality. Plugins can be written in Rust for maximum performance.

## Markdown Support

Rusty Jekyll includes advanced Markdown processing with support for:

- Standard Markdown syntax
- GitHub Flavored Markdown (GFM)
- Code syntax highlighting
- Math rendering with KaTeX

## Liquid Templating

Full support for Jekyll's Liquid template system, including:

- Liquid tags and filters
- Front matter processing
- Layouts and includes

## Themes

Rusty Jekyll is compatible with Jekyll themes. You can use existing Jekyll themes or create your own.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

### Development Setup

1. Clone the repository
2. Install dependencies with `cargo build`
3. Run tests with `cargo test`
4. Run benchmarks with `cargo bench`

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgements

- Inspired by the original [Jekyll](https://jekyllrb.com/) project
- Built with Rust's excellent ecosystem of libraries
- Thanks to all contributors and supporters

## Roadmap

- [x] Core Jekyll compatibility
- [x] Markdown processing
- [x] Liquid templating
- [x] Theme support
- [x] Command-line interface
- [ ] Plugin marketplace
- [ ] Additional Markdown extensions
- [ ] Improved error reporting

---

Built with ❤️ in Rust
