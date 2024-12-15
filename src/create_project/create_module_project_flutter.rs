use crate::command::Command;
use colored::Colorize;
use indoc::indoc;
use std::fs::File;
use std::io::Write;
use std::{fs, io};

pub struct CreateModuleProjectFlutter;

impl CreateModuleProjectFlutter {
    pub fn build() {
        println!("{}", "Configuring other modules...".yellow());
        // Library
        Self::create_module(
            "core",
            "./packages/library/core",
            vec!["envied", "easy_localization", "injectable"],
            vec![
                "injectable_generator",
                "build_runner",
                "lint",
                "envied_generator",
            ],
            vec![],
        );
        Self::create_module(
            "resource",
            "./packages/library/resource",
            vec![
                "flutter_screenutil",
                "freezed_annotation",
                "easy_localization",
                "flutter_svg",
                "lottie",
                "json_annotation",
            ],
            vec![
                "json_serializable",
                "injectable_generator",
                "build_runner",
                "lint",
                "freezed",
                "flutter_gen_runner",
            ],
            vec![],
        );

        // Domain
        Self::create_module(
            "domain_auth",
            "./packages/domain/domain_auth",
            vec!["injectable", "get_it", "freezed_annotation"],
            vec!["lint", "injectable_generator", "build_runner", "freezed"],
            vec![("core", "../../library/core")],
        );
        Self::create_module(
            "domain_common",
            "./packages/domain/domain_common",
            vec!["injectable", "get_it", "freezed_annotation"],
            vec!["lint", "injectable_generator", "build_runner", "freezed"],
            vec![("core", "../../library/core")],
        );

        // Data
        Self::create_module(
            "data_auth",
            "./packages/data/data_auth",
            vec!["injectable", "get_it"],
            vec![
                "lint",
                "json_serializable",
                "injectable_generator",
                "build_runner",
            ],
            vec![
                ("core", "../../library/core"),
                ("domain_auth", "../../domain/domain_auth"),
            ],
        );
        Self::create_module(
            "data_common",
            "./packages/data/data_common",
            vec!["dio", "chucker_flutter", "injectable", "get_it"],
            vec![
                "lint",
                "json_serializable",
                "injectable_generator",
                "build_runner",
            ],
            vec![
                ("core", "../../library/core"),
                ("domain_common", "../../domain/domain_common"),
            ],
        );

        // Presentation
        Self::create_module(
            "feature_common",
            "./packages/presentation/feature_common",
            vec![
                "flutter_screenutil",
                "freezed_annotation",
                "flutter_bloc",
                "get_it",
                "injectable",
                "rxdart",
                "auto_route",
                "fluttertoast",
                "shimmer",
                "permission_handler",
                "flutter_svg",
            ],
            vec![
                "auto_route_generator",
                "injectable_generator",
                "build_runner",
                "freezed",
                "lint",
            ],
            vec![
                ("resource", "../../library/resource"),
                ("core", "../../library/core"),
            ],
        );
        Self::create_module(
            "feature_login",
            "./packages/presentation/feature_login",
            vec![
                "flutter_screenutil",
                "freezed_annotation",
                "flutter_bloc",
                "get_it",
                "injectable",
                "auto_route",
            ],
            vec![
                "auto_route_generator",
                "injectable_generator",
                "build_runner",
                "freezed",
                "lint",
            ],
            vec![
                ("resource", "../../library/resource"),
                ("core", "../../library/core"),
            ],
        );

        Self::update_app_pubspec_dependencies(
            vec![
                "get_it",
                "injectable",
                "auto_route",
                "firebase_crashlytics",
                "firebase_core",
                "firebase_messaging",
                "flutter_local_notifications",
                "easy_localization",
                "flutter_flavorizr",
                "change_app_package_name",
            ],
            vec![
                "auto_route_generator",
                "injectable_generator",
                "build_runner",
                "lint",
            ],
            vec![
                ("resource", "../library/resource"),
                ("core", "../library/core"),
                ("domain_common", "../domain/domain_common"),
                ("domain_auth", "../domain/domain_auth"),
                ("data_common", "../data/data_common"),
                ("data_auth", "../data/data_auth"),
                ("feature_common", "../presentation/feature_common"),
                ("feature_login", "../presentation/feature_login"),
            ],
        );
    }

    fn create_module(
        module_name: &str,
        module_path: &str,
        dependencies: Vec<&str>,
        dev_dependencies: Vec<&str>,
        local_dependencies: Vec<(&str, &str)>,
    ) {
        const BAR_LENGTH: usize = 50;
        let steps = vec![
            "Creating directories",
            "Generating pubspec.yaml",
            "Adding dependencies",
            "Adding dev dependencies",
            "Adding default files",
        ];
        let total_steps = steps.len();
        let mut progress = 0.0;

        let mut update_progress = |step: usize| {
            progress = step as f32 / total_steps as f32;
            let filled_length = (progress * BAR_LENGTH as f32).round() as usize;

            let bar = "=".repeat(filled_length).to_string()
                + ">"
                + &" ".repeat(BAR_LENGTH - filled_length);

            let step_text = steps.get(step - 1).unwrap_or(&"Finalizing");

            print!(
                "\r{} {} [{}] {:.0}% - {}{}",
                "Building".blue(),
                module_name.blue(),
                bar,
                progress * 100.0,
                step_text,
                " ".repeat(20)
            );
            io::stdout().flush().unwrap();
        };

        // Step 1: Create directories
        fs::create_dir_all(module_path).expect("Failed to create module directory");
        fs::create_dir_all(format!("{}/lib/src", module_path))
            .expect("Failed to create lib directory");
        fs::create_dir_all(format!("{}/test", module_path))
            .expect("Failed to create test directory");
        update_progress(1);

        // Step 2: Generate pubspec.yaml
        let sdk_version = Self::read_environment_from_main_project();
        let mut pubspec_content = format!(
            indoc! {r#"
            name: {}
            description: "Modular project for {}"
            version: 1.0.0
            publish_to: 'none'

            environment:
              {}

            dependencies:
              flutter:
                sdk: flutter

            "#},
            module_name, module_name, sdk_version
        );

        for (dependency_name, dependency_path) in &local_dependencies {
            pubspec_content.push_str(&format!(
                "  {}:\n    path: {}\n",
                dependency_name, dependency_path
            ));
        }

        if !dev_dependencies.is_empty() {
            pubspec_content.push_str("dev_dependencies:\n");
            for dev_dependency in &dev_dependencies {
                pubspec_content.push_str(&format!("  {}: any\n", dev_dependency));
            }
        }

        let pubspec_path = format!("{}/pubspec.yaml", module_path);
        let mut pubspec_file = File::create(&pubspec_path).expect("Failed to create pubspec.yaml");
        pubspec_file
            .write_all(pubspec_content.as_bytes())
            .expect("Failed to write pubspec.yaml");
        update_progress(2);

        // Step 3: Add dependencies
        Self::add_dependencies(dependencies, module_path, false);
        update_progress(3);

        // Step 4: Add dev dependencies
        Self::add_dependencies(dev_dependencies, module_path, true);
        update_progress(4);

        // Step 5: Add default files
        Self::add_default_files(module_path);
        Self::add_analysis_options(module_path);
        update_progress(5);

        // Clear the progress bar line
        print!("\r{}\r", " ".repeat(BAR_LENGTH + 100)); // Clear the current line
        println!(
            "{}",
            format!("Module {} created successfully!", module_name).green()
        );
    }

    fn add_default_files(module_path: &str) {
        let gitignore_path = format!("{}/.gitignore", module_path);
        let gitignore_content = include_str!("../../assets/.gitignore");
        Command::write_file(&gitignore_path, gitignore_content);
    }

    pub fn add_analysis_options(module_path: &str) {
        let analysis_options_path = format!("{}/analysis_options.yaml", module_path);
        let analysis_options_content = include_str!("../../assets/analysis_options.yaml");
        Command::write_file(&analysis_options_path, analysis_options_content);
    }

    pub fn read_environment_from_main_project() -> String {
        let main_pubspec_path = "./packages/app/pubspec.yaml".to_string();

        let pubspec_content = fs::read_to_string(&main_pubspec_path)
            .expect("Failed to read app module's pubspec.yaml");

        for line in pubspec_content.lines() {
            if line.trim_start().starts_with("sdk:") {
                return line.trim().to_string();
            }
        }

        panic!("No `sdk` environment found in the app module's pubspec.yaml");
    }

    fn update_app_pubspec_dependencies(
        dependencies: Vec<&str>,
        dev_dependencies: Vec<&str>,
        local_dependencies: Vec<(&str, &str)>,
    ) {
        const BAR_LENGTH: usize = 50;
        let steps = vec![
            "Reading app's pubspec.yaml",
            "Updating local dependencies",
            "Writing updated pubspec.yaml",
            "Adding dependencies",
            "Adding dev dependencies",
        ];
        let total_steps = steps.len();
        let mut progress = 0.0;

        let mut update_progress = |step: usize| {
            progress = step as f32 / total_steps as f32;
            let filled_length = (progress * BAR_LENGTH as f32).round() as usize;
            let bar = "=".repeat(filled_length).to_string()
                + ">"
                + &" ".repeat(BAR_LENGTH - filled_length);

            let step_text = steps.get(step - 1).unwrap_or(&"Finalizing");

            print!(
                "\r{} {} [{}] {:.0}% - {}{}",
                "Updating".blue(),
                "App Module".blue(),
                bar,
                progress * 100.0,
                step_text,
                " ".repeat(20)
            );
            io::stdout().flush().unwrap();
        };

        let app_pubspec_path = "./packages/app/pubspec.yaml";
        update_progress(1); // Step 1: Reading app's pubspec.yaml

        let mut pubspec_content =
            fs::read_to_string(app_pubspec_path).expect("Failed to read app's pubspec.yaml");

        if !pubspec_content.contains("dependencies:") {
            pubspec_content.push_str("\ndependencies:\n");
        }

        let flutter_sdk_entry = "  flutter:\n    sdk: flutter";
        if let Some(flutter_index) = pubspec_content.find(flutter_sdk_entry) {
            let before_flutter = &pubspec_content[..flutter_index + flutter_sdk_entry.len()];
            let after_flutter = &pubspec_content[flutter_index + flutter_sdk_entry.len()..];

            let mut local_dependencies_content = String::new();
            for (dependency_name, dependency_path) in &local_dependencies {
                let dependency_entry =
                    format!("  {}:\n    path: {}", dependency_name, dependency_path);
                if !pubspec_content.contains(&dependency_entry) {
                    local_dependencies_content.push_str(&format!("\n{}", dependency_entry));
                }
            }

            pubspec_content = format!(
                "{}\n{}\n{}",
                before_flutter, local_dependencies_content, after_flutter
            );
        }
        update_progress(2); // Step 2: Updating local dependencies

        fs::write(app_pubspec_path, pubspec_content)
            .expect("Failed to write updated app's pubspec.yaml");
        update_progress(3); // Step 3: Writing updated pubspec.yaml

        Self::add_dependencies(dependencies, "./packages/app", false);
        update_progress(4); // Step 4: Adding dependencies

        Self::add_dependencies(dev_dependencies, "./packages/app", true);
        update_progress(5); // Step 5: Adding dev dependencies

        // Clear the progress bar line
        print!("\r{}\r", " ".repeat(BAR_LENGTH + 100)); // Clear the current line
        println!(
            "{}",
            "Updated pubspec.yaml for app with new dependencies.".green()
        );
    }

    fn add_dependencies(dependencies: Vec<&str>, path: &str, is_dev: bool) {
        for dependency in dependencies {
            let mut command = std::process::Command::new("flutter");
            command.arg("pub").arg("add").arg(dependency);

            if is_dev {
                command.arg("--dev");
            }

            let output = command
                .current_dir(path)
                .output()
                .expect("Failed to add dependency");

            if !output.status.success() {
                eprintln!(
                    "Failed to add {} dependency {}: {}",
                    if is_dev { "dev" } else { "regular" },
                    dependency,
                    String::from_utf8_lossy(&output.stderr)
                );
            }
        }
    }
}
