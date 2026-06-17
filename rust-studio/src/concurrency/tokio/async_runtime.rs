use tokio::process::Command;
#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let output = Command::new("echo").arg("hello").arg("world").output();

    let output = output.await?;
    assert!(output.status.success());
    assert_eq!(output.stdout, b"hello world\n");
    Ok(())
}
