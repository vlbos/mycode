use std::sync::Arc;
use std::thread;
fn main1() {
        let data = Arc::new(46);
    // data
    let thread1 = {
        let data = Arc::clone(&data);
        thread::spawn(move || {
            // data
            println!("Thread 1: {}", data);
        })
    };

    let thread2 = {
        let data = Arc::clone(&data);
        thread::spawn(move || {
            // data
            println!("Thread 2: {}", data);
        })
    };
        thread1.join().unwrap();
    thread2.join().unwrap();
}

use std::sync::{ Mutex};
// use std::thread;
fn main() {
    main1();
    let counter = Arc::new(Mutex::new(0));
        let mut handles = vec![];
    for _ in 0..5 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
                        let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    
    }

        for handle in handles {
            handle.join().unwrap();
        }
        println!("Final count: {}", *counter.lock().unwrap());
}
