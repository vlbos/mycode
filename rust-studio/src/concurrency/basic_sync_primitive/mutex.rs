use std::sync::{Arc, Mutex};
use std::thread;
fn main1() {
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

// use std::sync::{Arc, Mutex};
// use std::thread;
fn main2() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    for _ in 0..5 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            if let Ok(mut num) = counter.try_lock() {
                *num += 1;
            } else {
                println!("Thread failed to acquire lock.");
            }
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    println!("Final count: {}", *counter.lock().unwrap());
}

use std::sync::{LockResult, PoisonError};
// use std::thread;
fn main3() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    for _ in 0..5 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let result: LockResult<_> = counter.lock();
            match result {
                Ok(mut num) => {
                    *num += 1;
                    // panic
                    if *num == 3 {
                        panic!("Simulated panic!");
                    }
                }
                Err(poisoned) => {
                    // "poisoned"
                    println!("Thread encountered a poisoned lock: {:?}", poisoned);
                    // MutexGuard
                    let mut num = poisoned.into_inner();
                    *num += 1;
                }
            }
        });

        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }

    println!("Final count: {}", *counter.lock().unwrap());
}

// use std::sync::{Arc, Mutex};
// use std::thread;
fn main4() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..5 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            // !!!!!!!!!!!!!!
            {
                let mut num = counter.lock().unwrap();
                *num += 1;
                // MutexGuard
            }
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    println!("Final count: {}", *counter.lock().unwrap());
}

// use std::sync::{Arc, Mutex};
// use std::thread;
fn main() {
    main1();
    main2();
    main3();
    main4();
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    for _ in 0..5 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
            drop(num);
        });

        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    println!("Final count: {}", *counter.lock().unwrap());
}
