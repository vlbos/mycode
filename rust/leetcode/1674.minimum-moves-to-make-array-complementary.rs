/*
 * @lc app=leetcode id=1674 lang=rust
 *
 * [1674] Minimum Moves to Make Array Complementary
 */

// @lc code=start
impl Solution {
    pub fn min_moves(nums: Vec<i32>, limit: i32) -> i32 {
        let n = nums.len();
        let limit = limit as usize;
        let (mut ans, mut dp) = (vec![0; limit * 2 + 2], vec![0; limit * 2 + 1]);
        let mut mn = 0;
        for i in 0..n / 2 {
            let (vii, vjj) = if nums[i] < nums[n - i - 1] {
                (nums[n - i - 1], nums[i])
            } else {
                (nums[i], nums[n - i - 1])
            };
            let (vi, vj) = (vii as usize, vjj as usize);
            dp[vi + vj] -= 1;
            ans[vj + 1] -= 1;
            ans[vi + limit + 1] += 1;
        }
        for i in 1..2 * limit + 1 {
            ans[i] += ans[i - 1];
            mn = mn.min(ans[i] + dp[i]);
        }
        mn + n as i32
    }
}
// @lc code=end
