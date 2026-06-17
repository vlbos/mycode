pub(crate) fn main() {
    use crossbeam_channel::{after, select, tick};
    use std::time::{Duration, Instant};
    let start = Instant::now();
    let ticker = tick(Duration::from_millis(50));
    let timeout = after(Duration::from_secs(1));
    loop {
        select! {
        recv(ticker) -> _=> println!("elapsed: {:?}", start.elapsed()),
        recv(timeout) ->_=> break,
        }
    }
}
