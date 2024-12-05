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
}
