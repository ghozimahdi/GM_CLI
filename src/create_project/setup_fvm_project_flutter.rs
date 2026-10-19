use crate::command::Command;
use colored::Colorize;
use indoc::indoc;
use std::fs;
use std::fs::OpenOptions;
use std::io::{Read, Write};

pub struct SetupFVMProjectFlutter;

impl SetupFVMProjectFlutter {
    pub fn build(project_name: &str) {
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

        let flutter_version: String;

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
            flutter_version = version;
        } else {
            println!(
                "{}",
                "No global Flutter version detected. Installing stable via FVM...".yellow()
            );
            Command::run(&["fvm", "install", "stable"]);
            Command::run(&["fvm", "use", "stable"]);
            flutter_version = "stable".to_string();
        }

        Self::generate_vscode_config(project_name, &flutter_version);

        println!("{}", "FVM setup completed!".green());
    }

    fn generate_vscode_config(project_name: &str, flutter_version: &str) {
        let vscode_dir = ".vscode";
        fs::create_dir_all(vscode_dir).expect("Failed to create .vscode directory");

        // Generate settings.json
        let settings_content = indoc! {r#"
{
    "dart.flutterSdkPath": ".fvm/versions/{flutter_version}",
    "files.autoSave": "afterDelay",
    "files.autoSaveDelay": 0,
    "[dart]": {
        "editor.formatOnSave": true,
        "editor.formatOnType": true,
        "editor.codeActionsOnSave": {
            "source.fixAll": "always",
            "source.organizeImports": "always"
        }
    },
    "dart.flutterHotReloadOnSave": "never",
    "gotoNextPreviousMember.symbolKinds": [
        "class",
        "method",
        "function",
        "constructor",
        "widget"
    ],
    "gotoNextPreviousMember.selectEntireSymbol": false,
    "gotoNextPreviousMember.symbolPosition": "center",
    "files.exclude": {
        "**/.git": true,
        "**/.svn": true,
        "**/.hg": true,
        "**/.DS_Store": true,
        "**/Thumbs.db": true,
        ".mdnotes": true,
        ".vxnotes": true
    }
}
"#}
        .replace("{flutter_version}", flutter_version);

        let settings_path = format!("{}/settings.json", vscode_dir);
        fs::write(&settings_path, settings_content).expect("Failed to write settings.json");
        println!("{}", format!("Generated {}", settings_path).green());

        // Generate launch.json
        let launch_content = indoc! {r#"
{
    "version": "0.2.0",
    "configurations": [
        {
            "name": "{project_name}_dev",
            "request": "launch",
            "type": "dart",
            "program": "app/lib/main_dev.dart",
            "args": [
                "--flavor",
                "dev"
            ]
        },
        {
            "name": "{project_name}_staging",
            "request": "launch",
            "type": "dart",
            "program": "app/lib/main_staging.dart",
            "args": [
                "--flavor",
                "staging"
            ]
        },
        {
            "name": "{project_name}_prod",
            "request": "launch",
            "type": "dart",
            "program": "app/lib/main_prod.dart",
            "args": [
                "--flavor",
                "prod"
            ]
        },
        {
            "name": "{project_name}_dev_release",
            "request": "launch",
            "type": "dart",
            "program": "app/lib/main_dev.dart",
            "args": [
                "--flavor",
                "dev",
                "--release"
            ]
        },
        {
            "name": "{project_name}_prod_release",
            "request": "launch",
            "type": "dart",
            "program": "app/lib/main_prod.dart",
            "args": [
                "--flavor",
                "prod",
                "--release"
            ]
        }
    ]
}
"#}
        .replace("{project_name}", project_name);

        let launch_path = format!("{}/launch.json", vscode_dir);
        fs::write(&launch_path, launch_content).expect("Failed to write launch.json");
        println!("{}", format!("Generated {}", launch_path).green());
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
