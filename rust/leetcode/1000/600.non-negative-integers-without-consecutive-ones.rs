/*
 * @lc app=leetcode id=600 lang=rust
 *
 * [600] Non-negative Integers without Consecutive Ones
 */

// @lc code=start
impl Solution {
    pub fn find_integers(n: i32) -> i32 {
        let mut dp = vec![0; 31];
        dp[0] = 1;
        dp[1] = 1;
        let mut ans = 0;
        let mut pre = 0;
        for i in 2..31 {
            dp[i] = dp[i - 1] + dp[i - 2];
        }
        for i in (0..30).rev() {
            let val = (1 << i);
            if n & val > 0 {
                ans += dp[i + 1];
                if pre == 1 {
                    break;
                }
                pre = 1;
            } else {
                pre = 0;
            }
            if i == 0 {
                ans += 1;
            }
        }
        ans
    }
}
// @lc code=end
