use executor_service::Executors;
use std::sync::Arc;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;
use std::thread;
use std::thread::sleep;
use std::time::Duration;
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    executor_service_example();

    // let pool = Executors::new_fixed_thread_pool(4)?;

    let mut pool = Executors::new_cached_thread_pool(None)?;

    let _ = pool.execute(|| println!("hello"));

    // // future
    // pool.spawn(async {

    //     // ...
    // });
    // let result = pool.submit_sync(|| {
    //     // run task
    //     let result=1;
    //     return result;
    // })?;
    // ThreadPoolExecutor::builder()
    //     .core_threads(4)
    //     .max_threads(8)
    //     .build()?;
    Ok(())
}
pub fn executor_service_example() {
    use executor_service::Executors;

    let mut executor_service = Executors::new_fixed_thread_pool(10).expect(
        "Failed to create the thread
pool",
    );

    let counter = Arc::new(AtomicUsize::new(0));

    for _ in 0..10 {
        let counter = counter.clone();
        let _ = executor_service.execute(move || {
            thread::sleep(Duration::from_millis(100));
            counter.fetch_add(1, Ordering::SeqCst);
        });
    }

    thread::sleep(Duration::from_millis(1000));

    assert_eq!(counter.load(Ordering::SeqCst), 10);

    let mut executor_service = Executors::new_fixed_thread_pool(2).expect(
        "Failed
to create the thread pool",
    );

    let some_param = "Mr White";
    let res = executor_service
        .submit_sync(move || {
            sleep(Duration::from_secs(5));

            println!("Hello {:}", some_param);
            println!("Long computation finished");
            2
        })
        .expect("Failed to submit function");

    println!("Result: {:#?}", res);
    assert_eq!(res, 2);
}
