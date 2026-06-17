use parking_lot::FairMutex;
use std::sync::Arc;
use std::sync::mpsc::channel;
use std::thread;
fn main() {
    const N: usize = 10;
    let data = Arc::new(FairMutex::new(0));
    let (tx, rx) = channel();
    for _ in 0..10 {
        let (data, tx) = (Arc::clone(&data), tx.clone());
        thread::spawn(move || {
            let mut data = data.lock(); // MutexGuard
            *data += 1;
            if *data == N {
                tx.send(()).unwrap();
            }
        });
    }
    rx.recv().unwrap();
}
