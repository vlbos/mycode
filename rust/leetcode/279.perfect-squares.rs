/*
 * @lc app=leetcode id=279 lang=rust
 *
 * [279] Perfect Squares
 */

// @lc code=start
impl Solution {
    pub fn num_squares(n: i32) -> i32 {
        let n = n as usize;
        let mut dp =vec![0;n+1];
        for i in 1..=n{
            let mut min =i32::MAX;
            let mut j = 1;
            while j*j<=i{
                min = min.min(dp[i-j*j]);
                j+=1;
            }
            dp[i]=min+1;
        }
        dp[n]
    }
}
// @lc code=end

