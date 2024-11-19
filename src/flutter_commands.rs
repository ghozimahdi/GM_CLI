use crate::module_manager::ModuleManager;
use crate::{flutter_gen_runner, gen_easy_localization};
use colored::Colorize;
use dialoguer::{theme::ColorfulTheme, Select};
use std::path::Path;
use std::process::Command as StdCommand;
use std::{thread, time::Duration};

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

            // Cari path modul berdasarkan nama scope
            if let Some(scope_name) = scope {
                let module_path = Self::find_module_directory(scope_name);
                if let Some(module_path) = module_path {
                    let pubspec_lock_path = module_path.join("pubspec.lock");
                    if std::fs::metadata(&pubspec_lock_path).is_ok() {
                        if std::fs::remove_file(&pubspec_lock_path).is_ok() {
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

    fn find_module_directory(module_name: &str) -> Option<std::path::PathBuf> {
        let current_dir = std::env::current_dir().expect("Gagal mendapatkan direktori saat ini");
        let entries = std::fs::read_dir(current_dir).expect("Gagal membaca direktori saat ini");

        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_dir() && path.file_name().map_or(false, |name| name == module_name) {
                    return Some(path);
                }
            }
        }
        None
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
