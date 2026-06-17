 use tokio::sync::mpsc;
    async fn some_computation(input: u32) -> String {
        format!("the result of computation {}", input)
    }
#[tokio::main]
    pub async fn main() {
        let (tx, mut rx) = mpsc::channel(100);
        tokio::spawn(async move {
            for i in 0..10 {
                let res = some_computation(i).await;
                tx.send(res).await.unwrap(); // 10
            }
        });
        while let Some(res) = rx.recv().await {
            // 10 ,
            println!("got = {}", res);
        }
    }