// use std::time::Duration;
// use tokio::sync::oneshot;
use tokio::time::timeout;
#[tokio::main]
pub async fn main() {
    let (_tx, rx) = oneshot::channel::<i32>();
    // Wrap the future with a`Timeout` set to expire in 10 milliseconds.
    if let Err(_) = timeout(Duration::from_millis(10), rx).await {
        println!("did not receive value within 10 ms");
    }

    use std::time::Duration;
    use tokio::sync::oneshot;
    use tokio::time::{Instant, timeout_at};
    let (_tx, rx) = oneshot::channel::<i32>();
    // Wrap the future with a`Timeout` set to expire 10 milliseconds into the
    // future.
    if let Err(_) = timeout_at(Instant::now() + Duration::from_millis(10), rx).await {
        println!("did not receive value within 10 ms");
    }
}
