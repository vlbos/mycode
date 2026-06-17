
pub fn crossfire_mpsc() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    
    rt.block_on(async move {
let (tx, rx) = crossfire::mpsc::bounded_async::<i32>(100);
        tokio::spawn(async move {
            for i in 0i32..10 {
                let _ = tx.send(i).await;
                println!("sent {}", i);
            }
        });
        loop {
            if let Ok(_i) = rx.recv().await {
                println!("recv {}", _i);
            } else {
                println!("rx closed");
                break;
            }
        }
    });
}

pub fn crossfire_mpmc() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    
    rt.block_on(async move {
let (tx, rx) = crossfire::mpmc::bounded_async::<i32>(100);
        let mut sender_handles = vec![];
        for _ in 0..4 {
            let tx = tx.clone();
            let handle = tokio::spawn(async move {
                for i in 0i32..10 {
                    let _ = tx.send(i).await;
                    println!("sent {}", i);
                }
            });
            sender_handles.push(handle);
        }
        let mut handles = vec![];
        for i in 0..4 {
            let rx = rx.clone();
            let handle = tokio::spawn(async move {
                loop {
                    if let Ok(_i) = rx.recv().await {
                        println!("thread {} recv {}", i, _i);
                    } else {
                        println!("rx closed");
                        break;
                    }
                }
            });
            handles.push(handle);
        }
        for handle in sender_handles {
            handle.await.unwrap();
        }
        drop(tx);
        for handle in handles {
            handle.await.unwrap();
        }
    });
}
fn main(){
crossfire_mpmc();
crossfire_mpsc();
}