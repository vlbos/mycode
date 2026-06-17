  use crossbeam_channel::{bounded, Receiver, Sender};
    use std::thread;
    pub(crate) fn main() {
        let (sender, receiver): (Sender<i32>, Receiver<i32>) = bounded(10);
        let producer = thread::spawn(move || {
            for i in 0..10 {
                sender.send(i).unwrap();
            }
        });
        let consumer = thread::spawn(move || {
            for _ in 0..10 {
                let data = receiver.recv().unwrap();
                println!("Received: {}", data);
            }
        });

        producer.join().unwrap();
        consumer.join().unwrap();
    }