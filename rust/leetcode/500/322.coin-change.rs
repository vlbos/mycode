/*
 * @lc app=leetcode id=322 lang=rust
 *
 * [322] Coin Change
 */

// @lc code=start
impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        let a = amount  as usize;
        let mut dp = vec![amount+1;a+1];
        dp[0]=0;
        for i in 1..=a{
                for c in &coins{
                     if *c<=i as i32{
                            dp[i]=dp[i].min(dp[i-*c as usize]+1 );
                    }
                }
        }
        if dp[a]>amount {
                return -1;
        }
        dp[a]
        }
}
// @lc code=end

