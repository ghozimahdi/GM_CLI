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
}
