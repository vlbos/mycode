use std::thread;
fn main() {
    let (tx, rx) = kanal::unbounded();
    thread::spawn(move || {
        (0..10).for_each(|i| {
            tx.send(i).unwrap();
        });
        drop(tx)
    });
    let received: u32 = rx.sum();
    println!("received sum: {}", received);

    let rt = tokio::runtime::Runtime::new().unwrap();

    let (tx, rx) = kanal::unbounded_async();
    rt.block_on(async move {
        tokio::spawn(async move {
            tx.send(5).await.unwrap();
        });
        println!("rx: {}", rx.recv().await.unwrap());
    });
}
