use std::sync::{Arc, RwLock};
use std::thread;
fn main1() {
    // RwLock
    let counter = Arc::new(RwLock::new(0));


    let mut read_handles = vec![];
    for _ in 0..3 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {

            let num = counter.read().unwrap();
            println!("Reader {:?}: {}", thread::current().id(), *num);
        });
        read_handles.push(handle);
    }

    let write_handle = thread::spawn(move || {
    
        let mut num = counter.write().unwrap();
        *num += 1;
        println!(
            "Writer {:?}: Incremented counter to {}",
            thread::current().id(),
            *num
        );
    });

    for handle in read_handles {
        handle.join().unwrap();
    }

    write_handle.join().unwrap();
}

// use std::sync::{Arc, RwLock};
// use std::thread;
fn main2() {
    // RwLock
    let counter = Arc::new(RwLock::new(0));

    let read_handle = {
        let counter = Arc::clone(&counter);
        thread::spawn(move || {

            let num = counter.read().unwrap();
            println!("Reader {:?}: {}", thread::current().id(), *num);

            thread::sleep(std::time::Duration::from_secs(10));
        })
    };

    let write_handle = {
        let counter = Arc::clone(&counter);
        thread::spawn(move || {

            thread::sleep(std::time::Duration::from_secs(1));


            let mut num = counter.write().unwrap();
            *num += 1;
            println!(
                "Writer {:?}: Incremented counter to {}",
                thread::current().id(),*num
            );
        })
    };

    read_handle.join().unwrap();
    write_handle.join().unwrap();
}

fn main3() {
    // RwLock
    let counter = Arc::new(RwLock::new(0));

    let read_handle = {
        let counter = counter.clone();
        thread::spawn(move || {

            let num = counter.read().unwrap();
            println!("Reader#1: {}", *num);

            thread::sleep(std::time::Duration::from_secs(10));
        })
    };

    let write_handle = {
        let counter = counter.clone();
        thread::spawn(move || {

            thread::sleep(std::time::Duration::from_secs(1));

            let mut num = counter.write().unwrap();
            *num += 1;
            println!("Writer : Incremented counter to {}", *num);
        })
    };

    let read_handle_2 = {
        let counter = counter.clone();
        thread::spawn(move || {

            thread::sleep(std::time::Duration::from_secs(2));

            let num = counter.read().unwrap();
            println!("Reader#2: {}", *num);
        })
    };

    read_handle.join().unwrap();
    write_handle.join().unwrap();
    read_handle_2.join().unwrap();
}

// use std::sync::{Arc, RwLock};
// use std::thread;
fn main() {
    main1();
    main2();
    main3();
    // RwLock
    let counter = Arc::new(RwLock::new(0));

    let read_and_write_handle = {
        let counter = Arc::clone(&counter);
        thread::spawn(move || {

            let num = counter.read().unwrap();
            println!("Reader {:?}: {}", thread::current().id(), *num);

            let mut num = counter.write().unwrap();
            *num += 1;
            println!(
                "Reader {:?}: Incremented counter to {}",
                thread::current().id(),*num
            );
        })
    };

    let write_and_read_handle = {
        let counter = Arc::clone(&counter);
        thread::spawn(move || {

            let mut num = counter.write().unwrap();
            *num += 1;
            println!(
                "Writer {:?}: Incremented counter to {}",
                thread::current().id(),*num
            );

            let num = counter.read().unwrap();
            println!("Writer {:?}: {}", thread::current().id(), *num);
        })
    };

    read_and_write_handle.join().unwrap();

    write_and_read_handle.join().unwrap();
}
