use crate::{modules, table};
use colored::Colorize;
use std::io::Write;

pub fn build_runner() {
    let available_modules = modules::get_available_modules();

    if available_modules.is_empty() {
        println!("{}", "Tidak ada modul yang ditemukan.".yellow());
        return;
    }

    let categorized_modules = modules::categorize_modules(&available_modules);
    table::print_side_by_side_table(
        &categorized_modules.data,
        &categorized_modules.domain,
        &categorized_modules.feature,
        &categorized_modules.other,
    );

    let total_modules = available_modules.len();
    print!(
        "Total modules available: {}, Pilih module (contoh: 1,2,3 atau 0 untuk semua): ",
        total_modules.to_string().green()
    );
    std::io::stdout().flush().unwrap();

    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Gagal membaca input");
    let input = input.trim();

    if input == "0" {
        println!("{}", "Building all modules...".green());
        modules::run_build_command(None);
    } else {
        modules::handle_selected_modules(&input, &categorized_modules.map);
    }

    println!("{}", "Memulai ulang terminal...".cyan());
}
