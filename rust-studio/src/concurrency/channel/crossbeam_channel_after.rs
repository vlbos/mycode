use crossbeam_channel::{after, select};
use std::time::Duration;
pub fn main() {
    let timeout = Duration::from_secs(2);

    let timeout_channel = after(timeout);
    loop {
        select! {
        recv(timeout_channel) ->_=> {
        println!("Timeout reached!");
        break;
        }
        default => {
                println!("Performing other operations...");
        }
        }
    }
}
