/*
 * @lc app=leetcode id=732 lang=rust
 *
 * [732] My Calendar III
 */

// @lc code=start
use std::collections::BTreeMap;
struct MyCalendarThree {
    delta: BTreeMap<i32, i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyCalendarThree {
    fn new() -> Self {
        Self {
            delta: BTreeMap::new(),
        }
    }

    fn book(&mut self, start: i32, end: i32) -> i32 {
        self.delta.entry(start).and_modify(|x| *x += 1).or_insert(1);
        self.delta.entry(end).and_modify(|x| *x -= 1).or_insert(-1);
        let mut active = 0;
        let mut ans = 0;
        for &v in self.delta.values() {
            active += v;
            if active > ans {
                ans = active;
            }
        }
        ans
    }
}

/**
 * Your MyCalendarThree object will be instantiated and called as such:
 * let obj = MyCalendarThree::new();
 * let ret_1: i32 = obj.book(start, end);
 */
// @lc code=end
