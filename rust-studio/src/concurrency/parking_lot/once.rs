use parking_lot::Once;
use std::thread;
static mut VAL: usize = 0;
static INIT: Once = Once::new();
fn get_cached_val() -> usize {
    unsafe {
        INIT.call_once(|| {
            println!("initializing once");
            thread::sleep(std::time::Duration::from_secs(1));
            VAL = 100;
        });
        VAL
    }
}
pub fn main() {
    let _handle = thread::spawn(|| {
        println!("thread 1 get_cached_val: {}", get_cached_val());
    });
    println!("get_cached_val: {}", get_cached_val());
}
