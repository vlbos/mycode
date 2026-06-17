pub fn flume_example() {
    let (tx, rx) = flume::unbounded();
    thread::spawn(move || {
        (0..10).for_each(|i| {
            tx.send(i).unwrap();
        })
    });
    let received: u32 = rx.iter().sum();
    assert_eq!((0..10).sum::<u32>(), received);
}

use flume::{Receiver, Sender, bounded};
use std::thread;
fn main1() {
    // 3
    let (sender, receiver): (Sender<i32>, Receiver<i32>) = bounded(3);
        let producer = thread::spawn(move || {
        for i in 0..5 {
            sender.send(i).unwrap();
            println!("Produced: {}", i);
            std::thread::sleep(std::time::Duration::from_millis(200));
        }
    });
    let consumer = thread::spawn(move || {
        for _ in 0..5 {
            let data = receiver.recv().unwrap();
            println!("Received: {}", data);
        }
    });

        producer.join().unwrap();
    consumer.join().unwrap();
}
fn main() {
    flume_example();
    main1();
    let (tx0, rx0) = flume::unbounded();
    let (tx1, rx1) = flume::unbounded();
    std::thread::spawn(move || {
        tx0.send(true).unwrap();
        tx1.send(42).unwrap();
    });
    flume::Selector::new()
        .recv(&rx0, |b| println!("Received {:?}", b))
        .recv(&rx1, |n| println!("Received {:?}", n))
        .wait();

    let rt = tokio::runtime::Runtime::new().unwrap();
    let (tx, rx) = flume::unbounded();
    rt.block_on(async move {
        tokio::spawn(async move {
            tx.send_async(5).await.unwrap();
        });
        println!("flume async rx: {}", rx.recv_async().await.unwrap());
    });
}
