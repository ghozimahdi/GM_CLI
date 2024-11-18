use colored::Colorize;
use std::process::Command as StdCommand;

pub fn easy_localization() {
    println!("{}", "Generating easy_localization files...".green());
    let output = StdCommand::new("melos")
        .arg("exec")
        .arg("--depends-on=easy_localization")
        .arg("--")
        .arg("flutter")
        .arg("pub")
        .arg("run")
        .arg("easy_localization:generate")
        .arg("-f")
        .arg("keys")
        .arg("-S")
        .arg("assets/translations")
        .output()
        .expect("Failed to generate localization");

    if !output.status.success() {
        println!("{}", "Localization generation failed.".red());
    } else {
        println!("{}", "Localization generated successfully.".green());
    }
}
