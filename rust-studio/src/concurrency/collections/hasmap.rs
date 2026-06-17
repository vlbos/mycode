use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;
fn main() {
    // Arc Mutex HashMap
    let shared_map = Arc::new(Mutex::new(HashMap::new()));
    // HashMap
    let mut handles = vec![];
    for i in 0..5 {
        let shared_map = Arc::clone(&shared_map);
        let handle = thread::spawn(move || {
            let mut map = shared_map.lock().unwrap();
            // HashMap
            map.insert(i, i * i);
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    // HashMap
    let final_map = shared_map.lock().unwrap();
    println!("Final HashMap: {:?}", *final_map);
}
