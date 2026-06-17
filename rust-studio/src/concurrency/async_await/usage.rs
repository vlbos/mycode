use std::thread;

fn download(url: &str) {
    println!("开始下载: {}", url);
    // 模拟下载过程
    thread::sleep(std::time::Duration::from_secs(2));
    println!("下载完成: {}", url);
}
fn get_two_sites() {
        let thread_one = thread::spawn(|| download("https://course.rs"));
    let thread_two = thread::spawn(|| download("https://fancy.rs"));
        thread_one.join().expect("thread one panicked");
    thread_two.join().expect("thread two panicked");
}
fn main1() {
    get_two_sites();
}
async fn download_async(url: &str) {
    println!("async 开始下载: {}", url);
    // 模拟异步等待
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
    println!("async 下载完成: {}", url);
}
use tokio::join;
async fn get_two_sites_async() {
    //`future` `future`
    //`future`
    let future_one = download_async("https://www.foo.com");
    let future_two = download_async("https://www.bar.com");
    //`future`
    join!(future_one, future_two);
}

#[tokio::main]
async fn main() {
    main1();
    get_two_sites_async().await;
}