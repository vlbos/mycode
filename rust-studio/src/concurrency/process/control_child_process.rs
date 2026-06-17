use std::process::Command;
fn main1() {
    let mut child = Command::new("echo")
        .arg("Hello, Rust!")
        .spawn()
        .expect("Failed to start command");
    let status = child.wait().expect("Failed to wait for command");
    println!("Command exited with: {:?}", status);
}

use std::process::Stdio;
fn main2() {
    let mut child = Command::new("sleep")
        .arg("10")
        .stdout(Stdio::null())
        .spawn()
        .expect("Failed to start command");
    child.kill().expect("Failed to send signal");
}

use std::io::Write;
// use std::process::{Command, Stdio};
fn main() {
    main1();
    main2();
    let mut child = Command::new("cat")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start command");
    if let Some(mut stdin) = child.stdin.take() {
        stdin
            .write_all(b"Hello, Rust!\n")
            .expect("Failed to write to stdin");
    }
    let output = child
        .wait_with_output()
        .expect("Failed to wait for command");
    println!("Output: {:?}", String::from_utf8_lossy(&output.stdout));
}
