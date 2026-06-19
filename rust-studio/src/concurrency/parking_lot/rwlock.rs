use parking_lot::RwLock;
use std::sync::Arc;
use std::thread;
pub fn main() {
    const N: usize = 10;
    let lock = Arc::new(RwLock::new(5));
    let handles: Vec<_> = (0..N)
        .map(|i| {
            let lock = Arc::clone(&lock);
            thread::spawn(move || {
                if i % 2 == 0 {
                    let mut num = lock.write();
                    *num += 1;
                } else {
                    let num = lock.read();
                    println!("thread {} read {}", i, num);
                }
            })
        })
        .collect();
    for handle in handles {
        handle.join().unwrap();
    }
}
