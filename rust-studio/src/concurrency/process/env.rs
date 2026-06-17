use std::process::Command;
fn main() {
    let output = Command::new("printenv")
        .env("MY_VAR", "HelloRust")
        .output()
        .expect("Failed to execute command");
    println!("Output: {:?}", String::from_utf8_lossy(&output.stdout));
}
