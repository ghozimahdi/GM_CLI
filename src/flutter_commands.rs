use crate::flutter_gen_runner;
use crate::gen_easy_localization;
use crate::module_manager::ModuleManager;
use clap::ArgMatches;
use colored::Colorize;
use dialoguer::{theme::ColorfulTheme, Select};
use std::process::Command as StdCommand;

pub struct FlutterCommandHandler;

impl FlutterCommandHandler {
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

    pub fn handle_flutter_commands(matches: &ArgMatches) {
        if matches.get_flag("build") {
            Self::flutter_build_runner();
        }
        if matches.get_flag("localize") {
            gen_easy_localization::easy_localization();
        }
        if matches.get_flag("assets") {
            flutter_gen_runner::gen_assets();
        }
        if matches.get_flag("all") {
            Self::flutter_build_runner();
            gen_easy_localization::easy_localization();
            flutter_gen_runner::gen_assets();
        }
    }

    pub fn run_flutter_get(&self) {
        Self::execute_command("flutter", &["pub", "get"]);
    }

    pub fn run_flutter_clean(&self) {
        Self::execute_command("flutter", &["clean"]);
    }

    pub fn flutter_get() {
        if let Some(selected_modules) = ModuleManager::select_modules() {
            let choices = &["Yes", "No"];
            let build_runner_selection = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Apakah ingin menjalankan build_runner?")
                .default(1)
                .items(choices)
                .interact()
                .unwrap();

            let do_build_runner = build_runner_selection == 0;

            for (_, module) in selected_modules {
                println!("{}: {}", "Running pub get for module".green(), module);
                Self::run_get_command(Some(&module));

                if do_build_runner {
                    println!("{}: {}", "Running build_runner for module".green(), module);
                    Self::run_build_command(Some(&module));
                }
            }
        }

        println!("{}", "Proses pub get selesai.".cyan());
    }

    fn run_get_command(scope: Option<&String>) {
        let mut command = std::process::Command::new("melos");
        command.arg("exec");
        if let Some(scope) = scope {
            command.arg("--scope").arg(scope);
        }
        command.arg("--").arg("flutter").arg("pub").arg("get");

        let status = command
            .status()
            .expect("Gagal menjalankan perintah flutter pub get");
        if !status.success() {
            eprintln!("{}", "Pub get gagal.".red());
        }
    }

    pub fn flutter_clear() {
        if let Some(selected_modules) = ModuleManager::select_modules() {
            let choices = &["Yes", "No"];
            let pub_get_selection = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Apakah ingin melakukan flutter pub get?")
                .default(1)
                .items(choices)
                .interact()
                .unwrap();

            let do_flutter_get = pub_get_selection == 0;

            let build_runner_selection = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Apakah ingin menjalankan build_runner?")
                .default(1)
                .items(choices)
                .interact()
                .unwrap();

            let do_build_runner = build_runner_selection == 0;

            for (_, module) in selected_modules {
                println!("{}: {}", "Clearing module".green(), module);
                Self::run_clear_command(Some(&module));

                if do_flutter_get {
                    println!(
                        "{}: {}",
                        "Running flutter pub get for module".green(),
                        module
                    );
                    Self::run_get_command(Some(&module));
                }

                if do_build_runner {
                    println!("{}: {}", "Running build_runner for module".green(), module);
                    Self::run_build_command(Some(&module));
                }
            }
        }

        println!("{}", "Proses clear selesai.".cyan());
    }

    fn run_clear_command(scope: Option<&String>) {
        let mut command = std::process::Command::new("melos");
        command.arg("exec");
        if let Some(scope) = scope {
            command.arg("--scope").arg(scope);
        }
        command.arg("--").arg("flutter").arg("clean");

        let status = command
            .status()
            .expect("Gagal menjalankan perintah flutter clean");
        if !status.success() {
            eprintln!("{}", "Clear gagal.".red());
        }
    }

    pub fn flutter_build_runner() {
        if let Some(selected_modules) = ModuleManager::select_modules() {
            for (_, module) in selected_modules {
                println!("{}: {}", "Building module".green(), module);
                Self::run_build_command(Some(&module));
            }
        }

        println!("{}", "Proses build selesai.".cyan());
    }

    fn run_build_command(scope: Option<&String>) {
        let mut command = std::process::Command::new("melos");
        command.arg("exec");
        if let Some(scope) = scope {
            command.arg("--scope").arg(scope);
        }
        command
            .arg("--")
            .arg("dart")
            .arg("pub")
            .arg("run")
            .arg("build_runner")
            .arg("build")
            .arg("-d");

        let status = command.status().expect("Gagal menjalankan build command");
        if !status.success() {
            eprintln!("{}", "Build gagal.".red());
        }
    }
}
