/*
 * @lc app=leetcode id=528 lang=rust
 *
 * [528] Random Pick with Weight
 */

// @lc code=start
use rand::distributions::WeightedIndex;
use rand::prelude::*;
struct Solution {
    rng: ThreadRng,
    dist: WeightedIndex<i32>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Solution {

    fn new(w: Vec<i32>) -> Self {
        let rng = thread_rng();
        let dist = WeightedIndex::new(w).unwrap();
        Self{rng,dist}
    }
    
    fn pick_index(&mut self) -> i32 {
       self.rng.sample(&self.dist) as i32
    }
}

/**
 * Your Solution object will be instantiated and called as such:
 * let obj = Solution::new(w);
 * let ret_1: i32 = obj.pick_index();
 */
// @lc code=end

