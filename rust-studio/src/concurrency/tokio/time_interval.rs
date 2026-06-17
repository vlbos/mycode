   use tokio::time::{self, Duration};
    #[tokio::main]
    pub async fn main() {
        let mut interval = time::interval(Duration::from_millis(10));
        interval.tick().await; // ticks immediately
        interval.tick().await; // ticks after 10ms
        interval.tick().await; // ticks after 10ms
        // approximately 20ms have elapsed.
    }