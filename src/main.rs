extern crate core;

use crate::create_project::create_project_flutter::CreateProjectFlutter;
use crate::flutter_build_app::FlutterBuildApp;
use crate::flutter_commands::FlutterCommandHandler;
use clap::{Arg, ArgAction, Command};
use colored::Colorize;

mod flutter_build_app;
mod flutter_commands;
mod flutter_gen_runner;
mod gen_easy_localization;
mod header;
mod module_manager;
mod table;
mod version;
pub mod command;

mod create_project {
    pub mod create_file_project_flutter;
    pub mod create_module_project_flutter;
    pub mod create_project_flutter;
    pub mod setup_melos_project_flutter;
}

fn main() {
    let handler = FlutterCommandHandler;

    version::handle_version();
    header::print_header();

    let matches = Command::new("gm")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Ghozi Mahdi <ghozimahdi@example.com>")
        .about("CLI tool for Flutter project management")
        .subcommand(
            Command::new("clear")
                .about("Run flutter_clean & remove pubspec.lock for selected modules"),
        )
        .subcommand(Command::new("app").about("Build APK or IPA interactively"))
        .subcommand(Command::new("create").about("Create a Flutter project with modular support"))
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
                .help("Generate assets"),
        )
        .arg(
            Arg::new("flutter_clean")
                .short('c')
                .long("flutter_clean")
                .action(ArgAction::SetTrue)
                .help("Flutter Clean"),
        )
        .arg(
            Arg::new("flutter_get")
                .short('g')
                .long("flutter_get")
                .action(ArgAction::SetTrue)
                .help("Flutter Pub Get"),
        )
        .arg(
            Arg::new("all")
                .long("all")
                .action(ArgAction::SetTrue)
                .help("Run build_runner, generate easy_localization, and assets"),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("clear", _)) => FlutterCommandHandler::flutter_clear(),
        Some(("create", _)) => CreateProjectFlutter::build(),
        Some(("get", _)) => FlutterCommandHandler::flutter_get(),
        Some(("app", _)) => FlutterBuildApp::build_app(),
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
}
