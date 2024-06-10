// This code shows how you can use join! to run multiple futures concurrently,
//  and wait for them to complete.

use std::time::Duration;
use tokio::time::sleep;

pub async fn task_1(time: u64) {
    sleep(Duration::from_millis(time)).await;
    println!("task_1");
}

pub async fn task_2(time: u64) {
    sleep(Duration::from_millis(time)).await;
    println!("task_2");
}

pub async fn task_3(time: u64) {
    sleep(Duration::from_millis(time)).await;
    println!("task_3");
}

#[tokio::test]
async fn test_join() {
    tokio::join!(task_1(100), task_2(200), task_3(300));
    println!("all tasks done");
}


//  This code shows how you can use select! to run multiple futures concurrently,
//  and wait for the first one to complete.


#[tokio::test]
async fn test_select() {
    tokio::select! {
        _ = task_1(100) => println!("task_1 done"),
        _ = task_2(200) => println!("task_2 done"),
        _ = task_3(300) => println!("task_3 done"),
    }
    println!("one task done");
}



// This code shows how you can use spawn to run multiple futures in parallel, 
// and wait for them to complete. 
// We pass the following to the #[tokio::test] attribute macro: flavor = "multi_thread", 
// worker_threads = 5 which tells it to run the test on multiple threads (max of 5).

#[tokio::test(flavor = "multi_thread", worker_threads = 5)]
async fn test_spawn() {
    let handle_1 = tokio::spawn(task_1(100));
    let handle_2 = tokio::spawn(task_2(100));
    let handle_3 = tokio::spawn(task_3(100));

    handle_1.await.unwrap();
    handle_2.await.unwrap();
    handle_3.await.unwrap();
    println!("all tasks done");
}