use std::sync::mpsc;
use std::sync::mpsc::sync_channel;
use std::thread;
pub fn main1() {
    let (sender, receiver) = mpsc::channel();
    thread::spawn(move || {
        let message = "Hello from the producer!";
        sender.send(message).expect("Failed to send message");
    });

    let received_message = receiver.recv().expect("Failed to receive message");
    println!("Received message: {}", received_message);
}

// use std::sync::mpsc;
// use std::thread;
pub fn main2() {
    let (sender, receiver) = mpsc::channel();
    for i in 0..3 {
        let tx = sender.clone();
        thread::spawn(move || {
            tx.send(i).expect("Failed to send message");
        });
    }
    for _ in 0..3 {
        let received_message = receiver.recv().expect("Failed to receive message");
        println!("Received message: {}", received_message);
    }
}

pub fn main3() {
    let (tx, rx) = sync_channel(3);
    for _ in 0..3 {
        let tx = tx.clone();
        // cloned tx dropped within thread
        thread::spawn(move || tx.send("ok").unwrap());
    }

    drop(tx);
    // Unbounded receiver waiting for all senders to complete.
    while let Ok(msg) = rx.recv() {
        println!("{msg}");
    }
    println!("mpsc_example4 completed");
}
// use std::sync::mpsc;
// use std::thread;
pub fn main() {
    main1();
    main2();
    main3();
    let (sender, receiver) = mpsc::sync_channel::<i32>(0);
    thread::spawn(move || {
        for i in 0..5 {
            sender.send(i).expect("Failed to send message");
            println!("Sent message: {}", i);
        }
    });
    thread::spawn(move || {
        for _ in 0..5 {
            let received_message = receiver.recv().expect("Failed to receive message");
            println!("Received message: {}", received_message);
        }
    });
    thread::sleep(std::time::Duration::from_secs(10));
}
