use crate::table;
use colored::Colorize;
use std::collections::HashMap;
use std::io::{self, Write};
use std::process::{exit, Command as ProcessCommand};

pub struct CategorizedModules {
    pub data: Vec<String>,
    pub domain: Vec<String>,
    pub feature: Vec<String>,
    pub other: Vec<String>,
    pub map: HashMap<usize, String>,
}

pub struct ModuleManager;

impl ModuleManager {
    pub fn get_available_modules() -> Vec<String> {
        let output = ProcessCommand::new("melos")
            .arg("list")
            .output()
            .expect("Failed to run the melos list command");

        if !output.status.success() {
            eprintln!("{}: Failed to retrieve module list.", "Error".red());
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

        CategorizedModules {
            data,
            domain,
            feature,
            other,
            map,
        }
    }

    pub fn select_modules() -> Option<HashMap<usize, String>> {
        let available_modules = Self::get_available_modules();

        if available_modules.is_empty() {
            println!("{}", "No modules found.".yellow());
            return None;
        }

        let categorized_modules = Self::categorize_modules(&available_modules);

        table::print_side_by_side_table(
            &categorized_modules.data,
            &categorized_modules.domain,
            &categorized_modules.feature,
            &categorized_modules.other,
        );

        let total_modules = available_modules.len();
        print!(
            "Total modules available: {}, Select modules (example: 1,2,3 or 0 for all): ",
            total_modules.to_string().green()
        );
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        let input = input.trim();

        if input == "0" {
            println!("{}", "All modules selected.".green());
            return Some(categorized_modules.map);
        }

        let selected_indices: Vec<usize> = input
            .split(',')
            .filter_map(|s| s.trim().parse().ok())
            .collect();

        let mut selected_modules = HashMap::new();
        for index in selected_indices {
            if let Some(module) = categorized_modules.map.get(&index) {
                selected_modules.insert(index, module.clone());
            } else {
                println!("{}: Module with number {} not found.", "Error".red(), index);
            }
        }

        if selected_modules.is_empty() {
            println!("{}", "No valid modules selected.".yellow());
            return None;
        }

        Some(selected_modules)
    }
}
