use crossbeam_deque::{Injector, Stealer, Worker};
use std::iter;
use std::sync::Arc;
fn find_task<T>(local: &Worker<T>, global: &Injector<T>, stealers: &[Stealer<T>]) -> Option<T> {
    local.pop().or_else(|| {
        iter::repeat_with(|| {
            global
                .steal_batch_and_pop(local)
                .or_else(|| stealers.iter().map(|s| s.steal()).collect())
        })
        .find(|s| !s.is_retry())
        .and_then(|s| s.success())
    })
}

pub fn main1() {
    let local_worker: Worker<i32> = Worker::new_fifo();
    let global_injector: Injector<i32> = Injector::new();
    let stealer1: Stealer<i32> = local_worker.stealer();
    let stealer2: Stealer<i32> = local_worker.stealer();
    let stealers = vec![stealer1, stealer2];
    // find_task
    if let Some(task) = find_task(&local_worker, &global_injector, &stealers) {
        println!("Found task: {:?}", task);
    } else {
        println!("No task found.");
    }
}
pub fn main2() {
    use crossbeam::queue::ArrayQueue;
    let queue = Arc::new(ArrayQueue::new(100));
    for i in 0..10 {
        let queue = queue.clone();
        thread::spawn(move || {
            queue.push(i).unwrap();
        });
    }
    for _ in 0..10 {
        let queue = queue.clone();
        thread::spawn(move || {
            while let Some(item) = queue.pop() {
                println!("Consumed {}", item);
            }
        });
    }
}
use crossbeam_queue::SegQueue;
use std::thread;
pub fn main() {
    main1();
    main2();
    // SegQueue
    let seg_queue = Arc::new(SegQueue::new());
    let seg_queue_prod = seg_queue.clone();
    let producer = thread::spawn(move || {
        for i in 0..5 {
            seg_queue_prod.push(i);
            println!("Produced: {}", i);
        }
    });
    let consumer = thread::spawn(move || {
        for _ in 0..5 {
            let value = seg_queue.pop();
            println!("Consumed: {:?}", value);
        }
    });
    producer.join().unwrap();
    consumer.join().unwrap();
}
