use std::time::Duration;
use ticker::Ticker;
pub fn ticker_example() {
    let ticker = Ticker::new(0..10, Duration::from_secs(1));
    for i in ticker {
        println!("{:?}", i)
    }
}
pub fn main() {
    ticker_example();
}
