 use std::sync::atomic::Ordering;
use std::sync::atomic::AtomicI64;
use std::sync::Arc;
use go_spawn::{go,join};
pub fn go_thread() {
    let counter = Arc::new(AtomicI64::new(0));
    let counter_cloned = counter.clone();

    // Spawn a thread that captures values by move.
    go! {
    for _ in 0..100 {
    counter_cloned.fetch_add(1, Ordering::SeqCst);


    }
     }

    assert!(join!().is_ok());
    assert_eq!(counter.load(Ordering::SeqCst), 100);
}
fn main(){
go_thread();
}