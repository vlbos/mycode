use crossbeam_utils::Backoff;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering::SeqCst;
fn fetch_mul(a: &AtomicUsize, b: usize) -> usize {
    let backoff = Backoff::new();
    loop {
        let val = a.load(SeqCst);
        if a.compare_exchange(val, val.wrapping_mul(b), SeqCst, SeqCst)
            .is_ok()
        {
            return val;
        }
        backoff.spin();
    }
}

// use crossbeam_utils::Backoff;
use std::sync::atomic::AtomicBool;
// use std::sync::atomic::Ordering::SeqCst;
use std::thread;
fn blocking_wait(ready: &AtomicBool) {
    let backoff = Backoff::new();
    while !ready.load(SeqCst) {
        if backoff.is_completed() {
            thread::park();
        } else {
            backoff.snooze();
        }
    }
}
fn main() {
    use crossbeam_utils::CachePadded;
    let array = [CachePadded::new(1i8), CachePadded::new(2i8)];
    let addr1 = &*array[0] as *const i8 as usize;
    let addr2 = &*array[1] as *const i8 as usize;
    assert!(addr2 - addr1 >= 64);
    assert_eq!(addr1 % 64, 0);
    assert_eq!(addr2 % 64, 0);

    // use crossbeam_utils::CachePadded;
    use std::sync::atomic::AtomicUsize;
    struct Queue<T> {
        head: CachePadded<AtomicUsize>,
        tail: CachePadded<AtomicUsize>,
        buffer: *mut T,
    }

    use crossbeam_utils::thread;
    let var = vec![1, 2, 3];
    thread::scope(|s| {
        s.spawn(|_| {
            println!("A child thread borrowing`var`: {:?}", var);
        });
    })
    .unwrap();

    let u = AtomicUsize::new(10);
    let v = fetch_mul(&u, 11);
    println!("{v}");
    let b = AtomicBool::new(false);
    blocking_wait(&b);
}
