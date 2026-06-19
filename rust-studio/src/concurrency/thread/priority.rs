use std::thread;
pub fn start_thread_with_priority() {
    let handle1 = thread::spawn(|| {
        assert!(set_current_thread_priority(ThreadPriority::Min).is_ok());
        println!("Hello from a thread5!");
    });

    let handle2 = thread::spawn(|| {
        assert!(set_current_thread_priority(ThreadPriority::Max).is_ok());
        println!("Hello from a thread6!");
    });

    handle1.join().unwrap();
    handle2.join().unwrap();
}

use std::convert::TryInto;
use thread_priority::*;

pub fn main1() {
    start_thread_with_priority();
    assert!(
        set_current_thread_priority(ThreadPriority::Crossplatform(0.try_into().unwrap())).is_ok()
    );
}

// use thread_priority::*;

pub fn main2() {
    main1();
    // assert!(
    //     set_current_thread_priority(ThreadPriority::Os(WinAPIThreadPriority::Lowest.into()))
    //         .is_ok()
    // );
}
pub fn main3() {
    main2();
    use thread_priority::ThreadBuilderExt;
    use thread_priority::*;

    let thread = std::thread::Builder::new()
        .name("MyNewThread".to_owned())
        .spawn_with_priority(ThreadPriority::Max, |result| {
            // This is printed out from within the spawned thread.
            println!("Set priority result: {:?}", result);
            assert!(result.is_ok());
        })
        .unwrap();
    let _ = thread.join();
}
pub fn thread_builder() {
    let thread1 = ThreadBuilder::default()
        .name("MyThread")
        .priority(ThreadPriority::Max)
        .spawn(|result| {
            println!("Set priority result: {:?}", result);
            assert!(result.is_ok());
        })
        .unwrap();

    let thread2 = ThreadBuilder::default()
        .name("MyThread")
        .priority(ThreadPriority::Max)
        .spawn_careless(|| {
            println!("We don't care about the priority result.");
        })
        .unwrap();

    thread1.join().unwrap();
    thread2.join().unwrap();
}

// use thread_priority::*;
pub fn main() {
    main3();
    thread_builder();
    assert!(std::thread::current().get_priority().is_ok());
    println!(
        "This thread's native id is: {:?}",
        std::thread::current().get_native_id()
    );
}
