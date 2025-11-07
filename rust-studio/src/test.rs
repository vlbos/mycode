extern crate rand;

use rand::thread_rng;
use std::time::Instant;
use rand::seq::SliceRandom;

fn main() {
    let mut vec: Vec<u8> = (0..300_000_000u32).map(|x| (x % 255) as u8).collect();
    let start = Instant::now();
    vec.shuffle(&mut thread_rng());
    let end = Instant::now();
    println!("{:?}", end.duration_since(start));
}