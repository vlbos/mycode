/*
 * @lc app=leetcode id=215 lang=rust
 *
 * [215] Kth Largest Element in an Array
 */

// @lc code=start
impl Solution {
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        let mut n = nums;
        n.sort_by(|a,b|b.cmp(a));
        n[k as usize -1]
    }
}
// @lc code=end

