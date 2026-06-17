use crossbeam_channel::{at, select};
use std::time::{Duration, Instant};
pub(crate) fn main() {
    let start_time = Instant::now();
    let target_time = start_time + Duration::from_secs(2);
    let timeout_channel = at(target_time);
    loop {
        select! {
        recv(timeout_channel) ->_ => {
        println!("Timeout reached!");
        break;
        }
        }
    }
}
