use std::process::Command;
use std::process::Stdio;
pub fn pipe() {
    let producer = Command::new("echo")
        .arg("Hello, Rust!")
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start producer command");

    let consumer = Command::new("grep")
        .arg("Rust")
        .stdin(producer.stdout.unwrap())
        .output()
        .expect("Failed to start consumer command");

    let output = String::from_utf8_lossy(&consumer.stdout);
    println!("Output: {:?}", output);
}

pub fn main() {
    let command = "echo \"Hello, Rust!\" | grep Rust";
    let output = Command::new("sh")
        .arg("-c")
        .arg(command)
        .output()
        .expect("Failed to execute command");
    println!("Output: {:?}", String::from_utf8_lossy(&output.stdout));
}
