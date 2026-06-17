use crossbeam_channel::{unbounded, Receiver, Sender};
    use std::thread;
    pub(crate) fn main() {
        let (sender, receiver): (Sender<i32>, Receiver<i32>) = unbounded();
        let producer = thread::spawn(move || {
            for i in 0.. {
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