/*
 * @lc app=leetcode id=583 lang=rust
 *
 * [583] Delete Operation for Two Strings
 */

// @lc code=start
impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let w1  = word1.len();
        let w2= word2.len();
        let mut dp =vec![vec![0;w2+1];w1+1];
        for i in 1..=w1{
            dp[i][0]=i;
        }
        for i in 1..=w2{
            dp[0][i]=i;
        }
        for i in 1..=w1{
            let c1 = word1.chars().nth(i-1).unwrap();
            for j in 1..=w2{
                 let c2 = word2.chars().nth(j-1).unwrap();
                 if c1==c2{
                    dp[i][j]=dp[i-1][j-1];
                } else{
                    dp[i][j]=dp[i-1][j].min(dp[i][j-1])+1;
                }
            }
        }
        dp[w1][w2] as i32
    }
}
// @lc code=end

