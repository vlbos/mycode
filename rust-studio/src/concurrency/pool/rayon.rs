
use rayon::ThreadPoolBuilder;

fn main1() {
    let _builder = ThreadPoolBuilder::new();

    let _builder = ThreadPoolBuilder::new().num_threads(4); 

    let builder = ThreadPoolBuilder::new().thread_name(|i| {
        format!(
            "
worker-{}",
            i
        )
    });

    let _pool = ThreadPoolBuilder::new()
        .num_threads(4)
        .thread_name(|i| format!("worker-{}", i))
        .build()
        .unwrap(); // unwrap()

    rayon::ThreadPoolBuilder::new()
        .num_threads(22)
        .build_global()
        .unwrap();
}
fn fib(n: usize) -> usize {
    if n == 0 || n == 1 {
        return n;
    }
    let (a, b) = rayon::join(|| fib(n - 1), || fib(n - 2)); // rayon
    return a + b;
}

pub fn rayon_threadpool() {
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(8)
        .build()
        .unwrap();
    let n = pool.install(|| fib(20));
    println!("{}", n);
}

pub fn rayon_threadpool2() {
    scoped_tls::scoped_thread_local!(static POOL_DATA: Vec<i32>);
    let pool_data = vec![1, 2, 3];

    // We haven’t assigned any TLS data yet.
    assert!(!POOL_DATA.is_set());
    rayon::ThreadPoolBuilder::new()
        .build_scoped(
            // Borrow pool_data in TLS for each thread.
            |thread| POOL_DATA.set(&pool_data, || thread.run()),
            // Do some work that needs the TLS data.
            |pool| pool.install(|| assert!(POOL_DATA.is_set())),
        )
        .unwrap();

    //Once we've returned,`pool_data` is no longer borrowed.
    drop(pool_data);
}
fn main(){
main1();
rayon_threadpool();
rayon_threadpool2();
}