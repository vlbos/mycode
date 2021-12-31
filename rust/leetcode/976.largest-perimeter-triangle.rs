/*
 * @lc app=leetcode id=976 lang=rust
 *
 * [976] Largest Perimeter Triangle
 */

// @lc code=start
impl Solution {
    pub fn largest_perimeter(nums: Vec<i32>) -> i32 {
         let mut n = nums;
        n.sort();
        let mut a = 0;
        for i in (2..n.len()).rev() {
            if n[i - 2] + n[i - 1] > n[i] {
                return n[i - 2] + n[i - 1] + n[i];
            }
        }
        0
    }
}
// @lc code=end

