use std::sync::Arc;
use std::sync::Mutex;
fn main() {
    let (book_reviews_w, book_reviews_r) = evmap::new();

    // start some writers.
    // since evmap does not support concurrent writes, we need
    // to protect the write handle by a mutex.
    let w = Arc::new(Mutex::new(book_reviews_w));
    let writers: Vec<_> = (0..4)
        .map(|i| {
            let w = w.clone();
            std::thread::spawn(move || {
                let mut w = w.lock().unwrap();
                w.insert(i, true);
                w.publish();
            })
        })
        .collect();
    // eventually we should see all the writes
    while book_reviews_r.len() < 4 {
        std::thread::yield_now();
    }
    // all the threads should eventually finish writing
    for w in writers.into_iter() {
        assert!(w.join().is_ok());
    }
}
