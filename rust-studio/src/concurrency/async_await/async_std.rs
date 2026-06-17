use async_std::task;
async fn hello_async() {
    println!("Hello, async world!");
}
fn main() {
    task::block_on(hello_async());
}
