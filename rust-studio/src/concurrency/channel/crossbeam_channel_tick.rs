use crossbeam_channel::{select, tick};
use std::time::Duration;
pub fn main() {
    let tick_interval = Duration::from_secs(1);
    let ticker = tick(tick_interval);
    for _ in 0..5 {
        select! {
        recv(ticker) ->_=> {
        println!("Tick!");
        }
        }
    }
}
