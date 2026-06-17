use dashmap::DashMap;
use std::sync::Arc;
fn main() {
    let map = Arc::new(DashMap::new());
    let mut handles = vec![];
    for i in 0..10 {
        let map = Arc::clone(&map);
        handles.push(std::thread::spawn(move || {
            map.insert(i, i);
        }));
    }
    for handle in handles {
        handle.join().unwrap();
    }
    println!("DashMap: {:?}", map);
}
