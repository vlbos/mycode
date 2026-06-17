use tokio::time::{Duration, Instant, interval_at};
#[tokio::main]
pub async fn main() {
    let start = Instant::now() + Duration::from_millis(50);
    let mut interval = interval_at(start, Duration::from_millis(10));
    interval.tick().await; // ticks after 50ms
    interval.tick().await; // ticks after 10ms
    interval.tick().await; // ticks after 10ms
    // approximately 70ms have elapsed.
}
