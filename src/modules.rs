use std::collections::HashMap;
use std::process::{Command as ProcessCommand, exit};
use colored::*;

pub struct CategorizedModules {
    pub data: Vec<String>,
    pub domain: Vec<String>,
    pub feature: Vec<String>,
    pub other: Vec<String>,
    pub map: HashMap<usize, String>,
}

pub fn get_available_modules() -> Vec<String> {
    let output = ProcessCommand::new("melos")
        .arg("list")
        .output()
        .expect("Gagal menjalankan perintah melos list");

    if !output.status.success() {
        eprintln!("{}: Gagal mendapatkan daftar modul.", "Error".red());
        exit(1);
    }

    String::from_utf8_lossy(&output.stdout)
        .lines()
        .map(|line| line.split_whitespace().next().unwrap_or("").to_string())
        .collect()
}

pub fn categorize_modules(modules: &[String]) -> CategorizedModules {
    let mut data = Vec::new();
    let mut domain = Vec::new();
    let mut feature = Vec::new();
    let mut other = Vec::new();
    let mut map = HashMap::new();

    for (index, module) in modules.iter().enumerate() {
        let idx = index + 1;
        if module.contains("data") {
            data.push(format!("{}. {}", idx, module));
        } else if module.contains("domain") {
            domain.push(format!("{}. {}", idx, module));
        } else if module.contains("feature") || module.contains("presentation") {
            feature.push(format!("{}. {}", idx, module));
        } else {
            other.push(format!("{}. {}", idx, module));
        }
        map.insert(idx, module.clone());
    }

    CategorizedModules { data, domain, feature, other, map }
}

pub fn run_build_command(scope: Option<&String>) {
    let mut command = ProcessCommand::new("melos");
    command.arg("exec");
    if let Some(scope) = scope {
        command.arg("--scope").arg(scope);
    }
    command
        .arg("--")
        .arg("dart")
        .arg("pub")
        .arg("run")
        .arg("build_runner")
        .arg("build")
        .arg("-d");

    let status = command.status().expect("Gagal menjalankan build command");
    if !status.success() {
        eprintln!("{}", "Build gagal.".red());
    }
}

pub fn handle_selected_modules(input: &str, module_map: &HashMap<usize, String>) {
    let selected_indices: Vec<usize> = input
        .split(',')
        .filter_map(|s| s.trim().parse().ok())
        .collect();

    for index in selected_indices {
        if let Some(selected_module) = module_map.get(&index) {
            println!(
                "{}: {}",
                "Building module".green(),
                selected_module
            );
            run_build_command(Some(selected_module));
        } else {
            println!("{}: Modul dengan nomor {} tidak ditemukan.", "Error".red(), index);
        }
    }
}