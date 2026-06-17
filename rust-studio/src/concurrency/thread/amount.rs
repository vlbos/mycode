use std::{io, thread};

fn main1() -> Result<(), ()> {
    let count = thread::available_parallelism().unwrap().get();
    assert!(count >= 1_usize);

    Ok(())
}
// affinity (不支持 MacOS) crate 可以提供当前的 CPU 核数:
//  let cores: Vec<usize> = (0..affinity::get_core_num()).step_by(2).collect();
//  println!("cores : {:?}", &cores);
// 更多的场景下，我们使用 num_cpus 获取 CPU 的核数（逻辑核）：
//  use num_cpus;
//  let num = num_cpus::get();
fn main() {
    let _ = main1();
    let count = thread::available_parallelism().unwrap().get();
    println!("available_parallelism: {}", count);

    if let Some(count) = num_threads::num_threads() {
        println!("num_threads: {}", count);
    } else {
        println!("num_threads: not supported");
    }

    let count = thread_amount::thread_amount();
    if !count.is_none() {
        println!("thread_amount: {}", count.unwrap());
    }

    let count = num_cpus::get();
    println!("num_cpus: {}", count);
}
