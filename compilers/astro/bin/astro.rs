//! Astro command-line interface

use astro::tools::cmd;

#[tokio::main]
async fn main() {
    cmd::build();
}
