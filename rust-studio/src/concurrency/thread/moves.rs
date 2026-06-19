use std::thread;
pub fn start_one_thread_with_move() {
    let x = 100;

    let handle = thread::spawn(move || {
        println!("Hello from a thread with move, x={}!", x);
    });

    handle.join().unwrap();

    let handle = thread::spawn(move || {
        println!("Hello from a thread with move again, x={}!", x);
    });
    handle.join().unwrap();

    let handle = thread::spawn(|| {
        println!("Hello from a thread without move");
    });
    handle.join().unwrap();
}

// pub fn start_one_thread_with_move2() {
//     let x = vec![1, 2, 3];

//     let handle = thread::spawn(move || {
//         println!("Hello from a thread with move, x={:?}!", x);
//     });

//     handle.join().unwrap();

//     let handle = thread::spawn(move || {
//         println!("Hello from a thread with move again, x={:?}!", x);
//     });
//     handle.join().unwrap();

//     let handle = thread::spawn(|| {
//         println!("Hello from a thread without move");
//     });
//     handle.join().unwrap();
// }
pub fn main() {
    start_one_thread_with_move();
}
