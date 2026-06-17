// i64 AtomicI64
// pub unsafe fn from_ptr<'a>(ptr: *mut i64) -> &'a AtomicI64
// pub const fn as_ptr(&self) -> *mut i64
// pub fn get_mut(&mut self) -> &mut i64
// pub fn from_mut(v: &mut i64) -> &mut AtomicI64
// pub fn get_mut_slice(this: &mut [AtomicI64]) -> &mut [i64]
// pub fn from_mut_slice(v: &mut [i64]) -> &mut [AtomicI64]
// pub fn into_inner(self) -> i64
// // pub fn load(&self, order: Ordering) -> i64
// pub fn store(&self, val: i64, order: Ordering)
// pub fn swap(&self, val: i64, order: Ordering) -> i64
// pub fn compare_and_swap(&self, current: i64, new: i64, order: Ordering) -> i64 // pub fn compare_exchange(
// &self,
// current: i64,
// new: i64,
// success: Ordering,
// failure: Ordering
// ) -> Result<i64, i64>
// pub fn compare_exchange_weak(
// &self,
// current: i64,
// new: i64,
// success: Ordering,
// failure: Ordering
// ) -> Result<i64, i64>
// pub fn fetch_add(&self, val: i64, order: Ordering) -> i64
// pub fn fetch_sub(&self, val: i64, order: Ordering) -> i64
// pub fn fetch_and(&self, val: i64, order: Ordering) -> i64
// pub fn fetch_nand(&self, val: i64, order: Ordering) -> i64

// pub fn fetch_or(&self, val: i64, order: Ordering) -> i64
// pub fn fetch_xor(&self, val: i64, order: Ordering) -> i64
// pub fn fetch_update<F>(
// &self,
// set_order: Ordering,
// fetch_order: Ordering,
// f: F
// ) -> Result<i64, i64>
// where
// F: FnMut(i64) -> Option<i64>,
// pub fn fetch_max(&self, val: i64, order: Ordering) -> i64
// pub fn fetch_min(&self, val: i64, order: Ordering) -> i64
fn main() {
    use std::sync::atomic::{AtomicI64, Ordering};
    let atomic_num = AtomicI64::new(0);
        let num = atomic_num.load(Ordering::Relaxed);
        let old = atomic_num.fetch_add(10, Ordering::SeqCst);
        atomic_num.compare_and_swap(old, 100, Ordering::SeqCst);
        let swapped = atomic_num.swap(200, Ordering::Release);
        atomic_num.store(1000, Ordering::Relaxed);
}

use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
fn main() {
        let atomic_bool = AtomicBool::new(false);
    // true
    let producer_thread = thread::spawn(move || {
        // Ordering::Relaxed
        atomic_bool.store(true, Ordering::Relaxed);
    });
        let consumer_thread = thread::spawn(move || {
        // Ordering::Relaxed
        let value = atomic_bool.load(Ordering::Relaxed);
        println!("Received value: {}", value);
    });
        producer_thread.join().unwrap();
    consumer_thread.join().unwrap();
}

use std::sync::atomic::{AtomicBool, Ordering};

use std::thread;
fn main() {
        let atomic_bool = AtomicBool::new(false);
    // true
    let producer_thread = thread::spawn(move || {
        // true
        atomic_bool.store(true, Ordering::Release);
    });
        let consumer_thread = thread::spawn(move || {
        // true
        while !atomic_bool.load(Ordering::Acquire) {
            // Acquire
                    }
        println!("Received value: true");
    });
        producer_thread.join().unwrap();
    consumer_thread.join().unwrap();
}

use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
fn main() {
        let atomic_bool = AtomicBool::new(false);
    // true
    let producer_thread = thread::spawn(move || {
        // true
        atomic_bool.store(true, Ordering::Release);
    });
        let consumer_thread = thread::spawn(move || {
        // true
        while !atomic_bool.load(Ordering::Acquire) {
            // Release
                    }
        println!("Received value: true");
    });
        producer_thread.join().unwrap();
    consumer_thread.join().unwrap();
}

use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
fn main() {
        let atomic_bool = AtomicBool::new(false);
    // true
    let producer_thread = thread::spawn(move || {
        // true
        atomic_bool.store(true, Ordering::AcqRel);
    });

        let consumer_thread = thread::spawn(move || {
        // true
        while !atomic_bool.load(Ordering::AcqRel) {
            // AcqRel
                    }
        println!("Received value: true");
    });
        producer_thread.join().unwrap();
    consumer_thread.join().unwrap();
}

use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
fn main() {
        let atomic_bool = AtomicBool::new(false);
    // true
    let producer_thread = thread::spawn(move || {
        // true
        atomic_bool.store(true, Ordering::SeqCst);
    });
        let consumer_thread = thread::spawn(move || {
        // true

        while !atomic_bool.load(Ordering::SeqCst) {
            // SeqCst
                    }
        println!("Received value: true");
    });
        producer_thread.join().unwrap();
    consumer_thread.join().unwrap();
}
