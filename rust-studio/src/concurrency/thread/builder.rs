#![feature(thread_spawn_unchecked)]
use std::thread::Builder;
pub fn start_one_thread_by_builder() {
    let builder = thread::Builder::new()
        .name("foo".into()) // set thread name
        .stack_size(32 * 1024); // set stack size

    let handler = builder
        .spawn(|| {
            println!("Hello from a thread!");
        })
        .unwrap();

    handler.join().unwrap();
}

use std::thread;
fn main() {
    start_one_thread_by_builder();
    let builder = Builder::new();
    let x = 1;
    let thread_x = &x;
    let handler = unsafe {
        builder
            .spawn_unchecked(move || {
                println!("x = {}", *thread_x);
            })
            .unwrap()
    };

    // caller has to ensure ‘join()‘ is called, otherwise
    // it is possible to access freed memory if ‘x‘ gets
    // dropped before the thread closure is executed!
    handler.join().unwrap();
}
