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
        let mut output = Self::run_in_dir(&["pod", "install"], &podfile_dir);

        if !output.status.success() {
            println!("{}", "Retrying `pod install` with repo update...".yellow());
            output = Self::run_in_dir(&["pod", "install", "--repo-update"], &podfile_dir);

            if !output.status.success() {
                println!(
                    "{}",
                    "Final `pod install` failed. Checking logs for issues...".red()
                );

                let output_str = String::from_utf8_lossy(&output.stderr);

                if let Some(required_ios_version) = Self::extract_required_ios_version(&output_str)
                {
                    println!(
                        "{} {}",
                        "Detected iOS deployment target requirement:".cyan(),
                        required_ios_version
                    );

                    Self::update_ios_deployment_target(&required_ios_version, &podfile_dir);

                    println!(
                        "{}",
                        "Retrying `pod install` after updating deployment target...".yellow()
                    );

                    Self::run_in_dir(&["pod", "install"], &podfile_dir);
                } else {
                    println!(
                        "{}",
                        "No additional iOS deployment issues detected from logs.".green()
                    );
                }
            } else {
                println!("{}", "Pod install completed successfully.".green());
            }
        }
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

    fn extract_required_ios_version(output: &str) -> Option<String> {
        // Regex untuk mencocokkan log seperti: "platform `iOS` with version `12.0`"
        let re = Regex::new(r"platform `iOS` with version `(\d+\.\d+)`").unwrap();

        for line in output.lines() {
            println!("Checking line: {}", line); // Debug setiap baris
            if let Some(captures) = re.captures(line) {
                if let Some(version) = captures.get(1).map(|m| m.as_str().to_string()) {
                    println!("Extracted iOS version: {}", version); // Debug versi
                    return Some(version);
                }
            }
        }

        // Fallback parsing manual untuk log umum
        Self::manual_fallback_version_search(output)
    }

    fn manual_fallback_version_search(output: &str) -> Option<String> {
        for line in output.lines() {
            if line.contains("iOS") && line.contains("version") {
                let words: Vec<&str> = line.split_whitespace().collect();
                for word in words {
                    if word.chars().all(|c| c.is_digit(10) || c == '.') {
                        if word.parse::<f32>().unwrap_or(0.0) >= 8.0 {
                            println!("Manual fallback found iOS version: {}", word); // Debug fallback
                            return Some(word.to_string());
                        }
                    }
                }
            }
        }
        None
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

        let re_active = Regex::new(r"^platform :ios, '\d+\.\d+'").unwrap();
        let re_commented = Regex::new(r"^#\s*platform :ios, '\d+\.\d+'").unwrap();

        let updated_content = if re_active.is_match(&file_content) {
            re_active
                .replace(&file_content, format!("platform :ios, '{}'", new_version))
                .to_string()
        } else if re_commented.is_match(&file_content) {
            re_commented
                .replace(&file_content, format!("platform :ios, '{}'", new_version))
                .to_string()
        } else {
            format!("platform :ios, '{}'\n{}", new_version, file_content)
        };

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
