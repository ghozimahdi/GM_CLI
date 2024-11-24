use colored::Colorize;
use std::thread;
use std::time::Duration;
use std::{
    io::{self, Write},
    process::Command as StdCommand,
};

pub fn gen_assets() {
    let spinner = ['|', '/', '-', '\\'];
    let mut idx = 0;

    print!("{}", "Generating assets... ".green());
    io::stdout().flush().unwrap();

    let output = StdCommand::new("melos")
        .arg("exec")
        .arg("--depends-on=flutter_gen_runner")
        .arg("--")
        .arg("dart")
        .arg("run")
        .arg("build_runner")
        .arg("build")
        .arg("-d")
        .output()
        .expect("Failed to generate assets");

    while !output.status.success() && !output.status.success() {
        print!("\r{} {}", spinner[idx], "Generating assets...".green());
        io::stdout().flush().unwrap();
        idx = (idx + 1) % spinner.len();
        thread::sleep(Duration::from_millis(100));

        if !output.status.success() {
            break;
        }
    }

    if !output.status.success() {
        println!("{}", "Asset generation failed.".red());
    } else {
        println!("{}", "Assets generated successfully.".green());
    }
}
