use std::sync::Arc;
use std::sync::mpsc::channel;
use std::thread;
pub fn mutex_example() {
    use parking_lot::Mutex;
    const N: usize = 10;
    let data = Arc::new(Mutex::new(0));
    let data2 = &data.clone();
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
    println!("mutex_example: {}", data2.lock());
}

const N: usize = 10;
pub fn main() {
    let mutex = Arc::new(Mutex::new(()));
    let handles: Vec<_> = (0..N)
        .map(|i| {
            let mutex = Arc::clone(&mutex);
            thread::spawn(move || match mutex.try_lock() {
                Some(_guard) => println!("thread {} got the lock", i),
                None => println!("thread {} did not get the lock", i),
            })
        })
        .collect();
    for handle in handles {
        handle.join().unwrap();
    }
    println!("mutex_example3: done");

    use parking_lot::Mutex;
    let mutex = Mutex::new(1);
    // mutex
    unsafe {
        mutex.force_unlock(); // mutex
    }

    // use parking_lot::Mutex;
    use std::mem;
    let mutex = Mutex::new(1);
    // mem::forget
    let _guard = mem::forget(mutex.lock());
    // mutex

    // mutex
    unsafe {
        mutex.force_unlock();
    }
}
