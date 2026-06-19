use std::sync::{Arc, Mutex};
use std::thread;
pub fn main() {
    // Arc Mutex Vec
    let shared_vec = Arc::new(Mutex::new(Vec::new()));
    // Vec
    let mut handles = vec![];
    for i in 0..5 {
        let shared_vec = Arc::clone(&shared_vec);
        let handle = thread::spawn(move || {
            let mut vec = shared_vec.lock().unwrap();
            // Vec
            vec.push(i);
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    // Vec
    let final_vec = shared_vec.lock().unwrap();
    println!("Final Vec: {:?}", *final_vec);
}
