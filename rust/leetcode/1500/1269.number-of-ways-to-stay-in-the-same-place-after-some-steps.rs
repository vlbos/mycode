/*
 * @lc app=leetcode id=1269 lang=rust
 *
 * [1269] Number of Ways to Stay in the Same Place After Some Steps
 */

// @lc code=start
impl Solution {
    pub fn num_ways(steps: i32, arr_len: i32) -> i32 {
        let m = steps.min(arr_len - 1) as usize;
        let mut dp = vec![0; m + 1];
        dp[0] = 1;
        for _ in 0..steps {
            let mut dpn = vec![0; m + 1];
            for j in 0..=m {
                dpn[j] = dp[j];
                if j > 0 {
                    dpn[j] = (dpn[j] + dp[j - 1]) % 1_000_000_007;
                }
                if j + 1 <= m {
                    dpn[j] = (dpn[j] + dp[j + 1]) % 1_000_000_007;
                }
            }
            dp = dpn;
        }
        dp[0]
    }
}
// @lc code=end
