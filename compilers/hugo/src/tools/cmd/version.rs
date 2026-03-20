//! Version 命令实现

use console::style;

/// Version 命令
pub struct VersionCommand;

impl VersionCommand {
    /// 执行 version 命令
    pub async fn execute() {
        println!("{}", style("Hugo compatible static site generator").cyan());
        println!("  Version: {}", style(env!("CARGO_PKG_VERSION")).green());
        println!("  Built with: Rust");
        println!();
        println!("{}", style("VuTeX documentation system").cyan());
        println!("  Version: {}", style(env!("CARGO_PKG_VERSION")).green());
    }
}
