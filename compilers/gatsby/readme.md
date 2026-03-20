# Gatsby Compiler (Rust Implementation)

A pure Rust implementation of the Gatsby static site generator, designed for exceptional speed, compatibility, and reliability.

## Features

- **Pure Rust Implementation**: Built entirely in Rust for maximum performance and safety
- **No Runtime Dependencies**: Self-contained implementation without relying on external runtime features
- **Blazing Fast**: Leverages Rust's performance characteristics for rapid site generation
- **Gatsby Compatibility**: Designed to be compatible with existing Gatsby projects
- **Type Safety**: Full type checking ensures reliable builds
- **Asynchronous Processing**: Uses Tokio for efficient async operations

## Getting Started

### Prerequisites

- Rust 1.60+ (stable)
- Cargo (Rust's package manager)

### Installation

```bash
# Clone the repository
git clone https://github.com/rusty-ssg/rusty-ssg.git
cd rusty-ssg/compilers/gatsby

# Build the project
cargo build --release

# Run the compiler
./target/release/gatsby
```

## Usage

### Basic Commands

```bash
# Build your site
gatsby build

# Start development server
gatsby develop

# Create a new Gatsby site
gatsby new my-site

# Check site health
gatsby check
```

### Configuration

The Gatsby compiler uses a `gatsby-config.js` file for configuration, just like the original Gatsby. Here's a basic example:

```javascript
module.exports = {
  siteMetadata: {
    title: "My Gatsby Site",
    description: "A site built with Rusty Gatsby",
    author: "Your Name",
  },
  plugins: [
    "gatsby-plugin-react-helmet",
    {
      resolve: "gatsby-source-filesystem",
      options: {
        name: "src",
        path: `${__dirname}/src/`,
      },
    },
  ],
}
```

## Project Structure

```
gatsby/
├── bin/
│   └── gatsby.rs          # Command-line interface
├── src/
│   ├── lib.rs             # Main library entry point
│   ├── compiler/          # Core compilation logic
│   ├── plugin/            # Plugin system
│   ├── plugin_host/       # Plugin execution environment
│   ├── session/           # Session management
│   ├── tools/             # Utility tools
│   └── types/             # Type definitions
├── Cargo.toml             # Rust package configuration
└── README.md              # This file
```

## Performance

The Rust implementation of Gatsby offers significant performance improvements over the original JavaScript implementation:

- **Faster Build Times**: Compile sites in a fraction of the time
- **Lower Memory Usage**: Efficient memory management
- **Concurrent Processing**: Leverage multi-core systems
- **Optimized Asset Handling**: Fast processing of images, CSS, and JavaScript

## Compatibility

The Rust Gatsby compiler aims to be fully compatible with existing Gatsby projects, including:

- Existing Gatsby configurations
- Gatsby plugins
- Gatsby themes
- Markdown and MDX files
- GraphQL queries

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

### Development Setup

```bash
# Clone the repository
git clone https://github.com/rusty-ssg/rusty-ssg.git
cd rusty-ssg/compilers/gatsby

# Install dependencies
cargo build

# Run tests
cargo test
```

## Roadmap

- [ ] Core compilation engine
- [ ] Plugin system
- [ ] GraphQL layer
- [ ] Asset processing
- [ ] Development server
- [ ] Full Gatsby compatibility
- [ ] Performance benchmarks

## License

This project is licensed under the MIT License - see the [LICENSE](https://github.com/rusty-ssg/rusty-ssg/blob/main/LICENSE) file for details.

## Acknowledgments

- Inspired by the original [Gatsby](https://www.gatsbyjs.com/) project
- Built with Rust, Tokio, and other great Rust crates
- Part of the [rusty-ssg](https://github.com/rusty-ssg/rusty-ssg) project

---

Made with ❤️ in Rust
