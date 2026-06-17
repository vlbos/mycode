use poolite::Builder;
fn main1() -> Result<(), Box<dyn std::error::Error>> {
    let pool = poolite::Pool::new()?;
    pool.push(|| println!("hello"));
    pool.scoped(|scope| {
        scope.push(|| println!("hello"));
    });
    // let pool = poolite::Pool::builder().thread_num(8).build()?;
    Ok(())
}
use poolite::Pool;

use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};

///`cargo run --example arc_mutex`
fn main2() {
    let pool = Pool::new().unwrap();
    // You also can use RwLock instead of Mutex if you read more than write.
    let map = Arc::new(Mutex::new(BTreeMap::<i32, i32>::new()));
    for i in 0..10 {
        let map = map.clone();
        pool.push(move || test(i, map));
    }

    pool.join(); //wait for the pool

    for (k, v) in map.lock().unwrap().iter() {
        println!("key: {}\tvalue: {}", k, v);
    }
}

fn test(msg: i32, map: Arc<Mutex<BTreeMap<i32, i32>>>) {
    let res = fib(msg);
    let mut maplock = map.lock().unwrap();
    maplock.insert(msg, res);
}

fn fib(msg: i32) -> i32 {
    match msg {
        0..2 => 1,
        x => fib(x - 1) + fib(x - 2),
    }
}
fn test1(msg: i32) {
    println!("key: {}\tvalue: {}", msg, fib(msg));
}
fn main() {
    let _ = main1();
    main2();
    let pool = Builder::new()
        .min(1)
        .max(9)
        .daemon(None) // Close
        .timeout(None) //Close
        .name("Worker")
        .stack_size(1024 * 1024 * 2) //2Mib
        .build()
        .unwrap();

    for i in 0..38 {
        pool.push(move || test1(i));
    }

    pool.join(); //wait for the pool
    println!("{:?}", pool);
}
