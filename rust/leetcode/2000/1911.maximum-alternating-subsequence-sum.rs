/*
 * @lc app=leetcode id=1911 lang=rust
 *
 * [1911] Maximum Alternating Subsequence Sum
 */

// @lc code=start
impl Solution {
    pub fn max_alternating_sum(nums: Vec<i32>) -> i64 {
        let mut ans = [nums[0] as i64, 0];
        for &num in nums.iter().skip(1) {
            ans = [
                ans[0].max(ans[1] + num as i64),
                ans[1].max(ans[0] - num as i64),
            ];
        }
        ans[0]
    }
}
// @lc code=end
