use clap::{Arg, ArgAction, Command};
use colored::*;
use std::process::Command as StdCommand;

mod build_runner;
mod flutter_gen_runner;
mod gen_easy_localization;
mod header;
mod modules;
mod table;
mod version;

fn main() {
    version::handle_version();
    header::print_header();

    let matches = Command::new("gm")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Ghozi Mahdi <ghozimahdi@example.com>")
        .about("CLI tool for Flutter project management")
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
            Arg::new("flutter_clear")
                .short('d')
                .long("flutter_clear")
                .action(ArgAction::SetTrue)
                .help("Flutter Clean + Delete pub lock"),
        )
        .arg(
            Arg::new("all")
                .long("all")
                .action(ArgAction::SetTrue)
                .help("Run build_runner, generate easy_localization, and assets"),
        )
        .get_matches();

    if matches.get_flag("flutter_get") {
        run_flutter_get();
        return;
    }
    if matches.get_flag("flutter_clean") {
        run_flutter_clean();
        return;
    }
    if matches.get_flag("flutter_clear") {
        run_flutter_clear();
        return;
    }

    if matches.get_flag("build") {
        build_runner::build_runner();
    }
    if matches.get_flag("localize") {
        gen_easy_localization::easy_localization();
    }
    if matches.get_flag("assets") {
        flutter_gen_runner::gen_assets();
    }
    if matches.get_flag("all") {
        build_runner::build_runner();
        gen_easy_localization::easy_localization();
        flutter_gen_runner::gen_assets();
    }
    if !(matches.get_flag("build")
        || matches.get_flag("localize")
        || matches.get_flag("assets")
        || matches.get_flag("all"))
    {
        show_help()
    }
}

fn show_help() {
    println!("{}", "Usage: gm -b | -l | -a | --all".yellow());
}

fn execute_command(program: &str, args: &[&str]) {
    println!(
        "{}",
        format!("Running {} {}...", program, args.join(" ")).green()
    );

    let output = StdCommand::new(program)
        .args(args)
        .output()
        .expect(&format!("Failed to execute {} {}", program, args.join(" ")));

    if output.status.success() {
        println!("{}", String::from_utf8_lossy(&output.stdout));
    } else {
        println!("{}", String::from_utf8_lossy(&output.stderr).red());
    }
}

fn run_flutter_get() {
    execute_command("flutter", &["pub", "get"]);
}

fn run_flutter_clean() {
    execute_command("flutter", &["clean"]);
}

fn run_flutter_clear() {
    execute_command("flutter", &["clean"]);

    println!("{}", "Deleting pubspec.lock...".green());
    if std::fs::remove_file("pubspec.lock").is_ok() {
        println!("{}", "Deleted pubspec.lock".green());
    } else {
        println!("{}", "Failed to delete pubspec.lock".red());
    }
}
