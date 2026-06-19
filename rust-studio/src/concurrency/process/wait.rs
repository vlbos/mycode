use std::process::Command;
pub fn main() {
    let mut child = Command::new("ls").spawn().expect("Failed to start command");
    let status = child.wait().expect("Failed to wait for command");
    println!("Command exited with: {:?}", status);
}
