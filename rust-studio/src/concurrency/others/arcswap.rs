use std::sync::Arc;
use std::thread;
use arc_swap::ArcSwap;
pub fn arc_swap_example() {
    let value = ArcSwap::from(Arc::new(5));
    thread::scope(|scope| {
        scope.spawn(|| {
            let new_value = Arc::new(4);
            value.store(new_value);
        });
        for _ in 0..10 {
            scope.spawn(|| {
                loop {
                    let v = value.load();
                    println!("value is {}", v);
                    return;
                }
            });
        }
    })
   
}
fn main(){
arc_swap_example();
}