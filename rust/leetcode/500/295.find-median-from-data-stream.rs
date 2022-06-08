/*
 * @lc app=leetcode id=295 lang=rust
 *
 * [295] Find Median from Data Stream
 */

// @lc code=start
use std::collections::BinaryHeap;
use std::cmp::Reverse;
struct MedianFinder {
    que_min: BinaryHeap<i32>,
    que_max: BinaryHeap<Reverse<i32>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MedianFinder {
    fn new() -> Self {
        Self {
            que_min: BinaryHeap::new(),
            que_max: BinaryHeap::new(),
        }
    }

    fn add_num(&mut self, num: i32) {
        if self.que_min.is_empty() || *self.que_min.peek().unwrap() >= num {
            self.que_min.push(num);
            if self.que_min.len() > self.que_max.len() + 1 {
                if let Some(val) = self.que_min.pop() {
                    self.que_max.push(Reverse(val));
                }
            }
        } else {
            self.que_max.push(Reverse(num));
            if self.que_max.len() > self.que_min.len() {
                if let Some(Reverse(val)) = self.que_max.pop() {
                    self.que_min.push(val);
                }
            }
        }
    }

    fn find_median(&self) -> f64 {
        if (self.que_min.len() + self.que_max.len()) % 2 > 0 {
            *self.que_min.peek().unwrap() as _
        } else {
            (*self.que_min.peek().unwrap() as f64 + self.que_max.peek().unwrap().0 as f64) / 2.0
        }
    }
}

/**
 * Your MedianFinder object will be instantiated and called as such:
 * let obj = MedianFinder::new();
 * obj.add_num(num);
 * let ret_2: f64 = obj.find_median();
 */
// @lc code=end
