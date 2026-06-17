use std::thread;
// pub fn wrong_start_threads_without_scoped() {
//     let mut a = vec![1, 2, 3];
//     let mut x = 0;

//     thread::spawn(move || {
//         println!("hello from the first scoped thread");
//         dbg!(&a);
//     });
//     thread::spawn(move || {
//         println!("hello from the second scoped thread");
//         x += a[0] + a[2];
//     });
//     println!("hello from the main thread");

//     // After the scope, we can modify and access our variables again:
//     a.push(4);
//     assert_eq!(x, a.len());
// }

pub fn start_scoped_threads() {
    let mut a = vec![1, 2, 3];
    let mut x = 0;

    thread::scope(|s| {
        s.spawn(|| {
            println!("hello from the first scoped thread");
            dbg!(&a);
        });
        s.spawn(|| {
            println!("hello from the second scoped thread");
            x += a[0] + a[2];
        });
        println!("hello from the main thread");
    });

    // After the scope, we can modify and access our variables again:
    a.push(4);
    assert_eq!(x, a.len());
}
fn main() {
    // wrong_start_threads_without_scoped();
    start_scoped_threads();
}
