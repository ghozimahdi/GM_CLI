extern crate core;

use crate::create_project::create_file_project_flutter::CreateFileProjectFlutter;
use crate::create_project::create_project_flutter::CreateProjectFlutter;
use crate::create_project::create_single_module_project_flutter::CreateSingleModuleProjectFlutter;
use crate::flutter_build_app::FlutterBuildApp;
use crate::flutter_commands::FlutterCommandHandler;
use crate::version::{generate_saweria_qr_code, show_donate, show_info};
use clap::{Arg, ArgAction, ArgMatches, Command};
use colored::Colorize;
use dialoguer::theme::ColorfulTheme;
use dialoguer::Select;
use std::process::exit;
use std::process::Command as StdCommand;

pub mod command;
mod flutter_build_app;
mod flutter_commands;
mod flutter_gen_runner;
mod gen_localization;
mod header;
mod module_manager;
mod table;
mod version;

mod create_project {
    pub mod create_file_project_flutter;
    pub mod create_module_project_flutter;
    pub mod create_project_flutter;
    pub mod create_single_module_project_flutter;
    pub mod ios_analyzing_project_flutter;
    pub mod setup_fvm_project_flutter;
    pub mod setup_melos_project_flutter;
}

fn main() {
    if !is_flutter_installed() {
        println!(
            "{}",
            "Flutter is not installed. Please install Flutter and ensure it's in your PATH.".red()
        );
        exit(1);
    }

    if !is_melos_installed() {
        println!(
            "{}",
            "Melos is not installed. Installing Melos globally...".yellow()
        );
        if !install_melos() {
            println!(
                "{}",
                "Failed to install Melos. Please install it manually.".red()
            );
        }
    }

    let handler = FlutterCommandHandler;

    let matches = Command::new("gm")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Ghozi Mahdi <ghozimahdi@carpkingdom.com>")
        .about("CLI tool for Flutter project management")
        .subcommand(
            Command::new("clear")
                .about("Run flutter_clean & remove pubspec.lock for selected modules"),
        )
        .subcommand(Command::new("app").about("Build APK or IPA interactively"))
        .subcommand(
            Command::new("flutter")
                .about("Run Flutter commands like 'pub add <package>'")
                .arg(
                    Arg::new("CMD")
                        .help("Command to run, e.g., 'pub add flutter_native_splash'")
                        .required(true)
                        .num_args(1..)
                        .value_name("CMD"),
                ),
        )
        .subcommand(
            Command::new("dart")
                .about("Run Dart commands like 'pub add <package>'")
                .arg(
                    Arg::new("CMD")
                        .help("Command to run, e.g., 'pub add flutter_native_splash'")
                        .required(true)
                        .num_args(1..)
                        .value_name("CMD"),
                ),
        )
        .subcommand(
            Command::new("create")
                .about("Create a Flutter project or module")
                .subcommand(Command::new("project").about("Create a Flutter project"))
                .subcommand(Command::new("module").about("Create a Flutter module")),
        )
        .subcommand(Command::new("build").about("Run build_runner after module selection"))
        .subcommand(Command::new("get").about("Run `flutter pub get` after selecting a module"))
        .subcommand(
            Command::new("donate")
                .about("Show gratitude and support the development of this CLI tool"),
        )
        .subcommand(
            Command::new("route")
                .about("Automatically generate route files for Flutter project")
                .arg(
                    Arg::new("name")
                        .help("Route name (e.g., register, filter, profile)")
                        .index(1),
                ),
        )
        .subcommand(
            Command::new("info")
                .about("Displays prerequisites and usage instructions for the gm CLI"),
        )
        .arg(
            Arg::new("build")
                .short('b')
                .long("build")
                .action(ArgAction::SetTrue)
                .help("Run build_runner after module selection"),
        )
        .arg(
            Arg::new("localize")
                .short('l')
                .long("localize")
                .action(ArgAction::SetTrue)
                .help("Generate intl_localization files"),
        )
        .arg(
            Arg::new("assets")
                .short('a')
                .long("assets")
                .action(ArgAction::SetTrue)
                .help("Generate assets with `flutter gen`"),
        )
        .arg(
            Arg::new("flutter_clean")
                .short('c')
                .long("flutter_clean")
                .action(ArgAction::SetTrue)
                .help("Flutter Clean for single module"),
        )
        .arg(
            Arg::new("flutter_get")
                .short('g')
                .long("flutter_get")
                .action(ArgAction::SetTrue)
                .help("Flutter Pub Get for single module"),
        )
        .arg(Arg::new("all").long("all").action(ArgAction::SetTrue).help(
            "Run build_runner, generate intl_localization, and assets after selecting a module",
        ))
        .get_matches();

    if matches.subcommand_name() != Some("donate") && matches.subcommand_name() != Some("info") {
        header::print_header();
    }

    match matches.subcommand() {
        Some(("clear", _)) => FlutterCommandHandler::flutter_clear(),
        Some(("create", sub)) => handle_create(Some(sub)),
        Some(("get", _)) => FlutterCommandHandler::flutter_get(),
        Some(("build", _)) => FlutterCommandHandler::flutter_build_runner(),
        Some(("app", _)) => FlutterBuildApp::build_app(),
        Some(("donate", _)) => show_donate(),
        Some(("route", sub)) => {
            let route_name = sub.get_one::<String>("name").map(|s| s.as_str());
            CreateFileProjectFlutter::create_abstract_route(route_name);
        }
        Some(("info", _)) => show_info(),
        Some(("flutter", sub)) | Some(("dart", sub)) => {
            let cmd_parts: Vec<&str> = sub
                .get_many::<String>("CMD")
                .unwrap()
                .map(String::as_str)
                .collect();

            let flutter_command = cmd_parts.join(" ");

            #[cfg(debug_assertions)]
            println!("Running command: flutter {}", flutter_command);

            FlutterCommandHandler::flutter_pub(&flutter_command);
        }
        _ => {
            if matches.get_flag("flutter_get") {
                handler.run_flutter_get();
                return;
            }
            if matches.get_flag("flutter_clean") {
                handler.run_flutter_clean();
                return;
            }
            FlutterCommandHandler::handle_flutter_commands(&matches);
            if !(matches.get_flag("build")
                || matches.get_flag("localize")
                || matches.get_flag("assets")
                || matches.get_flag("all"))
            {
                show_help()
            }
        }
    }
}

fn is_flutter_installed() -> bool {
    match StdCommand::new("flutter").arg("--version").output() {
        Ok(output) => output.status.success(),
        Err(_) => false,
    }
}

fn is_melos_installed() -> bool {
    match StdCommand::new("fvm").args(&["dart", "run", "melos", "--version"]).output() {
        Ok(output) => output.status.success(),
        Err(_) => false,
    }
}

fn install_melos() -> bool {
    match StdCommand::new("fvm").arg("dart")
        .args(&["pub", "global", "activate", "melos"])
        .status()
    {
        Ok(status) => status.success(),
        Err(_) => false,
    }
}

fn handle_create(sub_matches: Option<&ArgMatches>) {
    match sub_matches.and_then(|m| m.subcommand()) {
        Some(("project", _)) => CreateProjectFlutter::build(),
        Some(("module", _)) => CreateSingleModuleProjectFlutter::build(),
        None => prompt_create_choice(),
        _ => println!(
            "{}",
            "Invalid subcommand! Use `gm create --help` for usage.".red()
        ),
    }
}

fn prompt_create_choice() {
    let choices = &["Create a project", "Create a module"];
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Do you want to create a project or a module?")
        .default(0)
        .items(choices)
        .interact()
        .unwrap();

    match selection {
        0 => CreateProjectFlutter::build(),
        1 => CreateSingleModuleProjectFlutter::build(),
        _ => println!("{}", "Invalid choice. Please try again.".red()),
    }
}

fn show_help() {
    println!("{}", "Usage: gm --help".yellow());
    println!(
        "{} {}\n",
        "Version:".yellow(),
        env!("CARGO_PKG_VERSION").yellow()
    );
    generate_saweria_qr_code();
}
