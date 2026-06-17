pub fn smol_async() {
    smol::block_on(async { println!("Hello from smol") });
}
fn main(){
smol_async();
}