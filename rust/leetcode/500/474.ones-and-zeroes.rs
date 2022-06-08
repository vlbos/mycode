/*
 * @lc app=leetcode id=474 lang=rust
 *
 * [474] Ones and Zeroes
 */

// @lc code=start
impl Solution {
    pub fn find_max_form(strs: Vec<String>, m: i32, n: i32) -> i32 {
        let l=strs.len();
        let m = m as usize;
        let n = n as usize;
        let mut dp =vec![vec![vec![0;n+1];m+1];l+1];
        for i in 1..=l{
             let mut zerosones= vec![0;2];
              for b in strs[i-1].bytes(){
                    zerosones[(b-b'0') as usize]+=1;
              }
              for j in 0..=m{
                    for k in 0..=n{
                        dp[i][j][k]=dp[i-1][j][k];
                        if j>=zerosones[0]&&k>=zerosones[1] {
                            dp[i][j][k]=dp[i][j][k].max(dp[i-1][j-zerosones[0]][k-zerosones[1]]+1);
                        }
                    }
              }
        }
        dp[l][m][n]
    }
}
// @lc code=end

