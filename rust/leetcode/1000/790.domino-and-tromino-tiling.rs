/*
 * @lc app=leetcode id=790 lang=rust
 *
 * [790] Domino and Tromino Tiling
 */

// @lc code=start
impl Solution {
    pub fn num_tilings(n: i32) -> i32 {
        let inf = 1_000_000_007;
        let mut dp=vec![1i64,0,0,0];
        for _ in 0..n{
            let mut ndp=vec![0i64,0,0,0]; 
            ndp[0]=(dp[0]+dp[3])%inf;
            ndp[1]=(dp[0]+dp[2])%inf;
            ndp[2]=(dp[0]+dp[1])%inf;
            ndp[3]=(dp[0]+dp[1]+dp[2])%inf;
            dp=ndp;
        }
        dp[0] as _
    }
}
// @lc code=end

