use colored::*;
use std::io::{self, Write};
use std::process::{Command as StdCommand, Stdio};
use std::thread;
use std::time::Duration;

pub fn gen_assets() {
    let spinner = ['|', '/', '-', '\\'];
    let mut idx = 0;

    print!("{}", "Generating assets... ".green());
    io::stdout().flush().unwrap();

    let mut child = StdCommand::new("melos")
        .arg("exec")
        .arg("--depends-on=flutter_gen_runner")
        .arg("--")
        .arg("dart")
        .arg("run")
        .arg("build_runner")
        .arg("build")
        .arg("-d")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to start asset generation process");

    // Spinner loop while the process is running
    loop {
        // Check if the process has finished
        match child.try_wait() {
            Ok(Some(status)) => {
                if status.success() {
                    println!("\r{}", "Assets generated successfully.".green());
                } else {
                    println!("\r{}", "Asset generation failed.".red());
                }
                break;
            }
            Ok(None) => {
                // Update spinner animation
                print!("\r{} {}", spinner[idx], "Generating assets...".green());
                io::stdout().flush().unwrap();
                idx = (idx + 1) % spinner.len();
                thread::sleep(Duration::from_millis(100));
            }
            Err(e) => {
                println!("\r{}: {}", "Error checking process status".red(), e);
                break;
            }
        }
    }
}
