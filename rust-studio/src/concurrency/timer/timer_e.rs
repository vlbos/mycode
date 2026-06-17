use smol::Timer;
use std::time::Duration;
pub(crate) async fn main() {
    let timer = Timer::after(Duration::from_secs(1));
    // smol::block_on(async {
    timer.await;
    println!(" ");
    // });
}
