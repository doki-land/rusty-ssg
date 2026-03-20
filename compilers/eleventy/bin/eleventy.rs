//! Eleventy command-line interface

use eleventy::tools::cmd;

#[tokio::main]
async fn main() {
    cmd::run();
}
