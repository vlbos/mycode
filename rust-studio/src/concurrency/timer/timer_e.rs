use smol::Timer;
use std::time::Duration;
#[tokio::main]
pub async fn main() {
    let timer = Timer::after(Duration::from_secs(1));
    // smol::block_on(async {
    timer.await;
    println!(" ");
    // });
}
