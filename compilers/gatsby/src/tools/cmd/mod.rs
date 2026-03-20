//! CLI 命令模块

pub mod build;
pub mod check;
#[cfg(feature = "dev")]
pub mod develop;
pub mod init;
pub mod new;
pub mod version;

pub use build::BuildCommand;
pub use check::CheckCommand;
#[cfg(feature = "dev")]
pub use develop::DevelopCommand;
pub use init::InitCommand;
pub use new::NewCommand;
pub use version::VersionCommand;
