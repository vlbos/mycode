/*
 * @lc app=leetcode id=1984 lang=rust
 *
 * [1984] Minimum Difference Between Highest and Lowest of K Scores
 */

// @lc code=start
impl Solution {
    pub fn minimum_difference(nums: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let mut nums = nums;
        nums.sort();
        let mut ans = i32::MAX;
        for i in 0..nums.len() + 1 - k {
            ans = ans.min(nums[i + k - 1] - nums[i]);
        }
        ans
    }
}
// @lc code=end
