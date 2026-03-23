//! CLI 命令模块

pub mod build;
pub mod check;
pub mod clean;
pub mod info;
#[cfg(feature = "dev")]
pub mod develop;
pub mod init;
pub mod new;
pub mod plugin;
pub mod telemetry;
pub mod version;

pub use build::BuildCommand;
pub use check::CheckCommand;
pub use clean::CleanCommand;
pub use info::InfoCommand;
#[cfg(feature = "dev")]
pub use develop::DevelopCommand;
pub use init::InitCommand;
pub use new::NewCommand;
pub use plugin::PluginCommand;
pub use telemetry::TelemetryCommand;
pub use version::VersionCommand;
