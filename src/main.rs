use std::collections::HashMap;
use std::process::{Command as ProcessCommand, exit};
use colored::*;

fn main() {
    let output = ProcessCommand::new("melos")
        .arg("list")
        .output()
        .expect("Gagal menjalankan perintah melos list");

    if !output.status.success() {
        eprintln!("{}: Gagal mendapatkan daftar modul.", "Error".red());
        exit(1);
    }

    let available_modules: Vec<String> = String::from_utf8_lossy(&output.stdout)
        .lines()
        .map(|line| line.split_whitespace().next().unwrap_or("").to_string())
        .collect();

    if available_modules.is_empty() {
        println!("{}", "Tidak ada modul yang ditemukan.".yellow());
        return;
    }

    let mut data_modules = Vec::new();
    let mut domain_modules = Vec::new();
    let mut feature_modules = Vec::new();
    let mut other_modules = Vec::new();
    let mut module_map = HashMap::new();

    let mut index = 1;

    for module in &available_modules {
        if module.contains("data") {
            data_modules.push(format!("{}. {}", index, module));
        } else if module.contains("domain") {
            domain_modules.push(format!("{}. {}", index, module));
        } else if module.contains("feature") || module.contains("presentation") {
            feature_modules.push(format!("{}. {}", index, module));
        } else {
            other_modules.push(format!("{}. {}", index, module));
        }
        module_map.insert(index, module.clone());
        index += 1;
    }

    print_side_by_side_table(&data_modules, &domain_modules, &feature_modules, &other_modules);

    println!(
        "Total modules available: {}, Pilih module (contoh: 1,2,3 atau 0 untuk semua):",
        available_modules.len()
    );
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Gagal membaca input");

    let input = input.trim();

    if input == "0" {
        println!("{}", "Building all modules...".green());
        run_build_command(None);
    } else {
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

    println!("{}", "Memulai ulang terminal...".cyan());
}

fn run_build_command(scope: Option<&String>) {
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

fn print_side_by_side_table(
    data_modules: &[String],
    domain_modules: &[String],
    feature_modules: &[String],
    other_modules: &[String],
) {
    let max_rows = 30;

    println!(
        "+------------------------------+------------------------------+------------------------------+------------------------------+"
    );
    println!(
        "| {:<28} | {:<28} | {:<28} | {:<28} |",
        "Data Modules", "Domain Modules", "Feature Modules", "Other Modules"
    );
    println!(
        "+------------------------------+------------------------------+------------------------------+------------------------------+"
    );

    for i in 0..max_rows {
        let data_module = data_modules.get(i).map(String::as_str).unwrap_or("");
        let domain_module = domain_modules.get(i).map(String::as_str).unwrap_or("");
        let feature_module = feature_modules.get(i).map(String::as_str).unwrap_or("");
        let other_module = other_modules.get(i).map(String::as_str).unwrap_or("");

        if !data_module.is_empty()
            || !domain_module.is_empty()
            || !feature_module.is_empty()
            || !other_module.is_empty()
        {
            println!(
                "| {:<28} | {:<28} | {:<28} | {:<28} |",
                data_module, domain_module, feature_module, other_module
            );
        }
    }

    println!(
        "+------------------------------+------------------------------+------------------------------+------------------------------+"
    );
}