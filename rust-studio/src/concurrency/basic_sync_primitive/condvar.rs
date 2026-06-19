use std::sync::{Arc, Condvar, Mutex};
use std::thread;
pub fn main() {
    // Mutex Condvar
    let mutex = Arc::new(Mutex::new(false));
    let condvar = Arc::new(Condvar::new());

    let mut handles = vec![];
    for id in 0..3 {
        let mutex = Arc::clone(&mutex);
        let condvar = Arc::clone(&condvar);
        let handle = thread::spawn(move || {
            // Mutex
            let mut guard = mutex.lock().unwrap();
            // true
            while !*guard {
                guard = condvar.wait(guard).unwrap();
            }

            println!("Thread {} woke up", id);
        });
        handles.push(handle);
    }

    thread::sleep(std::time::Duration::from_secs(2));

    {
        let mut guard = mutex.lock().unwrap();
        *guard = true;
        condvar.notify_all();
    }
    for handle in handles {
        handle.join().unwrap();
    }
}
