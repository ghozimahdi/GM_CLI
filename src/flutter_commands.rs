use crate::module_manager::ModuleManager;
use crate::{flutter_gen_runner, gen_easy_localization};
use colored::Colorize;
use dialoguer::{theme::ColorfulTheme, Select};
use serde::Deserialize;
use std::path::{Path, PathBuf};
use std::process::Command as StdCommand;
use std::{fs, thread, time::Duration};

#[derive(Deserialize, Debug)]
struct Module {
    name: String,
    location: String,
}

pub struct FlutterCommandHandler;

impl FlutterCommandHandler {
    fn flutter_command() -> &'static str {
        if Path::new(".fvm").exists() {
            "fvm flutter"
        } else {
            "flutter"
        }
    }

    fn execute_command(args: &[&str]) {
        let command_prefix = Self::flutter_command();
        let full_command = format!("{} {}", command_prefix, args.join(" "));

        println!("{}", format!("Running {}...", full_command).green());

        let output = StdCommand::new(command_prefix)
            .args(args)
            .output()
            .expect(&format!("Failed to execute {}", full_command));

        if output.status.success() {
            println!("{}", String::from_utf8_lossy(&output.stdout));
        } else {
            println!("{}", String::from_utf8_lossy(&output.stderr).red());
        }
    }

    pub fn handle_flutter_commands(matches: &clap::ArgMatches) {
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
        Self::execute_command(&["pub", "get"]);
    }

    pub fn run_flutter_clean(&self) {
        Self::execute_command(&["clean"]);
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
                thread::sleep(Duration::from_secs(1));

                if do_build_runner {
                    println!("{}: {}", "Running build_runner for module".green(), module);
                    Self::run_build_command(Some(&module));
                }
            }
        }

        println!("{}", "Proses pub get selesai.".cyan());
    }

    fn run_get_command(scope: Option<&String>) {
        let mut command = StdCommand::new("melos");
        command.arg("exec");
        if let Some(scope) = scope {
            command.arg("--scope").arg(scope);
        }
        command
            .arg("--")
            .arg(Self::flutter_command())
            .arg("pub")
            .arg("get");

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
                thread::sleep(Duration::from_secs(1));

                if do_flutter_get {
                    println!(
                        "{}: {}",
                        "Running flutter pub get for module".green(),
                        module
                    );
                    Self::run_get_command(Some(&module));
                    thread::sleep(Duration::from_secs(1));
                }

                if do_build_runner {
                    println!("{}: {}", "Running build_runner for module".green(), module);
                    Self::run_build_command(Some(&module));
                    thread::sleep(Duration::from_secs(1));
                }
            }
        }

        println!("{}", "Proses clear selesai.".cyan());
    }

    fn run_clear_command(scope: Option<&String>) {
        let mut command = StdCommand::new("melos");
        command.arg("exec");
        if let Some(scope) = scope {
            command.arg("--scope").arg(scope);
        }
        command.arg("--").arg(Self::flutter_command()).arg("clean");

        let status = command
            .status()
            .expect("Gagal menjalankan perintah flutter clean");

        if !status.success() {
            eprintln!("{}", "Clear gagal.".red());
        } else {
            println!("{}", "Flutter clean berhasil.".green());
            if let Some(scope_name) = scope {
                if let Some(module_path) = Self::get_module_path_from_melos(scope_name) {
                    let pubspec_lock_path = module_path.join("pubspec.lock");
                    if fs::metadata(&pubspec_lock_path).is_ok() {
                        if fs::remove_file(&pubspec_lock_path).is_ok() {
                            println!(
                                "{}: {}",
                                "Berhasil menghapus pubspec.lock di modul".green(),
                                pubspec_lock_path.display()
                            );
                        } else {
                            println!(
                                "{}: {}",
                                "Gagal menghapus pubspec.lock di modul".yellow(),
                                pubspec_lock_path.display()
                            );
                        }
                    } else {
                        println!(
                            "{}: {}",
                            "File pubspec.lock tidak ditemukan di modul".yellow(),
                            pubspec_lock_path.display()
                        );
                    }
                } else {
                    println!(
                        "{}: {}",
                        "Direktori modul tidak ditemukan untuk modul".red(),
                        scope_name
                    );
                }
            }
        }
    }

    fn get_module_path_from_melos(module_name: &str) -> Option<PathBuf> {
        let output = StdCommand::new("melos")
            .arg("list")
            .arg("--json")
            .output()
            .expect("Gagal menjalankan perintah melos list");

        if !output.status.success() {
            eprintln!(
                "{}: {}",
                "Melos list gagal dijalankan".red(),
                String::from_utf8_lossy(&output.stderr)
            );
            return None;
        }

        let raw_output = String::from_utf8_lossy(&output.stdout);
        let end_of_json = raw_output.rfind(']');

        if let Some(pos) = end_of_json {
            let valid_json = &raw_output[..=pos];
            match serde_json::from_str::<Vec<Module>>(valid_json) {
                Ok(modules) => {
                    for module in modules {
                        if module.name.trim().to_lowercase() == module_name.trim().to_lowercase() {
                            let module_path = PathBuf::from(module.location);
                            return Some(module_path);
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Error parsing Melos JSON output: {}", e);
                    eprintln!("Cleaned output: {}", valid_json);
                }
            }
        } else {
            eprintln!("No valid JSON found in output.");
        }

        None
    }
    pub fn flutter_build_runner() {
        if let Some(mut selected_modules) = ModuleManager::select_modules() {
            // Inisialisasi grup prioritas
            let mut common_modules: Vec<(usize, String)> = Vec::new();
            let mut domain_modules: Vec<(usize, String)> = Vec::new();
            let mut data_modules: Vec<(usize, String)> = Vec::new();
            let mut presentation_modules: Vec<(usize, String)> = Vec::new();

            // Kelompokkan modul berdasarkan prioritas
            for (index, module) in selected_modules.drain() {
                if module.contains("common") {
                    common_modules.push((index, module));
                } else if module.contains("domain") {
                    domain_modules.push((index, module));
                } else if module.contains("data") {
                    data_modules.push((index, module));
                } else {
                    presentation_modules.push((index, module));
                }
            }

            // Urutkan berdasarkan prioritas: common -> domain -> data -> presentation/feature
            let prioritized_modules = common_modules
                .into_iter()
                .chain(domain_modules.into_iter())
                .chain(data_modules.into_iter())
                .chain(presentation_modules.into_iter())
                .collect::<Vec<_>>();

            // Jalankan build untuk setiap modul berdasarkan prioritas
            for (_, module) in prioritized_modules {
                println!("{}: {}", "Building module".green(), module);
                Self::run_build_command(Some(&module));
            }
        }

        println!("{}", "Proses build selesai.".cyan());
    }

    fn run_build_command(scope: Option<&String>) {
        let mut command = StdCommand::new("melos");
        command.arg("exec");
        if let Some(scope) = scope {
            command.arg("--scope").arg(scope);
        }
        command
            .arg("--")
            .arg(Self::flutter_command())
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
