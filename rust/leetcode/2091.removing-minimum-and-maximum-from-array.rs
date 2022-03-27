/*
 * @lc app=leetcode id=2091 lang=rust
 *
 * [2091] Removing Minimum and Maximum From Array
 */

// @lc code=start
impl Solution {
    pub fn minimum_deletions(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mn = *nums.iter().min().unwrap();
        let mx = *nums.iter().max().unwrap();
        let min_idx = nums.iter().position(|x| *x == mn).unwrap();
        let max_idx = nums.iter().position(|x| *x == mx).unwrap();
        let l = min_idx.min(max_idx);
        let r = min_idx.max(max_idx);
        (r+1).min(n-l).min(l+1+n-r) as _
    }
}
// @lc code=end
