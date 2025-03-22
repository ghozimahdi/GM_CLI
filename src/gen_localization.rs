use colored::Colorize;
use std::{
    io::{self, Write},
    process::{Command as StdCommand, Stdio},
    thread,
    time::Duration,
};

pub fn intl_localization() {
    println!("{}", "Generating intl_utils files...".green());

    let mut child = StdCommand::new("melos")
        .arg("exec")
        .arg("--depends-on=intl_utils")
        .arg("--")
        .arg("dart")
        .arg("run")
        .arg("intl_utils:generate")
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .expect("Failed to generate localization");

    let mut progress = 0.0;
    const BAR_LENGTH: usize = 50;

    while let Ok(None) = child.try_wait() {
        progress += 0.01;
        if progress > 1.0 {
            progress = 1.0;
        }

        let filled_length = (progress * BAR_LENGTH as f32).round() as usize;
        let bar: String = "=".repeat(filled_length) + &" ".repeat(BAR_LENGTH - filled_length);
        print!("\r[{}] {:.0}%", bar, progress * 100.0);
        io::stdout().flush().unwrap();

        thread::sleep(Duration::from_millis(100));
    }

    let status = child.wait().expect("Failed to wait on child process");
    print!("\r[{}] 100%\n", "=".repeat(BAR_LENGTH));

    if status.success() {
        println!("{}", "Localization generated successfully.".green());
    } else {
        println!("{}", "Localization generation failed.".red());
    }
}
