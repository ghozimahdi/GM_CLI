use colored::Colorize;
use indoc::indoc;
use std::fs;
use std::fs::File;
use std::io::Write;

pub struct CreateModuleProjectFlutter;

impl CreateModuleProjectFlutter {
    pub fn build() {
        println!("{}", "Configuring other modules...".yellow());
        // Data
        Self::create_module("data_auth", "./packages/data/data_auth", vec![], vec![]);
        Self::create_module("data_common", "./packages/data/data_common", vec![], vec![]);

        // Domain
        Self::create_module(
            "domain_auth",
            "./packages/domain/domain_auth",
            vec![],
            vec![],
        );

        Self::create_module(
            "domain_common",
            "./packages/domain/domain_common",
            vec![],
            vec![],
        );

        // Presentation
        Self::create_module(
            "feature_common",
            "./packages/presentation/feature_common",
            vec![],
            vec![],
        );
        Self::create_module(
            "feature_login",
            "./packages/presentation/feature_login",
            vec![],
            vec![],
        );

        // Library
        Self::create_module("core", "./packages/library/core", vec![], vec![]);
        Self::create_module(
            "design_system",
            "./packages/library/design_system",
            vec![],
            vec![],
        );
    }

    fn create_module(
        module_name: &str,
        module_path: &str,
        dependencies: Vec<&str>,
        dev_dependencies: Vec<&str>,
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

        for dependency in dependencies {
            pubspec_content.push_str(&format!("  {}\n", dependency));
        }

        if !dev_dependencies.is_empty() {
            pubspec_content.push_str("\ndev_dependencies:\n");
            for dev_dependency in dev_dependencies {
                pubspec_content.push_str(&format!("  {}\n", dev_dependency));
            }
        }

        let pubspec_path = format!("{}/pubspec.yaml", module_path);
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
}
