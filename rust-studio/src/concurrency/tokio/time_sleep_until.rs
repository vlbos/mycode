use tokio::time::{Duration, Instant, sleep_until};
#[tokio::main]
pub async fn main() {
    sleep_until(Instant::now() + Duration::from_millis(100)).await;
    println!("100 ms have elapsed");
}
