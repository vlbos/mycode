use std::sync::mpsc::channel;

use std::sync::Arc;
use std::sync::Barrier;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;
fn main1() {
    // 4
    let pool = threadpool::ThreadPool::new(4);

    let (sender, receiver) = channel();

    for i in 0..8 {
        let sender = sender.clone();
        pool.execute(move || {
            let result = i * 2;
            sender.send(result).expect(" ");
        });
    }

    for _ in 0..8 {
        let result = receiver.recv().expect(" ");
        println!(" : {}", result);
    }
}
fn main() {
    main1();
    // create at least as many workers as jobs or you will deadlock yourself
    let n_workers = 42;
    let n_jobs = 23;
    let pool = threadpool::ThreadPool::new(n_workers);
    let an_atomic = Arc::new(AtomicUsize::new(0));

    assert!(n_jobs <= n_workers, "too many jobs, will deadlock");

    // barrier,
    let barrier = Arc::new(Barrier::new(n_jobs + 1));
    for _ in 0..n_jobs {
        let barrier = barrier.clone();
        let an_atomic = an_atomic.clone();

        pool.execute(move || {
            an_atomic.fetch_add(1, Ordering::Relaxed);

            barrier.wait();
        });
    }

    barrier.wait();
    assert_eq!(an_atomic.load(Ordering::SeqCst), /* n_jobs = */ 23);
}
