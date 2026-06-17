use std::time::Duration;
fn main() {
    // ThreadPool::builder()
    //     .core_threads(4)
    //     .max_threads(8)
    //     .keep_alive(Duration::from_secs(30))
    //     .build();
    //     // pool.execute(|| println!("hello"));

    //     // pool.execute(async {
    //     // ...
    // });

    // let result = pool.execute(|| Ok(1 + 2))?;

    // let res = result
    //     .unwrap()
    //     .get_result_timeout(std::time::Duration::from_secs(3));

    // assert!(res.is_err());
    // if let Err(err) = res {
    //     matches!(err.kind(), threadpool_executor::error::ErrorKind::TimeOut);
    // }
    // let mut task = pool.execute(|| {}).unwrap();
    // task.cancel();
    threadpool_executor_example();
}
pub fn threadpool_executor_example() {
    let pool = threadpool_executor::ThreadPool::new(1);
    let mut expectation = pool.execute(|| "hello, thread pool!").unwrap();
    assert_eq!(expectation.get_result().unwrap(), "hello, thread pool!");

    let pool = threadpool_executor::threadpool::Builder::new()
        .core_pool_size(1)
        .maximum_pool_size(3)
        .keep_alive_time(std::time::Duration::from_secs(300))
        .exeed_limit_policy(threadpool_executor::threadpool::ExceedLimitPolicy::Wait)
        .build();

    pool.execute(|| {
        std::thread::sleep(std::time::Duration::from_secs(3));
    })
    .unwrap();
    let mut exp = pool.execute(|| {}).unwrap();
    let _=exp.cancel();
}
