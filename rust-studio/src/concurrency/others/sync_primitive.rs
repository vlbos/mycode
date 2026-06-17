use std::sync::Arc;
use std::thread;
pub fn async_lock_mutex() {
    use async_lock::Mutex;

    let lock = Arc::new(Mutex::new(0));
    let lock1 = lock.clone();
    smol::block_on(async {
        let mut guard = lock1.lock().await;
        *guard += 1;
    });
    let lock2 = lock.clone();
    smol::block_on(async {
        let guard = lock2.lock().await;
        println!("lock2 {}", *guard);
    });
}
pub fn async_lock_rwlock() {
    use async_lock::RwLock;
    let lock = Arc::new(RwLock::new(0));
    let lock1 = lock.clone();
    smol::block_on(async {
        let mut guard = lock1.write().await;
        *guard += 1;
    });
    let lock2 = lock.clone();
    smol::block_on(async {
        let guard = lock2.read().await;
        println!("lock2 {}", *guard);
    });
}
pub fn async_lock_barrier() {
    use async_lock::Barrier;
    let barrier = Arc::new(Barrier::new(5));
    thread::scope(|s| {
        for _ in 0..5 {
            let barrier = barrier.clone();
            s.spawn(move || {
                smol::block_on(async {
                    println!("before wait");
                    barrier.wait().await;
                    println!("after wait");
                });
            });
        }
    });
}

pub fn waitgroup_example() {
    smol::block_on(async {
        let wg = waitgroup::WaitGroup::new();
        for _ in 0..100 {
            let w = wg.worker();
            let _ = smol::spawn(async move {
                // do work
                drop(w); // drop w means task finished
            });
        }
        wg.wait().await;
    })
}

pub fn wg_example() {
    use std::sync::Arc;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::thread::{sleep, spawn};
    use std::time::Duration;

    use wg::WaitGroup;
    let wg = WaitGroup::new();
    let ctr = Arc::new(AtomicUsize::new(0));
    for _ in 0..5 {
        let ctrx = ctr.clone();
        let t_wg = wg.add(1);
        spawn(move || {
            // mock some time consuming task
            sleep(Duration::from_millis(50));
            ctrx.fetch_add(1, Ordering::Relaxed);
            // mock task is finished
            t_wg.done();
        });
    }
    wg.wait();
    assert_eq!(ctr.load(Ordering::Relaxed), 5);
}

pub fn awaitgroup_example() {
    use awaitgroup::WaitGroup;
    smol::block_on(async {
        let mut wg = WaitGroup::new();
        for _ in 0..5 {
            // Create a new worker.
            let worker = wg.worker();
            let _ = smol::spawn(async {
                // Do some work...
                // This task is done all of its work.
                worker.done();
            });
        }
        // Block until all other tasks have finished their work.

        wg.wait().await;
    });
}
fn main() {
    async_lock_mutex();
    async_lock_rwlock();
    async_lock_barrier();
    waitgroup_example();
    wg_example();
    awaitgroup_example();
}
