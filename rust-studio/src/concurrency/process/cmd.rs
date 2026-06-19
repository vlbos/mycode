use std::process::Command;
pub fn main() {
    let output = Command::new("ls")
        .arg("-l")
        .output()
        .expect("Failed to execute command");
    println!("Output: {:?}", output);
}
