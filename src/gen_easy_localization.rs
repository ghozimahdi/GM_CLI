use colored::Colorize;
use std::thread;
use std::time::Duration;
use std::{
    io::{self, Write},
    process::Command as StdCommand,
};

pub fn easy_localization() {
    let spinner = ['|', '/', '-', '\\'];
    let mut idx = 0;

    print!("{}", "Generating easy_localization files... ".green());
    io::stdout().flush().unwrap();

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

    while !output.status.success() {
        print!(
            "\r{} {}",
            spinner[idx],
            "Generating easy_localization files...".green()
        );
        io::stdout().flush().unwrap();
        idx = (idx + 1) % spinner.len();
        thread::sleep(Duration::from_millis(100));

        if output.status.success() {
            break;
        }
    }

    if !output.status.success() {
        println!("{}", "Localization generation failed.".red());
    } else {
        println!("{}", "Localization generated successfully.".green());
    }
}
