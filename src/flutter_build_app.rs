use colored::Colorize;
use dialoguer::{Input, Select};
use std::fs;

pub struct FlutterBuildApp;

impl FlutterBuildApp {
    pub fn build_app() {
        let platform = Self::select_platform();
        let flavor = Self::select_flavor();
        let version = Self::input_version();
        let entry_point = Self::input_entry_point(&flavor);

        if let Some(entry_point) = entry_point {
            if let Some(version) = version {
                println!("{}: {}", "Setting version to".green(), version);
                Self::set_version(&version);
            }

            match platform.as_str() {
                "android" => Self::build_android(&flavor, &entry_point),
                "ios" => Self::build_ios(&flavor, &entry_point),
                _ => eprintln!("{}", "Platform not supported!".red()),
            }
        } else {
            eprintln!(
                "{}: {}",
                "Entry point not found or not provided for flavor".red(),
                flavor
            );
        }
    }

    fn select_platform() -> String {
        let platforms = &["android", "ios"];
        let selection = Select::with_theme(&dialoguer::theme::ColorfulTheme::default())
            .with_prompt("Select platform")
            .default(0)
            .items(platforms)
            .interact()
            .unwrap();
        platforms[selection].to_string()
    }

    fn select_flavor() -> String {
        let flavors = &["prod", "staging", "dev"];
        let selection = Select::with_theme(&dialoguer::theme::ColorfulTheme::default())
            .with_prompt("Select flavor")
            .default(0)
            .items(flavors)
            .interact()
            .unwrap();
        flavors[selection].to_string()
    }

    fn input_version() -> Option<String> {
        let input: String = Input::with_theme(&dialoguer::theme::ColorfulTheme::default())
            .with_prompt("Enter version (e.g., 1.2.3+123) [Optional]")
            .allow_empty(true)
            .interact_text()
            .unwrap();

        if input.trim().is_empty() {
            None
        } else {
            Some(input)
        }
    }

    fn input_entry_point(flavor: &str) -> Option<String> {
        let input: String = Input::with_theme(&dialoguer::theme::ColorfulTheme::default())
            .with_prompt("Enter entry point file path [Optional]")
            .allow_empty(true)
            .interact_text()
            .unwrap();

        if input.trim().is_empty() {
            Self::find_entry_point(flavor)
        } else {
            let path = input.trim().to_string();
            if fs::metadata(&path).is_ok() {
                println!("{}: {}", "Using provided entry point".green(), path);
                Some(path)
            } else {
                eprintln!("{}: {}", "Provided entry point does not exist".red(), path);
                None
            }
        }
    }

    fn set_version(version: &str) {
        let root_path = ".";
        let pubspec_files = Self::find_pubspec_files(root_path);

        if pubspec_files.is_empty() {
            eprintln!("{}", "No pubspec.yaml files found.".red());
            return;
        }

        for pubspec_path in pubspec_files {
            println!(
                "{}: {}",
                "Updating version in".green(),
                pubspec_path.display()
            );

            let content = fs::read_to_string(&pubspec_path)
                .unwrap_or_else(|_| panic!("Failed to read {}", pubspec_path.display()));

            let new_content = content
                .lines()
                .map(|line| {
                    if line.starts_with("version:") {
                        format!("version: {}", version)
                    } else {
                        line.to_string()
                    }
                })
                .collect::<Vec<_>>()
                .join("\n");

            fs::write(&pubspec_path, new_content)
                .unwrap_or_else(|_| panic!("Failed to write to {}", pubspec_path.display()));

            println!("{}", "Version updated successfully.".green());
        }
    }

    fn find_pubspec_files(root: &str) -> Vec<std::path::PathBuf> {
        let mut pubspec_files = Vec::new();

        if let Ok(entries) = fs::read_dir(root) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    if path.is_dir() {
                        pubspec_files.extend(Self::find_pubspec_files(&path.to_string_lossy()));
                    } else if path
                        .file_name()
                        .map_or(false, |name| name == "pubspec.yaml")
                    {
                        pubspec_files.push(path);
                    }
                }
            }
        }

        pubspec_files
    }

    fn build_android(flavor: &str, entry_point: &str) {
        let mut command = std::process::Command::new("flutter");
        command
            .arg("build")
            .arg("apk")
            .arg("--flavor")
            .arg(flavor)
            .arg("--release")
            .arg("--target")
            .arg(entry_point);

        let status = command
            .status()
            .expect("Failed to execute flutter build apk");
        if status.success() {
            println!("{}", "APK build succeeded.".green());
        } else {
            eprintln!("{}", "APK build failed.".red());
        }
    }

    fn build_ios(flavor: &str, entry_point: &str) {
        let mut command = std::process::Command::new("flutter");
        command
            .arg("build")
            .arg("ios")
            .arg("--flavor")
            .arg(flavor)
            .arg("--release")
            .arg("--target")
            .arg(entry_point);

        let status = command
            .status()
            .expect("Failed to execute flutter build ios");
        if status.success() {
            println!("{}", "IPA build succeeded.".green());
        } else {
            eprintln!("{}", "IPA build failed.".red());
        }
    }

    fn find_entry_point(flavor: &str) -> Option<String> {
        let possible_files = vec![
            format!("lib/main_{}.dart", flavor),
            format!("lib/main_{}.dart", flavor.to_lowercase()),
            format!("lib/main_{}.dart", flavor.to_uppercase()),
            "lib/main.dart".to_string(),
        ];

        for file in possible_files {
            if fs::metadata(&file).is_ok() {
                println!("{}: {}", "Using detected entry point".green(), file);
                return Some(file);
            }
        }

        None
    }
}
