use crate::command::Command;
use colored::Colorize;
use std::fs;
use std::fs::OpenOptions;
use std::io::{Read, Write};

pub struct SetupFVMProjectFlutter;

impl SetupFVMProjectFlutter {
    pub fn build() {
        println!(
            "{}",
            "Setting up FVM for Flutter version management...".yellow()
        );

        let flutter_version_output = std::process::Command::new("flutter")
            .arg("--version")
            .output();

        let global_flutter_version = if let Ok(output) = flutter_version_output {
            if output.status.success() {
                let version_info = String::from_utf8_lossy(&output.stdout);
                if let Some(version_line) = version_info.lines().next() {
                    let version_regex = regex::Regex::new(r"Flutter (\d+\.\d+\.\d+)").unwrap();
                    if let Some(captures) = version_regex.captures(version_line) {
                        Some(captures[1].to_string())
                    } else {
                        None
                    }
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        };

        Self::add_fvm_to_gitignore();

        if let Some(version) = global_flutter_version {
            println!(
                "{}",
                format!("Detected global Flutter version: {}", version).green()
            );

            let fvm_cache_path = format!("~/.fvm/versions/{}", version);
            if std::path::Path::new(&fvm_cache_path).exists() {
                println!(
                    "{}",
                    format!("Version {} already cached in FVM. Using it.", version).green()
                );
            } else {
                println!(
                    "{}",
                    format!("Caching version {} with FVM...", version).yellow()
                );
                Command::run(&["fvm", "install", &version]);
            }

            println!(
                "{}",
                format!("Setting project to use version {}...", version).yellow()
            );
            Command::run(&["fvm", "use", &version]);
        } else {
            println!(
                "{}",
                "No global Flutter version detected. Installing stable via FVM...".yellow()
            );
            Command::run(&["fvm", "install", "stable"]);
            Command::run(&["fvm", "use", "stable"]);
        }

        println!("{}", "FVM setup completed!".green());
    }

    fn add_fvm_to_gitignore() {
        let gitignore_path = ".gitignore";

        let gitignore_content_from_assets = include_str!("../../assets/.gitignore");

        let mut gitignore_content = String::new();
        if let Ok(mut file) = fs::File::open(gitignore_path) {
            file.read_to_string(&mut gitignore_content)
                .expect("Failed to read existing .gitignore");
        }

        if !gitignore_content.contains(".fvm/") {
            println!("{}", "Adding .fvm/ to .gitignore...".yellow());
            let mut file = OpenOptions::new()
                .create(true)
                .write(true)
                .append(false)
                .open(gitignore_path)
                .expect("Failed to open .gitignore");

            let updated_gitignore_content =
                format!("{}\n{}", gitignore_content, gitignore_content_from_assets);

            file.write_all(updated_gitignore_content.as_bytes())
                .expect("Failed to write to .gitignore");
            println!("{}", ".fvm/ added to .gitignore.".green());
        } else {
            println!("{}", ".fvm/ already exists in .gitignore.".green());
        }
    }
}
