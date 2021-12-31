/*
 * @lc app=leetcode id=729 lang=rust
 *
 * [729] My Calendar I
 */

// @lc code=start
use std::collections::BTreeMap;
struct MyCalendar {
 tree: BTreeMap<i32, i32>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyCalendar {

    fn new() -> Self {
         Self{tree:BTreeMap::new()}
    }
    
    fn book(&mut self, start: i32, end: i32) -> bool {
        for (&s, _) in self.tree.range(start..).take(1) {
            if s < end { return false; }
        }
        for (_, &e) in self.tree.range(..start).rev().take(1) {
            if e > start { return false; }
        }
        self.tree.insert(start, end);
        true
    }
}

/**
 * Your MyCalendar object will be instantiated and called as such:
 * let obj = MyCalendar::new();
 * let ret_1: bool = obj.book(start, end);
 */
// @lc code=end

