fn fib(n: usize) -> usize {
    if n == 0 || n == 1 {
        return n;
    }
    let (a, b) = rayon::join(|| fib(n - 1), || fib(n - 2)); // runs inside of`pool
    return a + b;
}
fn main1() {
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(4)
        .build()
        .unwrap();
    let n = pool.install(|| fib(20));
    println!("{}", n);
}
use std::sync::atomic::{AtomicUsize, Ordering};
fn main2() {
    // 5
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(5)
        .build()
        .unwrap();
        let v: Vec<usize> = pool.broadcast(|ctx| ctx.index() * ctx.index());
    // [0, 1, 4, 9, 16]
    assert_eq!(v, &[0, 1, 4, 9, 16]);
        let count = AtomicUsize::new(0);
    // AtomicUsize
    pool.broadcast(|_| count.fetch_add(1, Ordering::Relaxed));
    // 5
    assert_eq!(count.into_inner(), 5);
}

fn main() {
    main1();
    main2();
    rayon::ThreadPoolBuilder::new()
        .num_threads(22)
        .build_global()
        .unwrap();
}
