/*
 * @lc app=leetcode id=1155 lang=rust
 *
 * [1155] Number of Dice Rolls With Target Sum
 */

// @lc code=start
impl Solution {
    pub fn num_rolls_to_target(n: i32, k: i32, target: i32) -> i32 {
        let modmax = 1000_000_007;
        let (n, k, t) = (n as usize, k as usize, target as usize);
        let mut dp = vec![0; t + 1];
        dp[0] = 1;
        for _ in 0..n {
            for i in (0..=t).rev() {
                dp[i] = 0;
                for j in 1..=i.min(k) {
                    dp[i] = (dp[i] + dp[i - j]) % modmax;
                }
            }
        }
        dp[t]
    }
}
// @lc code=end
