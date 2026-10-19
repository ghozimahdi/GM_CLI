use colored::*;
use std::io::{self, BufRead, BufReader, Write};
use std::process::{Command as StdCommand, Stdio};

pub fn gen_assets() {
    println!("{}", "Generating assets...".green());

    let mut child = StdCommand::new("fvm")
        .arg("dart")
        .arg("run")
        .arg("melos")
        .arg("exec")
        .arg("--depends-on=flutter_gen_runner")
        .arg("--")
        .arg("dart")
        .arg("run")
        .arg("build_runner")
        .arg("build")
        .arg("-d")
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .expect("Failed to start asset generation process");

    let stdout = child.stdout.take().expect("Failed to capture stdout");
    let stdout_reader = BufReader::new(stdout);

    let mut progress = 0.0;
    const BAR_LENGTH: usize = 50;

    for line in stdout_reader.lines() {
        if let Ok(line) = line {
            if line.contains("Writing") {
                progress = 1.0;
            } else {
                progress += 0.01;
                if progress > 1.0 {
                    progress = 1.0;
                }
            }

            let filled_length = (progress * BAR_LENGTH as f32).round() as usize;
            let bar: String = "=".repeat(filled_length) + &" ".repeat(BAR_LENGTH - filled_length);
            print!("\r[{}] {:.0}%", bar, progress * 100.0);
            io::stdout().flush().unwrap();

            if progress == 1.0 {
                break;
            }
        }
    }

    let status = child.wait().expect("Failed to wait on child process");

    print!("\r[{}] 100%\n", "=".repeat(BAR_LENGTH));

    if status.success() {
        println!("{}", "Assets generated successfully.".green());
    } else {
        println!("{}", "Asset generation failed.".red());
    }
}
