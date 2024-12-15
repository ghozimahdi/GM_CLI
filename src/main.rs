extern crate core;

use crate::create_project::create_project_flutter::CreateProjectFlutter;
use crate::flutter_build_app::FlutterBuildApp;
use crate::flutter_commands::FlutterCommandHandler;
use crate::version::{generate_saweria_qr_code, show_info};
use clap::{Arg, ArgAction, Command};
use colored::Colorize;

pub mod command;
mod flutter_build_app;
mod flutter_commands;
mod flutter_gen_runner;
mod gen_easy_localization;
mod header;
mod module_manager;
mod table;
mod version;

mod create_project {
    pub mod create_file_project_flutter;
    pub mod create_module_project_flutter;
    pub mod create_project_flutter;
    pub mod ios_analyzing_project_flutter;
    pub mod setup_fvm_project_flutter;
    pub mod setup_melos_project_flutter;
}

fn main() {
    let handler = FlutterCommandHandler;
    header::print_header();

    let matches = Command::new("gm")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Ghozi Mahdi <ghozimahdi@carpkingdom.com>")
        .about("CLI tool for Flutter project management")
        .subcommand(
            Command::new("clear")
                .about("Run flutter_clean & remove pubspec.lock for selected modules"),
        )
        .subcommand(Command::new("app").about("Build APK or IPA interactively"))
        .subcommand(Command::new("create").about("Create a Flutter project with modular support"))
        .subcommand(Command::new("build").about("Run build_runner after module selection"))
        .subcommand(Command::new("get").about("Run `flutter pub get` after selecting a module"))
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
                .help("Generate easy_localization files"),
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
            "Run build_runner, generate easy_localization, and assets after selecting a module",
        ))
        .get_matches();

    match matches.subcommand() {
        Some(("clear", _)) => FlutterCommandHandler::flutter_clear(),
        Some(("create", _)) => CreateProjectFlutter::build(),
        Some(("get", _)) => FlutterCommandHandler::flutter_get(),
        Some(("build", _)) => FlutterCommandHandler::flutter_build_runner(),
        Some(("app", _)) => FlutterBuildApp::build_app(),
        Some(("info", _)) => show_info(),
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
fn show_help() {
    println!("{}", "Usage: gm --help".yellow());
    println!(
        "{} {}\n",
        "Version:".yellow(),
        env!("CARGO_PKG_VERSION").yellow()
    );
    generate_saweria_qr_code();
}
