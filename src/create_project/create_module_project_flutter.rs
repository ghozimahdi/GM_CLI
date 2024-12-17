use crate::command::Command;
use colored::Colorize;
use indoc::indoc;
use std::fs::File;
use std::io::Write;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;
use std::{fs, io, thread};

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
            "domain_common",
            "./packages/domain/domain_common",
            vec!["injectable", "get_it", "freezed_annotation"],
            vec!["lint", "injectable_generator", "build_runner", "freezed"],
            vec![("core", "../../library/core")],
        );
        Self::create_module(
            "domain_auth",
            "./packages/domain/domain_auth",
            vec!["injectable", "get_it", "freezed_annotation"],
            vec!["lint", "injectable_generator", "build_runner", "freezed"],
            vec![
                ("core", "../../library/core"),
                ("domain_common", "../../domain/domain_common"),
            ],
        );

        // Data
        Self::create_module(
            "data_common",
            "./packages/data/data_common",
            vec![
                "dio",
                "chucker_flutter",
                "injectable",
                "get_it",
                "shared_preferences",
            ],
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
                ("data_common", "../../data/data_common"),
                ("domain_auth", "../../domain/domain_auth"),
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
                ("core", "../../library/core"),
                ("feature_common", "../../presentation/feature_common"),
                ("resource", "../../library/resource"),
            ],
        );
        Self::create_module(
            "feature_dashboard",
            "./packages/presentation/feature_dashboard",
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
                ("core", "../../library/core"),
                ("feature_common", "../../presentation/feature_common"),
                ("resource", "../../library/resource"),
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
                ("feature_dashboard", "../presentation/feature_dashboard"),
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
        let steps: Vec<(&str, Box<dyn FnOnce()>)> = vec![
            (
                "Creating directories",
                Box::new(|| {
                    fs::create_dir_all(module_path).expect("Failed to create module directory");
                    fs::create_dir_all(format!("{}/lib/src", module_path))
                        .expect("Failed to create lib directory");
                    fs::create_dir_all(format!("{}/test", module_path))
                        .expect("Failed to create test directory");
                }),
            ),
            (
                "Generating pubspec.yaml",
                Box::new(|| {
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
                        pubspec_content.push_str("  flutter_test:\n    sdk: flutter\n");
                    }

                    let pubspec_path = format!("{}/pubspec.yaml", module_path);
                    let mut pubspec_file =
                        File::create(&pubspec_path).expect("Failed to create pubspec.yaml");
                    pubspec_file
                        .write_all(pubspec_content.as_bytes())
                        .expect("Failed to write pubspec.yaml");
                }),
            ),
            (
                "Adding dependencies",
                Box::new(|| {
                    Self::add_dependencies(dependencies.clone(), module_path, false);
                }),
            ),
            (
                "Adding dev dependencies",
                Box::new(|| {
                    Self::add_dependencies(dev_dependencies.clone(), module_path, true);
                }),
            ),
            (
                "Adding default files",
                Box::new(|| {
                    Self::add_default_files(module_path);
                    Self::add_analysis_options(module_path);
                }),
            ),
        ];

        Self::run_steps_with_loading(steps);

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
        let steps: Vec<(&str, Box<dyn FnOnce()>)> = vec![
            (
                "Reading app's pubspec.yaml",
                Box::new(|| {
                    thread::sleep(Duration::from_millis(300));
                }),
            ),
            (
                "Updating local dependencies",
                Box::new(|| {
                    let app_pubspec_path = "./packages/app/pubspec.yaml";
                    let mut pubspec_content = fs::read_to_string(app_pubspec_path)
                        .expect("Failed to read app's pubspec.yaml");

                    if !pubspec_content.contains("dependencies:") {
                        pubspec_content.push_str("\ndependencies:\n");
                    }

                    let flutter_sdk_entry = "  flutter:\n    sdk: flutter";
                    if let Some(flutter_index) = pubspec_content.find(flutter_sdk_entry) {
                        let before_flutter =
                            &pubspec_content[..flutter_index + flutter_sdk_entry.len()];
                        let after_flutter =
                            &pubspec_content[flutter_index + flutter_sdk_entry.len()..];

                        let mut local_dependencies_content = String::new();
                        for (dependency_name, dependency_path) in &local_dependencies {
                            let dependency_entry =
                                format!("  {}:\n    path: {}", dependency_name, dependency_path);
                            if !pubspec_content.contains(&dependency_entry) {
                                local_dependencies_content
                                    .push_str(&format!("\n{}", dependency_entry));
                            }
                        }

                        pubspec_content = format!(
                            "{}\n{}\n{}",
                            before_flutter, local_dependencies_content, after_flutter
                        );
                    }

                    fs::write(app_pubspec_path, pubspec_content)
                        .expect("Failed to write updated app's pubspec.yaml");
                }),
            ),
            (
                "Writing updated pubspec.yaml",
                Box::new(|| {
                    thread::sleep(Duration::from_millis(300));
                }),
            ),
            (
                "Adding dependencies",
                Box::new(|| {
                    Self::add_dependencies(dependencies.clone(), "./packages/app", false);
                }),
            ),
            (
                "Adding dev dependencies",
                Box::new(|| {
                    Self::add_dependencies(dev_dependencies.clone(), "./packages/app", true);
                }),
            ),
        ];

        Self::run_steps_with_loading(steps);

        println!(
            "{}",
            "Updated pubspec.yaml for app with new dependencies.".green()
        );
    }

    fn run_steps_with_loading(steps: Vec<(&str, Box<dyn FnOnce()>)>) {
        for (step_text, step_action) in steps {
            Self::run_with_loading(step_text, step_action);
        }
    }

    fn run_with_loading(step_text: &str, step_action: Box<dyn FnOnce()>) {
        let loading_frames = ["⠏", "⠛", "⠹", "⠼", "⠧", "⠇"];
        let is_loading = Arc::new(AtomicBool::new(true));
        let is_loading_clone = Arc::clone(&is_loading);
        let step_text_clone = step_text.to_string();

        let handle = thread::spawn(move || {
            let mut frame_index = 0;
            while is_loading_clone.load(Ordering::Relaxed) {
                print!(
                    "\r{} [{}] {}",
                    "[loading]".yellow(),
                    loading_frames[frame_index].yellow(),
                    step_text_clone.dimmed()
                );
                io::stdout().flush().unwrap();
                thread::sleep(Duration::from_millis(100));
                frame_index = (frame_index + 1) % loading_frames.len();
            }
        });

        step_action();

        is_loading.store(false, Ordering::Relaxed);
        handle.join().unwrap();

        print!("\r{} {}{}", "✓".green(), step_text.bold(), " ".repeat(30));
        println!();
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
