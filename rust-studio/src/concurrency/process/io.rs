use std::process::{Command, Stdio};
pub fn main() {
    let output = Command::new("echo")
        .arg("Hello, Rust!")
        .stdout(Stdio::piped())
        .output()
        .expect("Failed to execute command");
    println!("Output: {:?}", String::from_utf8_lossy(&output.stdout));
}
