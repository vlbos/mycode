use std::sync::{Arc, Barrier};
use std::thread;
 use rand::Rng;
use std::time::{self,Duration};
fn main1() {
    // Barrier
    let barrier = Arc::new(Barrier::new(3)); // 3
        let mut handles = vec![];
    for id in 0..3 {
        let barrier = Arc::clone(&barrier);
        let handle = thread::spawn(move || {
                        println!("Thread {} working", id);
            thread::sleep(std::time::Duration::from_secs(id as u64));
                        barrier.wait();
                        println!("Thread {} resumed", id);
        });
        handles.push(handle);
    }

        for handle in handles {
        handle.join().unwrap();
    }
}
fn main() {
    main1();
    let barrier = Arc::new(Barrier::new(10));
    let mut handles = vec![];
    for _ in 0..10 {
        let barrier = barrier.clone();
        handles.push(thread::spawn(move || {
            println!("before wait1");
            let dur = rand::thread_rng().gen_range(100..1000);
            thread::sleep(std::time::Duration::from_millis(dur));
            //step1
            barrier.wait();
            println!("after wait1");
            thread::sleep(time::Duration::from_secs(1));
            //step2
            barrier.wait();
            println!("after wait2");
        }));

      
    }
  for handle in handles {
            handle.join().unwrap();
        }
}
