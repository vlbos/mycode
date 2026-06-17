use arc_swap::ArcSwap;
use std::sync::Arc;
fn main() {
    // ArcSwap
    let data = ArcSwap::new(1.into());
        println!("Initial Value: {}", data.load());
        data.store(Arc::new(2));
        println!("New Value: {}", data.load());
}
