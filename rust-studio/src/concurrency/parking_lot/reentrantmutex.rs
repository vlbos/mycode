use parking_lot::RawMutex;
use parking_lot::RawThreadId;
use parking_lot::ReentrantMutex;
// pub type ReentrantMutex<T> = ReentrantMutex<RawMutex, RawThreadId, T>;

pub fn reentrantmutex_example() {
    let lock = ReentrantMutex::new(());
    reentrant(&lock, 10);
    println!("reentrantMutex_example: done");
}
fn reentrant(lock: &ReentrantMutex<()>, i: usize) {
    if i == 0 {
        return;
    }
    let _lock = lock.lock();
    reentrant(lock, i - 1);
}

fn main() {
    reentrantmutex_example();
}
