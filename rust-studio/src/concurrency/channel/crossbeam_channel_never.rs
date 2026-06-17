  use crossbeam_channel::{never, select};
    pub(crate) fn main() {
        let never_channel: crossbeam_channel::Receiver<i32>  = never();
        loop {
            select! {
            recv(never_channel) ->_=> {
                    unreachable!();
            }
            default => {
                    println!("Performing other operations...");
            }
            }
        }
    }