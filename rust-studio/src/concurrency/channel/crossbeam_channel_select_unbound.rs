use crossbeam_channel::{select, unbounded, Receiver, Sender};
    use std::thread;
    pub(crate) fn main() {
        let (sender1, receiver1): (Sender<String>, Receiver<String>) = unbounded();
        let (sender2, receiver2): (Sender<String>, Receiver<String>) = unbounded();
        let producer1 = thread::spawn(move || {
            for i in 0..5 {
                sender1.send(format!("Channel 1: Message {}", i)).unwrap();
                thread::sleep(std::time::Duration::from_millis(200));
            }
        });
        let producer2 = thread::spawn(move || {
            for i in 0..5 {
                sender2.send(format!("Channel 2: Message {}", i)).unwrap();
                thread::sleep(std::time::Duration::from_millis(300));
            }
        });
        // select!
        let consumer = thread::spawn(move || {
            for _ in 0..10 {
                select! {
                recv(receiver1) -> msg1 => {
                match msg1 {
                Ok(msg) => println!("Received from Channel 1: {}", msg),
                Err(_) => println!("Channel 1 closed"),
                }
                }
                recv(receiver2) -> msg2 => {
                match msg2 {
                Ok(msg) => println!("Received from Channel 2: {}", msg),
                Err(_) => println!("Channel 2 closed"),

                }
                }
                }
            }
        });
        producer1.join().unwrap();
        producer2.join().unwrap();
        consumer.join().unwrap();
    }