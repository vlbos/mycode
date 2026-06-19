use std::thread;
pub fn main() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let (tx, mut rx) = futures_channel::mpsc::channel(3);
    rt.block_on(async move {
        tokio::spawn(async move {
            for _ in 0..3 {
                let mut tx = tx.clone();
                thread::spawn(move || tx.start_send("ok"));
            }
            drop(tx);
        });
        // Unbounded receiver waiting for all senders to complete.
        while let Ok(msg) = rx.try_recv() {
            println!("{:?}", msg);
        }
        println!("futures_channel_mpsc_example completed");
    });

    use futures::channel::oneshot;
    use std::time::Duration;
    let (sender, receiver) = oneshot::channel::<i32>();
    thread::spawn(|| {
        println!("THREAD: sleeping zzz...");
        thread::sleep(Duration::from_millis(1000));
        println!("THREAD: i'm awake! sending.");
        sender.send(3).unwrap();
    });
    println!("MAIN: doing some useful stuff");
    futures::executor::block_on(async {
        println!("MAIN: waiting for msg...");
        println!("MAIN: got: {:?}", receiver.await)
    });
}
