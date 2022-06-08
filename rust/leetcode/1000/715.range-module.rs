/*
 * @lc app=leetcode id=715 lang=rust
 *
 * [715] Range Module
 */

// @lc code=start
use std::collections::BTreeMap;
struct RangeModule {
    ranges: BTreeMap<i32,i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RangeModule {
    fn new() -> Self {
        Self {
            ranges: BTreeMap::new(),
        }
    }

    fn add_range(&mut self, left: i32, right: i32) {
        let mut removed = Vec::new();
        let (mut new_left, mut new_right) = (left, right);

        for (&r,&l) in self.ranges.range(left..) {
            if new_right < l {
                break;
            }
            new_left = new_left.min(l);
            new_right = new_right.max(r);
            removed.push(r);
        }
        for d in &removed {
            self.ranges.remove(d);
        }
        self.ranges.insert(new_right,new_left);
    }

    fn query_range(&self, left: i32, right: i32) -> bool {
        if let Some((&r,&l)) = self.ranges.range(left..).next() {
            return left >= l && right <= r;
        }
        false
    }

    fn remove_range(&mut self, left: i32, right: i32) {
        let mut removed = Vec::new();
        let mut todo = Vec::new();

        for (&r,&l) in self.ranges.range(left..){
            if right < l {
                break;
            }
            if l < left {
                todo.push((left, l));
            }
            if right < r {
                todo.push((r, right));
            }
            removed.push(r);
        }
        for d in &removed {
            self.ranges.remove(d);
        }
        for t in &todo {
            self.ranges.insert(t.0,t.1);
        }
    }
}

/**
 * Your RangeModule object will be instantiated and called as such:
 * let obj = RangeModule::new();
 * obj.add_range(left, right);
 * let ret_2: bool = obj.query_range(left, right);
 * obj.remove_range(left, right);
 */
// @lc code=end
