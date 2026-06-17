use std::io::Write;
use std::process::{Command, Stdio};
use std::io::Read;
fn main1() {

    let mut child = Command::new("echo")
        .arg("Hello, Rust!")
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start command");

    let mut output = String::new();
    child
        .stdout
        .unwrap()
        .read_to_string(&mut output)
        .expect("Failed to read from st");
    println!("Output: {:?}", output);
}

// use std::process::{Command, Stdio};
fn main() {
    main1();
    let mut child = Command::new("echo")
        .arg("Hello, Rust!")
        .stdout(Stdio::null())
        .spawn()
        .expect("Failed to start command");

    let status = child.wait().expect("Failed to wait for command");
    println!("Command exited with: {:?}", status);
}
