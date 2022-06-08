/*
 * @lc app=leetcode id=1000 lang=rust
 *
 * [1000] Minimum Cost to Merge Stones
 */

// @lc code=start
impl Solution {
    pub fn merge_stones(stones: Vec<i32>, k: i32) -> i32 {
        let n = stones.len();
        let k = k as usize;
        if (n - 1) % (k - 1) > 0 {
            return -1;
        }
        let mut dp = vec![vec![0; n+1]; n +1];
        let mut sum = vec![0; n + 1];
        for i in 1..=n {
            sum[i] = sum[i - 1] + stones[i - 1];
        }
        for len in k..=n {
            for i in 1..=n - len + 1 {
                let mut j = i + len - 1;
                dp[i][j] = i32::MAX;
                for p in (i..j).step_by(k - 1) {
                    dp[i][j] = dp[i][j].min(dp[i][p] + dp[p + 1][j]);
                }
                if (j - i) % (k - 1) == 0 {
                    dp[i][j] += sum[j] - sum[i - 1];
                }
            }
        }
        dp[1][n]
    }
}
// @lc code=end
