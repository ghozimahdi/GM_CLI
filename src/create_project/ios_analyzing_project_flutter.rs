use colored::Colorize;
use regex::Regex;
use std::fs::{self};
use std::io::Read;
use std::path::Path;
use std::process::Command;

pub struct IOSAnalyzingProjectFlutter;

impl IOSAnalyzingProjectFlutter {
    pub fn build(app_path: &str) {
        println!("Building iOS project at path: {}", app_path);
        Self::run_pod_install_and_fix(app_path);
    }

    pub fn run_pod_install_and_fix(app_path: &str) {
        let podfile_dir = format!("{}/ios", app_path);

        if !Self::validate_podfile_path(&format!("{}/Podfile", podfile_dir)) {
            println!("{}", "Podfile not found. Exiting.".red());
            return;
        }

        println!("{}", "Running `pod install`...".yellow());
        let output = Self::run_in_dir(&["pod", "install"], &podfile_dir);

        if !output.status.success() {
            println!(
                "{}",
                "Initial `pod install` failed. Checking logs for issues...".red()
            );

            let output_str = String::from_utf8_lossy(&output.stderr);

            let max_ios_version = Self::detect_latest_ios_version().unwrap_or(20.0);
            println!(
                "{} {}",
                "Detected maximum iOS version for the system:".cyan(),
                max_ios_version
            );

            let mut current_ios_version =
                if let Some(initial_version) = Self::extract_initial_ios_version(&output_str) {
                    println!(
                        "{} {}",
                        "Detected assigned iOS deployment target:".cyan(),
                        initial_version
                    );
                    initial_version
                } else {
                    println!("{}", "Defaulting to iOS 12.0".yellow());
                    12.0
                };

            while current_ios_version <= max_ios_version {
                current_ios_version += 1.0;
                println!(
                    "{} {}",
                    "Trying `pod install` with iOS deployment target:".yellow(),
                    current_ios_version
                );

                Self::update_ios_deployment_target(&current_ios_version.to_string(), &podfile_dir);

                let retry_output = Self::run_in_dir(&["pod", "install"], &podfile_dir);

                if retry_output.status.success() {
                    println!("{}", "Pod install completed successfully.".green());
                    break;
                } else {
                    println!(
                        "{}",
                        "Pod install failed. Incrementing iOS deployment target and retrying..."
                            .red()
                    );
                    current_ios_version += 1.0;
                }
            }

            if current_ios_version > max_ios_version {
                println!(
                    "{}",
                    "Exceeded maximum iOS deployment target. Please check the logs.".red()
                );
            }
        } else {
            println!("{}", "Pod install completed successfully.".green());
        }
    }

    fn detect_latest_ios_version() -> Option<f32> {
        let output = Command::new("xcrun")
            .args(["--sdk", "iphoneos", "--show-sdk-version"])
            .output()
            .ok()?;

        let sdk_version = String::from_utf8_lossy(&output.stdout);
        sdk_version.trim().parse::<f32>().ok()
    }

    fn extract_initial_ios_version(output: &str) -> Option<f32> {
        let re = Regex::new(r"Automatically assigning platform `iOS` with version `(\d+\.\d+)`")
            .unwrap();

        for line in output.lines() {
            if let Some(captures) = re.captures(line) {
                if let Some(version) = captures.get(1).map(|m| m.as_str().parse::<f32>().ok()) {
                    return version;
                }
            }
        }

        None
    }

    fn run_in_dir(args: &[&str], dir: &str) -> std::process::Output {
        let command = args[0];
        let command_args = &args[1..];

        println!("Running command '{}' in directory: {}", command, dir);

        let output = Command::new(command)
            .args(command_args)
            .current_dir(dir)
            .output()
            .expect("Failed to execute command");

        if !output.status.success() {
            eprintln!(
                "Error: Command '{}' failed with status {}",
                command, output.status
            );
            eprintln!("stderr: {}", String::from_utf8_lossy(&output.stderr));
        } else {
            println!("Command '{}' succeeded.", command);
        }

        output
    }

    fn validate_podfile_path(podfile_path: &str) -> bool {
        println!("Checking for Podfile at: {}", podfile_path);

        if Path::new(podfile_path).exists() {
            println!("Podfile found at: {}", podfile_path);
            true
        } else {
            println!("Podfile not found at: {}", podfile_path);
            false
        }
    }

    fn update_ios_deployment_target(new_version: &str, podfile_dir: &str) {
        let podfile_path = format!("{}/Podfile", podfile_dir);

        if !Path::new(&podfile_path).exists() {
            println!("{}", "Error: Podfile not found.".red());
            return;
        }

        let mut file_content = String::new();
        if let Ok(mut file) = fs::OpenOptions::new().read(true).open(&podfile_path) {
            file.read_to_string(&mut file_content)
                .expect("Failed to read Podfile");
        } else {
            println!("{}", "Error: Could not open Podfile.".red());
            return;
        }

        #[cfg(debug_assertions)]
        println!("Original Podfile content:\n{}", file_content);

        let filtered_content: String = file_content
            .lines()
            .filter(|line| !line.trim_start().starts_with("platform :ios"))
            .filter(|line| !line.trim_start().starts_with("# platform :ios, "))
            .collect::<Vec<_>>()
            .join("\n");

        let updated_content = format!("platform :ios, '{}'\n{}", new_version, filtered_content);

        #[cfg(debug_assertions)]
        println!("Updated Podfile content:\n{}", updated_content);

        if let Err(e) = fs::write(&podfile_path, updated_content) {
            println!("{}: {}", "Error writing to Podfile".red(), e);
        } else {
            println!(
                "{} {}",
                "Deployment target updated to iOS".green(),
                new_version
            );
        }
    }
}
