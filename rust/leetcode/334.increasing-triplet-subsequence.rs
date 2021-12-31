/*
 * @lc app=leetcode id=334 lang=rust
 *
 * [334] Increasing Triplet Subsequence
 */

// @lc code=start
impl Solution {
    pub fn increasing_triplet(nums: Vec<i32>) -> bool {
        if nums.len() < 3 {
            return false;
        }
        let mut small = i32::MAX;
        let mut mid = i32::MAX;
        for n in nums {
            if n <= small {
                small = n;
            } else if n <= mid {
                mid = n;
            } else {
                return true;
            }
        }
        false
    }
}
// @lc code=end
