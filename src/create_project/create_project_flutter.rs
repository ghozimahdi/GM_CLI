use crate::command::Command;
use crate::create_project::create_file_project_flutter::CreateFileProjectFlutter;
use crate::create_project::create_module_project_flutter::CreateModuleProjectFlutter;
use crate::create_project::setup_fvm_project_flutter::SetupFVMProjectFlutter;
use crate::create_project::setup_melos_project_flutter::SetupMelosProjectFlutter;
use colored::Colorize;
use dialoguer::{theme::ColorfulTheme, Select};
use regex::Regex;
use std::fs::{self};
use std::io;
use std::io::Write;

pub struct CreateProjectFlutter;

impl CreateProjectFlutter {
    pub fn build() {
        let mut project_name = String::new();
        print!("Enter the project name: ");
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut project_name)
            .expect("Failed to read project name");

        let trimmed_project_name = project_name.trim().to_lowercase();
        let mut project_name: String = trimmed_project_name.clone();

        if project_name.is_empty() {
            eprintln!("Error: Project name cannot be empty.");
            std::process::exit(1);
        }

        project_name = project_name.replace(' ', "_");

        let project_name_re = Regex::new(r"^[a-z0-9_]+$").unwrap();
        if !project_name_re.is_match(&project_name) {
            eprintln!("Error: Project name can only contain letters, numbers, and underscores.");
            std::process::exit(1);
        }

        let mut package_name = String::new();
        print!("Enter the package name: ");
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut package_name)
            .expect("Failed to read package name");

        let trimmed_package_name = package_name.trim().to_lowercase();
        let package_name: String = trimmed_package_name.clone();

        let package_name_re = Regex::new(r"^[a-z][a-z0-9_]*(\.[a-z][a-z0-9_]*)*$").unwrap();
        if !package_name_re.is_match(&package_name) {
            eprintln!(
                "Error: Package name must follow Dart package name rules (e.g., com.example.app)."
            );
            std::process::exit(1);
        }

        println!("Validated project name: {}", project_name);
        println!("Validated package name: {}", package_name);

        let choices = &["Yes", "No"];
        let modular_selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Do you want to use a modular structure?")
            .default(0)
            .items(choices)
            .interact()
            .unwrap();

        let modular = modular_selection == 0;

        println!("Creating project folder: {}", project_name);
        fs::create_dir_all(&project_name).expect("Failed to create project folder");

        std::env::set_current_dir(&project_name).expect("Failed to navigate to project folder");

        if modular {
            println!("Creating modular structure...");
            fs::create_dir_all("packages/app").expect("Failed to create app directory");

            println!("Generating Flutter project in packages/app...");
            let app_path = "./packages/app";
            Command::run(&[
                "flutter",
                "create",
                "--template=app",
                "--org",
                &package_name,
                "--platforms=android,ios",
                app_path,
            ]);

            println!("Setting up Melos...");
            SetupMelosProjectFlutter::build(&project_name);

            println!("Setting up FVM...");
            SetupFVMProjectFlutter::build();

            CreateModuleProjectFlutter::build();
            CreateModuleProjectFlutter::add_analysis_options(app_path);
            CreateFileProjectFlutter::build(&project_name, &package_name);

            println!("{}", "Running `flutter pub get`...".yellow());
            Command::run(&["fvm", "flutter", "pub", "get"]);

            println!("{}", "Bootstrapping with Melos...".yellow());
            Command::run(&["melos", "bootstrap"]);
            Command::run(&["melos", "locale"]);
            Command::run(&["melos", "build"]);
            Command::run(&["melos", "flavors"]);

            println!(
                "{}",
                "Flutter modular project generation completed!".green()
            );
        } else {
            println!("Generating simple Flutter project...");
            Command::run(&[
                "flutter",
                "create",
                "--org",
                &package_name,
                "--platforms=android,ios",
                ".",
            ]);

            println!("{}", "Running `flutter pub get`...".yellow());
            Command::run(&["fvm", "flutter", "pub", "get"]);

            println!("{}", "Simple Flutter project generation completed!".green());
        }
    }
}
