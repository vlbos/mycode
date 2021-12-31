/*
 * @lc app=leetcode id=576 lang=rust
 *
 * [576] Out of Boundary Paths
 */

// @lc code=start
impl Solution {
    pub fn find_paths(m: i32, n: i32, max_move: i32, start_row: i32, start_column: i32) -> i32 {
        let um = m as usize;
        let un= n  as usize;
        let MOD=1000000007;
        let ds = vec![vec![-1,0],vec![1,0],vec![0,-1],vec![0,1]];
        let mut dp=vec![vec![0;un];um];
        dp[start_row as usize][start_column as usize]=1;
        let mut ans = 0;
        for i in 0..max_move{
            let mut dpnew = vec![vec![0;un];um];
            for j in 0..um{
                for k in 0..un{
                    let count = dp[j][k];
                    if count>0{
                          for  d in &ds{
                            let j1= j as i32+d[0];
                            let k1=k as i32+d[1];
                            if j1<0||j1==m||k1<0||k1==n{
                                    ans= (ans+count)%MOD;
                            }
                            else{
                                let uj1 = j1 as usize;
                                let uk1 = k1 as usize;
                                dpnew[uj1][uk1] = (dpnew[uj1][uk1]+count)%MOD;
                            }
                          }
                    }
                }
            }
            dp=dpnew;
        }
        ans
    }
}
// @lc code=end

