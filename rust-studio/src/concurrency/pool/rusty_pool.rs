use rusty_pool::ThreadPool;
use std::sync::Arc;
use std::sync::atomic::AtomicI32;
use std::sync::atomic::Ordering;
use std::thread;
use std::time::Duration;

pub fn rusty_pool_example() {
    let pool = rusty_pool::ThreadPool::default();

    for _ in 1..10 {
        pool.execute(|| {
            println!("Hello from a rusty_pool!");
        });
    }

    pool.join();

    let handle = pool.evaluate(|| {
        thread::sleep(Duration::from_secs(5));
        return 4;
    });
    let result = handle.await_complete();
    assert_eq!(result, 4);
}

async fn some_async_fn(x: i32, y: i32) -> i32 {
    x + y
}

async fn other_async_fn(x: i32, y: i32) -> i32 {
    x - y
}

pub fn rusty_pool_example2() {
    let pool = rusty_pool::ThreadPool::default();

    let handle = pool.complete(async {
        let a = some_async_fn(4, 6).await; // 10
        let b = some_async_fn(a, 3).await; // 13
        let c = other_async_fn(b, a).await; // 3
        some_async_fn(c, 5).await // 8
    });
    assert_eq!(handle.await_complete(), 8);

    let count = Arc::new(AtomicI32::new(0));
    let clone = count.clone();
    pool.spawn(async move {
        let a = some_async_fn(3, 6).await; // 9
        let b = other_async_fn(a, 4).await; // 5
        let c = some_async_fn(b, 7).await; // 12
        clone.fetch_add(c, Ordering::SeqCst);
    });
    pool.join();
    assert_eq!(count.load(Ordering::SeqCst), 12);
}

pub fn rusty_pool_example3() {
    let pool = ThreadPool::default();
    for _ in 0..10 {
        pool.execute(|| thread::sleep(Duration::from_secs(10)))
    }

    // join
    pool.join_timeout(Duration::from_secs(5));

    let count = Arc::new(AtomicI32::new(0));
    for _ in 0..15 {
        let clone = count.clone();
        pool.execute(move || {
            thread::sleep(Duration::from_secs(5));
            clone.fetch_add(1, Ordering::SeqCst);
        });
    }

    // ThreadPool

    pool.shutdown_join();
    assert_eq!(count.load(Ordering::SeqCst), 15);
}
pub fn main() {
    rusty_pool_example();
    rusty_pool_example2();
    rusty_pool_example3();
}
