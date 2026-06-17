use fast_threadpool::ThreadPoolConfig;
pub fn fast_threadpool_example() -> Result<(), fast_threadpool::ThreadPoolDisconnected> {
    let threadpool =
        fast_threadpool::ThreadPool::start(ThreadPoolConfig::default(), ()).into_sync_handler();

    assert_eq!(4, threadpool.execute(|_| { 2 + 2 })?);

    Ok(())
}
fn main() {
    let _ = fast_threadpool_example();
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let threadpool = fast_threadpool::ThreadPool::start(ThreadPoolConfig::default(), ())
            .into_async_handler();
        assert_eq!(4, threadpool.execute(|_| { 2 + 2 }).await.unwrap());
    });
}
