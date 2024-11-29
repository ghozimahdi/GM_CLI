use colored::Colorize;
use dialoguer::{theme::ColorfulTheme, Select};
use indoc::indoc;
use regex::Regex;
use std::fs::{self, File};
use std::io;
use std::io::Write;
use std::process::Command;

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
            Self::create_modular_structure();

            println!("Generating Flutter project in packages/app...");
            let app_path = "./packages/app";
            Self::run_command(&[
                "flutter",
                "create",
                "--template=app",
                "--org",
                &package_name,
                "--platforms=android,ios",
                app_path,
            ]);

            println!("Setting up Melos...");
            Self::setup_melos(&project_name);

            println!("{}", "Configuring other modules...".yellow());
            Self::create_module("data_auth", "./packages/data/data_auth", vec![], vec![]);
            Self::create_module(
                "domain_auth",
                "./packages/domain/domain_auth",
                vec![],
                vec![],
            );
            Self::create_module(
                "feature_login",
                "./packages/presentation/feature_login",
                vec![],
                vec![],
            );

            println!("{}", "Running `flutter pub get`...".yellow());
            Self::run_command(&["flutter", "pub", "get"]);

            println!("{}", "Bootstrapping with Melos...".yellow());
            Self::run_command(&["melos", "bootstrap"]);

            println!(
                "{}",
                "Flutter modular project generation completed!".green()
            );
        } else {
            println!("Generating simple Flutter project...");
            Self::run_command(&[
                "flutter",
                "create",
                "--org",
                &package_name,
                "--platforms=android,ios",
                ".",
            ]);

            println!("{}", "Running `flutter pub get`...".yellow());
            Self::run_command(&["flutter", "pub", "get"]);

            println!("{}", "Simple Flutter project generation completed!".green());
        }
    }

    fn run_command(command: &[&str]) {
        let status = Command::new(command[0])
            .args(&command[1..])
            .status()
            .expect("Failed to execute command");

        if !status.success() {
            panic!("Command {:?} failed", command);
        }
    }

    fn create_modular_structure() {
        let directories = [
            "packages/app",
            "packages/data/data_auth",
            "packages/domain/domain_auth",
            "packages/presentation/feature_login",
        ];

        for dir in directories.iter() {
            fs::create_dir_all(format!("{}/lib", dir)).expect("Failed to create lib directory");
            fs::create_dir_all(format!("{}/test", dir)).expect("Failed to create test directory");
        }
    }

    fn read_environment_from_main_project() -> String {
        let main_pubspec_path = "./packages/app/pubspec.yaml".to_string();

        let pubspec_content = fs::read_to_string(&main_pubspec_path)
            .expect("Failed to read app module's pubspec.yaml");

        for line in pubspec_content.lines() {
            if line.trim_start().starts_with("sdk:") {
                return line.trim().to_string(); // Ambil baris yang mengandung `sdk`
            }
        }

        panic!("No `sdk` environment found in the app module's pubspec.yaml");
    }

    fn setup_melos(project_name: &str) {
        let sdk_version = Self::read_environment_from_main_project();
        let melos_config = format!("name: {}\npackages:\n  - packages/**\n", project_name);

        let mut melos_file = File::create("melos.yaml").expect("Failed to create melos.yaml");
        melos_file
            .write_all(melos_config.as_bytes())
            .expect("Failed to write melos.yaml");

        let pubspec_content = format!(
            indoc! {r#"
            name: {}
            description: Modular project for Flutter with Melos
            version: 1.0.0

            environment:
              {}

            dev_dependencies:
              melos: any
            "#},
            project_name, sdk_version
        );

        let mut pubspec_file = File::create("pubspec.yaml").expect("Failed to create pubspec.yaml");
        pubspec_file
            .write_all(pubspec_content.as_bytes())
            .expect("Failed to write pubspec.yaml");

        println!("Adding Melos dependency...");
        Self::run_command(&["flutter", "pub", "add", "melos", "--dev"]);
        println!("Upgrading Melos to the latest version...");
        Self::run_command(&["flutter", "pub", "upgrade", "melos"]);
        println!("Running `flutter pub get`...");
        Self::run_command(&["flutter", "pub", "get"]);
    }

    fn create_module(
        module_name: &str,
        module_path: &str,
        dependencies: Vec<&str>,
        dev_dependencies: Vec<&str>,
    ) {
        println!("Creating module: {}", module_name);

        fs::create_dir_all(module_path).expect("Failed to create module directory");

        let pubspec_path = format!("{}/pubspec.yaml", module_path);
        let mut pubspec_content = format!(
            "name: {}\ndescription: A module for {}.\nversion: 1.0.0\n\nenvironment:\n  sdk: \">=3.0.0 <4.0.0\"\n\ndependencies:\n  flutter:\n    sdk: flutter\n",
            module_name, module_name
        );

        for dependency in dependencies {
            pubspec_content.push_str(&format!("  {}\n", dependency));
        }

        if !dev_dependencies.is_empty() {
            pubspec_content.push_str("\ndev_dependencies:\n");
            for dev_dependency in dev_dependencies {
                pubspec_content.push_str(&format!("  {}\n", dev_dependency));
            }
        }

        let mut pubspec_file = File::create(&pubspec_path).expect("Failed to create pubspec.yaml");
        pubspec_file
            .write_all(pubspec_content.as_bytes())
            .expect("Failed to write pubspec.yaml");

        Self::add_default_files(module_path);
    }

    fn add_default_files(module_path: &str) {
        let gitignore_content = "/build/\n/.dart_tool/\n/.idea/\n.packages\n";
        let gitignore_path = format!("{}/.gitignore", module_path);
        let mut gitignore_file =
            File::create(&gitignore_path).expect("Failed to create .gitignore");
        gitignore_file
            .write_all(gitignore_content.as_bytes())
            .expect("Failed to write .gitignore");
    }
}
