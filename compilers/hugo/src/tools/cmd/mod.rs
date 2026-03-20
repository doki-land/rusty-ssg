//! CLI 命令模块

pub mod build;
pub mod check;
#[cfg(feature = "dev")]
pub mod dev;
pub mod init;
pub mod new;
pub mod server;
pub mod version;

pub use build::BuildCommand;
pub use check::CheckCommand;
#[cfg(feature = "dev")]
pub use dev::DevCommand;
pub use init::InitCommand;
pub use new::NewCommand;
#[cfg(feature = "dev")]
pub use server::ServerCommand;
pub use version::VersionCommand;
