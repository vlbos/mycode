/*
 * @lc app=leetcode id=650 lang=rust
 *
 * [650] 2 Keys Keyboard
 */

// @lc code=start
impl Solution {
    pub fn min_steps(n: i32) -> i32 {
        let n = n as usize;
        let mut dp =vec![0;n +1];
        for i in 2..=n{
            dp[i]=i32::MAX;
            let mut j = 1;
            while j*j<=i{
                if i%j==0{
                    dp[i]=dp[i].min(dp[j]+(i/j) as i32);
                    dp[i]=dp[i].min(dp[i/j]+j as i32);
                }
                j+=1;
            }
        }
        dp[n]
    }
}
// @lc code=end

