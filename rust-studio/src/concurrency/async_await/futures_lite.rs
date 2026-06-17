use futures_lite::future;
async fn hello_async() {
    println!("Hello, async world!");
}
fn main() {
    future::block_on(hello_async());
}
