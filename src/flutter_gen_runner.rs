use colored::Colorize;
use std::process::Command as StdCommand;

pub fn gen_assets() {
    println!("{}", "Generating assets...".green());
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

    if !output.status.success() {
        println!("{}", "Asset generation failed.".red());
    } else {
        println!("{}", "Assets generated successfully.".green());
    }
}
