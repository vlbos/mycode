/*
 * @lc app=leetcode id=1444 lang=rust
 *
 * [1444] Number of Ways of Cutting a Pizza
 */

// @lc code=start
impl Solution {
    pub fn ways(pizza: Vec<String>, k: i32) -> i32 {
        let (m, n, k) = (pizza.len(), pizza[0].len(), k as usize);
        let mut dp = vec![vec![vec![0; k]; n]; m];
        let mut nums = vec![vec![0; n + 1]; m + 1];
        for i in (0..m).rev() {
            for j in (0..n).rev() {
                nums[i][j] += nums[i + 1][j] + nums[i][j + 1] - nums[i + 1][j + 1]
                    + if pizza[i].as_bytes()[j] == b'A' { 1 } else { 0 };
            }
        }
        for i in (0..m).rev() {
            for j in (0..n).rev() {
                if nums[i][j] > 0 {
                    dp[i][j][0] = 1;
                }
                for p in 1..k {
                    for r in i + 1..m {
                        if nums[i][j] - nums[r][j] > 0 {
                            dp[i][j][p] += dp[r][j][p - 1];
                            dp[i][j][p] %= 1_000_000_007;
                        }
                    }
                    for r in j + 1..n {
                        if nums[i][j] - nums[i][r] > 0 {
                            dp[i][j][p] += dp[i][r][p - 1];
                            dp[i][j][p] %= 1_000_000_007;
                        }
                    }
                }
            }
        }
        dp[0][0][k - 1]
    }
}
// @lc code=end
