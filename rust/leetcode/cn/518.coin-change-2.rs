/*
 * @lc app=leetcode id=518 lang=rust
 *
 * [518] Coin Change 2
 */

// @lc code=start
impl Solution {
    pub fn change(amount: i32, coins: Vec<i32>) -> i32 {
        let a = amount as usize;
        let mut dp = vec![0;a+1];
        dp[0]=1;
        for c in coins{
            for i in c..=amount{
                dp[i as usize]+=dp[(i-c) as usize];
            }
        }
        dp[a]
    }
}
// @lc code=end

