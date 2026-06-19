use tokio::time::{Duration, sleep};
async fn my_timer() {
    println!(" ");

    sleep(Duration::from_secs(5)).await;
    println!("5 ");
}

#[tokio::main]
pub async fn main() {
    my_timer().await;

    futures_timer_example().await;
}

pub async fn futures_timer_example() {
    use futures_timer::Delay;
    use std::time::Duration;
    // smol::block_on(async {
    for _ in 0..5 {
        Delay::new(Duration::from_secs(1)).await;
        println!(" ");
    }
    // });
}
