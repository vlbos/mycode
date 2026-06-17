fn main() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let (tx, rx) = async_channel::unbounded();
    rt.block_on(async move {
        tokio::spawn(async move {
            tx.send(5).await.unwrap();
        });
        println!("rx: {}", rx.recv().await.unwrap());
    });
}
