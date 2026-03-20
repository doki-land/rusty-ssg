//! Gatsby command-line interface

use gatsby::tools::cmd;

#[tokio::main]
async fn main() {
    cmd::build();
}
