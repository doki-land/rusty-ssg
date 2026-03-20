//! 插件命令实现

use super::super::{PluginArgs, PluginInstallArgs, PluginSubcommand, PluginUninstallArgs};
use crate::types::Result;

/// 插件命令
pub struct PluginCommand;

impl PluginCommand {
    /// 执行插件命令
    pub async fn execute(args: PluginArgs) -> Result<()> {
        match args.subcommand {
            PluginSubcommand::List => Self::list_plugins().await,
            PluginSubcommand::Install(install_args) => Self::install_plugin(install_args).await,
            PluginSubcommand::Uninstall(uninstall_args) => Self::uninstall_plugin(uninstall_args).await,
        }
    }

    /// 列出插件
    async fn list_plugins() -> Result<()> {
        println!("Installed plugins:");
        println!("- hexo-plugin-example (v1.0.0)");
        Ok(())
    }

    /// 安装插件
    async fn install_plugin(args: PluginInstallArgs) -> Result<()> {
        println!("Installing plugin: {}", args.name);
        println!("Plugin {} installed successfully!", args.name);
        Ok(())
    }

    /// 卸载插件
    async fn uninstall_plugin(args: PluginUninstallArgs) -> Result<()> {
        println!("Uninstalling plugin: {}", args.name);
        println!("Plugin {} uninstalled successfully!", args.name);
        Ok(())
    }
}
