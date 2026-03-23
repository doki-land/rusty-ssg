use clap::Parser;
use gatsby::tools::{
    CleanArgs, GatsbyCli, GatsbyCommands, InstallArgs, PluginArgs, PluginSubCommand, TelemetryArgs, TelemetrySubCommand,
    UninstallArgs, cmd::InfoCommand,
};

#[test]
fn test_cli_parser() {
    // 测试build命令解析
    let cli = GatsbyCli::try_parse_from(["gatsby", "build", "--destination", "dist"]).unwrap();
    match cli.command {
        GatsbyCommands::Build(args) => {
            assert_eq!(args.destination.to_str().unwrap(), "dist");
        }
        _ => panic!("Expected Build command"),
    }

    // 测试init命令解析
    let cli = GatsbyCli::try_parse_from(["gatsby", "init", "my-site"]).unwrap();
    match cli.command {
        GatsbyCommands::Init(args) => {
            assert_eq!(args.name, Some("my-site".to_string()));
        }
        _ => panic!("Expected Init command"),
    }

    // 测试clean命令解析
    let cli = GatsbyCli::try_parse_from(["gatsby", "clean", "--force"]).unwrap();
    match cli.command {
        GatsbyCommands::Clean(args) => {
            assert!(args.force);
        }
        _ => panic!("Expected Clean command"),
    }

    // 测试info命令解析
    let cli = GatsbyCli::try_parse_from(["gatsby", "info"]).unwrap();
    match cli.command {
        GatsbyCommands::Info => {
            // 测试通过
        }
        _ => panic!("Expected Info command"),
    }
}

#[test]
fn test_plugin_command_parser() {
    // 测试plugin install命令解析
    let cli =
        GatsbyCli::try_parse_from(["gatsby", "plugin", "install", "gatsby-plugin-sharp", "gatsby-transformer-remark"]).unwrap();
    match cli.command {
        GatsbyCommands::Plugin(args) => match args.subcommand {
            PluginSubCommand::Install(install_args) => {
                assert_eq!(install_args.plugins, ["gatsby-plugin-sharp", "gatsby-transformer-remark"]);
            }
            _ => panic!("Expected Install subcommand"),
        },
        _ => panic!("Expected Plugin command"),
    }

    // 测试plugin uninstall命令解析
    let cli = GatsbyCli::try_parse_from(["gatsby", "plugin", "uninstall", "gatsby-plugin-sharp"]).unwrap();
    match cli.command {
        GatsbyCommands::Plugin(args) => match args.subcommand {
            PluginSubCommand::Uninstall(uninstall_args) => {
                assert_eq!(uninstall_args.plugins, ["gatsby-plugin-sharp"]);
            }
            _ => panic!("Expected Uninstall subcommand"),
        },
        _ => panic!("Expected Plugin command"),
    }

    // 测试plugin list命令解析
    let cli = GatsbyCli::try_parse_from(["gatsby", "plugin", "list"]).unwrap();
    match cli.command {
        GatsbyCommands::Plugin(args) => {
            match args.subcommand {
                PluginSubCommand::List => {
                    // 测试通过
                }
                _ => panic!("Expected List subcommand"),
            }
        }
        _ => panic!("Expected Plugin command"),
    }
}

#[test]
fn test_telemetry_command_parser() {
    // 测试telemetry enable命令解析
    let cli = GatsbyCli::try_parse_from(["gatsby", "telemetry", "enable"]).unwrap();
    match cli.command {
        GatsbyCommands::Telemetry(args) => {
            match args.subcommand {
                TelemetrySubCommand::Enable => {
                    // 测试通过
                }
                _ => panic!("Expected Enable subcommand"),
            }
        }
        _ => panic!("Expected Telemetry command"),
    }

    // 测试telemetry disable命令解析
    let cli = GatsbyCli::try_parse_from(["gatsby", "telemetry", "disable"]).unwrap();
    match cli.command {
        GatsbyCommands::Telemetry(args) => {
            match args.subcommand {
                TelemetrySubCommand::Disable => {
                    // 测试通过
                }
                _ => panic!("Expected Disable subcommand"),
            }
        }
        _ => panic!("Expected Telemetry command"),
    }

    // 测试telemetry status命令解析
    let cli = GatsbyCli::try_parse_from(["gatsby", "telemetry", "status"]).unwrap();
    match cli.command {
        GatsbyCommands::Telemetry(args) => {
            match args.subcommand {
                TelemetrySubCommand::Status => {
                    // 测试通过
                }
                _ => panic!("Expected Status subcommand"),
            }
        }
        _ => panic!("Expected Telemetry command"),
    }
}

#[tokio::test]
async fn test_info_command_execution() {
    // 测试info命令执行（不会失败）
    InfoCommand::execute().await;
}
