/*
 * @lc app=leetcode id=818 lang=rust
 *
 * [818] Race Car
 */

// @lc code=start
impl Solution {
    pub fn racecar(target: i32) -> i32 {
        let mut dp = vec![0, 1, 4];
        let tt = target as usize;
        dp.extend(vec![usize::MAX; tt]);
        for t in 3..=tt {
            let k = format!("{:b}", t).len();
            if t == (1 << k) - 1 {
                dp[t] = k;
                continue;
            }
            for j in 0..k - 1 {
                dp[t] = dp[t].min(dp[t - (1 << (k - 1)) + (1 << j)] + k - 1 + j + 2);
            }
            if (1 << k) - 1 - t < t {
                dp[t] = dp[t].min(dp[(1 << k) - 1 - t] + k + 1);
            }
        }
        dp[tt] as _
    }
}
// @lc code=end
