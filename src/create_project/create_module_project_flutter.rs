use crate::command::Command;
use crate::create_project::create_single_module_project_flutter::CreateSingleModuleProjectFlutter;
use colored::Colorize;
use include_dir::{include_dir, Dir};
use std::fs::File;
use std::io::Write;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;
use std::{fs, io, thread};
// TODO 1: create module common
// TODO 2: create default files for module common
// TODO 3: change localization with l10n

static ASSETS: Dir = include_dir!("assets");

pub struct CreateModuleProjectFlutter;

impl CreateModuleProjectFlutter {
    pub fn build() {
        println!("{}", "Configuring other modules...".yellow());
        // Library
        Self::create_module(
            "common",
            "./packages/library/common",
            vec!["injectable", "async:^2.11.0"],
            vec!["injectable_generator", "build_runner", "lint"],
            vec![],
        );

        // Domain
        Self::create_module(
            "domain_common",
            "./packages/domain/domain_common",
            vec!["injectable", "get_it", "freezed_annotation"],
            vec!["lint", "injectable_generator", "build_runner", "freezed"],
            vec![("common", "../../library/common")],
        );
        Self::create_module(
            "domain_auth",
            "./packages/domain/domain_auth",
            vec!["injectable", "get_it", "freezed_annotation"],
            vec!["lint", "injectable_generator", "build_runner", "freezed"],
            vec![
                ("common", "../../library/common"),
                ("domain_common", "../../domain/domain_common"),
            ],
        );

        // Data
        Self::create_module(
            "data_common",
            "./packages/data/data_common",
            vec![
                "dio",
                "retrofit",
                "chucker_flutter",
                "freezed_annotation",
                "json_annotation",
                "injectable",
                "get_it",
                "shared_preferences",
            ],
            vec![
                "lint",
                "retrofit_generator",
                "json_serializable",
                "injectable_generator",
                "freezed",
                "build_runner",
            ],
            vec![
                ("common", "../../library/common"),
                ("domain_common", "../../domain/domain_common"),
            ],
        );
        Self::create_module(
            "data_auth",
            "./packages/data/data_auth",
            vec![
                "dio",
                "retrofit",
                "injectable",
                "get_it",
                "freezed_annotation",
                "json_annotation",
            ],
            vec![
                "lint",
                "retrofit_generator",
                "json_serializable",
                "injectable_generator",
                "freezed",
                "build_runner",
            ],
            vec![
                ("common", "../../library/common"),
                ("data_common", "../../data/data_common"),
                ("domain_auth", "../../domain/domain_auth"),
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
                "intl",
                "lottie:^3.3.0",
                "json_annotation",
            ],
            vec![
                "json_serializable",
                "auto_route_generator",
                "injectable_generator",
                "build_runner",
                "freezed",
                "lint",
                "intl_utils",
                "flutter_gen_runner",
            ],
            vec![
                ("domain_common", "../../domain/domain_common"),
                ("common", "../../library/common"),
            ],
        );
        Self::create_module(
            "feature_auth",
            "./packages/presentation/feature_auth",
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
                ("common", "../../library/common"),
                ("feature_common", "../../presentation/feature_common"),
                ("domain_common", "../../domain/domain_common"),
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
                ("common", "../../library/common"),
                ("feature_common", "../../presentation/feature_common"),
                ("domain_common", "../../domain/domain_common"),
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
                "freezed_annotation",
                "flutter_flavorizr",
                "change_app_package_name",
                "flutter_bloc",
            ],
            vec![
                "auto_route_generator",
                "injectable_generator",
                "build_runner",
                "freezed",
                "lint",
            ],
            vec![
                ("common", "../../library/common"),
                ("domain_common", "../domain/domain_common"),
                ("domain_auth", "../domain/domain_auth"),
                ("data_common", "../data/data_common"),
                ("data_auth", "../data/data_auth"),
                ("feature_common", "../presentation/feature_common"),
                ("feature_auth", "../presentation/feature_auth"),
                ("feature_dashboard", "../presentation/feature_dashboard"),
            ],
        );
    }

    pub fn create_module(
        module_name: &str,
        module_path: &str,
        dependencies: Vec<&str>,
        dev_dependencies: Vec<&str>,
        local_dependencies: Vec<(&str, &str)>,
    ) {
        let packages = module_path.trim_start_matches("./").to_string();
        CreateSingleModuleProjectFlutter::update_melos_config(&packages);

        let module_name = module_name.to_string();
        let module_path = module_path.to_string();
        let dependencies = dependencies
            .iter()
            .map(|&s| s.to_string())
            .collect::<Vec<_>>();
        let dev_dependencies = dev_dependencies
            .iter()
            .map(|&s| s.to_string())
            .collect::<Vec<_>>();
        let local_dependencies = local_dependencies
            .iter()
            .map(|&(k, v)| (k.to_string(), v.to_string()))
            .collect::<Vec<_>>();

        let steps: Vec<(&str, Box<dyn FnOnce() + Send>)> = vec![
            (
                "Creating directories",
                Box::new({
                    let module_path = module_path.clone();
                    move || {
                        fs::create_dir_all(&module_path)
                            .expect("Failed to create module directory");
                        fs::create_dir_all(format!("{}/lib/src", module_path))
                            .expect("Failed to create lib directory");
                        fs::create_dir_all(format!("{}/test", module_path))
                            .expect("Failed to create test directory");
                    }
                }),
            ),
            (
                "Generating pubspec.yaml",
                Box::new({
                    let module_name = module_name.clone();
                    let module_path = module_path.clone();
                    let dev_dependencies = dev_dependencies.clone();

                    let local_dependencies = local_dependencies.clone();
                    move || {
                        let sdk_version = Self::read_environment_from_main_project();
                        let template_content = ASSETS
                            .get_file("pubspec_template.yaml")
                            .expect("Failed to read pubspec template from assets")
                            .contents_utf8()
                            .expect("Failed to convert pubspec template to UTF-8");

                        let mut pubspec_content = template_content
                            .replace("{{module_name}}", &module_name)
                            .replace("{{sdk_version}}", &sdk_version);

                        for (dependency_name, dependency_path) in &local_dependencies {
                            pubspec_content.push_str(&format!(
                                "  {}:\n    path: {}\n",
                                dependency_name, dependency_path
                            ));
                        }

                        if !dev_dependencies.is_empty() {
                            pubspec_content.push_str("\ndev_dependencies:\n");
                            pubspec_content.push_str("  flutter_test:\n    sdk: flutter\n");
                        }

                        let pubspec_path = format!("{}/pubspec.yaml", module_path);
                        let mut pubspec_file =
                            File::create(&pubspec_path).expect("Failed to create pubspec.yaml");
                        pubspec_file
                            .write_all(pubspec_content.as_bytes())
                            .expect("Failed to write pubspec.yaml");
                    }
                }),
            ),
            (
                "Adding dependencies",
                Box::new({
                    let dependencies = dependencies.clone();
                    let module_name = module_name.clone();
                    move || {
                        Self::add_dependencies(dependencies, &module_name, false);
                    }
                }),
            ),
            (
                "Adding dev dependencies",
                Box::new({
                    let dev_dependencies = dev_dependencies.clone();
                    let module_name = module_name.clone();
                    move || {
                        Self::add_dependencies(dev_dependencies, &module_name, true);
                    }
                }),
            ),
            (
                "Adding default files",
                Box::new({
                    let module_path = module_path.clone();
                    move || {
                        Self::add_default_files(&module_path);
                        Self::add_analysis_options(&module_path);
                    }
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
        let main_pubspec_path = "./app/pubspec.yaml".to_string();

        let pubspec_content = fs::read_to_string(&main_pubspec_path)
            .expect("Failed to read app module's pubspec.yaml");

        for line in pubspec_content.lines() {
            if line.trim_start().starts_with("sdk:") {
                return line.trim().to_string();
            }
        }

        panic!("No `sdk` environment found in the app module's pubspec.yaml");
    }

    pub fn update_app_pubspec_dependencies(
        dependencies: Vec<&str>,
        dev_dependencies: Vec<&str>,
        local_dependencies: Vec<(&str, &str)>,
    ) {
        let app_module = "app";

        let dependencies = dependencies
            .iter()
            .map(|&s| s.to_string())
            .collect::<Vec<_>>();
        let dev_dependencies = dev_dependencies
            .iter()
            .map(|&s| s.to_string())
            .collect::<Vec<_>>();
        let local_dependencies = local_dependencies
            .iter()
            .map(|&(k, v)| (k.to_string(), v.to_string()))
            .collect::<Vec<_>>();

        let steps: Vec<(&str, Box<dyn FnOnce() + Send>)> = vec![
            (
                "Reading app's pubspec.yaml",
                Box::new(move || {
                    thread::sleep(Duration::from_millis(300));
                }),
            ),
            (
                "Updating local dependencies",
                Box::new({
                    let local_dependencies = local_dependencies.clone();
                    move || {
                        let app_pubspec_path = "./app/pubspec.yaml";
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
                                let dependency_entry = format!(
                                    "  {}:\n    path: {}",
                                    dependency_name, dependency_path
                                );
                                if !pubspec_content.contains(&dependency_entry) {
                                    local_dependencies_content
                                        .push_str(&format!("\n{}", dependency_entry));
                                }
                            }

                            pubspec_content = format!(
                                "{}{}{}",
                                before_flutter.trim_end(),
                                local_dependencies_content.trim_end(),
                                after_flutter
                            );
                        }

                        fs::write(app_pubspec_path, pubspec_content)
                            .expect("Failed to write updated app's pubspec.yaml");
                    }
                }),
            ),
            (
                "Writing updated pubspec.yaml",
                Box::new(move || {
                    thread::sleep(Duration::from_millis(300));
                }),
            ),
            (
                "Adding dependencies",
                Box::new({
                    let dependencies = dependencies.clone();
                    move || {
                        Self::add_dependencies(dependencies, app_module, false);
                    }
                }),
            ),
            (
                "Adding dev dependencies",
                Box::new({
                    let dev_dependencies = dev_dependencies.clone();
                    move || {
                        Self::add_dependencies(dev_dependencies, app_module, true);
                    }
                }),
            ),
        ];

        Self::run_steps_with_loading(steps);

        println!(
            "{}",
            "Updated pubspec.yaml for app with new dependencies.".green()
        );
    }

    pub fn run_steps_with_loading(steps: Vec<(&str, Box<dyn FnOnce() + Send>)>) {
        for (step_text, step_action) in steps {
            Self::run_with_loading(step_text, step_action);
        }
    }

    fn run_with_loading(step_text: &str, step_action: Box<dyn FnOnce() + Send>) {
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

    fn add_dependencies(dependencies: Vec<String>, module: &str, is_dev: bool) {
        for dependency in dependencies {
            let pub_command = if is_dev {
                format!("pub add {} --dev", dependency)
            } else {
                format!("pub add {}", dependency)
            };

            let mut command = std::process::Command::new("melos");
            command
                .arg("exec")
                .arg("--scope")
                .arg(module)
                .arg("--")
                .arg("flutter");

            for arg in pub_command.split_whitespace() {
                command.arg(arg);
            }

            let output = command
                .output()
                .expect("Failed to add dependency using melos");

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
