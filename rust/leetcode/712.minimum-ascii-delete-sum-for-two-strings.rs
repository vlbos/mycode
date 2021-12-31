/*
 * @lc app=leetcode id=712 lang=rust
 *
 * [712] Minimum ASCII Delete Sum for Two Strings
 */

// @lc code=start
impl Solution {
    pub fn minimum_delete_sum(s1: String, s2: String) -> i32 {
        let n1 = s1.len();
        let n2 = s2.len();
        let mut dp = vec![vec![0;n2+1];n1+1];
        let sv1=s1.bytes().collect::<Vec<u8>>();
        let sv2=s2.bytes().collect::<Vec<u8>>();
        for i in (0..n1).rev(){
            dp[i][n2]=dp[i+1][n2]+sv1[i] as i32;
        }
        for i in (0..n2).rev(){
            dp[n1][i]=dp[n1][i+1]+sv2[i] as i32;
        }
        for i in (0..n1).rev(){
            for j in (0..n2).rev(){
                    if sv1[i]==sv2[j]{
                        dp[i][j]=dp[i+1][j+1];
                    }else{
                        dp[i][j]=(dp[i+1][j]+sv1[i] as i32).min(dp[i][j+1]+sv2[j] as i32);
                    }
            }
        }
        dp[0][0]
    }
}
// @lc code=end

