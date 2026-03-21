//! Astro command-line interface

use clap::{Parser, Subcommand};
use astro::tools::cmd;

/// Astro static site generator
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Build the project for production
    Build {
        /// Path to the project directory
        #[arg(default_value = ".")]
        path: String,
        
        /// Output directory
        #[arg(short, long, default_value = "dist")]
        outdir: String,
    },
    
    /// Start development server
    Dev {
        /// Path to the project directory
        #[arg(default_value = ".")]
        path: String,
        
        /// Port to run the dev server on
        #[arg(short, long, default_value = "3000")]
        port: u16,
    },
    
    /// Preview the built site
    Preview {
        /// Path to the build output directory
        #[arg(default_value = "dist")]
        path: String,
        
        /// Port to run the preview server on
        #[arg(short, long, default_value = "3000")]
        port: u16,
    },
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    
    match cli.command {
        Commands::Build { path, outdir } => {
            cmd::build(&path, &outdir);
        }
        Commands::Dev { path, port } => {
            cmd::dev(&path, port);
        }
        Commands::Preview { path, port } => {
            cmd::preview(&path, port);
        }
    }
}
