use futures_lite::future;
async fn hello_async() {
    println!("Hello, async world!");
}
pub fn main() {
    future::block_on(hello_async());
}
