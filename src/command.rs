use std::fs;
use std::io::Write;

pub struct Command;

impl Command {
    pub fn run(command: &[&str]) {
        let status = std::process::Command::new(command[0])
            .args(&command[1..])
            .status()
            .expect("Failed to execute command");

        if !status.success() {
            panic!("Command {:?} failed", command);
        }
    }

    pub fn write_file(file_path: &str, content: &str) {
        let mut file = fs::File::create(file_path).expect("Failed to create file");
        file.write_all(content.as_bytes())
            .expect("Failed to write to file");
    }

    pub fn run_in_dir(args: &[&str], dir: &str) -> std::process::Output {
        let command = args[0];
        let command_args = &args[1..];

        println!("Running command '{}' in directory: {}", command, dir);

        let output = std::process::Command::new(command)
            .args(command_args)
            .current_dir(dir)
            .output()
            .expect("Failed to execute command");

        if !output.status.success() {
            eprintln!(
                "Error: Command '{}' failed with status {}",
                command, output.status
            );
            eprintln!("stderr: {}", String::from_utf8_lossy(&output.stderr));
        } else {
            println!("Command '{}' succeeded.", command);
        }

        output
    }
}
