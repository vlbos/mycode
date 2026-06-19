use async_std::task;
async fn hello_async() {
    println!("Hello, async world!");
}
pub fn main() {
    task::block_on(hello_async());
}
