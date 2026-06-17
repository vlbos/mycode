
pub fn scoped_threadpool() {
    let mut pool = scoped_threadpool::Pool::new(4);

    let mut vec = vec![0, 1, 2, 3, 4, 5, 6, 7];

    // Use the threads as scoped threads that can reference anything outside this closure
    pool.scoped(|s| {
        // Create references to each element in the vector ...
        for e in &mut vec {
            // ... and add 1 to it in a seperate thread

            s.execute(move || {
                *e += 1;
            });
        }
    });

    assert_eq!(vec, vec![1, 2, 3, 4, 5, 6, 7, 8]);
}
fn main() {
    println!("scoped_threadpool");
    scoped_threadpool();
}