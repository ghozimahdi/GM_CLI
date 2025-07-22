use crate::module_manager::ModuleManager;
use crate::{flutter_gen_runner, gen_localization};
use colored::Colorize;
use dialoguer::{theme::ColorfulTheme, Select};
use std::path::Path;
use std::process::Command as StdCommand;
use std::{thread, time::Duration};
pub struct FlutterCommandHandler;

impl FlutterCommandHandler {
    fn flutter_command() -> &'static str {
        if Path::new(".fvm").exists() {
            "fvm dart"
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
            gen_localization::intl_localization();
        }
        if matches.get_flag("assets") {
            flutter_gen_runner::gen_assets();
        }
        if matches.get_flag("all") {
            Self::flutter_build_runner();
            gen_localization::intl_localization();
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
                .with_prompt("Do you want to run build_runner after pub get?")
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

        println!("{}", "Pub get process completed.".cyan());
    }

    fn run_get_command(scope: Option<&String>) {
        let mut command = StdCommand::new("fvm");
        command.arg("dart").arg("run").arg("melos");
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
            .expect("Failed to run flutter pub get command");
        if !status.success() {
            eprintln!("{}", "Pub get failed.".red());
        }
    }

    pub fn flutter_pub(pub_command: &str) {
        if let Some(selected_modules) = ModuleManager::select_modules() {
            for (index, module) in selected_modules {
                println!(
                    "{}: Running `flutter {}` in module `{}`...",
                    format!("[{}]", index).cyan(),
                    pub_command.green(),
                    module.yellow()
                );

                let mut command = StdCommand::new("fvm");
                command.arg("dart").arg("run").arg("melos");
                command
                    .arg("exec")
                    .arg("--scope")
                    .arg(&module)
                    .arg("--")
                    .arg("flutter")
                    .arg(pub_command);

                let status = command.status().expect("Failed to execute melos command");

                if status.success() {
                    println!(
                        "{}",
                        format!("Command executed successfully in `{}`.", module).green()
                    );
                } else {
                    eprintln!(
                        "{}",
                        format!("Error executing command in `{}`.", module).red()
                    );
                }

                thread::sleep(Duration::from_secs(1));
            }
        }

        println!(
            "{}",
            "Flutter pub process completed for all selected modules.".cyan()
        );
    }

    pub fn flutter_clear() {
        if let Some(selected_modules) = ModuleManager::select_modules() {
            let choices = &["Yes", "No"];
            let pub_get_selection = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Do you want to run flutter pub get after the clean process?")
                .default(0)
                .items(choices)
                .interact()
                .unwrap();

            let do_flutter_get = pub_get_selection == 0;

            let build_runner_selection = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Do you want to run build_runner after the clean process?")
                .default(0)
                .items(choices)
                .interact()
                .unwrap();

            let do_build_runner = build_runner_selection == 0;

            for (_, module) in selected_modules {
                println!("{}: {}", "Clearing module".green(), module);
                Self::run_clear_command(&module);
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

        println!("{}", "Clear process completed.".cyan());
    }

    fn run_clear_command(scope: &str) {
        let mut command = StdCommand::new("fvm");
        command.arg("dart").arg("run").arg("melos");
        command
            .arg("exec")
            .arg("--scope")
            .arg(scope)
            .arg("--")
            .arg("flutter")
            .arg("clean");

        let status = command
            .status()
            .expect("Failed to run flutter clean command");

        if !status.success() {
            eprintln!("{}", "Clear failed.".red());
        } else {
            println!(
                "{}",
                format!("Flutter clean successful for module: {}", scope).green()
            );
        }
    }

    pub fn flutter_build_runner() {
        if let Some(mut selected_modules) = ModuleManager::select_modules() {
            // Initialize priority groups
            let mut domain_common_modules: Vec<(usize, String)> = Vec::new();
            let mut common_modules: Vec<(usize, String)> = Vec::new();
            let mut domain_modules: Vec<(usize, String)> = Vec::new();
            let mut feature_common_modules: Vec<(usize, String)> = Vec::new();
            let mut data_modules: Vec<(usize, String)> = Vec::new();
            let mut presentation_modules: Vec<(usize, String)> = Vec::new();

            // Categorize modules based on priority
            for (index, module) in selected_modules.drain() {
                if module.contains("common") && module.contains("domain") {
                    domain_common_modules.push((index, module));
                } else if module.contains("common") {
                    common_modules.push((index, module));
                } else if module.contains("domain") {
                    domain_modules.push((index, module));
                } else if module.contains("feature_common") {
                    feature_common_modules.push((index, module));
                } else if module.contains("data") {
                    data_modules.push((index, module));
                } else {
                    presentation_modules.push((index, module));
                }
            }

            // Sort by priority: common -> domain -> feature_common -> data -> presentation/feature
            let prioritized_modules = domain_common_modules
                .into_iter()
                .chain(common_modules.into_iter())
                .chain(domain_modules.into_iter())
                .chain(data_modules.into_iter())
                .chain(feature_common_modules.into_iter()) // Add feature_common after domain_modules
                .chain(presentation_modules.into_iter())
                .collect::<Vec<_>>();

            // Run build for each module based on priority
            for (_, module) in prioritized_modules {
                println!("{}: {}", "Building module".green(), module);
                Self::run_build_command(Some(&module));
            }
        }

        println!("{}", "Build process completed.".cyan());
    }

    fn run_build_command(scope: Option<&String>) {
        let mut command = StdCommand::new("fvm");
        command.arg("dart").arg("run").arg("melos");
        command.arg("exec");
        if let Some(scope) = scope {
            command.arg("--scope").arg(scope);
        }
        command
            .arg("--")
            .arg(Self::flutter_command())
            .arg("run")
            .arg("build_runner")
            .arg("build")
            .arg("-d");

        let status = command.status().expect("Failed to run build command");
        if !status.success() {
            eprintln!("{}", "Build failed.".red());
        }
    }
}
