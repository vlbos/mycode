/*
 * @lc app=leetcode id=1877 lang=rust
 *
 * [1877] Minimize Maximum Pair Sum in Array
 */

// @lc code=start
impl Solution {
    pub fn min_pair_sum(nums: Vec<i32>) -> i32 {
        let mut ans = 0;
        let mut nums = nums;
        nums.sort();
        for i in 0..nums.len() / 2 {
            ans = ans.max(nums[i] + nums[nums.len()-i-1]);
        }
        ans
    }
}
// @lc code=end
