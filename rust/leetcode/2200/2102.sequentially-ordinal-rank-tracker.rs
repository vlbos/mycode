/*
 * @lc app=leetcode id=2102 lang=rust
 *
 * [2102] Sequentially Ordinal Rank Tracker
 */

// @lc code=start
use std::collections::BinaryHeap;
use std::cmp::Reverse;
struct SORTracker {
h:BinaryHeap<(i32,String)>,
t:BinaryHeap<Reverse<(i32,String)>>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl SORTracker {

    fn new() -> Self {
        Self{t:BinaryHeap::new(),h:BinaryHeap::new()}
    }
    
    fn add(&mut self, name: String, score: i32) {
        self.h.push((-score,name));
        self.t.push(Reverse(self.h.pop().unwrap()));
    }
    
    fn get(&mut self) -> String {
        self.h.push(self.t.pop().unwrap().0);
        self.h.peek().unwrap().1.clone()
    }
}

/**
 * Your SORTracker object will be instantiated and called as such:
 * let obj = SORTracker::new();
 * obj.add(name, score);
 * let ret_2: String = obj.get();
 */
// @lc code=end

