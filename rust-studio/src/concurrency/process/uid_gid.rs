use std::os::unix::process::CommandExt;
use std::process::Command;
pub fn main() {
    // UID GID
    let output = Command::new("whoami")
        .uid(1000) // UID
        .gid(1000) // GID
        .output()
        .expect("Failed to execute command");
    println!("Output: {:?}", String::from_utf8_lossy(&output.stdout));
}
