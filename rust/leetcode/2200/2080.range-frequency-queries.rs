/*
 * @lc app=leetcode id=2080 lang=rust
 *
 * [2080] Range Frequency Queries
 */

// @lc code=start
use std::collections::HashMap;
struct RangeFreqQuery {
    occurence: HashMap<i32, Vec<i32>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RangeFreqQuery {
    fn new(arr: Vec<i32>) -> Self {
        let mut occurence = HashMap::new();
        for (i, &a) in arr.iter().enumerate() {
            occurence.entry(a).or_insert(Vec::new()).push(i as i32);
        }
        Self { occurence }
    }

    fn query(&self, left: i32, right: i32, value: i32) -> i32 {
        let d = Vec::new();
        let q = self.occurence.get(&value).unwrap_or(&d);
        let l = q.partition_point(|&x| x < left);
        let r = q.partition_point(|&x| x <= right);
        (r - l) as _
    }
}
/**
 * Your RangeFreqQuery object will be instantiated and called as such:
 * let obj = RangeFreqQuery::new(arr);
 * let ret_1: i32 = obj.query(left, right, value);
 */
// @lc code=end
