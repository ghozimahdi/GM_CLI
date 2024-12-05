use crate::command::Command;
use colored::Colorize;
use indoc::indoc;
use std::fs;
use std::fs::File;
use std::io::Write;

pub struct CreateModuleProjectFlutter;

impl CreateModuleProjectFlutter {
    pub fn build() {
        println!("{}", "Configuring other modules...".yellow());
        // Library
        Self::create_module(
            "core",
            "./packages/library/core",
            vec![],
            vec!["injectable_generator", "build_runner", "lint"],
            vec![],
        );
        Self::create_module(
            "design_system",
            "./packages/library/design_system",
            vec![
                "flutter_screenutil",
                "freezed_annotation",
                "easy_localization",
            ],
            vec![
                "json_serializable",
                "build_runner",
                "lint",
                "freezed",
                "injectable_generator",
                "flutter_gen_runner",
            ],
            vec![],
        );

        // Domain
        Self::create_module(
            "domain_auth",
            "./packages/domain/domain_auth",
            vec![],
            vec![],
            vec![("core", "../../library/core")],
        );
        Self::create_module(
            "domain_common",
            "./packages/domain/domain_common",
            vec![],
            vec![],
            vec![("core", "../../library/core")],
        );

        // Data
        Self::create_module(
            "data_auth",
            "./packages/data/data_auth",
            vec![],
            vec![],
            vec![
                ("core", "../../library/core"),
                ("domain_auth", "../../domain/domain_auth"),
            ],
        );
        Self::create_module(
            "data_common",
            "./packages/data/data_common",
            vec![],
            vec![],
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
            ],
            vec![
                "auto_route_generator",
                "injectable_generator",
                "build_runner",
                "freezed",
                "lint",
            ],
            vec![
                ("design_system", "../../library/design_system"),
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
                ("design_system", "../../library/design_system"),
                ("core", "../../library/core"),
            ],
        );

        Self::update_app_pubspec_dependencies(
            vec!["get_it", "injectable", "auto_route"],
            vec![
                "auto_route_generator",
                "injectable_generator",
                "build_runner",
                "lint",
            ],
            vec![
                ("design_system", "../library/design_system"),
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
        println!("Creating module: {}", module_name);

        fs::create_dir_all(module_path).expect("Failed to create module directory");
        fs::create_dir_all(format!("{}/lib", module_path)).expect("Failed to create lib directory");
        fs::create_dir_all(format!("{}/test", module_path))
            .expect("Failed to create test directory");

        let sdk_version = Self::read_environment_from_main_project();

        let mut pubspec_content = format!(
            indoc! {r#"
            name: {}
            description: Modular project for {}
            version: 1.0.0

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

        pubspec_content.push_str("\n");

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

        Self::add_dependencies(dependencies, module_path, false);
        Self::add_dependencies(dev_dependencies, module_path, true);
        Self::add_default_files(module_path);
    }

    fn add_default_files(module_path: &str) {
        let gitignore_path = format!("{}/.gitignore", module_path);
        let gitignore_content = include_str!("../../assets/.gitignore");
        Command::write_file(&gitignore_path, gitignore_content);
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
        let app_pubspec_path = "./packages/app/pubspec.yaml";
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

        fs::write(app_pubspec_path, pubspec_content)
            .expect("Failed to write updated app's pubspec.yaml");

        Self::add_dependencies(dependencies, "./packages/app", false);
        Self::add_dependencies(dev_dependencies, "./packages/app", true);

        println!("Updated pubspec.yaml for app with new dependencies.");
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
