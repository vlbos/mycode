use std::process::Command;
pub fn main() {
    let output = Command::new("ls")
        .arg("-l")
        .current_dir("/Users")
        .output()
        .expect("Failed to execute command");
    println!("Output: {:?}", String::from_utf8_lossy(&output.stdout));
}
